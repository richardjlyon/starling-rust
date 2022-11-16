// Implement functionality to format for beancount

use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Utc};
use convert_case::{Case, Casing};
use regex::Regex;
use starling::schemas::accounts::{Account, SignedCurrencyAndAmount};
use starling::schemas::transactions::{Direction, SpendingCategory, Status, Transaction};

struct BeanTransaction {
    account: Account,
    transaction: Transaction,
}

pub fn transaction(account: &Account, transaction: &Transaction) -> String {
    format!(
        "{date} {status} {counter_party_name} \"{reference}\"\n  {balance_sheet_account:<25} {amount:>15} {user_note}\n  {income_statement_account}",
        date = fmt_date(&transaction.settlement_time),
        status = fmt_status(&transaction.status),
        counter_party_name = fmt_counterparty_name(&transaction.counter_party_name),
        reference = fmt_reference(transaction.reference.as_deref().unwrap_or_default()),
        balance_sheet_account = fmt_balance_sheet_account(&account.name),
        income_statement_account = fmt_income_statement_account(&transaction.spending_category, &transaction.direction),
        amount = fmt_amount(&transaction),
        user_note = fmt_user_note(&transaction.user_note.as_deref().unwrap_or_default()),
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

fn fmt_reference(reference: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    let clean_string = re.replace_all(reference, " ");
    clean_string.to_string()
}

fn fmt_balance_sheet_account(account_name: &String) -> String {
    format!("Assets:Starling:{}", account_name)
}

fn fmt_income_statement_account(
    spending_category: &SpendingCategory,
    direction: &Direction,
) -> String {
    let direction = match direction {
        Direction::IN => "Income",
        Direction::OUT => "Expenses",
    };

    let category = spending_category.as_ref();
    format!("{}:{}", direction, category.to_case(Case::Pascal))
}

fn fmt_amount(transaction: &Transaction) -> String {
    format!(
        "{} {}",
        transaction.to_decimal().to_string(),
        transaction.amount.currency
    )
}

fn fmt_user_note(user_note: &str) -> String {
    match user_note.is_empty() {
        true => String::new(),
        false => format!("; {}", user_note),
    }
}
