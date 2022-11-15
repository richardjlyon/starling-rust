mod bean;
mod schemas;

use anyhow::Context;
use std::io::Write;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let personal = starling::Client::new("personal");
    let business = starling::Client::new("business");
    let now = chrono::Utc::now();

    let mut file = std::fs::File::create("starling.bean")?;

    for client in &[personal, business] {
        let accounts = client.accounts().await.context("failed to list accounts")?;

        for account in accounts {
            tracing::info!("fetching transactions for {}/{}", client.name, account.name);
            let transactions = client
                .transactions(&account.account_uid, now - chrono::Duration::days(365), now)
                .await
                .context("when fetching transactions")?;

            for transaction in transactions {
                let entry = bean::transaction::transaction(&account, &transaction);
                println!("{}", entry);

                // writeln!(file, "{}", transaction).context("when writing to bean")?;
            }
        }
    }

    Ok(())
}
