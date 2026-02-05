#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
#![allow(clippy::items_after_statements)]
use loco_rs::prelude::*;

#[debug_handler]
pub async fn index(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    format::render().view(
        &v,
        "configuration/index.html",
        data!({
            "nav_active": "configuration",
        }),
    )
}

pub fn routes() -> Routes {
    Routes::new().add("configuration", get(index))
}
