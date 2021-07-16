use std::{fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Read the config file from the given path
pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Unable to read config file: {}", path.as_ref().display()))?;
    Ok(toml::from_str(&contents).with_context(|| "Invalid config file format")?)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    site: SiteConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SiteConfig {
    base_url: String,
    title: Option<String>,
}
