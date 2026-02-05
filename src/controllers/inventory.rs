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
        "inventory/list.html",
        data!({
            "nav_active": "inventory",
        }),
    )
}

pub fn routes() -> Routes {
    Routes::new().add("inventory", get(list))
}
