use anyhow::Result;
use clap::Clap;

use crate::Opts;

use super::Execute;

/// Creates a new tofu project in the current directory
#[derive(Clap)]
pub struct Init {

}

impl Execute for Init {
    fn execute(&self, opts: &Opts) -> Result<()> {
        todo!()
    }
}
