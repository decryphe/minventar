use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum InventoryItems {
    Table,
    Id,
    Manufacturer,
    ItemType,
    Size,
    Uom,
    OrderSource,
    ProductNumber,
    Barcode,
    Name,
    StockQty,
    MinStockQty,
    Ordered,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(InventoryItems::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(InventoryItems::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(InventoryItems::Manufacturer).string().null())
                .col(ColumnDef::new(InventoryItems::ItemType).string().null())
                .col(ColumnDef::new(InventoryItems::Size).string().null())
                .col(ColumnDef::new(InventoryItems::Uom).string().null())
                .col(ColumnDef::new(InventoryItems::OrderSource).string().null())
                .col(
                    ColumnDef::new(InventoryItems::ProductNumber)
                        .string()
                        .null(),
                )
                .col(ColumnDef::new(InventoryItems::Barcode).string().null())
                .col(ColumnDef::new(InventoryItems::Name).string().not_null())
                .col(
                    ColumnDef::new(InventoryItems::StockQty)
                        .integer()
                        .not_null()
                        .default(0),
                )
                .col(
                    ColumnDef::new(InventoryItems::MinStockQty)
                        .integer()
                        .not_null()
                        .default(0),
                )
                .col(
                    ColumnDef::new(InventoryItems::Ordered)
                        .boolean()
                        .not_null()
                        .default(false),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(InventoryItems::Table).to_owned())
            .await
    }
}
