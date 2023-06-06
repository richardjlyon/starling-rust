//! Structures and methods for processing `/api/v2/accounts/` endpoints

use super::client::StarlingApiClient;
use anyhow::Result;
use chrono::{DateTime, Utc};
use format_num::NumberFormat;
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
    pub cleared: SignedCurrencyAndAmount,
    #[serde(rename = "effectiveBalance")]
    pub effective: SignedCurrencyAndAmount,
    #[serde(rename = "pendingTransactions")]
    pub pending: SignedCurrencyAndAmount,
    #[serde(rename = "totalClearedBalance")]
    pub total_cleared: SignedCurrencyAndAmount,
    #[serde(rename = "acceptedOverdraft")]
    pub overdraft: SignedCurrencyAndAmount,
    #[serde(rename = "totalEffectiveBalance")]
    pub total_effective: SignedCurrencyAndAmount,
}

#[derive(Deserialize, Debug)]
pub struct SignedCurrencyAndAmount {
    currency: String,
    #[serde(rename = "minorUnits")]
    minor_units: i64,
}

impl SignedCurrencyAndAmount {
    pub fn as_float(&self) -> f32 {
        self.minor_units as f32 / 100.0
    }

    pub fn as_string(&self) -> String {
        let num = NumberFormat::new();
        let amount = num.format(" >10,.2f", self.as_float());
        format!("{} {}", self.currency, amount)
    }
}

// Implement `/api/v2/accounts/{accountUid}/balance`
//
impl StarlingApiClient {
    pub async fn balance(&self, account_uid: &String) -> Result<Balance> {
        let mut resp = surf::get(format!(
            "{}/accounts/{}/balance",
            &self.base_url, &account_uid
        ))
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", &self.key))
        .await
        .unwrap();

        Ok(resp.body_json::<Balance>().await.unwrap())
    }
}
