use std::env::current_dir;

use anyhow::{anyhow, bail, Context, Result};
use argh::FromArgs;

use crate::Args;

use super::Execute;

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "init",
    description = "create a new tofu project in the current directory"
)]
pub struct Init {
    #[argh(
        option,
        description = "force tofu to generate a new project, regardless of existing files in the directory",
        default = "false"
    )]
    force: bool,
}

impl Execute for Init {
    fn execute(&self, args: &Args) -> Result<()> {
        let dir = current_dir()?;

        if dir.read_dir()?.next().is_some() && !self.force {
            bail!(
                "Can't create a new tofu project in a non-empty directory. Use --force to skip this check."
            );
        }

        todo!()
    }
}
