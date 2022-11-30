//! Handles representing and rendering accounting events in the the Beancount
//! lkanguage syntax
//!

use chrono::{DateTime, Duration, Utc};
use regex::Regex;
use rust_decimal::Decimal;

use crate::starling::schemas::account::Account as StarlingAccount;
use crate::starling::schemas::balance::Balance as StarlingBalance;
use crate::starling::schemas::transaction::{Direction, Transaction as StarlingTransaction};
use crate::starling::schemas::transaction::{SpendingCategory, Status};

use convert_case::{Case, Casing};
use std::collections::HashSet;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;

const EQUITY_ACCOUNT: &str = "Equity:Opening-Balances";

pub struct Bean {
    pub accounts: Vec<StarlingAccount>,
    pub balances: Vec<BeanBalance>,
    pub transactions: Vec<BeanTransaction>,
    out_file: File,
}

pub struct BeanTransaction {
    pub account_name: String,
    pub transaction: StarlingTransaction,
}

pub struct BeanBalance {
    pub account_name: String,
    pub balance: StarlingBalance,
    pub transactions: Vec<StarlingTransaction>,
}

impl Bean {
    // make an initialised bean object
    pub fn new() -> Self {
        // create a new file for writing
        let file_name = String::from("starling.beancount");
        if std::path::Path::new(&file_name).exists() {
            fs::remove_file(&file_name).unwrap();
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)
            .unwrap();

        Self {
            transactions: Vec::new(),
            accounts: Vec::new(),
            balances: Vec::new(),
            out_file: file,
        }
    }

    pub fn add_account(&mut self, account: StarlingAccount) {
        self.accounts.push(account);
    }

    pub fn add_transactions(&mut self, mut transactions: Vec<BeanTransaction>) {
        self.transactions.append(&mut transactions);
    }

    pub fn add_balance(&mut self, balance: BeanBalance) {
        self.balances.push(balance);
    }

    // write beanfile to file system
    pub fn write(&mut self) {
        self.transactions.sort_by(|a, b| {
            a.transaction
                .transaction_time
                .cmp(&b.transaction.transaction_time)
        });
        self.write_preamble();
        self.write_account_declarations();
        self.write_opening_balance_deposits();
        self.write_transactions();
        self.write_balance_assertions();
    }

    fn write_preamble(&mut self) {
        let line1 = "option \"title\" \"Starling Ledger\"";
        let line2 = "option \"operating_currency\" \"GBP\"";
        let preamble = format!("{}\n{}", line1, line2);

        write!(self.out_file, "{}\n\n", preamble).unwrap();
    }

    fn write_account_declarations(&mut self) {
        let start_date = self.start_date();

        let mut income_statements = HashSet::new();

        for tx in self.transactions.iter() {
            income_statements.insert(fmt_income_statement_account(&tx.transaction));
        }

        for account in self.accounts.iter() {
            writeln!(
                self.out_file,
                "{} open {:<37} {}",
                &start_date,
                fmt_account_name(&account.name),
                account.currency
            )
            .unwrap();
        }

        for income_statement in income_statements.iter() {
            writeln!(
                self.out_file,
                "{} open {:<37} GBP",
                &start_date, income_statement
            )
            .unwrap();
        }

        write!(
            self.out_file,
            "{} open {:<37} GBP\n\n",
            &start_date, EQUITY_ACCOUNT
        )
        .unwrap();
    }

    fn write_opening_balance_deposits(&mut self) {
        let start_date = self.start_date();
        for balance in self.balances.iter() {
            // compute opening balance
            let mut open = balance.balance.effective_balance.clone().as_decimal();

            for tx in balance.transactions.iter() {
                if tx.status != Status::Upcoming {
                    open += tx.clone().as_signed_decimal();
                }
            }

            let line1 = format!("{} * \"Deposit\"", start_date);
            let line2 = format!(
                "  {:<40} {:>10} {}",
                fmt_account_name(&balance.account_name),
                open,
                &balance.transactions[0].amount.currency
            );
            let line3 = format!("  {}", EQUITY_ACCOUNT);

            write!(self.out_file, "{}\n{}\n{}\n\n", line1, line2, line3).unwrap();
        }
    }

    fn write_transactions(&mut self) {
        for tx in self.transactions.iter() {
            if tx.transaction.status == Status::Upcoming {
                return;
            };

            let date = fmt_date(&tx.transaction.transaction_time);
            let status = fmt_status(&tx.transaction.status);
            let counter_party_name = fmt_counterparty_name(&tx.transaction.counter_party_name);
            let reference = fmt_reference(&tx.transaction.reference);
            let note = fmt_note(&tx.transaction.user_note);
            let amount = fmt_amount(&tx.transaction);

            let line1 = format!(
                "{} {} {} {} {}",
                date, status, counter_party_name, reference, note
            );

            let line2 = format!(
                "  {:<40} {:>10} {}",
                fmt_account_name(&tx.account_name),
                amount,
                &tx.transaction.amount.currency
            );

            let line3 = format!(
                "  {:<40} {:>10} {}",
                fmt_income_statement_account(&tx.transaction),
                -amount,
                &tx.transaction.amount.currency
            );

            let transaction = format!("{}\n{}\n{}", line1, line2, line3);

            writeln!(self.out_file, "{}\n", transaction).unwrap();
        }
    }

    fn write_balance_assertions(&mut self) {
        // beancount balances has to be specified for end of day, so this is a hack
        let tomorrow = chrono::Utc::now() + Duration::days(1);
        let tomorrow = fmt_date(&tomorrow);
        for balance in self.balances.iter() {
            let account_name = fmt_account_name(&balance.account_name);
            let amount = balance.balance.effective_balance.as_decimal();
            let currency = balance.balance.effective_balance.currency.clone();

            writeln!(
                self.out_file,
                "{} balance {:<27} {} {}",
                tomorrow, account_name, amount, currency
            )
            .unwrap();
        }
    }

    // compute the date of the earliest transaxction
    fn start_date(&self) -> String {
        let date = self.transactions[0].transaction.transaction_time;
        date.format("%Y-%m-%d").to_string()
    }
}

impl Default for Bean {
    fn default() -> Self {
        Self::new()
    }
}
// construct an account name for the given account
// e.g. Assets::Starling::Personal
fn fmt_account_name(account_name: &String) -> String {
    format!("Assets:Starling:{}", account_name)
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
    format!("\"{}\"", name)
}

fn fmt_reference(reference: &Option<String>) -> String {
    match reference {
        Some(string) => {
            let re = Regex::new(r"\s+").unwrap();
            let clean_string = re.replace_all(string, " ");
            format!("\"{}\"", clean_string)
        }
        None => String::new(),
    }
}

fn fmt_note(note: &Option<String>) -> String {
    match note {
        Some(note) => format!(" ; {}", note),
        None => String::new(),
    }
}

pub fn fmt_income_statement_account(tx: &StarlingTransaction) -> String {
    let category_type = match SpendingCategory::is_income(&tx.spending_category) {
        true => "Income",
        false => "Expenses",
    };
    let category = &tx.spending_category.as_ref().to_case(Case::Pascal);
    format!("{}:{}", category_type, category)
}

fn fmt_amount(tx: &StarlingTransaction) -> Decimal {
    match tx.direction {
        Direction::In => tx.amount.as_decimal(),
        Direction::Out => -tx.amount.as_decimal(),
    }
}
