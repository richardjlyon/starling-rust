use chrono::{
    format::{DelayedFormat, StrftimeItems},
    DateTime, Utc,
};
use convert_case::{Case, Casing};
use regex::Regex;
use rust_decimal::Decimal;

use crate::starling::schemas::{
    accounts::Account,
    transactions::{Direction, SpendingCategory, Status, Transaction},
};

/// Beancount `Transactions` directive
pub fn transaction(
    date: DateTime<Utc>,
    status: Status,
    counter_party_name: &String,
    reference: &String,
    balance_sheet_account: &String,
    income_statement_account: &String,
    amount: Decimal,
    currency: &String,
) -> String {
    format!(
        "{date} {status} {counter_party_name} {reference}\n  {balance_sheet_account:<50} {amount:>10}\n{income_statement_account:<50} {negative_amount:>10}",
        date = fmt_date(&date),
        status = fmt_status(&status),
        counter_party_name = fmt_counterparty_name(&counter_party_name),
        reference = fmt_reference(&reference),
        amount = fmt_amount_with_currency(&amount, &currency, true),
        negative_amount =fmt_amount_with_currency(&amount, &currency, false)
    )
}

fn fmt_amount_with_currency(amount: &Decimal, currency: &String, make_negative: bool) -> String {
    match make_negative {
        false => format!("{} {}", amount.to_string(), currency),
        true => format!("{} {}", (Decimal::ZERO - amount).to_string(), currency),
    }
}

pub fn transactions(account: &Account, transaction: &Transaction) -> String {
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
        Status::Settled => "*",
        _ => "!",
    }
}

fn fmt_counterparty_name(name: &str) -> String {
    format!("\"{}\"", name.to_string())
}

fn fmt_reference(reference: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    let clean_string = re.replace_all(reference, " ");
    format!("\"{}\"", clean_string.to_string())
}

fn fmt_balance_sheet_account(account_name: &String) -> String {
    format!("Assets:Starling:{}", account_name)
}

fn fmt_income_statement_account(
    spending_category: &SpendingCategory,
    direction: &Direction,
) -> String {
    let direction = match direction {
        Direction::In => "Income",
        Direction::Out => "Expenses",
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

#[cfg(test)]
mod tests {
    use crate::starling::schemas::transactions::Status;
    use chrono::{DateTime, TimeZone};
    use rust_decimal::Decimal;

    use super::transaction;

    #[test]
    fn it_formats_transaction() {
        let date = chrono::Utc::now();
        let status = Status::Settled;
        let counter_party_name = String::from("Tesco");
        let reference = String::from("TESCO-STORES 6557 EDINBURGH GBR");
        let balance_sheet_account = String::from("Assets:Starling:Business");
        let income_statement_account = String::from("Expenses:BillsAndServices");
        let amount = Decimal::new(12345, 2);
        let currency = String::from("GBP");

        let result = transaction(
            date,
            status,
            &counter_party_name,
            &reference,
            &balance_sheet_account,
            &income_statement_account,
            amount,
            &currency,
        );

        assert_eq!(
            "2022-11-16 * \"Tesco\" \"TESCO-STORES 6557 EDINBURGH GBR\"\n  Assets:Starling:Business                           -123.45 GBP\nExpenses:BillsAndServices                          123.45 GBP", result,
            "Responses should be equal"
        );
    }
}
