use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    IN,
    OUT,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CounterpartyType {
    CATEGORY,
    CHEQUE,
    CUSTOMER,
    PAYEE,
    MERCHANT,
    SENDER,
    STARLING,
    LOAN,
}

#[derive(Serialize, Deserialize, Debug, strum::AsRefStr)]
pub enum SpendingCategory {
    BIKE,
    BILLS_AND_SERVICES,
    BUCKET_LIST,
    CAR,
    CASH,
    CELEBRATION,
    CHARITY,
    CHILDREN,
    COFFEE,
    DEBT_REPAYMENT,
    DIY,
    DRINKS,
    EATING_OUT,
    EDUCATION,
    EMERGENCY,
    ENTERTAINMENT,
    ESSENTIAL_SPEND,
    EXPENSES,
    FAMILY,
    FITNESS,
    FUEL,
    GAMBLING,
    GAMING,
    GARDEN,
    GENERAL,
    GIFTS,
    GROCERIES,
    HOBBY,
    HOLIDAYS,
    HOME,
    IMPULSE_BUY,
    INCOME,
    INSURANCE,
    INVESTMENTS,
    LIFESTYLE,
    MAINTENANCE_AND_REPAIRS,
    MEDICAL,
    MORTGAGE,
    NON_ESSENTIAL_SPEND,
    PAYMENTS,
    PERSONAL_TRANSFERS,
    PETS,
    PROJECTS,
    RELATIONSHIPS,
    RENT,
    SAVING,
    SHOPPING,
    SUBSCRIPTIONS,
    TAKEAWAY,
    TAXI,
    TRANSPORT,
    TREATS,
    WEDDING,
    WELLBEING,
    NONE,
    REVENUE,
    OTHER_INCOME,
    CLIENT_REFUNDS,
    INVENTORY,
    STAFF,
    TRAVEL,
    WORKPLACE,
    REPAIRS_AND_MAINTENANCE,
    ADMIN,
    MARKETING,
    BUSINESS_ENTERTAINMENT,
    INTEREST_PAYMENTS,
    BANK_CHARGES,
    OTHER,
    FOOD_AND_DRINK,
    EQUIPMENT,
    PROFESSIONAL_SERVICES,
    PHONE_AND_INTERNET,
    VEHICLES,
    DIRECTORS_WAGES,
    VAT,
    CORPORATION_TAX,
    SELF_ASSESSMENT_TAX,
    INVESTMENT_CAPITAL,
    TRANSFERS,
    LOAN_PRINCIPAL,
    PERSONAL,
    DIVIDENDS,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    UPCOMING,
    PENDING,
    REVERSED,
    SETTLED,
    DECLINED,
    REFUNDED,
    RETRYING,
    ACCOUNT_CHECK,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub feed_items: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoundUp {
    description: String,
    goal_category_uid: uuid::Uuid,
    amount: super::accounts::SignedCurrencyAndAmount,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct TransactionId(pub uuid::Uuid);

impl Display for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryId(pub uuid::Uuid);

impl Display for CategoryId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(rename = "feedItemUid")]
    transaction_uid: TransactionId,
    category_uid: CategoryId,
    pub amount: super::accounts::SignedCurrencyAndAmount,
    pub direction: Direction,
    updated_at: DateTime<Utc>,
    transaction_time: DateTime<Utc>,
    pub settlement_time: DateTime<Utc>,
    source: String,
    source_sub_type: Option<String>,
    pub status: Status,
    transacting_application_user_uid: Option<uuid::Uuid>,
    counter_party_type: CounterpartyType,
    counter_party_uid: Option<uuid::Uuid>,
    pub counter_party_name: String,
    counter_party_sub_entity_uid: Option<uuid::Uuid>,
    counter_party_sub_entity_name: Option<String>,
    counter_party_sub_entity_sub_identifier: Option<String>,
    exchange_rate: Option<f32>,
    pub reference: Option<String>,
    country: String,
    pub spending_category: SpendingCategory,
    pub user_note: Option<String>,
    round_up: Option<RoundUp>,
}

impl Transaction {
    // convert the amoun to a signed float value
    pub fn amount_as_float(&self) -> f32 {
        let amount: f32 = self.amount.minor_units as f32;
        match self.direction {
            Direction::IN => amount / 100.0,
            Direction::OUT => -amount / 100.0,
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let date = self.settlement_time.format("%Y-%m-%d");
        let status = match self.status {
            Status::SETTLED => "*",
            _ => "!",
        };
        let account = match self.spending_category {
            SpendingCategory::INCOME => String::from_str("Income").unwrap(),
            _ => String::from_str("Other").unwrap(),
        };
        let amount = self.amount.minor_units;

        let reference = self.reference.as_deref().unwrap_or_default();

        write!(
            f,
            "{date} {status} \"{counter_party_name:<25}\" \"{reference}\"\n  {account:<10} {amount}",
            date = date,
            status = status,
            counter_party_name = self.counter_party_name,
            reference = reference,
            account = account,
            amount = amount
        )
    }
}

// alias (multiple)
// DESER = {key: ()} -> {alias: ()}
// SER   = {alias: ()} -> {alias: ()}

// rename
// DESER = {key: ()} -> {alias: ()}
// SER   = {alias: ()} -> {key: ()}
