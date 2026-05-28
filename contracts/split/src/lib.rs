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

use soroban_sdk::{contract, contractimpl, symbol_short, token, Address, Bytes, Env, Symbol, Vec};
use types::{AuditEntry, CompletionProof, Invoice, InvoiceStatus, Payment, SubscriptionParams};

// ---------------------------------------------------------------------------
// Storage helpers
// ---------------------------------------------------------------------------

fn counter_key() -> Symbol {
    symbol_short!("counter")
}

fn invoice_key(id: u64) -> (Symbol, u64) {
    (symbol_short!("inv"), id)
}

fn admin_key() -> Symbol {
    symbol_short!("admin")
}

fn paused_key() -> Symbol {
    symbol_short!("paused")
}

fn load_invoice(env: &Env, id: u64) -> Invoice {
    env.storage()
        .persistent()
        .get(&invoice_key(id))
        .expect("invoice not found")
}

fn save_invoice(env: &Env, id: u64, invoice: &Invoice) {
    env.storage().persistent().set(&invoice_key(id), invoice);
}

fn audit_log_key(id: u64) -> (Symbol, u64) {
    (symbol_short!("log"), id)
}

fn subscription_params_key(id: u64) -> (Symbol, u64) {
    (symbol_short!("sub"), id)
}

fn append_audit_entry(env: &Env, id: u64, action: Symbol, actor: &Address) {
    let timestamp = env.ledger().timestamp();
    let entry = AuditEntry { action, actor: actor.clone(), timestamp };
    let mut log: Vec<AuditEntry> = env
        .storage()
        .persistent()
        .get(&audit_log_key(id))
        .unwrap_or_else(|| Vec::new(env));
    log.push_back(entry);
    env.storage().persistent().set(&audit_log_key(id), &log);
}

pub fn get_audit_log(env: &Env, id: u64) -> Vec<AuditEntry> {
    env.storage()
        .persistent()
        .get(&audit_log_key(id))
        .unwrap_or_else(|| Vec::new(env))
}

fn is_paused(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get(&paused_key())
        .unwrap_or(false)
}

fn require_not_paused(env: &Env) {
    assert!(!is_paused(env), "contract is paused");
}

fn require_admin(env: &Env, caller: &Address) {
    let admin: Address = env
        .storage()
        .persistent()
        .get(&admin_key())
        .expect("admin not set");
    assert!(admin == *caller, "caller is not admin");
    caller.require_auth();
}

// ---------------------------------------------------------------------------
// Contract
// ---------------------------------------------------------------------------

#[contract]
pub struct SplitContract;

#[contractimpl]
impl SplitContract {
    // -----------------------------------------------------------------------
    // Admin / pause
    // -----------------------------------------------------------------------

    /// Initialise the contract with an admin address. Must be called once.
    pub fn initialize(env: Env, admin: Address) {
        assert!(
            !env.storage().persistent().has(&admin_key()),
            "already initialized"
        );
        admin.require_auth();
        env.storage().persistent().set(&admin_key(), &admin);
    }

    /// Pause all state-changing operations. Requires admin auth.
    pub fn pause(env: Env, admin: Address) {
        require_admin(&env, &admin);
        env.storage().persistent().set(&paused_key(), &true);
    }

    /// Unpause the contract. Requires admin auth.
    pub fn unpause(env: Env, admin: Address) {
        require_admin(&env, &admin);
        env.storage().persistent().set(&paused_key(), &false);
    }

    // -----------------------------------------------------------------------
    // Invoice lifecycle
    // -----------------------------------------------------------------------

