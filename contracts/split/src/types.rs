use soroban_sdk::{contracttype, Address, Vec};

/// Status of an invoice lifecycle.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum InvoiceStatus {
    /// Invoice created, awaiting full payment.
    Pending,
    /// All shares paid; funds released to recipients.
    Released,
    /// Deadline passed before full funding; payers refunded.
    Refunded,
}

/// A single payment made toward an invoice.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Payment {
    /// Address of the payer.
    pub payer: Address,
    /// Amount paid in stroops (7 decimal places).
    pub amount: i128,
}

/// Optional invoice-level payment controls.
#[contracttype]
#[derive(Clone, Debug)]
pub struct InvoiceOptions {
    /// Minimum seconds between payments from the same payer on this invoice.
    pub payment_cooldown_secs: Option<u64>,
    /// Maximum number of payments allowed inside a rolling window.
    pub max_payments_per_window: Option<u32>,
    /// Rolling window size in seconds for the global invoice payment limit.
    pub payment_window_secs: Option<u64>,
}

/// Storage-efficient invoice data kept separately from extension settings.
#[contracttype]
#[derive(Clone, Debug)]
pub struct InvoiceCore {
    /// Address that created the invoice.
    pub creator: Address,
    /// Ordered list of recipient addresses.
    pub recipients: Vec<Address>,
    /// Amounts owed to each recipient (parallel to `recipients`).
    pub amounts: Vec<i128>,
    /// USDC token contract address.
    pub token: Address,
    /// Unix timestamp after which unfunded invoices can be refunded.
    pub deadline: u64,
    /// Total amount collected so far.
    pub funded: i128,
    /// Current lifecycle status.
    pub status: InvoiceStatus,
    /// All payments made toward this invoice.
    pub payments: Vec<Payment>,
}

/// Less frequently used invoice extension settings.
#[contracttype]
#[derive(Clone, Debug)]
pub struct InvoiceExt {
    /// Minimum seconds between payments from the same payer on this invoice.
    pub payment_cooldown_secs: Option<u64>,
    /// Maximum number of payments allowed inside a rolling window.
    pub max_payments_per_window: Option<u32>,
    /// Rolling window size in seconds for the global invoice payment limit.
    pub payment_window_secs: Option<u64>,
}

/// Public invoice view returned by `get_invoice`.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Invoice {
    /// Address that created the invoice.
    pub creator: Address,
    /// Ordered list of recipient addresses.
    pub recipients: Vec<Address>,
    /// Amounts owed to each recipient (parallel to `recipients`).
    pub amounts: Vec<i128>,
    /// USDC token contract address.
    pub token: Address,
    /// Unix timestamp after which unfunded invoices can be refunded.
    pub deadline: u64,
    /// Total amount collected so far.
    pub funded: i128,
    /// Current lifecycle status.
    pub status: InvoiceStatus,
    /// All payments made toward this invoice.
    pub payments: Vec<Payment>,
}

impl From<InvoiceOptions> for InvoiceExt {
    fn from(options: InvoiceOptions) -> Self {
        Self {
            payment_cooldown_secs: options.payment_cooldown_secs,
            max_payments_per_window: options.max_payments_per_window,
            payment_window_secs: options.payment_window_secs,
        }
    }
}

impl From<InvoiceCore> for Invoice {
    fn from(core: InvoiceCore) -> Self {
        Self {
            creator: core.creator,
            recipients: core.recipients,
            amounts: core.amounts,
            token: core.token,
            deadline: core.deadline,
            funded: core.funded,
            status: core.status,
            payments: core.payments,
        }
    }
}
