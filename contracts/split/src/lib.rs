//! StellarSplit — on-chain invoice & payment splitting contract.
//!
//! Allows a creator to define an invoice with multiple recipients and amounts.
//! Payers contribute funds; once fully funded the contract auto-routes USDC to
//! each recipient. If the deadline passes unfunded, payers are refunded.

#![no_std]

mod events;
mod types;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, symbol_short, token, Address, Env, Symbol, Vec};
use types::{Invoice, InvoiceCore, InvoiceExt, InvoiceOptions, InvoiceStatus, Payment};

const PAYMENT_WINDOW_CAP: u32 = 50;

// ---------------------------------------------------------------------------
// Storage helpers
// ---------------------------------------------------------------------------

/// Storage key for the auto-incrementing invoice counter.
fn counter_key() -> Symbol {
    symbol_short!("counter")
}

/// Composite storage key for an invoice: (symbol, id).
fn invoice_key(id: u64) -> (Symbol, u64) {
    (symbol_short!("inv"), id)
}

fn invoice_ext_key(id: u64) -> (Symbol, u64) {
    (symbol_short!("invext"), id)
}

fn payer_cooldown_key(id: u64, payer: Address) -> (Symbol, u64, Address) {
    (symbol_short!("pcd"), id, payer)
}

fn payment_window_key(id: u64) -> (Symbol, u64) {
    (symbol_short!("pwin"), id)
}

fn load_invoice(env: &Env, id: u64) -> InvoiceCore {
    env.storage()
        .persistent()
        .get(&invoice_key(id))
        .expect("invoice not found")
}

fn save_invoice(env: &Env, id: u64, invoice: &InvoiceCore) {
    env.storage()
        .persistent()
        .set(&invoice_key(id), invoice);
}

fn save_invoice_ext(env: &Env, id: u64, ext: &InvoiceExt) {
    env.storage()
        .persistent()
        .set(&invoice_ext_key(id), ext);
}

fn load_invoice_ext(env: &Env, id: u64) -> InvoiceExt {
    env.storage()
        .persistent()
        .get(&invoice_ext_key(id))
        .expect("invoice extension not found")
}

// ---------------------------------------------------------------------------
// Contract
// ---------------------------------------------------------------------------

#[contract]
pub struct SplitContract;

#[contractimpl]
impl SplitContract {
    /// Create a new invoice.
    ///
    /// # Arguments
    /// * `creator`    – address that owns the invoice (must authorise)
    /// * `recipients` – ordered list of recipient addresses
    /// * `amounts`    – amount owed to each recipient (parallel to `recipients`)
    /// * `token`      – USDC token contract address
    /// * `deadline`   – Unix timestamp; after this refunds become available
    ///
    /// # Returns
    /// The new invoice ID (monotonically increasing u64).
    pub fn create_invoice(
        env: Env,
        creator: Address,
        recipients: Vec<Address>,
        amounts: Vec<i128>,
        token: Address,
        deadline: u64,
    ) -> u64 {
        Self::create_invoice_with_options(
            env,
            creator,
            recipients,
            amounts,
            token,
            deadline,
            InvoiceOptions {
                payment_cooldown_secs: None,
                max_payments_per_window: None,
                payment_window_secs: None,
            },
        )
    }

    /// Create a new invoice with optional payment rate limits.
    pub fn create_invoice_with_options(
        env: Env,
        creator: Address,
        recipients: Vec<Address>,
        amounts: Vec<i128>,
        token: Address,
        deadline: u64,
        options: InvoiceOptions,
    ) -> u64 {
        creator.require_auth();

        assert!(
            recipients.len() == amounts.len(),
            "recipients and amounts length mismatch"
        );
        assert!(!recipients.is_empty(), "must have at least one recipient");
        assert!(
            deadline > env.ledger().timestamp(),
            "deadline must be in the future"
        );

        for amt in amounts.iter() {
            assert!(amt > 0, "amounts must be positive");
        }

        // Increment and persist the invoice counter.
        let id: u64 = env
            .storage()
            .persistent()
            .get(&counter_key())
            .unwrap_or(0u64)
            + 1;
        env.storage().persistent().set(&counter_key(), &id);

        let total: i128 = amounts.iter().sum();

        let invoice = InvoiceCore {
            creator: creator.clone(),
            recipients: recipients.clone(),
            amounts,
            token,
            deadline,
            funded: 0,
            status: InvoiceStatus::Pending,
            payments: Vec::new(&env),
        };

        save_invoice(&env, id, &invoice);
        save_invoice_ext(&env, id, &InvoiceExt::from(options));
        events::invoice_created(&env, id, &creator, total);

        id
    }

