use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

#[derive(Serialize, Deserialize, Debug)]
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
    amount: super::accounts::SignedCurrencyAndAmount,
    direction: Direction,
    updated_at: DateTime<Utc>,
    transaction_time: DateTime<Utc>,
    settlement_time: DateTime<Utc>,
    source: String,
    source_sub_type: String,
    status: Status,
    transacting_application_user_uid: uuid::Uuid,
    counter_party_type: CounterpartyType,
    counter_party_uid: uuid::Uuid,
    counter_party_name: String,
    counter_party_sub_entity_uid: uuid::Uuid,
    counter_party_sub_entity_name: Option<String>,
    counter_party_sub_entity_sub_identifier: Option<String>,
    exchange_rate: Option<f32>,
    reference: Option<String>,
    country: String,
    spending_category: SpendingCategory,
    user_note: Option<String>,
    round_up: Option<RoundUp>,
}

// alias (multiple)
// DESER = {key: ()} -> {alias: ()}
// SER   = {alias: ()} -> {alias: ()}

// rename
// DESER = {key: ()} -> {alias: ()}
// SER   = {alias: ()} -> {key: ()}
