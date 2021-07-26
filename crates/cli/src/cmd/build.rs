use anyhow::Result;
use argh::FromArgs;

use crate::cmd::{default_config_file, default_dir};
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
    fn execute(&self, _args: &Args) -> Result<()> {
        let _config = &self.config;
        let _dir = &self.dir;
        let _out = &self.out;
        Ok(())
    }
}
