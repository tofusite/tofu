use anyhow::Result;
use argh::FromArgs;

use crate::Args;

use super::Execute;

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "init",
    description = "create a new tofu project in the current directory"
)]
pub struct Init {}

impl Execute for Init {
    fn execute(&self, args: &Args) -> Result<()> {
        todo!()
    }
}
