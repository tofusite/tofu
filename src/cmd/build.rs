use std::path::Path;

use anyhow::Result;
use clap::Clap;

use crate::{Opts, site::config::read_config};

use super::Execute;

/// Builds the tofu site for production
#[derive(Clap)]
pub struct Build {
    /// Specifies the config file to use
    #[clap(short, long, default_value = "./tofu.toml")]
    config: String,

    /// Specifies the directory to build
    #[clap(short, long, default_value = ".")]
    dir: String,
}

impl Execute for Build {
    fn execute(&self, opts: &Opts) -> Result<()> {
        let dir = Path::new(&self.dir);
        let config = read_config(&self.config)?;



        Ok(())
    }
}
