pub enum AccountType {
    PRIMARY,
    ADDITONAL,
    LOAN,
    FIXED_TERM_DEPOSIT
}

pub struct Account {
    pub description: String,
    pub accounts: Vec<AccountV2>,
    pub account_type: AccountType,
    pub default_category: String,
    pub currency: String,
    pub created_at: String,
    pub name: String
}

#[derive(Debug)]
pub struct AccountV2 {
    pub description: String,
}