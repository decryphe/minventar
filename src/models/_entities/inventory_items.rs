use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "inventory_items")]
pub struct Model {
    pub barcode: Option<String>,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub item_type: Option<String>,
    pub manufacturer: Option<String>,
    pub min_stock_qty: i32,
    pub name: String,
    pub order_source: Option<String>,
    pub ordered: bool,
    pub product_number: Option<String>,
    pub size: Option<String>,
    pub stock_qty: i32,
    pub uom: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
