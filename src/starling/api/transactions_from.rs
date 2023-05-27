use chrono::DateTime;

use crate::{
    error::AppError,
    starling::{
        client::Client,
        schemas::{
            account::Account,
            transaction::{Transaction, TransactionResponse},
        },
    },
};

impl Client {
    // get transactions from the given date
    pub async fn transactions_from(
        &self,
        account: &Account,
        start_date: DateTime<chrono::Utc>,
    ) -> Result<Vec<Transaction>, AppError> {
        let url = format!(
            "feed/account/{}/category/{}",
            account.account_uid, account.default_category
        );

        let params = Params {
            changes_since: start_date,
        };

        self.get(&url, &params)
            .await
            .map(|d: TransactionResponse| d.feed_items)
    }
}

#[derive(serde::Serialize)]
struct Params {
    #[serde(rename = "changesSince")]
    changes_since: chrono::DateTime<chrono::Utc>,
}
