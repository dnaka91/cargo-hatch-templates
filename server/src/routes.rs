use axum::{routing::get, Extension, Router};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use crate::{handlers, settings::GlobalSettings};

pub fn build(settings: GlobalSettings) -> Router {
    Router::new()
        .route("/favicon.svg", get(handlers::favicon))
        .route("/", get(handlers::index))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(Extension(settings))
                .into_inner(),
        )
}
