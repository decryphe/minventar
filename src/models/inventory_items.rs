use loco_rs::prelude::*;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder,
};
use serde::{Deserialize, Serialize};

pub use super::_entities::inventory_items::{self, ActiveModel, Entity, Model};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InventoryItemParams {
    pub barcode: Option<String>,
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

impl Model {
    pub async fn adjust_stock(db: &DatabaseConnection, id: i32, delta: i32) -> ModelResult<Self> {
        let item = Self::find_by_id(db, id).await?;
        let stock_qty = (item.stock_qty + delta).max(0);
        let mut active_model = item.into_active_model();

        active_model.stock_qty = Set(stock_qty);

        active_model.update(db).await.map_err(Into::into)
    }

    pub async fn create(
        db: &DatabaseConnection,
        params: &InventoryItemParams,
    ) -> ModelResult<Self> {
        params.validate()?;

        let active_model = ActiveModel {
            barcode: Set(normalize_optional(params.barcode.as_deref())),
            item_type: Set(normalize_optional(params.item_type.as_deref())),
            manufacturer: Set(normalize_optional(params.manufacturer.as_deref())),
            min_stock_qty: Set(params.min_stock_qty),
            name: Set(params.name.trim().to_string()),
            order_source: Set(normalize_optional(params.order_source.as_deref())),
            ordered: Set(params.ordered),
            product_number: Set(normalize_optional(params.product_number.as_deref())),
            size: Set(normalize_optional(params.size.as_deref())),
            stock_qty: Set(params.stock_qty),
            uom: Set(normalize_optional(params.uom.as_deref())),
            ..Default::default()
        };

        active_model.insert(db).await.map_err(Into::into)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        Self::find_by_id_query(db, id)
            .await?
            .ok_or(ModelError::EntityNotFound)
    }

    pub async fn find_by_id_query(db: &DatabaseConnection, id: i32) -> ModelResult<Option<Self>> {
        Entity::find_by_id(id).one(db).await.map_err(Into::into)
    }

    pub async fn list_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        Entity::find()
            .order_by_asc(inventory_items::Column::Name)
            .all(db)
            .await
            .map_err(Into::into)
    }

    pub async fn list_to_order(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        Entity::find()
            .filter(
                Expr::col(inventory_items::Column::StockQty)
                    .lte(Expr::col(inventory_items::Column::MinStockQty)),
            )
            .order_by_asc(inventory_items::Column::Ordered)
            .order_by_asc(inventory_items::Column::Name)
            .all(db)
            .await
            .map_err(Into::into)
    }

    pub async fn set_ordered(db: &DatabaseConnection, id: i32, ordered: bool) -> ModelResult<Self> {
        let item = Self::find_by_id(db, id).await?;
        let mut active_model = item.into_active_model();
        active_model.ordered = Set(ordered);
        active_model.update(db).await.map_err(Into::into)
    }

    pub async fn update(
        self,
        db: &DatabaseConnection,
        params: &InventoryItemParams,
    ) -> ModelResult<Self> {
        params.validate()?;

        let mut active_model = self.into_active_model();
        active_model.barcode = Set(normalize_optional(params.barcode.as_deref()));
        active_model.item_type = Set(normalize_optional(params.item_type.as_deref()));
        active_model.manufacturer = Set(normalize_optional(params.manufacturer.as_deref()));
        active_model.min_stock_qty = Set(params.min_stock_qty);
        active_model.name = Set(params.name.trim().to_string());
        active_model.order_source = Set(normalize_optional(params.order_source.as_deref()));
        active_model.ordered = Set(params.ordered);
        active_model.product_number = Set(normalize_optional(params.product_number.as_deref()));
        active_model.size = Set(normalize_optional(params.size.as_deref()));
        active_model.stock_qty = Set(params.stock_qty);
        active_model.uom = Set(normalize_optional(params.uom.as_deref()));

        active_model.update(db).await.map_err(Into::into)
    }
}

impl InventoryItemParams {
    fn validate(&self) -> ModelResult<()> {
        if self.min_stock_qty < 0 {
            return Err(ModelError::msg("minimum stock quantity cannot be negative"));
        }

        if self.name.trim().is_empty() {
            return Err(ModelError::msg("name cannot be blank"));
        }

        if self.stock_qty < 0 {
            return Err(ModelError::msg("stock quantity cannot be negative"));
        }

        Ok(())
    }
}

fn normalize_optional(value: Option<&str>) -> Option<String> {
    value
        .map(|entry| entry.trim().to_string())
        .filter(|entry| !entry.is_empty())
}
