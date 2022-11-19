use crate::bean::BeanTransaction;
use crate::starling::schemas::transactions::{Direction, SpendingCategory, Status};
use chrono::{DateTime, Utc};
use convert_case::{Case, Casing};
use regex::Regex;
use rust_decimal::Decimal;
use std::fmt;

impl BeanTransaction {
    // construct a balance sheet account string
    // e.g. "Assets:Starling:Business"
    pub fn balance_sheet_account(&self) -> String {
        format!("Assets:Starling:{}", &self.account_name)
    }

    // construct an income statement account string
    // e.g. "Expense:BusinessEntertainment"
    pub fn income_statement_account(&self) -> String {
        let category_type = match SpendingCategory::is_income(&self.spending_category) {
            true => "Income",
            false => "Expenses",
        };
        let category = &self.spending_category.as_ref().to_case(Case::Pascal);
        format!("{}:{}", category_type, category)
    }
}

impl fmt::Display for BeanTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date = fmt_date(&self.date);
        let status = fmt_status(&self.status);
        let counter_party_name = fmt_counterparty_name(&self.counter_party_name);
        let reference = fmt_reference(&self.reference);
        let note = fmt_note(&self.note);
        let amount = fmt_amount(&self.amount, false);
        let amount_reversed = fmt_amount(&self.amount, true);

        let line1 = format!(
            "{} {} {} {} {}",
            date, status, counter_party_name, reference, note
        );
        let line2 = format!(
            "  {:<40} {:>10} {}",
            &self.balance_sheet_account(),
            amount,
            &self.currency
        );
        let line3 = format!(
            "  {:<40} {:>10} {}",
            &self.income_statement_account(),
            amount_reversed,
            &self.currency
        );

        write!(f, "{}\n{}\n{}", line1, line2, line3)
    }
}

fn fmt_date(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%d").to_string()
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

fn fmt_note(note: &str) -> String {
    match note.is_empty() {
        true => String::new(),
        false => format!("; {}", note),
    }
}

// fn fmt_balance_sheet_account(account_name: &String) -> String {
//     format!("Assets:Starling:{}", account_name)
// }

// fn fmt_income_statement_account(
//     spending_category: &SpendingCategory,
//     direction: &Direction,
// ) -> String {
//     let direction = match direction {
//         Direction::In => "Income",
//         Direction::Out => "Expenses",
//     };

//     let category = spending_category.as_ref();
//     format!("{}:{}", direction, category.to_case(Case::Pascal))
// }

fn fmt_amount(amount: &Decimal, reverse: bool) -> String {
    match reverse {
        false => format!("{}", amount.to_string()),
        true => format!("{}", (Decimal::ZERO - amount).to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::BeanTransaction;
    use crate::starling::schemas::transactions::{Direction, SpendingCategory, Status};
    use chrono::prelude::*;
    use rust_decimal::Decimal;

    #[test]
    fn it_constructs_balance_sheet_account() {
        let tx = BeanTransaction {
            date: chrono::Utc::now(),
            status: Status::Settled,
            account_name: String::from("Personal"),
            counter_party_name: String::from("Tesco"),
            reference: String::from("TESCO-STORES 6557 EDINBURGH GBR"),
            note: String::from("A note"),
            spending_category: SpendingCategory::Admin,
            balance_sheet_account: String::from("Assets:Starling:Business"),
            income_statement_account: String::from("Expenses:BillsAndServices"),
            amount: Decimal::new(12345, 2),
            direction: Direction::Out,
            currency: String::from("GBP"),
        };

        let expected = "Assets:Starling:Personal";
        assert_eq!(expected, tx.balance_sheet_account());
    }

    #[test]
    fn it_constructs_income_statement_account() {
        let tx = BeanTransaction {
            date: chrono::Utc::now(),
            status: Status::Settled,
            account_name: String::from("Personal"),
            counter_party_name: String::from("Tesco"),
            reference: String::from("TESCO-STORES 6557 EDINBURGH GBR"),
            note: String::from("A note"),
            spending_category: SpendingCategory::Admin,
            balance_sheet_account: String::from("Assets:Starling:Business"),
            income_statement_account: String::from("Expenses:BillsAndServices"),
            amount: Decimal::new(12345, 2),
            direction: Direction::Out,
            currency: String::from("GBP"),
        };

        let expected = "Expenses:Admin";
        assert_eq!(expected, tx.income_statement_account());
    }

    #[test]
    fn it_displays_transactions() {
        let tx = BeanTransaction {
            // ALEX How to get a date from YY/MM/DD ?
            date: Utc.with_ymd_and_hms(2022, 11, 18, 9, 10, 11).unwrap(),
            status: Status::Settled,
            account_name: String::from("Personal"),
            counter_party_name: String::from("Tesco"),
            reference: String::from("TESCO-STORES 6557 EDINBURGH GBR"),
            note: String::from("A note"),
            spending_category: SpendingCategory::Admin,
            balance_sheet_account: String::from("Assets:Starling:Business"),
            income_statement_account: String::from("Expenses:BillsAndServices"),
            amount: Decimal::new(12345, 2),
            direction: Direction::Out,
            currency: String::from("GBP"),
        };

        let expected = "2022-11-18 * \"Tesco\" \"TESCO-STORES 6557 EDINBURGH GBR\" ; A note\n  Assets:Starling:Personal                     123.45 GBP\n  Expenses:Admin                              -123.45 GBP";

        assert_eq!(expected, tx.to_string());
    }
}
