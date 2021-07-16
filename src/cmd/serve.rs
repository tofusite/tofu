use std::path::Path;

use anyhow::Result;
use clap::Clap;

use crate::{Opts, site::config::read_config};

use super::Execute;

/// Runs a tofu development server
#[derive(Clap)]
pub struct Serve {
    /// Specifies the config file to use
    #[clap(short, long, default_value = "./tofu.toml")]
    config: String,
    
    /// Specifies the directory to build
    #[clap(short, long, default_value = ".")]
    dir: String,

    /// Specifies the port to serve from
    #[clap(short, long, default_value = "8080")]
    port: u16
}

impl Execute for Serve {
    fn execute(&self, opts: &Opts) -> Result<()> {
        let dir = Path::new(&self.dir);
        let config = read_config(&self.config)?;
        
        Ok(())
    }
}
