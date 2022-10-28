#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
};

use anyhow::Result;
use axum::Server;
use tokio_shutdown::Shutdown;
use tracing::{info, Level};
use tracing_subscriber::{filter::Targets, prelude::*};

mod handlers;
mod response;
mod routes;
mod settings;
mod templates;

const ADDRESS: Ipv4Addr = if cfg!(debug_assertions) {
    Ipv4Addr::LOCALHOST
} else {
    Ipv4Addr::UNSPECIFIED
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            Targets::new()
                .with_target(env!("CARGO_PKG_NAME"), Level::TRACE)
                .with_target("tower_http", Level::TRACE)
                .with_default(Level::INFO),
        )
        .init();

    let settings = crate::settings::load()?;
    let addr = SocketAddr::from((ADDRESS, 8080));
    let shutdown = Shutdown::new()?;

    let server = Server::try_bind(&addr)?
        .serve(routes::build(settings).into_make_service())
        .with_graceful_shutdown(shutdown.handle());

    info!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
