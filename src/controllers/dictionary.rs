#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
#![allow(clippy::items_after_statements)]
use loco_rs::prelude::*;

#[debug_handler]
pub async fn list(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>
) -> Result<Response> {
    format::render().view(&v, "dictionary/list.html", data!({}))
}

#[debug_handler]
pub async fn remove(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>
) -> Result<Response> {
    format::render().view(&v, "dictionary/remove.html", data!({}))
}

#[debug_handler]
pub async fn update(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>
) -> Result<Response> {
    format::render().view(&v, "dictionary/update.html", data!({}))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("dictionaries/")
        .add("list", get(list))
        .add("remove", get(remove))
        .add("update", get(update))
}
