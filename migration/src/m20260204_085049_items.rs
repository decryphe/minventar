use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "items",
            &[
            
            ("id", ColType::PkAuto),
            
            ("manufacturer", ColType::String),
            ("item_type", ColType::String),
            ("size", ColType::String),
            ("uom", ColType::String),
            ("order_source", ColType::String),
            ("product_number", ColType::String),
            ("barcode", ColType::StringNull),
            ("name", ColType::String),
            ("stock_qty", ColType::Integer),
            ("min_stock_qty", ColType::Integer),
            ("ordered", ColType::Boolean),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "items").await
    }
}
