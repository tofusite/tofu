use anyhow::Result;
use clap::{crate_authors, crate_version, Clap};
use cmd::Execute;

use crate::cmd::SubCommand;

mod cmd;
mod site;

fn main() -> Result<()> {
    // Parse command line arguments
    let opts = Opts::parse();

    // Dispatch each subcommand to its respective handler
    match &opts.subcommand {
        SubCommand::Build(cmd) => cmd.execute(&opts),
        SubCommand::Init(cmd) => cmd.execute(&opts),
        SubCommand::Serve(cmd) => cmd.execute(&opts),
    }
}

/// A modern static site generator.
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}
