use axum::http::{header, HeaderMap, HeaderValue};

pub const DEFAULT_LOCALE: &str = "en-US";
pub const LOCALE_COOKIE_NAME: &str = "minventar_locale";
pub const SUPPORTED_LOCALES: [&str; 5] = ["de-DE", "en-US", "es-ES", "fr-FR", "sv-SE"];

#[must_use]
pub fn locale_from_headers(headers: &HeaderMap) -> &'static str {
    headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(locale_from_cookie_header)
        .and_then(normalize_locale)
        .unwrap_or(DEFAULT_LOCALE)
}

#[must_use]
pub fn is_supported_locale(locale: &str) -> bool {
    SUPPORTED_LOCALES.contains(&locale)
}

#[must_use]
pub fn sanitize_return_to(return_to: &str) -> &str {
    if return_to.starts_with('/') && !return_to.starts_with("//") {
        return_to
    } else {
        "/inventory"
    }
}

#[must_use]
pub fn locale_cookie_header(locale: &str) -> HeaderValue {
    match locale {
        "de-DE" => HeaderValue::from_static(
            "minventar_locale=de-DE; Path=/; Max-Age=31536000; SameSite=Lax",
        ),
        "es-ES" => HeaderValue::from_static(
            "minventar_locale=es-ES; Path=/; Max-Age=31536000; SameSite=Lax",
        ),
        "fr-FR" => HeaderValue::from_static(
            "minventar_locale=fr-FR; Path=/; Max-Age=31536000; SameSite=Lax",
        ),
        "sv-SE" => HeaderValue::from_static(
            "minventar_locale=sv-SE; Path=/; Max-Age=31536000; SameSite=Lax",
        ),
        _ => HeaderValue::from_static(
            "minventar_locale=en-US; Path=/; Max-Age=31536000; SameSite=Lax",
        ),
    }
}

fn locale_from_cookie_header(cookie_header: &str) -> Option<&str> {
    cookie_header.split(';').find_map(|entry| {
        let trimmed = entry.trim();
        let (name, value) = trimmed.split_once('=')?;
        (name == LOCALE_COOKIE_NAME).then_some(value)
    })
}

fn normalize_locale(locale: &str) -> Option<&'static str> {
    SUPPORTED_LOCALES
        .into_iter()
        .find(|supported| *supported == locale)
}
