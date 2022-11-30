//! get the account for the given currency
//!
use crate::starling::{
    client::Client,
    schemas::account::{Account, AccountResponse},
};

use crate::error::AppError;

impl Client {
    // get an account for the specified currency
    pub async fn account_for_currency(&self, currency: &str) -> Result<Account, AppError> {
        self.get("accounts", &())
            .await
            .map(|d: AccountResponse| d.accounts)
            .map(|d| {
                d.into_iter()
                    .find(|a| a.currency == currency)
                    .expect("one GBP account")
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_gets_an_account() {
        let client = Client::new("personal");
        let account = client.account_for_currency("GBP").await.unwrap();
        // ALEX Why does this not fail?
        assert_eq!(account.name, "sdfsdfsd");
        println!("{:#?}", account);
    }
}
