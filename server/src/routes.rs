use axum::{handler::get, routing::BoxRoute, AddExtensionLayer, Router};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use crate::{handlers, settings::GlobalSettings};

pub fn build(settings: GlobalSettings) -> Router<BoxRoute> {
    Router::new()
        .route("/favicon-16x16.png", get(handlers::favicon_16))
        .route("/favicon-32x32.png", get(handlers::favicon_32))
        .route("/", get(handlers::hello))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(AddExtensionLayer::new(settings))
                .into_inner(),
        )
        .check_infallible()
        .boxed()
}
