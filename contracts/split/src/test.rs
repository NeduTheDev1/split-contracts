#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, Vec,
};

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// Deploy the split contract and a mock USDC token; return (env, contract_id, token_id).
fn setup() -> (Env, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SplitContract, ());
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone()).address();

    // Mint tokens to test accounts via the admin interface.
    let stellar_asset = StellarAssetClient::new(&env, &token_id);
    stellar_asset.mint(&token_admin, &1_000_000_000);

    (env, contract_id, token_id)
}

fn client<'a>(env: &'a Env, contract_id: &Address) -> SplitContractClient<'a> {
    SplitContractClient::new(env, contract_id)
}

fn token_client<'a>(env: &'a Env, token_id: &Address) -> TokenClient<'a> {
    TokenClient::new(env, token_id)
}

fn invoice_options(
    payment_cooldown_secs: Option<u64>,
    max_payments_per_window: Option<u32>,
    payment_window_secs: Option<u64>,
) -> InvoiceOptions {
    InvoiceOptions {
        payment_cooldown_secs,
        max_payments_per_window,
        payment_window_secs,
    }
}

fn single_recipient_invoice(
    env: &Env,
    c: &SplitContractClient,
    token_id: &Address,
    total: i128,
    options: InvoiceOptions,
) -> u64 {
    let creator = Address::generate(env);
    let recipient = Address::generate(env);

    let mut recipients = Vec::new(env);
    recipients.push_back(recipient);
    let mut amounts = Vec::new(env);
    amounts.push_back(total);

    c.create_invoice_with_options(
        &creator,
        &recipients,
        &amounts,
        token_id,
        &9_999_u64,
        &options,
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_create_invoice() {
    let (env, contract_id, token_id) = setup();
    let c = client(&env, &contract_id);

    let creator = Address::generate(&env);
    let recipient = Address::generate(&env);

    let mut recipients = Vec::new(&env);
    recipients.push_back(recipient.clone());
    let mut amounts = Vec::new(&env);
    amounts.push_back(100_i128);

    // Set ledger time so deadline is in the future.
    env.ledger().set_timestamp(1_000);

    let id = c.create_invoice(&creator, &recipients, &amounts, &token_id, &2_000_u64);
    assert_eq!(id, 1);

    let invoice = c.get_invoice(&id);
    assert_eq!(invoice.status, InvoiceStatus::Pending);
    assert_eq!(invoice.funded, 0);
}

#[test]
fn test_pay_and_auto_release() {
    let (env, contract_id, token_id) = setup();
    let c = client(&env, &contract_id);
    let tk = token_client(&env, &token_id);

    let creator = Address::generate(&env);
    let payer = Address::generate(&env);
    let recipient = Address::generate(&env);

    // Fund payer.
    let stellar_asset = StellarAssetClient::new(&env, &token_id);
    stellar_asset.mint(&payer, &500);

    env.ledger().set_timestamp(1_000);

    let mut recipients = Vec::new(&env);
    recipients.push_back(recipient.clone());
    let mut amounts = Vec::new(&env);
    amounts.push_back(200_i128);

    let id = c.create_invoice(&creator, &recipients, &amounts, &token_id, &9_999_u64);

    // Pay full amount — should auto-release.
    c.pay(&payer, &id, &200_i128);

    let invoice = c.get_invoice(&id);
    assert_eq!(invoice.status, InvoiceStatus::Released);

    // Recipient should have received 200.
    assert_eq!(tk.balance(&recipient), 200);
}

#[test]
fn test_partial_pay_then_release() {
    let (env, contract_id, token_id) = setup();
    let c = client(&env, &contract_id);
    let tk = token_client(&env, &token_id);

    let creator = Address::generate(&env);
    let payer1 = Address::generate(&env);
    let payer2 = Address::generate(&env);
    let recipient = Address::generate(&env);

    let stellar_asset = StellarAssetClient::new(&env, &token_id);
    stellar_asset.mint(&payer1, &150);
    stellar_asset.mint(&payer2, &150);

    env.ledger().set_timestamp(1_000);

    let mut recipients = Vec::new(&env);
    recipients.push_back(recipient.clone());
    let mut amounts = Vec::new(&env);
    amounts.push_back(300_i128);

    let id = c.create_invoice(&creator, &recipients, &amounts, &token_id, &9_999_u64);

    c.pay(&payer1, &id, &150_i128);
    let invoice = c.get_invoice(&id);
    assert_eq!(invoice.status, InvoiceStatus::Pending);

    c.pay(&payer2, &id, &150_i128);
    let invoice = c.get_invoice(&id);
    assert_eq!(invoice.status, InvoiceStatus::Released);
    assert_eq!(tk.balance(&recipient), 300);
}

#[test]
fn test_refund_after_deadline() {
    let (env, contract_id, token_id) = setup();
    let c = client(&env, &contract_id);
    let tk = token_client(&env, &token_id);

    let creator = Address::generate(&env);
    let payer = Address::generate(&env);
    let recipient = Address::generate(&env);

    let stellar_asset = StellarAssetClient::new(&env, &token_id);
    stellar_asset.mint(&payer, &100);

    env.ledger().set_timestamp(1_000);

    let mut recipients = Vec::new(&env);
    recipients.push_back(recipient.clone());
    let mut amounts = Vec::new(&env);
    amounts.push_back(500_i128);

    let id = c.create_invoice(&creator, &recipients, &amounts, &token_id, &2_000_u64);

    // Partial payment.
    c.pay(&payer, &id, &100_i128);

    // Advance past deadline.
    env.ledger().set_timestamp(3_000);

    c.refund(&id);

    let invoice = c.get_invoice(&id);
    assert_eq!(invoice.status, InvoiceStatus::Refunded);
    // Payer should be refunded.
    assert_eq!(tk.balance(&payer), 100);
}

#[test]
#[should_panic(expected = "invoice deadline has passed")]
fn test_pay_after_deadline_panics() {
    let (env, contract_id, token_id) = setup();
    let c = client(&env, &contract_id);

    let creator = Address::generate(&env);
    let payer = Address::generate(&env);
    let recipient = Address::generate(&env);

    let stellar_asset = StellarAssetClient::new(&env, &token_id);
    stellar_asset.mint(&payer, &100);

    env.ledger().set_timestamp(1_000);

    let mut recipients = Vec::new(&env);
    recipients.push_back(recipient.clone());
    let mut amounts = Vec::new(&env);
    amounts.push_back(100_i128);

    let id = c.create_invoice(&creator, &recipients, &amounts, &token_id, &2_000_u64);

    env.ledger().set_timestamp(3_000);
    c.pay(&payer, &id, &100_i128);
}

#[test]
#[should_panic(expected = "payment exceeds remaining balance")]
fn test_overpayment_panics() {
    let (env, contract_id, token_id) = setup();
    let c = client(&env, &contract_id);

    let creator = Address::generate(&env);
    let payer = Address::generate(&env);
    let recipient = Address::generate(&env);

    let stellar_asset = StellarAssetClient::new(&env, &token_id);
    stellar_asset.mint(&payer, &1_000);

    env.ledger().set_timestamp(1_000);

    let mut recipients = Vec::new(&env);
    recipients.push_back(recipient.clone());
    let mut amounts = Vec::new(&env);
    amounts.push_back(100_i128);

    let id = c.create_invoice(&creator, &recipients, &amounts, &token_id, &9_999_u64);
    c.pay(&payer, &id, &200_i128);
}

#[test]
fn test_multi_recipient_release() {
    let (env, contract_id, token_id) = setup();
    let c = client(&env, &contract_id);
    let tk = token_client(&env, &token_id);

    let creator = Address::generate(&env);
    let payer = Address::generate(&env);
    let r1 = Address::generate(&env);
    let r2 = Address::generate(&env);
    let r3 = Address::generate(&env);

    let stellar_asset = StellarAssetClient::new(&env, &token_id);
    stellar_asset.mint(&payer, &600);

    env.ledger().set_timestamp(1_000);

    let mut recipients = Vec::new(&env);
    recipients.push_back(r1.clone());
    recipients.push_back(r2.clone());
    recipients.push_back(r3.clone());

    let mut amounts = Vec::new(&env);
    amounts.push_back(100_i128);
    amounts.push_back(200_i128);
    amounts.push_back(300_i128);

    let id = c.create_invoice(&creator, &recipients, &amounts, &token_id, &9_999_u64);
    c.pay(&payer, &id, &600_i128);

    assert_eq!(tk.balance(&r1), 100);
    assert_eq!(tk.balance(&r2), 200);
    assert_eq!(tk.balance(&r3), 300);
}

#[test]
#[should_panic(expected = "payment cooldown active")]
fn test_cooldown_blocks_same_payer_within_window() {
    let (env, contract_id, token_id) = setup();
    let c = client(&env, &contract_id);
    let stellar_asset = StellarAssetClient::new(&env, &token_id);

    let payer = Address::generate(&env);
    let other_payer = Address::generate(&env);
    stellar_asset.mint(&payer, &500);
    stellar_asset.mint(&other_payer, &500);

    env.ledger().set_timestamp(1_000);
    let id = single_recipient_invoice(
        &env,
        &c,
        &token_id,
        500,
        invoice_options(Some(60), None, None),
    );

    c.pay(&payer, &id, &100_i128);
    c.pay(&other_payer, &id, &100_i128);
    c.pay(&payer, &id, &100_i128);
}

#[test]
#[should_panic(expected = "payment rate limit exceeded")]
fn test_rate_limit_blocks_after_n_payments() {
    let (env, contract_id, token_id) = setup();
    let c = client(&env, &contract_id);
    let stellar_asset = StellarAssetClient::new(&env, &token_id);

    env.ledger().set_timestamp(1_000);
    let id = single_recipient_invoice(
        &env,
        &c,
        &token_id,
        500,
        invoice_options(None, Some(2), Some(60)),
    );

    for _ in 0..3 {
        let payer = Address::generate(&env);
        stellar_asset.mint(&payer, &100);
        c.pay(&payer, &id, &100_i128);
    }
}

#[test]
fn test_rate_limit_window_resets() {
    let (env, contract_id, token_id) = setup();
    let c = client(&env, &contract_id);
    let stellar_asset = StellarAssetClient::new(&env, &token_id);

    env.ledger().set_timestamp(1_000);
    let id = single_recipient_invoice(
        &env,
        &c,
        &token_id,
        500,
        invoice_options(None, Some(2), Some(60)),
    );

    for _ in 0..2 {
        let payer = Address::generate(&env);
        stellar_asset.mint(&payer, &100);
        c.pay(&payer, &id, &100_i128);
    }

    env.ledger().set_timestamp(1_061);
    let payer = Address::generate(&env);
    stellar_asset.mint(&payer, &100);
    c.pay(&payer, &id, &100_i128);
}

#[test]
#[should_panic(expected = "payment rate limit exceeded")]
fn test_cooldown_and_rate_limit_independent() {
    let (env, contract_id, token_id) = setup();
    let c = client(&env, &contract_id);
    let stellar_asset = StellarAssetClient::new(&env, &token_id);

    let payer = Address::generate(&env);
    let other_payer = Address::generate(&env);
    stellar_asset.mint(&payer, &500);
    stellar_asset.mint(&other_payer, &500);

    env.ledger().set_timestamp(1_000);
    let id = single_recipient_invoice(
        &env,
        &c,
        &token_id,
        500,
        invoice_options(Some(120), Some(1), Some(60)),
    );

    let ext = c.get_invoice_ext(&id);
    assert_eq!(ext.payment_cooldown_secs, Some(120));
    assert_eq!(ext.max_payments_per_window, Some(1));
    assert_eq!(ext.payment_window_secs, Some(60));

    c.pay(&payer, &id, &100_i128);
    c.pay(&other_payer, &id, &100_i128);
}
