use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Primary,
    Additonal,
    Loan,
    FixedTermDeposit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResponse {
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AccountId(pub uuid::Uuid);

impl Display for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_uid: AccountId,
    pub name: String,
    pub default_category: String,
    pub currency: String,
    pub created_at: String,
    pub account_type: AccountType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignedCurrencyAndAmount {
    pub currency: String,
    pub minor_units: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    cleared_balance: SignedCurrencyAndAmount,
    effective_balance: SignedCurrencyAndAmount,
    pending_transactions: SignedCurrencyAndAmount,
}
