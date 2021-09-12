use std::{fs, sync::Arc};

use anyhow::{Context, Result};
use serde::Deserialize;

pub type GlobalSettings = Arc<Settings>;

#[derive(Deserialize)]
pub struct Settings {}

pub fn load() -> Result<GlobalSettings> {
    let locations = &[
        concat!("/etc/", env!("CARGO_PKG_NAME"), "/config.toml"),
        concat!("/app/", env!("CARGO_PKG_NAME"), ".toml"),
        concat!(env!("CARGO_PKG_NAME"), ".toml"),
    ];
    let buf = locations
        .iter()
        .find_map(|loc| fs::read(loc).ok())
        .context("failed finding settings")?;

    Ok(Arc::new(
        toml::from_slice(&buf).context("failed parsing settings")?,
    ))
}
