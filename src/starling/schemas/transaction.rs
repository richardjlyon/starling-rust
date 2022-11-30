//! Starling `Transaction` schema
//!
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use super::SignedCurrencyAndAmount;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(rename = "feedItemUid")]
    transaction_uid: TransactionId,
    category_uid: CategoryId,
    pub amount: SignedCurrencyAndAmount,
    pub direction: Direction,
    updated_at: DateTime<Utc>,
    pub transaction_time: DateTime<Utc>,
    pub settlement_time: Option<DateTime<Utc>>,
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
    pub fn as_signed_decimal(&self) -> Decimal {
        match self.direction {
            Direction::Out => Decimal::new(self.amount.minor_units, 2),
            Direction::In => Decimal::ZERO - Decimal::new(self.amount.minor_units, 2),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionId(pub uuid::Uuid);

impl Display for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryId(pub uuid::Uuid);

impl Display for CategoryId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
    In,
    Out,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Upcoming,
    Pending,
    Reversed,
    Settled,
    Declined,
    Refunded,
    Retrying,
    AccountCheck,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CounterpartyType {
    Category,
    Cheque,
    Customer,
    Payee,
    Merchant,
    Sender,
    Starling,
    Loan,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, strum::AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpendingCategory {
    Bike,
    BillsAndServices,
    BucketList,
    Car,
    Cash,
    Celebration,
    Charity,
    Children,
    Coffee,
    DebtRepayment,
    Diy,
    Drinks,
    EatingOut,
    Education,
    Emergency,
    Entertainment,
    EssentialSpend,
    Expenses,
    Family,
    Fitness,
    Fuel,
    Gambling,
    Gaming,
    Garden,
    General,
    Gifts,
    Groceries,
    Hobby,
    Holidays,
    Home,
    ImpulseBuy,
    Income,
    Insurance,
    Investments,
    Lifestyle,
    MaintenanceAndRepairs,
    Medical,
    Mortgage,
    NonEssentialSpend,
    Payments,
    PersonalTransfers,
    Pets,
    Projects,
    Relationships,
    Rent,
    Saving,
    Shopping,
    Subscriptions,
    Takeaway,
    Taxi,
    Transport,
    Treats,
    Wedding,
    Wellbeing,
    None,
    Revenue,
    OtherIncome,
    ClientRefunds,
    Inventory,
    Staff,
    Travel,
    Workplace,
    RepairsAndMaintenance,
    Admin,
    Marketing,
    BusinessEntertainment,
    InterestPayments,
    BankCharges,
    Other,
    FoodAndDrink,
    Equipment,
    ProfessionalServices,
    PhoneAndInternet,
    Vehicles,
    DirectorsWages,
    Vat,
    CorporationTax,
    SelfAssessmentTax,
    InvestmentCapital,
    Transfers,
    LoanPrincipal,
    Personal,
    Dividends,
}

impl SpendingCategory {
    pub fn is_income(&self) -> bool {
        matches!(self, Self::Income | Self::OtherIncome)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoundUp {
    description: String,
    goal_category_uid: uuid::Uuid,
    amount: SignedCurrencyAndAmount,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub feed_items: Vec<Transaction>,
}
