//! Structures and methods for processing `/api/v2/accounts/` endpoints

use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::client::StarlingApiClient;

/// Represents a list of Starling accounts
#[derive(Deserialize, Debug)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

/// Represents a Starling account
#[derive(Deserialize, Debug)]
pub struct Account {
    #[serde(rename = "accountUid")]
    pub uid: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "defaultCategory")]
    pub default_category: String,
    pub name: String,
}
/// Represents a Starling Balance response
#[derive(Deserialize, Debug)]
pub struct Balance {
    #[serde(rename = "clearedBalance")]
    cleared: SignedCurrencyAndAmount,
    #[serde(rename = "effectiveBalance")]
    effective: SignedCurrencyAndAmount,
    #[serde(rename = "pendingTransactions")]
    pending: SignedCurrencyAndAmount,
    #[serde(rename = "totalClearedBalance")]
    total_cleared: SignedCurrencyAndAmount,
    #[serde(rename = "acceptedOverdraft")]
    overdraft: SignedCurrencyAndAmount,
    #[serde(rename = "totalEffectiveBalance")]
    total_effective: SignedCurrencyAndAmount,
}

#[derive(Deserialize, Debug)]
struct SignedCurrencyAndAmount {
    currency: String,
    #[serde(rename = "minurUnits")]
    minor_units: i64,
}

// Implement `/api/v2/accounts/{accountUid}/balance`
//
impl StarlingApiClient {
    async fn balance(&self, account_uid: &String) -> Balance {
        let mut resp = surf::get(format!(
            "{}/accounts/{}/balance",
            &self.base_url, &account_uid
        ))
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", &self.key))
        .await
        .unwrap();

        resp.body_json::<Balance>().await.unwrap()
    }
}
