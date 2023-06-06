//! Command Line Interface `Accounts` commands
//!

use crate::{db, starling::client::StarlingApiClient};
use anyhow::Result;

pub async fn list() -> Result<()> {
    println!("Account list:");
    for account in db::account::list().await? {
        println!("- {:#?}", account.name);
    }

    Ok(())
}

pub async fn balance() -> Result<()> {
    println!("Account balances:");
    for account in db::account::list().await? {
        let client = StarlingApiClient::new(&account.token);
        let balance = client.balance(&account.uid).await?;
        println!("- {}: {}", account.name, balance.effective.as_string());
    }
    Ok(())
}
