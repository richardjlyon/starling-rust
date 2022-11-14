use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum AccountType {
    PRIMARY,
    ADDITONAL,
    LOAN,
    FIXED_TERM_DEPOSIT
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub description: String,
    pub accounts: Vec<AccountV2>,
    pub account_type: AccountType,
    pub default_category: String,
    pub currency: String,
    pub created_at: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountV2 {
    pub description: String,
}