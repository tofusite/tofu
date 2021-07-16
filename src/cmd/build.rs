use std::path::Path;

use anyhow::Result;
use argh::FromArgs;

use crate::{Args, site::config::read_config};

use super::Execute;

#[derive(FromArgs)]
#[argh(subcommand, name = "build", description = "build the tofu site for production")]
pub struct Build {
    #[argh(option, description = "the config file to use")]
    config: String,
    #[argh(option, description = "the directory to build")]
    dir: String,
}

impl Execute for Build {
    fn execute(&self, args: &Args) -> Result<()> {
        let dir = Path::new(&self.dir);
        let config = read_config(&self.config)?;

        Ok(())
    }
}
