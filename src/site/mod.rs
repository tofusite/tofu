use std::{fs::{create_dir, File}, io::{BufReader, Read}, path::{Path, PathBuf}};

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use self::{cache::Cache, config::Config};

pub mod cache;
pub mod config;

const CACHE_PATH: &str = "./.tofu/cache.toml";

pub struct Site {
    config: Config,
    dir: PathBuf,
}

impl Site {
    pub fn new<P: AsRef<Path>>(config: Config, dir: P) -> Self {
        Self {
            config,
            dir: dir.as_ref().into(),
        }
    }

    /// Create the "./.tofu" directory if it doesn't exist already
    fn create_tofu_dir(&self) -> Result<()> {
        let dir = self.dir.join("./.tofu");

        if !dir.exists() {
            create_dir(dir)?;
        }

        Ok(())
    }

    /// Attempts to read the cache file for incremental builds
    /// If the cache file doesn't exist or is invalid it will return an empty Cache
    fn read_cache(&self) -> Cache {
        let cache_file = Path::new(CACHE_PATH);

        if !cache_file.exists() {
            return Cache::default();
        }

        // If for whatever reason we aren't able to load the cache, we can just use an empty cache and rebuild entirely
        Cache::load(&cache_file).unwrap_or_else(|_| Cache::default())
    }

    pub fn build<P: AsRef<Path>>(&self, out: P) -> Result<()> {
        self.create_tofu_dir()?;
        self.build_content(out)?;

        Ok(())
    }

    fn build_content<P: AsRef<Path>>(&self, out: P) -> Result<()> {
        let mut cache = self.read_cache();

        let dir = self.dir.join("./content/").canonicalize()?;
        for entry in WalkDir::new(&dir) {
            let entry = entry?;
            let path = entry.path().display().to_string();

            if entry.file_type().is_file() {
                let file = File::open(entry.path())?;
                let mut reader = BufReader::new(file);
                let digest = Site::hash(reader)?;

                if cache.dirty(&path, &digest) {
                    cache.insert(&path, &digest);

                    // TODO: Rebuild file
                    println!("Build: {}", path);
                } else {
                    println!("Cached: {}", path);
                }
            }
        }

        cache.save(CACHE_PATH)?;

        Ok(())
    }

    fn hash<R: Read>(mut reader: R) -> Result<String> {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];

        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }

            hasher.update(&buffer[..count]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }
}
