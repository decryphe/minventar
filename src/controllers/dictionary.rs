#![allow(clippy::items_after_statements)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::http::HeaderMap;
use axum::response::Redirect;
use axum_extra::extract::Form;
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::i18n::{locale_from_headers, SUPPORTED_LOCALES};
use crate::models::dictionary_entries::{DictionaryEntryParams, Model as DictionaryEntryModel};

#[derive(Debug, Deserialize)]
pub struct DictionaryDeleteForm;

#[derive(Debug, Deserialize)]
pub struct DictionarySaveForm {
    pub category: String,
    pub id: Option<i32>,
    pub value: String,
}

impl DictionarySaveForm {
    fn into_params(self) -> DictionaryEntryParams {
        DictionaryEntryParams {
            category: self.category,
            value: self.value,
        }
    }
}

#[debug_handler]
pub async fn delete(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Form(_form): Form<DictionaryDeleteForm>,
) -> Result<Response> {
    DictionaryEntryModel::delete(&ctx.db, id).await?;
    Ok(Redirect::to("/dictionary").into_response())
}

#[debug_handler]
pub async fn edit(
    Path(id): Path<i32>,
    headers: HeaderMap,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let current_entry = DictionaryEntryModel::find_by_id(&ctx.db, id).await?;
    let entry_groups = DictionaryEntryModel::list_grouped(&ctx.db).await?;
    let lang = locale_from_headers(&headers);

    format::render().view(
        &v,
        "dictionary/form.html",
        data!({
            "available_locales": SUPPORTED_LOCALES,
            "current_entry": current_entry,
            "current_path": format!("/dictionary/{id}/edit"),
            "entry_groups": entry_groups,
            "is_edit": true,
            "lang": lang,
            "nav_active": "dictionary",
        }),
    )
}

#[debug_handler]
pub async fn index(
    headers: HeaderMap,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let entry_groups = DictionaryEntryModel::list_grouped(&ctx.db).await?;
    let lang = locale_from_headers(&headers);

    format::render().view(
        &v,
        "dictionary/form.html",
        data!({
            "available_locales": SUPPORTED_LOCALES,
            "current_entry": serde_json::Value::Null,
            "current_path": "/dictionary",
            "entry_groups": entry_groups,
            "is_edit": false,
            "lang": lang,
            "nav_active": "dictionary",
        }),
    )
}

#[debug_handler]
pub async fn save(
    State(ctx): State<AppContext>,
    Form(form): Form<DictionarySaveForm>,
) -> Result<Response> {
    let id = form.id;
    let params = form.into_params();

    if let Some(id) = id {
        let entry = DictionaryEntryModel::find_by_id(&ctx.db, id).await?;
        entry.update(&ctx.db, &params).await?;
    } else {
        DictionaryEntryModel::create(&ctx.db, &params).await?;
    }

    Ok(Redirect::to("/dictionary").into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .add("dictionary", get(index))
        .add("dictionary/save", post(save))
        .add("dictionary/{id}/delete", post(delete))
        .add("dictionary/{id}/edit", get(edit))
}
