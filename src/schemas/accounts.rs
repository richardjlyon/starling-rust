use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Debug, Clone, strum::Display)]
pub enum Currencies {
    GBP,
    EUR,
}

impl Currencies {
    pub fn decimals(&self) -> u32 {
        match self {
            Currencies::GBP => 2,
            Currencies::EUR => 2,
            _ => 2,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignedCurrencyAndAmount {
    pub currency: Currencies,
    pub minor_units: i64,
}

impl SignedCurrencyAndAmount {
    pub fn to_decimal(&self) -> Decimal {
        let dec: u32 = self.currency.decimals();
        Decimal::new(self.minor_units, dec)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    cleared_balance: SignedCurrencyAndAmount,
    effective_balance: SignedCurrencyAndAmount,
    pending_transactions: SignedCurrencyAndAmount,
}
