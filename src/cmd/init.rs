use anyhow::Result;
use argh::FromArgs;

use crate::Args;

use super::Execute;

/// Creates a new tofu project in the current directory
#[derive(FromArgs)]
#[argh(subcommand, name = "init")]
pub struct Init {

}

impl Execute for Init {
    fn execute(&self, args: &Args) -> Result<()> {
        todo!()
    }
}
