use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Create the FeedItem table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FeedItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FeedItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FeedItem::FeedUid).string().not_null())
                    .col(ColumnDef::new(FeedItem::TransactionTime).timestamp().not_null())
                    .col(ColumnDef::new(FeedItem::CounterpartyID).integer().not_null())
                    .col(ColumnDef::new(FeedItem::Amount).float().not_null())
                    .col(ColumnDef::new(FeedItem::Currency).string().not_null())
                    .col(ColumnDef::new(FeedItem::SpendingCategory).string().not_null())
                    .col(ColumnDef::new(FeedItem::Reference).string().not_null())
                    .col(ColumnDef::new(FeedItem::UserNote).string().not_null())
                    .col(ColumnDef::new(FeedItem::Status).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Drop the FeedItem table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FeedItem::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum FeedItem {
    Table,
    Id,
    FeedUid,
    TransactionTime,
    CounterpartyID,
    Amount,
    Currency,
    SpendingCategory,
    Reference,
    UserNote,
    Status,
}
