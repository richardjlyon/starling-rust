//! Services for interacting with the database.
//!

use crate::{
    client::{Account, StarlingClient, StarlingFeedItem},
    entities::{counterparty, feed_item, prelude::*},
};

use sea_orm::*;
use std::env;

/// Inser or update a list of Starling transactions for the specified account and number of days.
///
/// If the transaction doesn't exist, insert it. If it exists and its status has changed, update it.
pub async fn insert_or_update(client: &dyn StarlingClient, account: &Account, days: i64) {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let database_name = env::var("DB_NAME").expect("DB_NAME is not set in .env file");
    let url = format!("{}/{}", database_url, database_name);
    let db = Database::connect(&url).await.expect("getting database");

    for item in client
        .transactions_since(
            &account.account_uid,
            &account.default_category,
            chrono::Duration::days(days),
        )
        .await
    {
        match feeditem_exists(&db, &item.uid).await {
            // if the feed item doesn't already exist
            None => {
                // insert or get the counterparty id
                let item_counterparty_uid = item.counterparty_uid.clone().unwrap_or_default();
                let counterparty_id = match counterparty_exists(&db, &item_counterparty_uid).await {
                    Some(counterparty) => counterparty.id,
                    None => {
                        let counterparty = counterparty_from_starling_feed_item(&item);
                        let record = Counterparty::insert(counterparty)
                            .exec(&db)
                            .await
                            .expect("inserting counterparty");
                        record.last_insert_id
                    }
                };

                // insert the new feed item
                let record = record_from_starling_feed_item(&item, counterparty_id);
                FeedItem::insert(record)
                    .exec(&db)
                    .await
                    .expect("inserting feed item");
            }
            // if the feed item does exist
            Some(record) => {
                // if the feed item status has changed
                if feeditem_has_changed(&record, &item) {
                    // update the feed item status
                    // TODO : refactor this to update status field only
                    let new_status = item.status.to_string();
                    let new_spending_category = item.spending_category;
                    let new_user_note = item.user_note.clone().unwrap_or_default();

                    let record = feed_item::ActiveModel {
                        status: ActiveValue::set(new_status.to_owned()),
                        spending_category: ActiveValue::set(new_spending_category.to_owned()),
                        user_note: ActiveValue::set(new_user_note.to_owned()),
                        id: ActiveValue::Set(record.id.to_owned()),
                        feed_uid: ActiveValue::Set(record.feed_uid.to_owned()),
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

// Return true if status or spending category has changed
fn feeditem_has_changed(record: &feed_item::Model, newitem: &StarlingFeedItem) -> bool {
    (record.status != newitem.status.to_string())
        || (record.spending_category != newitem.spending_category.to_string())
        || (record.user_note != newitem.user_note.clone().unwrap_or_default().to_string())
}

/// Return true if a feed item with the given feed uid exists in the database.
async fn feeditem_exists(db: &DatabaseConnection, feed_uid: &String) -> Option<feed_item::Model> {
    FeedItem::find()
        .filter(feed_item::Column::FeedUid.eq(feed_uid))
        .one(db)
        .await
        .expect("getting feed id")
}

/// Return true if a counterparty with the given counterparty uid exists in the database.
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
    item: &StarlingFeedItem,
    counterparty_id: i32,
) -> feed_item::ActiveModel {
    feed_item::ActiveModel {
        feed_uid: ActiveValue::Set(item.uid.to_owned()),
        transaction_time: ActiveValue::Set(item.transaction_time.to_owned()),
        counterparty_id: ActiveValue::Set(counterparty_id),
        amount: ActiveValue::set(item.amount()).to_owned(),
        spending_category: ActiveValue::set(item.spending_category.to_owned()),
        currency: ActiveValue::set(item.currency().to_owned()),
        reference: ActiveValue::set(item.reference.to_owned()),
        user_note: ActiveValue::set(item.user_note.clone().unwrap_or_default().to_owned()),
        status: ActiveValue::set(item.status.to_string()),
        ..Default::default()
    }
}

fn counterparty_from_starling_feed_item(item: &StarlingFeedItem) -> counterparty::ActiveModel {
    let item_counterparty_uid = item.counterparty_uid.clone().unwrap_or_default();
    counterparty::ActiveModel {
        uid: ActiveValue::Set(item_counterparty_uid.to_owned()),
        name: ActiveValue::Set(item.counterparty_name.to_owned()),
        r#type: ActiveValue::Set(item.counterparty_type.to_owned()),
        ..Default::default()
    }
}
