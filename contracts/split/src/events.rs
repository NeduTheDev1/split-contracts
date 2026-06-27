use soroban_sdk::{symbol_short, Address, Bytes, Env, String, Symbol, Vec};
use crate::types::TimelockAction;

/// Current contract event schema version.
///
/// Indexer migration guide:
/// When incrementing `EVENTS_SCHEMA_VERSION`, indexers must handle the new version
/// by branching on `event.data[0]` (the `v` field). Older events will carry the
/// previous version number; newer events will carry the incremented version.
/// Always verify that the schema version is understood before parsing the rest of
/// the event data payload to avoid deserialization errors.
pub const EVENTS_SCHEMA_VERSION: u32 = 1;

/// Emitted when a new invoice is created.
/// Topics: (split, created, invoice_id)
/// Data: (v, creator, total)
pub fn invoice_created(env: &Env, invoice_id: u64, creator: &Address, total: i128, cross_chain_ref: &Option<String>) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("created"), invoice_id),
        (EVENTS_SCHEMA_VERSION, creator.clone(), total, cross_chain_ref.clone()),
    );
}

/// Emitted when a payment is received toward an invoice.
/// Topics: (split, paid, invoice_id)
/// Data: (v, payer, amount)
pub fn payment_received(env: &Env, invoice_id: u64, payer: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("paid"), invoice_id),
        (EVENTS_SCHEMA_VERSION, payer.clone(), amount),
    );
}

/// Emitted when an invoice is fully funded and funds are released.
/// Topics: (split, released, invoice_id)
/// Data: (v, recipients)
pub fn invoice_released(env: &Env, invoice_id: u64, recipients: &Vec<Address>) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("released"), invoice_id),
        (EVENTS_SCHEMA_VERSION, recipients.clone()),
    );
}

/// Emitted when an invoice is refunded after deadline.
/// Topics: (split, refunded, invoice_id)
/// Data: (v)
pub fn invoice_refunded(env: &Env, invoice_id: u64) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("refunded"), invoice_id),
        (EVENTS_SCHEMA_VERSION,),
    );
}

/// Emitted once per payer when their refund is transferred.
/// Topics: (split, pay_ref, invoice_id)
/// Data: (v, payer, amount)
pub fn payer_refunded(env: &Env, invoice_id: u64, payer: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("pay_ref"), invoice_id),
        (EVENTS_SCHEMA_VERSION, payer.clone(), amount),
    );
}

/// Emitted when a recipient is added to a pending invoice.
/// Topics: (split, add_rec, invoice_id)
/// Data: (v, recipient, amount)
pub fn recipient_added(env: &Env, invoice_id: u64, recipient: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("add_rec"), invoice_id),
        (EVENTS_SCHEMA_VERSION, recipient.clone(), amount),
    );
}

/// Emitted when the creator adjusts recipient split amounts.
/// Topics: (split, adj_spl, invoice_id)
/// Data: (v, creator)
pub fn split_adjusted(env: &Env, invoice_id: u64, creator: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("adj_spl"), invoice_id),
        (EVENTS_SCHEMA_VERSION, creator.clone()),
    );
}

/// Emitted when an invoice is archived to instance storage.
/// Topics: (split, archived, invoice_id)
/// Data: (v)
pub fn invoice_archived(env: &Env, invoice_id: u64) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("archived"), invoice_id),
        (EVENTS_SCHEMA_VERSION,),
    );
}

/// Emitted when a delegate is assigned to an invoice.
/// Topics: (split, delegated, invoice_id)
/// Data: (v, delegate)
pub fn delegate_set(env: &Env, invoice_id: u64, delegate: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("delegated"), invoice_id),
        (EVENTS_SCHEMA_VERSION, delegate.clone()),
    );
}

/// Emitted when a delegate is revoked from an invoice.
/// Topics: (split, revoked, invoice_id)
/// Data: (v)
pub fn delegate_revoked(env: &Env, invoice_id: u64) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("revoked"), invoice_id),
        (EVENTS_SCHEMA_VERSION,),
    );
}

/// Emitted when NFT gate is set.
/// Topics: (split, nft_gate)
/// Data: (v, contract, admin)
pub fn nft_gate_set(env: &Env, contract: &Option<Address>, admin: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("nft_gate")),
        (EVENTS_SCHEMA_VERSION, contract.clone(), admin.clone()),
    );
}

/// Emitted when a timelock action is queued.
/// Topics: (split, action_q, action_id)
/// Data: (v, action, admin)
pub fn action_queued(env: &Env, action_id: u64, action: &TimelockAction, admin: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("action_q"), action_id),
        (EVENTS_SCHEMA_VERSION, action.clone(), admin.clone()),
    );
}

