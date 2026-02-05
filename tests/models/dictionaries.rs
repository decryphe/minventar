use loco_rs::testing::prelude::*;
use mininventar::{
    app::App,
    models::dictionary_entries::{DictionaryEntryParams, Model as DictionaryEntryModel},
};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn rejects_duplicate_dictionary_entries_per_category() {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    DictionaryEntryModel::create(
        &boot.app_context.db,
        &DictionaryEntryParams {
            category: "manufacturer".to_string(),
            value: "Acme".to_string(),
        },
    )
    .await
    .unwrap();

    let duplicate = DictionaryEntryModel::create(
        &boot.app_context.db,
        &DictionaryEntryParams {
            category: "manufacturer".to_string(),
            value: "Acme".to_string(),
        },
    )
    .await;

    assert!(duplicate.is_err());

    let other_category = DictionaryEntryModel::create(
        &boot.app_context.db,
        &DictionaryEntryParams {
            category: "order_source".to_string(),
            value: "Acme".to_string(),
        },
    )
    .await;

    assert!(other_category.is_ok());
}
