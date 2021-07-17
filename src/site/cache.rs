use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    #[serde(flatten)]
    entries: HashMap<String, String>,
}

impl Cache {
    /// Create a new cache initialized to the given HashMap
    pub fn new(entries: HashMap<String, String>) -> Self {
        Cache { entries }
    }

    /// Read a cache from a toml file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Unable to read cache file: {}", path.as_ref().display()))?;
        Ok(toml::from_str(&contents).with_context(|| "Invalid config file format")?)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = toml::to_string(self).with_context(|| "Unable to serialize cache")?;
        let contents = format!(
            "{}\n{}\n{}",
            "# This file is used to keep track of page changes for incremental builds. You shouldn't need to edit it.",
            "# The cache file is specific to this machine, you should NOT commit it to version control.",
            contents
        );
        Ok(fs::write(path, contents).with_context(|| "Unable to write cache file")?)
    }

    /// Get whether a cache entry is "dirty" (i.e. its been modified since it was cached)
    pub fn dirty<S: Into<String>>(&self, path: S, digest: S) -> bool {
        self.entries
            .get(&path.into())
            .map_or(true, |val| val != &digest.into())
    }

    /// Insert a value into the cache.
    /// Returns true if the entry existed and was updated, otherwise returns false
    pub fn insert<S: Into<String>>(&mut self, path: S, digest: S) -> bool {
        self.entries.insert(path.into(), digest.into()).is_some()
    }
}

impl Default for Cache {
    fn default() -> Self {
        Cache::new(HashMap::default())
    }
}
