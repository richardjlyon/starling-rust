pub(crate) mod schemas;

use anyhow::Context;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let client = starling::Client::new("personal");

    let accounts = client.accounts().await;
    // tracing::info!("Account 1: {:#?}", accounts[0]);

    let now = chrono::Utc::now();

    for account in accounts {
        let transactions = client
            .transactions(&account.account_uid, now - chrono::Duration::days(365), now)
            .await
            .context("when fetching transactions")?;

        // tracing::info!("Transactions: {:#?}", transactions);
        for transaction in transactions {
            println!("{}", transaction);
        }
    }

    Ok(())
}
