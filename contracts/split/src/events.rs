use soroban_sdk::{symbol_short, Address, Bytes, Env, Symbol, Vec};

/// Emitted when a new invoice is created.
pub fn invoice_created(env: &Env, invoice_id: u64, creator: &Address, total: i128, metadata: &Option<Bytes>) {
    env.events().publish(
        (symbol_short!("inv_crt"), invoice_id),
        (creator.clone(), total, metadata.clone()),
    );
}

/// Emitted when a payment is received toward an invoice.
pub fn payment_received(env: &Env, invoice_id: u64, payer: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("inv_pay"), invoice_id),
        (payer.clone(), amount),
    );
}

/// Emitted when an invoice is fully funded and funds are released.
pub fn invoice_released(env: &Env, invoice_id: u64, recipients: &Vec<Address>) {
    env.events().publish(
        (symbol_short!("inv_rel"), invoice_id),
        recipients.clone(),
    );
}

/// Emitted when an invoice is refunded after deadline.
pub fn invoice_refunded(env: &Env, invoice_id: u64) {
    env.events()
        .publish((symbol_short!("inv_ref"), invoice_id), ());
}

/// Emitted once per unique payer when their refund is transferred.
pub fn payer_refunded(env: &Env, invoice_id: u64, payer: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("pay_ref"), invoice_id),
        (payer.clone(), amount),
    );
}

/// Emitted when a recipient is added to a pending invoice.
pub fn recipient_added(env: &Env, invoice_id: u64, recipient: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("add_rec"), invoice_id),
        (recipient.clone(), amount),
    );
}

/// Emitted at the start of every public entry point for real-time contract health observability.
///
/// Topic: `(symbol_short!("monitor"), function_name)`
/// Data:  `(invoice_id, actor_address, ledger_timestamp)`
pub fn monitor_event(env: &Env, function: Symbol, invoice_id: u64, actor: &Address, timestamp: u64) {
    env.events().publish(
        (symbol_short!("monitor"), function),
        (invoice_id, actor.clone(), timestamp),
    );
}
