use std::convert::Infallible;

use askama::Template;
use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
    response::{self, IntoResponse},
};
use tracing::error;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        match self.0.render() {
            Ok(html) => response::Html(html).into_response(),
            Err(e) => {
                error!("failed rendering template: {:?}", e);
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Full::default())
                    .unwrap()
            }
        }
    }
}
