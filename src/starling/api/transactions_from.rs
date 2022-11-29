//! Get transactions for the given account from tne given date
//!

use crate::{
    error::AppError,
    starling::{
        client::Client,
        schemas::{
            account::AccountId,
            transaction::{Transaction, TransactionResponse},
        },
    },
};

impl Client {
    pub async fn transactions_from(
        &self,
        account_uid: AccountId,
        start_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Transaction>, AppError> {
        //
        let url = format!("feed/account/{}/settled-transactions-between", account_uid);

        let params = Params {
            min_transaction_timestamp: start_date,
            max_transaction_timestamp: chrono::Utc::now(),
        };

        self.get(&url, &params)
            .await
            .map(|d: TransactionResponse| d.feed_items)
    }
}

#[derive(serde::Serialize)]
struct Params {
    #[serde(rename = "minTransactionTimestamp")]
    min_transaction_timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "maxTransactionTimestamp")]
    max_transaction_timestamp: chrono::DateTime<chrono::Utc>,
}
