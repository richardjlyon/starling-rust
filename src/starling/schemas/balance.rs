use super::SignedCurrencyAndAmount;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    cleared_balance: SignedCurrencyAndAmount,
    pub effective_balance: SignedCurrencyAndAmount,
    pending_transactions: SignedCurrencyAndAmount,
}
