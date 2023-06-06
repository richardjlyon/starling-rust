//! Command Line Interface `Accounts` commands
//!

use crate::db;
use anyhow::Result;

pub async fn list() -> Result<()> {
    println!("Account list");

    for account in db::account::list().await? {
        println!("{:#?}", account.name);
    }

    Ok(())
}
