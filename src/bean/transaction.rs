// Implement functionality to format for beancount

use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Utc};
use starling::schemas::accounts::{Account, SignedCurrencyAndAmount};
use starling::schemas::transactions::{SpendingCategory, Status, Transaction};

pub fn transaction(account: &Account, transaction: &Transaction) -> String {
    // let date = transaction.settlement_time.format("%Y-%m-%d");

    let account = match transaction.spending_category {
        SpendingCategory::INCOME => "Income",
        _ => "Other",
    };

    format!(
        "{date} {status} {counter_party_name:<25} \"{reference}\"\n  {account:<10} {amount}",
        date = fmt_date(&transaction.settlement_time),
        status = fmt_status(&transaction.status),
        counter_party_name = fmt_counterparty_name(&transaction.counter_party_name),
        reference = fmt_reference(transaction.reference.as_deref().unwrap_or_default()),
        account = account,
        amount = fmt_amount(transaction.amount.minor_units),
    )
}

fn fmt_date(time: &DateTime<Utc>) -> DelayedFormat<StrftimeItems> {
    time.format("%Y-%m-%d")
}

fn fmt_status(status: &Status) -> &str {
    match status {
        Status::SETTLED => "*",
        _ => "!",
    }
}

fn fmt_counterparty_name(name: &str) -> String {
    format!("\"{}\"", name.to_string())
}

fn fmt_reference(reference: &str) -> &str {
    reference
}

fn fmt_amount(amount: i64) -> i64 {
    amount
}
