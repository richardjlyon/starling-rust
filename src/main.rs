//! TODO:
//! - serde
//! - itertools https://crates.io/crates/itertools
//! - iterator API https://doc.rust-lang.org/stable/std/iter/
//! - what is the difference between iter and into_iter
//! - thiserror
//! - tokio process https://docs.rs/tokio/latest/tokio/process/index.html
//!
//! BONUS:
//! - rayon https://crates.io/crates/rayon
//! - tokio tasks https://tokio.rs/tokio/tutorial/spawning
//! - tokio channels https://tokio.rs/tokio/tutorial/channels
//! - tokio streams https://tokio.rs/tokio/tutorial/streams

mod bean;
mod schemas;

use anyhow::Context;
use itertools::Itertools;
use starling::schemas::{accounts::Account, transactions::Transaction};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let personal = starling::Client::new("personal");
    let business = starling::Client::new("business");
    let now = chrono::Utc::now();

    struct TransactionData {
        account: Account,
        transaction: Transaction,
    }

    let file = std::fs::File::create("starling.bean")?;

    // let stream = futures::stream::iter(&[personal, business]);
    // expand each client
    // expand each account
    // expanc each transaction
    // sort transactions
    // collect into a vector

    // all transactions for personal and business
    let mut transaction_data: Vec<TransactionData> = Vec::new();
    let mut transaction_total = 0;

    // get all transactions for the specified period
    for client in &[personal, business] {
        let accounts = client.accounts().await.context("failed to list accounts")?;

        for account in accounts {
            tracing::info!("fetching transactions for {}/{}", client.name, account.name);
            let transactions = client
                .transactions(&account.account_uid, now - chrono::Duration::days(365), now)
                .await
                .context("when fetching transactions")?;

            transaction_total = 0;

            for transaction in transactions {
                transaction_total += transaction.amount.minor_units;
                transaction_data.push(TransactionData {
                    account: account.clone(),
                    transaction,
                });
            }

            tracing::info!(
                "Total for account `{}` = {}",
                account.name,
                transaction_total
            );
        }
    }

    // sort by date and make bean entries
    transaction_data
        .iter()
        .sorted_by_key(|t| t.transaction.settlement_time)
        .for_each(|t| {
            let entry = bean::transaction::transaction(&t.account, &t.transaction);
            println!("{}", entry);
        });

    Ok(())
}
