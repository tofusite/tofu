use std::path::Path;

use anyhow::Result;
use argh::FromArgs;

use crate::cmd::{default_config_file, default_dir};
use crate::site::build;
use crate::site::config::Config;
use crate::Args;

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
    #[argh(
        option,
        description = "the directory to output to",
        default = "String::from(\"./public\")"
    )]
    out: String,
}

impl Execute for Build {
    fn execute(&self, args: &Args) -> Result<()> {
        let config = Config::read(&self.config)?;
        let dir = Path::new(&self.dir);
        let out = Path::new(&self.out);

        build(config, dir, out)?;

        Ok(())
    }
}
