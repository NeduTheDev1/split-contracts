use soroban_sdk::{symbol_short, Address, Env, Vec, String};
use crate::types::TimelockAction;

/// Emitted when a new invoice is created.
/// Topics: (split, created, invoice_id)
/// Data: (creator, total)
pub fn invoice_created(env: &Env, invoice_id: u64, creator: &Address, total: i128, cross_chain_ref: &Option<String>) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("created"), invoice_id),
        (creator.clone(), total, cross_chain_ref.clone()),
    );
}

/// Emitted when a payment is received toward an invoice.
/// Topics: (split, paid, invoice_id)
/// Data: (payer, amount)
pub fn payment_received(env: &Env, invoice_id: u64, payer: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("paid"), invoice_id),
        (payer.clone(), amount),
    );
}

/// Emitted when an invoice is fully funded and funds are released.
/// Topics: (split, released, invoice_id)
/// Data: recipients
pub fn invoice_released(env: &Env, invoice_id: u64, recipients: &Vec<Address>) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("released"), invoice_id),
        recipients.clone(),
    );
}

/// Emitted when an invoice is refunded after deadline.
/// Topics: (split, refunded, invoice_id)
/// Data: ()
pub fn invoice_refunded(env: &Env, invoice_id: u64) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("refunded"), invoice_id),
        (),
    );
}

/// Emitted once per payer when their refund is transferred.
/// Topics: (split, pay_ref, invoice_id)
/// Data: (payer, amount)
pub fn payer_refunded(env: &Env, invoice_id: u64, payer: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("pay_ref"), invoice_id),
        (payer.clone(), amount),
    );
}

/// Emitted when a recipient is added to a pending invoice.
/// Topics: (split, add_rec, invoice_id)
/// Data: (recipient, amount)
pub fn recipient_added(env: &Env, invoice_id: u64, recipient: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("add_rec"), invoice_id),
        (recipient.clone(), amount),
    );
}

/// Emitted when the creator adjusts recipient split amounts.
/// Topics: (split, adj_spl, invoice_id)
/// Data: creator
pub fn split_adjusted(env: &Env, invoice_id: u64, creator: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("adj_spl"), invoice_id),
        creator.clone(),
    );
}

/// Emitted when an invoice is archived to instance storage.
/// Topics: (split, archived, invoice_id)
/// Data: ()
pub fn invoice_archived(env: &Env, invoice_id: u64) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("archived"), invoice_id),
        (),
    );
}

/// Emitted when a delegate is assigned to an invoice.
/// Topics: (split, delegated, invoice_id)
/// Data: delegate
pub fn delegate_set(env: &Env, invoice_id: u64, delegate: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("delegated"), invoice_id),
        delegate.clone(),
    );
}

/// Emitted when a delegate is revoked from an invoice.
/// Topics: (split, revoked, invoice_id)
/// Data: ()
pub fn delegate_revoked(env: &Env, invoice_id: u64) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("revoked"), invoice_id),
        (),
    );
}

/// Emitted when an invoice is partially released.
/// Topics: (split, part_rel, invoice_id)
/// Data: recipients
pub fn invoice_partially_released(env: &Env, invoice_id: u64, recipients: &Vec<Address>) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("part_rel"), invoice_id),
        recipients.clone(),
    );
}

/// Emitted when a payment reminder is triggered.
/// Topics: (split, reminder, invoice_id)
/// Data: who
pub fn payment_reminder(env: &Env, invoice_id: u64, who: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("reminder"), invoice_id),
        who.clone(),
    );
}

/// Emitted when a payment is matched via memo.
/// Topics: (split, matched, invoice_id)
/// Data: (payer, memo)
pub fn payment_matched(env: &Env, invoice_id: u64, memo: u64, payer: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("matched"), invoice_id),
        (memo, payer.clone()),
    );
}

/// Emitted when an invoice is cloned.
/// Topics: (cloned, source_id, new_id)
/// Data: ()
pub fn invoice_cloned(env: &Env, source_id: u64, new_id: u64) {
    env.events().publish(
        (symbol_short!("cloned"), source_id, new_id),
        (),
    );
}

/// Emitted when an invoice is paused.
/// Topics: (split, paused, invoice_id)
/// Data: (creator, reason, auto_resume_at)
pub fn invoice_paused(
    env: &Env,
    invoice_id: u64,
    creator: &Address,
    reason: &String,
    auto_resume_at: &Option<u64>,
) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("paused"), invoice_id),
        (creator.clone(), reason.clone(), *auto_resume_at),
    );
}

