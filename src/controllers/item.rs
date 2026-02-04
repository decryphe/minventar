#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

#[debug_handler]
pub async fn list(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>
) -> Result<Response> {
    format::render().view(&v, "item/list.html", data!({}))
}

#[debug_handler]
pub async fn remove(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>
) -> Result<Response> {
    format::render().view(&v, "item/remove.html", data!({}))
}

#[debug_handler]
pub async fn update(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>
) -> Result<Response> {
    format::render().view(&v, "item/update.html", data!({}))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("items/")
        .add("list", get(list))
        .add("remove", get(remove))
        .add("update", get(update))
}
