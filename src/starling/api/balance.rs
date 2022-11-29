//! Get the balance for the given account
//!

use crate::{
    error::AppError,
    starling::{
        client::Client,
        schemas::{account::AccountId, balance::Balance},
    },
};

impl Client {
    pub async fn balance(&self, account_uid: AccountId) -> Result<Balance, AppError> {
        //
        let url = format!("accounts/{}/balance", account_uid);
        self.get(&url, &()).await
    }
}
