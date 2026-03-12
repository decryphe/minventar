#![allow(clippy::items_after_statements)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::http::HeaderMap;
use axum::response::Redirect;
use axum_extra::extract::Form;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::i18n::{locale_from_headers, SUPPORTED_LOCALES};
use crate::models::{
    dictionary_entries,
    inventory_items::{InventoryItemParams, Model as InventoryItemModel},
};

#[derive(Debug, Deserialize)]
pub struct InventoryAdjustForm {
    pub delta: i32,
}

#[derive(Debug, Deserialize)]
pub struct InventoryOrderedForm {
    pub ordered: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InventorySaveForm {
    pub barcode: Option<String>,
    pub id: Option<i32>,
    pub item_type: Option<String>,
    pub manufacturer: Option<String>,
    pub min_stock_qty: i32,
    pub name: String,
    pub order_source: Option<String>,
    pub ordered: Option<String>,
    pub product_number: Option<String>,
    pub size: Option<String>,
    pub stock_qty: i32,
    pub uom: Option<String>,
}

impl InventorySaveForm {
    fn into_params(self) -> InventoryItemParams {
        InventoryItemParams {
            barcode: self.barcode,
            item_type: self.item_type,
            manufacturer: self.manufacturer,
            min_stock_qty: self.min_stock_qty,
            name: self.name,
            order_source: self.order_source,
            ordered: self.ordered.is_some(),
            product_number: self.product_number,
            size: self.size,
            stock_qty: self.stock_qty,
            uom: self.uom,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InventoryItemView {
    pub id: i32,
    pub item_type: Option<String>,
    pub manufacturer: Option<String>,
    pub min_stock_qty: i32,
    pub name: String,
    pub order_source: Option<String>,
    pub order_source_href: Option<String>,
    pub order_source_label: Option<String>,
    pub ordered: bool,
    pub product_number: Option<String>,
    pub stock_qty: i32,
    pub uom: Option<String>,
}

type ToOrderItemView = InventoryItemView;

#[debug_handler]
pub async fn index() -> Result<Response> {
    Ok(Redirect::to("/inventory").into_response())
}

#[debug_handler]
pub async fn adjust_stock(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Form(form): Form<InventoryAdjustForm>,
) -> Result<Response> {
    InventoryItemModel::adjust_stock(&ctx.db, id, form.delta).await?;
    Ok(Redirect::to("/inventory").into_response())
}

#[debug_handler]
pub async fn edit(
    Path(id): Path<i32>,
    headers: HeaderMap,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let dictionary_groups = dictionary_entries::Model::list_grouped(&ctx.db).await?;
    let item = InventoryItemModel::find_by_id(&ctx.db, id).await?;
    let lang = locale_from_headers(&headers);

    format::render().view(
        &v,
        "inventory/form.html",
        data!({
            "available_locales": SUPPORTED_LOCALES,
            "current_path": format!("/inventory/{id}/edit"),
            "dictionary_groups": dictionary_groups,
            "inventory_item": item,
            "is_edit": true,
            "lang": lang,
            "nav_active": "inventory",
        }),
    )
}

#[debug_handler]
pub async fn list(
    headers: HeaderMap,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let items = InventoryItemModel::list_all(&ctx.db)
        .await?
        .into_iter()
        .map(|item| InventoryItemView {
            id: item.id,
            item_type: item.item_type,
            manufacturer: item.manufacturer,
            min_stock_qty: item.min_stock_qty,
            name: item.name,
            order_source_label: substitute_product_number(
                item.order_source.as_deref(),
                item.product_number.as_deref(),
            ),
            order_source_href: build_order_source_href(
                item.order_source.as_deref(),
                item.product_number.as_deref(),
            ),
            order_source: item.order_source,
            ordered: item.ordered,
            product_number: item.product_number,
            stock_qty: item.stock_qty,
            uom: item.uom,
        })
        .collect::<Vec<_>>();
    let lang = locale_from_headers(&headers);

    format::render().view(
        &v,
        "inventory/list.html",
        data!({
            "available_locales": SUPPORTED_LOCALES,
            "current_path": "/inventory",
            "items": items,
            "lang": lang,
            "nav_active": "inventory",
        }),
    )
}

#[debug_handler]
pub async fn new(
    headers: HeaderMap,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let dictionary_groups = dictionary_entries::Model::list_grouped(&ctx.db).await?;
    let lang = locale_from_headers(&headers);

    format::render().view(
        &v,
        "inventory/form.html",
        data!({
            "available_locales": SUPPORTED_LOCALES,
            "current_path": "/inventory/new",
            "dictionary_groups": dictionary_groups,
            "inventory_item": serde_json::Value::Null,
            "is_edit": false,
            "lang": lang,
            "nav_active": "inventory",
        }),
    )
}

#[debug_handler]
pub async fn save(
    State(ctx): State<AppContext>,
    Form(form): Form<InventorySaveForm>,
) -> Result<Response> {
    let id = form.id;
    let params = form.into_params();

    if let Some(id) = id {
        let item = InventoryItemModel::find_by_id(&ctx.db, id).await?;
        item.update(&ctx.db, &params).await?;
    } else {
        InventoryItemModel::create(&ctx.db, &params).await?;
    }

    Ok(Redirect::to("/inventory").into_response())
}

#[debug_handler]
pub async fn set_ordered(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Form(form): Form<InventoryOrderedForm>,
) -> Result<Response> {
    InventoryItemModel::set_ordered(&ctx.db, id, form.ordered.is_some()).await?;
    Ok(Redirect::to("/inventory/to-order").into_response())
}

#[debug_handler]
pub async fn to_order(
    headers: HeaderMap,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let lang = locale_from_headers(&headers);
    let items = InventoryItemModel::list_to_order(&ctx.db)
        .await?
        .into_iter()
        .map(|item| ToOrderItemView {
            id: item.id,
            item_type: item.item_type,
            manufacturer: item.manufacturer,
            min_stock_qty: item.min_stock_qty,
            name: item.name,
            order_source_label: substitute_product_number(
                item.order_source.as_deref(),
                item.product_number.as_deref(),
            ),
            order_source_href: build_order_source_href(
                item.order_source.as_deref(),
                item.product_number.as_deref(),
            ),
            order_source: item.order_source,
            ordered: item.ordered,
            product_number: item.product_number,
            stock_qty: item.stock_qty,
            uom: item.uom,
        })
        .collect::<Vec<_>>();

    format::render().view(
        &v,
        "inventory/to_order.html",
        data!({
            "available_locales": SUPPORTED_LOCALES,
            "current_path": "/inventory/to-order",
            "items": items,
            "lang": lang,
            "nav_active": "to-order",
        }),
    )
}

fn build_order_source_href(
    order_source: Option<&str>,
    product_number: Option<&str>,
) -> Option<String> {
    let source = order_source?;
    if source.starts_with("http://") || source.starts_with("https://") {
        substitute_product_number(Some(source), product_number)
    } else {
        None
    }
}

fn substitute_product_number(source: Option<&str>, product_number: Option<&str>) -> Option<String> {
    source.map(|value| value.replace("{nr}", product_number.unwrap_or("")))
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(index))
        .add("inventory", get(list))
        .add("inventory/new", get(new))
        .add("inventory/save", post(save))
        .add("inventory/to-order", get(to_order))
        .add("inventory/{id}/adjust", post(adjust_stock))
        .add("inventory/{id}/edit", get(edit))
        .add("inventory/{id}/ordered", post(set_ordered))
}
