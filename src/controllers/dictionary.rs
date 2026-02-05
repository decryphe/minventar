#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
#![allow(clippy::items_after_statements)]
use loco_rs::prelude::*;

#[debug_handler]
pub async fn list(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    format::render().view(
        &v,
        "dictionary/list.html",
        data!({
            "nav_active": "configuration",
        }),
    )
}

#[debug_handler]
pub async fn remove(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    format::render().view(&v, "dictionary/remove.html", data!({}))
}

#[debug_handler]
pub async fn update(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    format::render().view(&v, "dictionary/update.html", data!({}))
}

pub fn routes() -> Routes {
    Routes::new()
        .add("configuration/dictionaries", get(list))
        .add("configuration/dictionaries/remove", get(remove))
        .add("configuration/dictionaries/update", get(update))
}
