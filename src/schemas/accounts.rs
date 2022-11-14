use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum AccountType {
    PRIMARY,
    ADDITONAL,
    LOAN,
    FIXED_TERM_DEPOSIT,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResponse {
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_uid: String,
    pub name: String,
    pub default_category: String,
    pub currency: String,
    pub created_at: String,
    pub account_type: AccountType,
}

#[derive(Serialize, Deserialize, Debug)]
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