    /// Pay toward an invoice.
    ///
    /// Transfers `amount` of the invoice token from `payer` to this contract.
    /// Auto-releases funds if the invoice becomes fully funded.
    ///
    /// # Arguments
    /// * `payer`      – address making the payment (must authorise)
    /// * `invoice_id` – target invoice
    /// * `amount`     – amount to pay in stroops
    pub fn pay(env: Env, payer: Address, invoice_id: u64, amount: i128) {
        payer.require_auth();

        let mut invoice = load_invoice(&env, invoice_id);

        assert!(
            invoice.status == InvoiceStatus::Pending,
            "invoice is not pending"
        );
        assert!(
            env.ledger().timestamp() <= invoice.deadline,
            "invoice deadline has passed"
        );
        assert!(amount > 0, "payment amount must be positive");

        let total: i128 = invoice.amounts.iter().sum();
        let remaining = total - invoice.funded;
        assert!(amount <= remaining, "payment exceeds remaining balance");

        let now = env.ledger().timestamp();
        let ext = load_invoice_ext(&env, invoice_id);
        Self::enforce_payment_limits(&env, invoice_id, &payer, &ext, now);

        // Transfer tokens from payer to this contract.
        let token_client = token::Client::new(&env, &invoice.token);
        token_client.transfer(&payer, &env.current_contract_address(), &amount);

        Self::record_payment_limits(&env, invoice_id, &payer, &ext, now);

        invoice.payments.push_back(Payment {
            payer: payer.clone(),
            amount,
        });
        invoice.funded += amount;

        events::payment_received(&env, invoice_id, &payer, amount);

        // Auto-release if fully funded.
        if invoice.funded >= total {
            Self::_release(&env, invoice_id, &mut invoice);
        } else {
            save_invoice(&env, invoice_id, &invoice);
        }
    }

    /// Release funds to all recipients once the invoice is fully funded.
    ///
    /// Can be called by anyone; validates full funding internally.
    pub fn release(env: Env, invoice_id: u64) {
        let mut invoice = load_invoice(&env, invoice_id);

        assert!(
            invoice.status == InvoiceStatus::Pending,
            "invoice is not pending"
        );

        let total: i128 = invoice.amounts.iter().sum();
        assert!(invoice.funded >= total, "invoice not fully funded");

        Self::_release(&env, invoice_id, &mut invoice);
    }

    /// Refund all payers if the deadline has passed and the invoice is not fully funded.
    ///
    /// Can be called by anyone after the deadline.
    pub fn refund(env: Env, invoice_id: u64) {
        let mut invoice = load_invoice(&env, invoice_id);

        assert!(
            invoice.status == InvoiceStatus::Pending,
            "invoice is not pending"
        );
        assert!(
            env.ledger().timestamp() > invoice.deadline,
            "deadline has not passed"
        );

        let token_client = token::Client::new(&env, &invoice.token);

        for payment in invoice.payments.iter() {
            token_client.transfer(
                &env.current_contract_address(),
                &payment.payer,
                &payment.amount,
            );
        }

        invoice.status = InvoiceStatus::Refunded;
        save_invoice(&env, invoice_id, &invoice);
        events::invoice_refunded(&env, invoice_id);
    }

    /// Retrieve an invoice by ID.
    pub fn get_invoice(env: Env, invoice_id: u64) -> Invoice {
        Invoice::from(load_invoice(&env, invoice_id))
    }

    /// Retrieve extension settings for an invoice by ID.
    pub fn get_invoice_ext(env: Env, invoice_id: u64) -> InvoiceExt {
        load_invoice_ext(&env, invoice_id)
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    /// Route funds to all recipients and mark the invoice as released.
    fn _release(env: &Env, invoice_id: u64, invoice: &mut InvoiceCore) {
        let token_client = token::Client::new(env, &invoice.token);

        for (recipient, amount) in invoice.recipients.iter().zip(invoice.amounts.iter()) {
            token_client.transfer(&env.current_contract_address(), &recipient, &amount);
        }

        invoice.status = InvoiceStatus::Released;
        save_invoice(env, invoice_id, invoice);
        events::invoice_released(env, invoice_id, &invoice.recipients);
    }

    fn enforce_payment_limits(
        env: &Env,
        invoice_id: u64,
        payer: &Address,
        ext: &InvoiceExt,
        now: u64,
    ) {
        if let Some(cooldown_secs) = ext.payment_cooldown_secs {
            let last_payment: Option<u64> = env
                .storage()
                .persistent()
                .get(&payer_cooldown_key(invoice_id, payer.clone()));

            if let Some(last_payment_at) = last_payment {
                assert!(
                    last_payment_at.saturating_add(cooldown_secs) <= now,
                    "payment cooldown active"
                );
            }
        }

        if let (Some(max_payments), Some(window_secs)) =
            (ext.max_payments_per_window, ext.payment_window_secs)
        {
            let recent = Self::active_payment_window(env, invoice_id, now, window_secs);
            assert!(
                recent.len() < max_payments,
                "payment rate limit exceeded"
            );
        }
    }

    fn record_payment_limits(
        env: &Env,
        invoice_id: u64,
        payer: &Address,
        ext: &InvoiceExt,
        now: u64,
    ) {
        if ext.payment_cooldown_secs.is_some() {
            env.storage()
                .persistent()
                .set(&payer_cooldown_key(invoice_id, payer.clone()), &now);
        }

        if let (Some(_), Some(window_secs)) =
            (ext.max_payments_per_window, ext.payment_window_secs)
        {
            let mut recent = Self::active_payment_window(env, invoice_id, now, window_secs);
            while recent.len() >= PAYMENT_WINDOW_CAP {
                recent.pop_front();
            }
            recent.push_back(now);
            env.storage()
                .persistent()
                .set(&payment_window_key(invoice_id), &recent);
        }
    }

    fn active_payment_window(
        env: &Env,
        invoice_id: u64,
        now: u64,
        window_secs: u64,
    ) -> Vec<u64> {
        let stored: Vec<u64> = env
            .storage()
            .persistent()
            .get(&payment_window_key(invoice_id))
            .unwrap_or(Vec::new(env));
        let mut active = Vec::new(env);

        for paid_at in stored.iter() {
            if paid_at.saturating_add(window_secs) > now {
                active.push_back(paid_at);
            }
        }

        while active.len() > PAYMENT_WINDOW_CAP {
            active.pop_front();
        }

        active
    }
}
