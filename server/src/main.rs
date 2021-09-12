#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::{env, net::SocketAddr};

use anyhow::Result;
use axum::Server;
use tokio::signal;
use tracing::info;

mod handlers;
mod response;
mod routes;
mod settings;
mod templates;

#[cfg(debug_assertions)]
const ADDRESS: [u8; 4] = [127, 0, 0, 1];
#[cfg(not(debug_assertions))]
const ADDRESS: [u8; 4] = [0, 0, 0, 0];

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env::set_var(
        "RUST_LOG",
        concat!(env!("CARGO_PKG_NAME"), "=trace,tower_http=trace,info"),
    );
    tracing_subscriber::fmt::init();

    let settings = crate::settings::load()?;
    let addr = SocketAddr::from((ADDRESS, 8080));

    let server = Server::try_bind(&addr)?
        .serve(routes::build(settings).into_make_service())
        .with_graceful_shutdown(shutdown());

    info!("Listening on {}", addr);

    server.await?;

    Ok(())
}

async fn shutdown() {
    signal::ctrl_c().await.ok();
    info!("Shutting down");
}
