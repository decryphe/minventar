use loco_rs::testing::prelude::*;
use loco_rs::TestServer;
use mininventar::{
    app::App,
    models::{
        dictionary_entries::Model as DictionaryEntryModel,
        inventory_items::Model as InventoryItemModel,
    },
};
use serial_test::serial;

const FORM_URLENCODED: &str = "application/x-www-form-urlencoded";

#[tokio::test]
#[serial]
async fn ssr_pages_render() {
    let (request, _ctx) = test_server().await;

    let inventory = request.get("/inventory").await;
    assert_eq!(inventory.status_code(), 200);
    assert!(inventory.text().contains("Current stock"));

    let inventory_form = request.get("/inventory/new").await;
    assert_eq!(inventory_form.status_code(), 200);
    assert!(inventory_form.text().contains("Add inventory item"));

    let to_order = request.get("/inventory/to-order").await;
    assert_eq!(to_order.status_code(), 200);
    assert!(to_order.text().contains("Items to be ordered"));

    let dictionary = request.get("/dictionary").await;
    assert_eq!(dictionary.status_code(), 200);
    assert!(dictionary.text().contains("Add dictionary entry"));
}

#[tokio::test]
#[serial]
async fn can_create_inventory_item_and_adjust_stock_via_routes() {
    let (request, ctx) = test_server().await;

    let create_response = request
        .post("/inventory/save")
        .text("name=Widget&stock_qty=1&min_stock_qty=2")
        .content_type(FORM_URLENCODED)
        .await;

    assert_eq!(create_response.status_code(), 303);
    assert_eq!(
        create_response.headers().get("location").unwrap(),
        "/inventory"
    );

    let item = InventoryItemModel::list_all(&ctx.db).await.unwrap();
    assert_eq!(item.len(), 1);
    assert_eq!(item[0].name, "Widget");

    let adjust_response = request
        .post(&format!("/inventory/{}/adjust", item[0].id))
        .text("delta=2")
        .content_type(FORM_URLENCODED)
        .await;

    assert_eq!(adjust_response.status_code(), 303);

    let adjusted_item = InventoryItemModel::find_by_id(&ctx.db, item[0].id)
        .await
        .unwrap();
    assert_eq!(adjusted_item.stock_qty, 3);
}

#[tokio::test]
#[serial]
async fn can_create_dictionary_entry_via_route() {
    let (request, ctx) = test_server().await;

    let response = request
        .post("/dictionary/save")
        .text("category=manufacturer&value=Acme")
        .content_type(FORM_URLENCODED)
        .await;

    assert_eq!(response.status_code(), 303);
    assert_eq!(response.headers().get("location").unwrap(), "/dictionary");

    let entries = DictionaryEntryModel::list_all(&ctx.db).await.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].category, "manufacturer");
    assert_eq!(entries[0].value, "Acme");
}

async fn test_server() -> (TestServer, loco_rs::app::AppContext) {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();
    let server = TestServer::builder()
        .mock_transport()
        .build(boot.router.clone().unwrap())
        .unwrap();

    (server, boot.app_context)
}
