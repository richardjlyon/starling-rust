//! Services for interacting with the database.
//!

use super::get_database;
use crate::db;
use crate::entities::counterparty;
use crate::starling::client::{StarlingApiClient, StarlingClient};
use crate::{
    entities::{prelude::*, transaction},
    starling::transaction::StarlingTransaction,
};
use anyhow::Result;

use chrono::Duration;
use sea_orm::*;

/// Insert or update a list of Starling transactions for the specified account and number of days.
///
/// If the transaction doesn't exist, insert it. If it exists and its status has changed, update it.
pub async fn insert_or_update(days: i64) -> Result<()> {
    let db = get_database().await.unwrap();
    for account in db::account::list().await? {
        // fetch the latest transactions

        let client = StarlingApiClient::new(&account.token);
        let transactions = client
            .transactions_since(
                &account.uid,
                &account.default_category,
                Duration::days(days),
            )
            .await;

        for transaction in transactions {
            match transaction_exists(&db, &transaction.uid).await {
                None => {
                    // insert or get the counterparty id

                    let item_counterparty_uid =
                        transaction.counterparty_uid.clone().unwrap_or_default();

                    let counterparty_id = match counterparty_exists(&db, &item_counterparty_uid)
                        .await
                    {
                        Some(counterparty) => counterparty.id,

                        None => {
                            let counterparty = counterparty_from_starling_feed_item(&transaction);
                            let record = Counterparty::insert(counterparty)
                                .exec(&db)
                                .await
                                .expect("inserting counterparty");
                            record.last_insert_id
                        }
                    };

                    // insert the new transaction

                    let record =
                        record_from_starling_feed_item(&transaction, counterparty_id, &account.uid);
                    Transaction::insert(record)
                        .exec(&db)
                        .await
                        .expect("inserting feed item");
                }

                Some(record) => {
                    if transaction_changed(&record, &transaction) {
                        // update the feed item status
                        // TODO : refactor this to update status field only

                        let new_status = transaction.status.to_string();
                        let new_spending_category = transaction.spending_category;
                        let new_user_note = transaction.user_note.clone().unwrap_or_default();

                        let record = transaction::ActiveModel {
                            account_uid: ActiveValue::Set(record.account_uid.to_owned()),
                            status: ActiveValue::set(new_status.to_owned()),
                            spending_category: ActiveValue::set(new_spending_category.to_owned()),
                            user_note: ActiveValue::set(new_user_note.to_owned()),
                            id: ActiveValue::Set(record.id.to_owned()),
                            uid: ActiveValue::Set(record.uid.to_owned()),
                            transaction_time: ActiveValue::set(record.transaction_time.to_owned()),
                            counterparty_id: ActiveValue::set(record.counterparty_id.to_owned()),
                            amount: ActiveValue::set(record.amount).to_owned(),
                            currency: ActiveValue::set(record.currency.to_owned()),
                            reference: ActiveValue::set(record.reference.to_owned()),
                        };
                        record.update(&db).await.expect("updating feed item");
                    }
                }
            }
        }
    }

    Ok(())
}

/// Return true if a feed item with the given feed uid exists in the database.
async fn transaction_exists(
    db: &DatabaseConnection,
    transaction_uid: &String,
) -> Option<transaction::Model> {
    Transaction::find()
        .filter(transaction::Column::Uid.eq(transaction_uid))
        .one(db)
        .await
        .expect("getting feed id")
}

// Return true if status or spending category has changed
fn transaction_changed(record: &transaction::Model, newitem: &StarlingTransaction) -> bool {
    (record.status != newitem.status.to_string())
        || (record.spending_category != newitem.spending_category)
        || (record.user_note != newitem.user_note.clone().unwrap_or_default().to_string())
}

// Return true if a counterparty with the given counterparty uid exists in the database.
async fn counterparty_exists(
    db: &DatabaseConnection,
    counterparty_uid: &String,
) -> Option<counterparty::Model> {
    Counterparty::find()
        .filter(counterparty::Column::Uid.eq(counterparty_uid))
        .one(db)
        .await
        .expect("getting counterparty id")
}

fn record_from_starling_feed_item(
    item: &StarlingTransaction,
    counterparty_id: i32,
    account_uid: &str,
) -> transaction::ActiveModel {
    transaction::ActiveModel {
        uid: ActiveValue::Set(item.uid.to_owned()),
        account_uid: ActiveValue::Set(account_uid.to_string()),
        transaction_time: ActiveValue::Set(item.transaction_time.to_owned()),
        counterparty_id: ActiveValue::Set(counterparty_id),
        amount: ActiveValue::set(item.amount()),
        spending_category: ActiveValue::set(item.spending_category.to_owned()),
        currency: ActiveValue::set(item.currency()),
        reference: ActiveValue::set(item.reference.clone().unwrap_or_default()),
        user_note: ActiveValue::set(item.user_note.clone().unwrap_or_default()),
        status: ActiveValue::set(item.status.to_string()),
        ..Default::default()
    }
}

fn counterparty_from_starling_feed_item(item: &StarlingTransaction) -> counterparty::ActiveModel {
    let item_counterparty_uid = item.counterparty_uid.clone().unwrap_or_default();
    counterparty::ActiveModel {
        uid: ActiveValue::Set(item_counterparty_uid),
        name: ActiveValue::Set(item.counterparty_name.to_owned()),
        r#type: ActiveValue::Set(item.counterparty_type.to_owned()),
        ..Default::default()
    }
}
