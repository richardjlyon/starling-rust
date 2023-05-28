use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Represents a list of Starling accounts
#[derive(Deserialize, Debug)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

/// Represents a Starling account
#[derive(Deserialize, Debug)]
pub struct Account {
    #[serde(rename = "accountUid")]
    pub account_uid: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "defaultCategory")]
    pub default_category: String,
    pub name: String,
}