    /// Create a new invoice.
    ///
    /// # Arguments
    /// * `creator`          – address that owns the invoice (must authorise)
    /// * `recipients`       – ordered list of recipient addresses
    /// * `amounts`          – amount owed to each recipient (parallel to `recipients`)
    /// * `token`            – USDC token contract address
    /// * `deadline`         – Unix timestamp; after this refunds become available
    /// * `bonus_pool`       – optional creator-funded bonus (0 = disabled)
    /// * `bonus_max_payers` – number of early payers that share the bonus
    /// * `metadata`         – optional IPFS CID or arbitrary bytes
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
        bonus_pool: i128,
        bonus_max_payers: u32,
        metadata: Option<Bytes>,
    ) -> u64 {
        require_not_paused(&env);
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
        assert!(bonus_pool >= 0, "bonus_pool must be non-negative");

        for amt in amounts.iter() {
            assert!(amt > 0, "amounts must be positive");
        }

        let id: u64 = env
            .storage()
            .persistent()
            .get(&counter_key())
            .unwrap_or(0u64)
            + 1;
        env.storage().persistent().set(&counter_key(), &id);

        let total: i128 = amounts.iter().sum();

        // Deposit bonus pool from creator if non-zero.
        if bonus_pool > 0 {
            let token_client = token::Client::new(&env, &token);
            token_client.transfer(&creator, &env.current_contract_address(), &bonus_pool);
        }

        let invoice = Invoice {
            creator: creator.clone(),
            recipients: recipients.clone(),
            amounts,
            token,
            deadline,
            funded: 0,
            status: InvoiceStatus::Pending,
            payments: Vec::new(&env),
            bonus_pool,
            bonus_max_payers,
            metadata: metadata.clone(),
        };

        save_invoice(&env, id, &invoice);
        events::invoice_created(&env, id, &creator, total, &metadata);

        id
    }

    /// Create a subscription chain of invoices for recurring monthly billing.
    pub fn create_subscription(
        env: Env,
        creator: Address,
        recipients: Vec<Address>,
        amounts: Vec<i128>,
        token: Address,
        months: u32,
    ) -> u64 {
        creator.require_auth();

        assert!(
            recipients.len() == amounts.len(),
            "recipients and amounts length mismatch"
        );
        assert!(!recipients.is_empty(), "must have at least one recipient");
        assert!(months > 0 && months <= 12, "months must be between 1 and 12");

        for amt in amounts.iter() {
            assert!(amt > 0, "amounts must be positive");
        }

        let deadline = env.ledger().timestamp() + 30 * 24 * 60 * 60;
        let id = Self::create_invoice(
            env.clone(),
            creator.clone(),
            recipients.clone(),
            amounts.clone(),
            token.clone(),
            deadline,
            0,
            0,
            None,
        );

        if months > 1 {
            let params = SubscriptionParams {
                creator: creator.clone(),
                recipients: recipients.clone(),
                amounts: amounts.clone(),
                token: token.clone(),
            };
            env.storage()
                .persistent()
                .set(&subscription_params_key(id), &params);
        }

        id
    }

    /// Pay toward an invoice.
    pub fn pay(env: Env, payer: Address, invoice_id: u64, amount: i128) {
        require_not_paused(&env);
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

        let token_client = token::Client::new(&env, &invoice.token);
        token_client.transfer(&payer, &env.current_contract_address(), &amount);

        invoice.payments.push_back(Payment { payer: payer.clone(), amount });
        invoice.funded += amount;

        append_audit_entry(&env, invoice_id, symbol_short!("pay"), &payer);
        events::payment_received(&env, invoice_id, &payer, amount);

        if invoice.funded >= total {
            Self::_release(&env, invoice_id, &mut invoice, &invoice.creator.clone());
        } else {
            save_invoice(&env, invoice_id, &invoice);
        }
    }

    /// Release funds to all recipients once the invoice is fully funded.
    pub fn release(env: Env, invoice_id: u64) {
        require_not_paused(&env);
        let caller = env.current_contract_address();
        let mut invoice = load_invoice(&env, invoice_id);

        assert!(
            invoice.status == InvoiceStatus::Pending,
            "invoice is not pending"
        );

        let total: i128 = invoice.amounts.iter().sum();
        assert!(invoice.funded >= total, "invoice not fully funded");

        Self::_release(&env, invoice_id, &mut invoice, &caller);
    }

    /// Refund all payers if the deadline has passed and the invoice is not fully funded.
    pub fn refund(env: Env, invoice_id: u64) {
        require_not_paused(&env);
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

        // Refund unused bonus pool back to creator.
        if invoice.bonus_pool > 0 {
            token_client.transfer(
                &env.current_contract_address(),
                &invoice.creator,
                &invoice.bonus_pool,
            );
        }

        invoice.status = InvoiceStatus::Refunded;
        save_invoice(&env, invoice_id, &invoice);
        let actor = env.current_contract_address();
        append_audit_entry(&env, invoice_id, symbol_short!("refund"), &actor);
        events::invoice_refunded(&env, invoice_id);
    }

    /// Cancel an invoice before any payments are made.
    pub fn cancel_invoice(env: Env, caller: Address, invoice_id: u64) {
        require_not_paused(&env);
        caller.require_auth();

        let mut invoice = load_invoice(&env, invoice_id);

        assert!(
            invoice.status == InvoiceStatus::Pending,
            "invoice is not pending"
        );
        assert!(invoice.creator == caller, "only creator can cancel");
        assert!(invoice.funded == 0, "cannot cancel invoice with payments");

        // Refund bonus pool to creator on cancel.
        if invoice.bonus_pool > 0 {
            let token_client = token::Client::new(&env, &invoice.token);
            token_client.transfer(
                &env.current_contract_address(),
                &invoice.creator,
                &invoice.bonus_pool,
            );
        }

        invoice.status = InvoiceStatus::Cancelled;
        save_invoice(&env, invoice_id, &invoice);
        append_audit_entry(&env, invoice_id, symbol_short!("cancel"), &caller);
    }

    /// Transfer invoice ownership to a new creator.
    ///
    /// Only the current creator can call this, and the invoice must be Pending.
    pub fn transfer_invoice(env: Env, invoice_id: u64, new_creator: Address) {
        require_not_paused(&env);
        let mut invoice = load_invoice(&env, invoice_id);

        assert!(
            invoice.status == InvoiceStatus::Pending,
            "invoice is not pending"
        );

        invoice.creator.require_auth();
        invoice.creator = new_creator;
        save_invoice(&env, invoice_id, &invoice);
    }

    /// Extend the deadline for an invoice.
    pub fn extend_deadline(env: Env, caller: Address, invoice_id: u64, new_deadline: u64) {
        require_not_paused(&env);
        caller.require_auth();

        let mut invoice = load_invoice(&env, invoice_id);

        assert!(
            invoice.status == InvoiceStatus::Pending,
            "invoice is not pending"
        );
        assert!(invoice.creator == caller, "only creator can extend deadline");
        assert!(
            new_deadline > env.ledger().timestamp(),
            "new deadline must be in the future"
        );

        invoice.deadline = new_deadline;
        save_invoice(&env, invoice_id, &invoice);
        append_audit_entry(&env, invoice_id, symbol_short!("extend"), &caller);
    }

    // -----------------------------------------------------------------------
    // Read-only
    // -----------------------------------------------------------------------

    pub fn get_invoice(env: Env, invoice_id: u64) -> Invoice {
        load_invoice(&env, invoice_id)
    }

    pub fn get_audit_log(env: Env, invoice_id: u64) -> Vec<AuditEntry> {
        get_audit_log(&env, invoice_id)
    }

    /// Generate a completion proof for a finalized invoice.
    pub fn get_completion_proof(env: Env, invoice_id: u64) -> CompletionProof {
        let invoice = load_invoice(&env, invoice_id);

        assert!(
            invoice.status == InvoiceStatus::Released || invoice.status == InvoiceStatus::Refunded,
            "invoice not finalized"
        );

        let mut bytes: Vec<u8> = Vec::new(&env);
        bytes.extend_from_slice(&invoice.creator.to_bytes());
        bytes.push(invoice.recipients.len() as u8);
        for r in invoice.recipients.iter() {
            bytes.extend_from_slice(&r.to_bytes());
        }
        bytes.push((invoice.amounts.len() & 0xFF) as u8);
        bytes.push(((invoice.amounts.len() >> 8) & 0xFF) as u8);
        for a in invoice.amounts.iter() {
            bytes.extend_from_slice(&a.to_le_bytes());
        }
        bytes.extend_from_slice(&invoice.token.to_bytes());
        bytes.extend_from_slice(&invoice.deadline.to_le_bytes());
        bytes.extend_from_slice(&invoice.funded.to_le_bytes());
        let s_byte = match invoice.status {
            InvoiceStatus::Pending => 0u8,
            InvoiceStatus::Released => 1u8,
            InvoiceStatus::Refunded => 2u8,
            InvoiceStatus::Cancelled => 3u8,
        };
        bytes.push(s_byte);

        let hash = env.crypto().sha256(&bytes).to_bytes();

        CompletionProof {
            id: invoice_id,
            status: invoice.status,
            funded: invoice.funded,
            timestamp: env.ledger().timestamp(),
            hash,
        }
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn _release(env: &Env, invoice_id: u64, invoice: &mut Invoice, actor: &Address) {
        let token_client = token::Client::new(env, &invoice.token);

        for (recipient, amount) in invoice.recipients.iter().zip(invoice.amounts.iter()) {
            token_client.transfer(&env.current_contract_address(), &recipient, &amount);
        }

        // Distribute bonus pool equally among first `bonus_max_payers` unique payers.
        if invoice.bonus_pool > 0 && invoice.bonus_max_payers > 0 {
            // Collect unique payers in order of first appearance.
            let mut unique_payers: Vec<Address> = Vec::new(env);
            for payment in invoice.payments.iter() {
                let already_seen = unique_payers.iter().any(|p| p == payment.payer);
                if !already_seen {
                    unique_payers.push_back(payment.payer.clone());
                    if unique_payers.len() >= invoice.bonus_max_payers {
                        break;
                    }
                }
            }

            if !unique_payers.is_empty() {
                let n = unique_payers.len() as i128;
                let per_payer = invoice.bonus_pool / n;
                let mut distributed: i128 = 0;
                for (i, payer) in unique_payers.iter().enumerate() {
                    // Give remainder to last payer to avoid dust.
                    let payout = if i as i128 == n - 1 {
                        invoice.bonus_pool - distributed
                    } else {
                        per_payer
                    };
                    token_client.transfer(&env.current_contract_address(), &payer, &payout);
                    distributed += payout;
                }
            }
        }

        invoice.status = InvoiceStatus::Released;
        save_invoice(env, invoice_id, invoice);
        append_audit_entry(env, invoice_id, symbol_short!("release"), actor);
        events::invoice_released(env, invoice_id, &invoice.recipients);

        // Check for subscription params and create next invoice if exists.
        if let Some(params) = env
            .storage()
            .persistent()
            .get::<_, SubscriptionParams>(&subscription_params_key(invoice_id))
        {
            let next_deadline = env.ledger().timestamp() + 30 * 24 * 60 * 60;
            let _next_id = Self::create_invoice(
                env.clone(),
                params.creator.clone(),
                params.recipients.clone(),
                params.amounts.clone(),
                params.token.clone(),
                next_deadline,
                0,
                0,
                None,
            );

            env.storage()
                .persistent()
                .remove(&subscription_params_key(invoice_id));
        }
    }
}
