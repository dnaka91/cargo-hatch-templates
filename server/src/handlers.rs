#![allow(clippy::unused_async)]

use axum::response::IntoResponse;

use crate::{response::HtmlTemplate, templates};

pub async fn hello() -> impl IntoResponse {
    HtmlTemplate(templates::Index)
}

pub async fn favicon_32() -> impl IntoResponse {
    include_bytes!("../assets/favicon-32x32.png").as_ref()
}

pub async fn favicon_16() -> impl IntoResponse {
    include_bytes!("../assets/favicon-16x16.png").as_ref()
}
