use anyhow::Result;
use argh::FromArgs;

use crate::cmd::Command;

mod cmd;
mod site;

fn main() -> Result<()> {
    // Parse command line arguments
    let args: Args = argh::from_env();

    // Dispatch each subcommand to its respective handler
    args.command.execute(&args)
}

/// A modern static site generator.
#[derive(FromArgs)]
pub struct Args {
    #[argh(subcommand)]
    command: Command,
}
