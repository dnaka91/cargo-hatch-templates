#![allow(clippy::unused_async)]

use axum::{http::header::CONTENT_TYPE, response::IntoResponse};

use crate::{response::HtmlTemplate, templates};

pub async fn index() -> impl IntoResponse {
    HtmlTemplate(templates::Index)
}

pub async fn favicon() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "image/svg+xml")],
        include_bytes!("../assets/favicon.svg").as_ref(),
    )
}
