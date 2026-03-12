#![allow(clippy::items_after_statements)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::{
    http::header,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::Form;
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::i18n::{is_supported_locale, locale_cookie_header, sanitize_return_to, DEFAULT_LOCALE};

#[derive(Debug, Deserialize)]
pub struct LanguageForm {
    pub lang: String,
    pub return_to: String,
}

#[debug_handler]
pub async fn set_language(Form(form): Form<LanguageForm>) -> Result<Response> {
    let locale = if is_supported_locale(&form.lang) {
        form.lang.as_str()
    } else {
        DEFAULT_LOCALE
    };
    let return_to = sanitize_return_to(&form.return_to);
    let mut response = Redirect::to(return_to).into_response();
    response
        .headers_mut()
        .append(header::SET_COOKIE, locale_cookie_header(locale));

    Ok(response)
}

pub fn routes() -> Routes {
    Routes::new().add("language", post(set_language))
}