/// Emitted when a timelock action is executed.
/// Topics: (split, action_e, action_id)
/// Data: (v, action)
pub fn action_executed(env: &Env, action_id: u64, action: &TimelockAction) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("action_e"), action_id),
        (EVENTS_SCHEMA_VERSION, action.clone()),
    );
}

/// Emitted when an invoice is partially released.
/// Topics: (split, part_rel, invoice_id)
/// Data: (v, recipients)
pub fn invoice_partially_released(env: &Env, invoice_id: u64, recipients: &Vec<Address>) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("part_rel"), invoice_id),
        (EVENTS_SCHEMA_VERSION, recipients.clone()),
    );
}

/// Emitted when a timelock action is cancelled.
/// Topics: (split, action_c, action_id)
/// Data: (v, action, admin)
pub fn action_cancelled(env: &Env, action_id: u64, action: &TimelockAction, admin: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("action_c"), action_id),
        (EVENTS_SCHEMA_VERSION, action.clone(), admin.clone()),
    );
}

/// Emitted when an invoice is admin frozen.
/// Topics: (split, admin_frz, invoice_id)
/// Data: (v, admin, reason)
pub fn invoice_admin_frozen(env: &Env, invoice_id: u64, admin: &Address, reason: &String) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("admin_frz"), invoice_id),
        (EVENTS_SCHEMA_VERSION, admin.clone(), reason.clone()),
    );
}

/// Emitted when an invoice is admin unfrozen.
/// Topics: (split, adm_unfrz, invoice_id)
/// Data: (v, admin)
pub fn invoice_admin_unfrozen(env: &Env, invoice_id: u64, admin: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("adm_unfrz"), invoice_id),
        (EVENTS_SCHEMA_VERSION, admin.clone()),
    );
}

/// Emitted when a partial refund is issued.
/// Topics: (split, part_ref, invoice_id)
/// Data: (v, creator, bps, amount)
pub fn partial_refund_issued(env: &Env, invoice_id: u64, creator: &Address, bps: u32, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("part_ref"), invoice_id),
        (EVENTS_SCHEMA_VERSION, creator.clone(), bps, amount),
    );
}

/// Emitted when invoices are batch archived.
/// Topics: (split, bat_arch)
/// Data: (v, count, ids)
pub fn batch_archived(env: &Env, count: u64, ids: &Vec<u64>) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("bat_arch")),
        (EVENTS_SCHEMA_VERSION, count, ids.clone()),
    );
}

/// Emitted when a payment reminder is triggered.
/// Topics: (split, reminder, invoice_id)
/// Data: (v, who)
pub fn payment_reminder(env: &Env, invoice_id: u64, who: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("reminder"), invoice_id),
        (EVENTS_SCHEMA_VERSION, who.clone()),
    );
}

/// Emitted when a payment is matched via memo.
/// Topics: (split, matched, invoice_id)
/// Data: (v, memo, payer)
pub fn payment_matched(env: &Env, invoice_id: u64, memo: u64, payer: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("matched"), invoice_id),
        (EVENTS_SCHEMA_VERSION, memo, payer.clone()),
    );
}

/// Emitted when an invoice is cloned.
/// Topics: (cloned, source_id, new_id)
/// Data: (v)
pub fn invoice_cloned(env: &Env, source_id: u64, new_id: u64) {
    env.events().publish(
        (symbol_short!("cloned"), source_id, new_id),
        (EVENTS_SCHEMA_VERSION,),
    );
}

/// Emitted when an invoice is paused.
/// Topics: (split, paused, invoice_id)
/// Data: (v, creator, reason, auto_resume_at)
pub fn invoice_paused(
    env: &Env,
    invoice_id: u64,
    creator: &Address,
    reason: &String,
    auto_resume_at: &Option<u64>,
) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("paused"), invoice_id),
        (EVENTS_SCHEMA_VERSION, creator.clone(), reason.clone(), *auto_resume_at),
    );
}

/// Emitted when an invoice is resumed.
/// Topics: (split, resumed, invoice_id)
/// Data: (v, creator)
pub fn invoice_resumed(env: &Env, invoice_id: u64, creator: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("resumed"), invoice_id),
        (EVENTS_SCHEMA_VERSION, creator.clone()),
    );
}

/// Emitted when an invoice is force resumed.
/// Topics: (split, forced, invoice_id)
/// Data: (v, admin_addr)
pub fn invoice_force_resumed(env: &Env, invoice_id: u64, admin_addr: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("forced"), invoice_id),
        (EVENTS_SCHEMA_VERSION, admin_addr.clone()),
    );
}

