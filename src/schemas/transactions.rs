use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub feed_items: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename = "feedItemUid")]
    transaction_uid: uuid::Uuid,
}

// alias (multiple)
// DESER = {key: ()} -> {alias: ()}
// SER   = {alias: ()} -> {alias: ()}

// rename
// DESER = {key: ()} -> {alias: ()}
// SER   = {alias: ()} -> {key: ()}
