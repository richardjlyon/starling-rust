//! Initialise command
//!
//! Generate a new .beancount file for Starling transactions for the specified
//! time period, or the last 7 days

use anyhow::Context;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use rust_decimal::Decimal;
use std::io::Write;

use budget::bean::BeanTransaction;
use budget::starling::client::Client as StarlingClient;

const DEFAULT_DAYS: i64 = 7;

struct DateRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}
#[derive(Debug)]
struct DatedAmount {
    date: DateTime<Utc>,
    amount: Decimal,
}
#[derive(Debug)]
struct BalanceData {
    open: DatedAmount,
    close: DatedAmount,
}

// Holds transaction and balance data for a single client.
#[derive(Debug)]
struct TransactionData {
    transactions: Vec<BeanTransaction>,
    balance_data: BalanceData,
}

impl TransactionData {
    async fn get(
        client: &StarlingClient,
        date_range: &DateRange,
    ) -> anyhow::Result<TransactionData> {
        //  
        let account = client.account().await.context("failed to list accounts")?;
        tracing::info!("fetching transactions for {}/{}", client.name, account.name);
        
        // get account balances

        let balance = client
        .balance(&account.account_uid)
        .await
        .context("when fetching balance")?;
        
        // fetch the transactions for the specified period

        let starling_transactions = client
        .transactions(&account.account_uid, date_range.start, date_range.end)
        .await
        .context("when fetching transactions")?;
        
        // compute open balance todo(rjlyon): fix

        let close = DatedAmount {
            date: date_range.end,
            amount: balance.cleared_balance.to_decimal(),
        };
        
        // compute close balance

        let mut open = DatedAmount {
            date: date_range.start,
            amount: balance.cleared_balance.to_decimal(), // not a huge fan of decimal
            // you can impl add on SignedCurrencyAmount
        };
        
        // construct the tranaction objects

        let mut transactions = vec![];
        for tx in starling_transactions {
            // tracing::info!("Transaction: {:#?}", transaction);
            let amount = tx.as_decimal();
            open.amount -= amount;

            let bean_tx = BeanTransaction {
                date: tx.settlement_time,
                status: tx.status,
                account_name: account.name.clone(),
                counter_party_name: tx.counter_party_name,
                reference: tx.reference.as_deref().unwrap_or_default().to_string(), // suspicious
                note: tx.user_note.as_deref().unwrap_or_default().to_string(),      // suspicious
                spending_category: tx.spending_category,
                balance_sheet_account: String::new(), // suspicious
                income_statement_account: String::new(), // suspicious
                amount,
                direction: tx.direction,
                currency: tx.amount.currency,
            };
            transactions.push(bean_tx);
        }

        Ok(TransactionData {
            transactions,
            balance_data: BalanceData { open, close },
        })
    }
}

/// generate a new .beancount file for the given date range
pub async fn initialise(
    start_date: &Option<String>,
) -> anyhow::Result<()> {
    //
    let date_range: DateRange = parse_dates(start_date);
    let personal = StarlingClient::new("personal");
    let business = StarlingClient::new("business");

    for client in &[personal, business] {
        //  for each account, get starling transactions
        let transaction_data = TransactionData::get(client, &date_range).await?;
        println!("{:?}", transaction_data.balance_data);

        // sort by date and print
        let mut file = std::fs::File::create("starling.bean")?;
        transaction_data
            .transactions
            .iter()
            .sorted_by_key(|tx| tx.date)
            .for_each(|tx| {
                write!(file, "{}\n\n", tx.to_string());
            });
    }

    Ok(())
}

fn parse_dates(start_date: &Option<String>) -> DateRange {
    let end_date = chrono::Utc::now();

    let start_date = match start_date {
        Some(date) => parse_date(date),
        None => end_date - chrono::Duration::days(DEFAULT_DAYS),
    };

    DateRange {
        start: start_date,
        end: end_date,
    }
}

fn parse_date(date: &str) -> DateTime<Utc> {
    DateTime::parse_from_str(date, "%Y-%m-%d")
        .unwrap()
        .with_timezone(&Utc)
}
