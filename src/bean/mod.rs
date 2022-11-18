//! Represents a beancount transaction, with a method for rendering in Beancount
//! Language Syntax
//!
use crate::starling::schemas::transactions::{Direction, SpendingCategory, Status};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub mod directives;

pub enum AccountTypes {
    Assets,
    Liabilities,
    Equity,
    Income,
    Expenses,
}

pub struct BeanTransaction {
    pub date: DateTime<Utc>,
    pub status: Status,
    pub account_name: String,
    pub counter_party_name: String,
    pub reference: String,
    pub note: String,
    pub spending_category: SpendingCategory,
    pub balance_sheet_account: String,
    pub income_statement_account: String,
    pub amount: Decimal,
    pub direction: Direction,
    pub currency: String,
}
