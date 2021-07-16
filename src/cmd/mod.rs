use anyhow::Result;
use clap::Clap;

use crate::Opts;

pub mod build;
pub mod init;
pub mod serve;

#[derive(Clap)]
pub enum SubCommand {
    Build(build::Build),
    Init(init::Init),
    Serve(serve::Serve)
}

pub trait Execute {
    fn execute(&self, opts: &Opts) -> Result<()>;
}
