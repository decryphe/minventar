use loco_rs::testing::prelude::*;
use mininventar::{
    app::App,
    models::inventory_items::{InventoryItemParams, Model as InventoryItemModel},
};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_create_and_adjust_inventory_stock() {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    let item = InventoryItemModel::create(
        &boot.app_context.db,
        &InventoryItemParams {
            barcode: Some("111".to_string()),
            item_type: Some("widget".to_string()),
            manufacturer: Some("Acme".to_string()),
            min_stock_qty: 3,
            name: "Washer".to_string(),
            order_source: Some("Catalog".to_string()),
            ordered: false,
            product_number: Some("WX-1".to_string()),
            size: Some("M".to_string()),
            stock_qty: 1,
            uom: Some("pcs".to_string()),
        },
    )
    .await
    .unwrap();

    let increased = InventoryItemModel::adjust_stock(&boot.app_context.db, item.id, 2)
        .await
        .unwrap();
    assert_eq!(increased.stock_qty, 3);

    let reduced = InventoryItemModel::adjust_stock(&boot.app_context.db, item.id, -10)
        .await
        .unwrap();
    assert_eq!(reduced.stock_qty, 0);
}
