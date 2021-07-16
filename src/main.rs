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

#[derive(FromArgs)]
#[argh(description = "A smarter static site generator.")]
pub struct Args {
    #[argh(subcommand)]
    command: Command,
}
