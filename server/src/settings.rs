use std::{fs, sync::Arc};

use anyhow::{Context, Result};
use serde::Deserialize;
use unidirs::{Directories, UnifiedDirs};

pub type GlobalSettings = Arc<Settings>;

#[derive(Deserialize)]
pub struct Settings {}

pub fn load() -> Result<GlobalSettings> {
    let path = UnifiedDirs::simple("rocks", "dnaka91", env!("CARGO_PKG_NAME"))
        .default()
        .context("failed finding project directories")?
        .config_dir()
        .join("config.toml");

    let buf = fs::read(path).context("failed reading settings file")?;
    let settings = toml::from_slice(&buf).context("failed parsing settings")?;

    Ok(Arc::new(settings))
}
