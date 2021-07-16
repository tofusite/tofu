use std::{fs::{File, create_dir}, path::Path};

use anyhow::{Context, Result};

use self::{cache::Cache, config::Config};

pub mod cache;
pub mod config;

pub fn build<P: AsRef<Path>>(config: Config, dir: P, out: P) -> Result<()> {
    println!(
        "{:#?} {} {}",
        config,
        dir.as_ref().to_string_lossy(),
        out.as_ref().to_string_lossy()
    );

    read_tofu_dir(dir)?;

    Ok(())
}

fn read_tofu_dir<P: AsRef<Path>>(dir: P) -> Result<()> {
    let tofu_dir = dir.as_ref().join("./.tofu");

    if !tofu_dir.exists() {
        create_dir(&tofu_dir).with_context(|| "Unable to create tofu data directory")?;
    }

    let cache_file = tofu_dir.join("./cache.toml");

    if !cache_file.exists() {
        File::create(&cache_file)?;
    }

    let mut cache = Cache::load(&cache_file)?;

    println!("{:#?}", cache);

    cache.insert("/", "abc");
    cache.insert("/aaa", "123");

    cache.save(&cache_file)?;

    Ok(())
}