/// Emitted when a pending payout is claimed by a recipient (issue #209).
/// Topics: (split, pend_pay, invoice_id)
/// Data: (v, recipient, amount)
pub fn pending_payout_claimed(env: &Env, invoice_id: u64, recipient: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("pend_pay"), invoice_id),
        (EVENTS_SCHEMA_VERSION, recipient.clone(), amount),
    );
}

/// Emitted at the start of every public entry point for real-time contract health observability.
///
/// Topic: `(symbol_short!("monitor"), function_name)`
/// Data:  `(v, invoice_id, actor_address, ledger_timestamp)`
pub fn monitor_event(env: &Env, function: Symbol, invoice_id: u64, actor: &Address, timestamp: u64) {
    env.events().publish(
        (symbol_short!("monitor"), function),
        (EVENTS_SCHEMA_VERSION, invoice_id, actor.clone(), timestamp),
    );
}

/// Emitted when an emergency withdrawal is executed.
/// Topics: (split, emrg_wd)
/// Data: (v, token, destination, amount)
pub fn emergency_withdrawal_executed(env: &Env, token: &Address, destination: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("emrg_wd")),
        (EVENTS_SCHEMA_VERSION, token.clone(), destination.clone(), amount),
    );
}

/// Replayed invoice_created event tagged with "replay" so indexers can distinguish it.
/// Topics: (split, created, invoice_id, replay)
/// Data: (v, creator, total, cross_chain_ref)
pub fn replay_invoice_created(env: &Env, invoice_id: u64, creator: &Address, total: i128, cross_chain_ref: &Option<soroban_sdk::String>) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("created"), invoice_id, symbol_short!("replay")),
        (EVENTS_SCHEMA_VERSION, creator.clone(), total, cross_chain_ref.clone()),
    );
}

/// Replayed payment_received event tagged with "replay".
/// Topics: (split, paid, invoice_id, replay)
/// Data: (v, payer, amount)
pub fn replay_payment_received(env: &Env, invoice_id: u64, payer: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("paid"), invoice_id, symbol_short!("replay")),
        (EVENTS_SCHEMA_VERSION, payer.clone(), amount),
    );
}

/// Replayed invoice_released event tagged with "replay".
/// Topics: (split, released, invoice_id, replay)
/// Data: (v, recipients)
pub fn replay_invoice_released(env: &Env, invoice_id: u64, recipients: &Vec<Address>) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("released"), invoice_id, symbol_short!("replay")),
        (EVENTS_SCHEMA_VERSION, recipients.clone()),
    );
}

/// Replayed invoice_refunded event tagged with "replay".
/// Topics: (split, refunded, invoice_id, replay)
/// Data: (v)
pub fn replay_invoice_refunded(env: &Env, invoice_id: u64) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("refunded"), invoice_id, symbol_short!("replay")),
        (EVENTS_SCHEMA_VERSION,),
    );
}


/// Emitted when a recipient is substituted (Issue #230).
/// Topics: (split, sub_rec, invoice_id)
/// Data: (v, old_recipient, new_recipient)
pub fn recipient_updated(env: &Env, invoice_id: u64, old_recipient: &Address, new_recipient: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("sub_rec"), invoice_id),
        (EVENTS_SCHEMA_VERSION, old_recipient.clone(), new_recipient.clone()),
    );
}

/// Emitted when a refund encounters insufficient balance and partial refunds are distributed.
/// Topics: (split, ref_short, invoice_id)
/// Data: (v, shortfall_amount)
pub fn refund_shortfall(env: &Env, invoice_id: u64, shortfall_amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("ref_short"), invoice_id),
        (EVENTS_SCHEMA_VERSION, shortfall_amount),
    );
}

/// Emitted when an escrow is migrated to a new token contract.
/// Topics: (split, migrated)
/// Data: (v, total, new_contract)
pub fn escrow_migrated(env: &Env, total: i128, new_contract: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("migrated")),
        (EVENTS_SCHEMA_VERSION, total, new_contract.clone()),
    );
}

/// Emitted when a third-party pays on behalf of a beneficiary (issue #277).
/// Topics: (split, del_pay, invoice_id)
/// Data: (payer, beneficiary, amount)
pub fn delegated_payment_received(env: &Env, invoice_id: u64, payer: &Address, beneficiary: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("del_pay"), invoice_id),
        (payer.clone(), beneficiary.clone(), amount),
    );
}

/// Emitted when a contract migration is executed (issue #279).
/// Topics: (split, migrated)
/// Data: (from_version, to_version, ledger)
pub fn contract_migrated(env: &Env, from_version: u32, to_version: u32, ledger: u32) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("migrated")),
        (from_version, to_version, ledger),
    );
}
