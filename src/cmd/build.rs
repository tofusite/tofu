use std::path::Path;

use anyhow::Result;
use argh::FromArgs;

use crate::cmd::{default_config_file, default_dir};
use crate::{site::config::read_config, Args};

use super::Execute;

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "build",
    description = "build the tofu site for production"
)]
pub struct Build {
    #[argh(
        option,
        description = "the config file to use",
        default = "default_config_file()"
    )]
    config: String,
    #[argh(
        option,
        description = "the directory to build",
        default = "default_dir()"
    )]
    dir: String,
}

impl Execute for Build {
    fn execute(&self, args: &Args) -> Result<()> {
        let dir = Path::new(&self.dir);
        let config = read_config(&self.config)?;

        Ok(())
    }
}
