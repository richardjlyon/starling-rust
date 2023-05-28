//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "feed_item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub feed_uid: String,
    pub transaction_time: DateTimeUtc,
    pub counterparty_id: i32,
    #[sea_orm(column_type = "Float")]
    pub amount: f32,
    pub currency: String,
    pub spending_category: String,
    pub reference: String,
    pub user_note: String,
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
