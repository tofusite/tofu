use std::path::Path;

use anyhow::Result;
use argh::FromArgs;

use crate::{Args, site::config::read_config};

use super::Execute;

#[derive(FromArgs)]
#[argh(subcommand, name = "build", description = "run a tofu development server")]
pub struct Serve {
    #[argh(option, description = "the config file to use")]
    config: String,
    #[argh(option, description = "the directory to build")]
    dir: String,
    #[argh(option, description = "the port to run a development server on")]
    port: u16
}

impl Execute for Serve {
    fn execute(&self, args: &Args) -> Result<()> {
        let dir = Path::new(&self.dir);
        let config = read_config(&self.config)?;
        
        Ok(())
    }
}