/// Emitted when an invoice is resumed.
/// Topics: (split, resumed, invoice_id)
/// Data: creator
pub fn invoice_resumed(env: &Env, invoice_id: u64, creator: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("resumed"), invoice_id),
        creator.clone(),
    );
}

/// Emitted when an invoice is force resumed.
/// Topics: (split, forced, invoice_id)
/// Data: admin_addr
pub fn invoice_force_resumed(env: &Env, invoice_id: u64, admin_addr: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("forced"), invoice_id),
        admin_addr.clone(),
    );
}

/// Emitted when a pending payout is claimed by a recipient (issue #209).
/// Topics: (split, pend_pay, invoice_id)
/// Data: (recipient, amount)
pub fn pending_payout_claimed(env: &Env, invoice_id: u64, recipient: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("pend_pay"), invoice_id),
        (recipient.clone(), amount),
    );
}

pub fn nft_gate_set(env: &Env, contract: &Option<Address>, admin: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("nft_set")),
        (contract.clone(), admin.clone()),
    );
}

pub fn action_queued(env: &Env, action_id: u64, action: &TimelockAction, admin: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("tl_queue"), action_id),
        (action.clone(), admin.clone()),
    );
}

pub fn action_executed(env: &Env, action_id: u64, action: &TimelockAction) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("tl_exec"), action_id),
        action.clone(),
    );
}

pub fn action_cancelled(env: &Env, action_id: u64, action: &TimelockAction, admin: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("tl_cncl"), action_id),
        (action.clone(), admin.clone()),
    );
}

pub fn invoice_admin_frozen(env: &Env, invoice_id: u64, admin: &Address, reason: &String) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("adm_frz"), invoice_id),
        (admin.clone(), reason.clone()),
    );
}

pub fn invoice_admin_unfrozen(env: &Env, invoice_id: u64, admin: &Address) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("adm_unf"), invoice_id),
        admin.clone(),
    );
}

pub fn batch_archived(env: &Env, count: u32, ids: &Vec<u64>) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("bat_arc")),
        (count, ids.clone()),
    );
}

pub fn partial_refund_issued(env: &Env, invoice_id: u64, creator: &Address, bps: u32, amount: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("prt_ref"), invoice_id),
        (creator.clone(), bps, amount),
    );
}

/// Issue #276: Emitted when cumulative platform volume crosses a milestone threshold.
/// Topics: (split, plt_v_ms, milestone_number)
/// Data: (total_volume, invoice_count, ledger)
pub fn platform_volume_milestone(env: &Env, total_volume: i128, invoice_count: u64, milestone_number: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("plt_v_ms"), milestone_number),
        (total_volume, invoice_count, env.ledger().sequence()),
    );
}

/// Issue #276: Emitted when a creator's lifetime volume crosses a milestone threshold.
/// Topics: (split, cr_v_ms, creator)
/// Data: (total_volume, invoice_count, milestone_number, ledger)
pub fn creator_volume_milestone(env: &Env, creator: &Address, total_volume: i128, invoice_count: u64, milestone_number: i128) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("cr_v_ms"), creator.clone()),
        (total_volume, invoice_count, milestone_number, env.ledger().sequence()),
    );
}

/// Issue #285: Emitted when fee tiers are updated.
/// Topics: (split, fee_tiers_updated)
/// Data: count of tiers
pub fn fee_tiers_updated(env: &Env, tier_count: u32) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("fee_upd")),
        tier_count,
    );
}

/// Issue #285: Emitted when a fee tier is applied at release time.
/// Topics: (split, fee_tier_applied, creator)
/// Data: (tier_index, fee_bps, creator_volume)
pub fn fee_tier_applied(env: &Env, creator: &Address, tier_index: u32, fee_bps: u32, creator_volume: u64) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("fee_app"), creator.clone()),
        (tier_index, fee_bps, creator_volume),
    );
}

/// Issue #299: Emitted when creator stats are updated.
/// Topics: (split, creator_stats_updated, creator)
/// Data: (total_invoices, total_raised, total_released, total_payers, avg_funding_time)
pub fn creator_stats_updated(env: &Env, creator: &Address, total_invoices: u32, total_raised: u64, total_released: u64, total_payers: u32, avg_funding_time_ledgers: u32) {
    env.events().publish(
        (symbol_short!("split"), symbol_short!("stats_upd"), creator.clone()),
        (total_invoices, total_raised, total_released, total_payers, avg_funding_time_ledgers),
    );
}
