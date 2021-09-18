use anyhow::Result;
use argh::FromArgs;

use crate::Args;

pub mod build;
pub mod init;
pub mod serve;

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Command {
    Build(build::Build),
    Init(init::Init),
    Serve(serve::Serve),
}

impl Command {
    pub fn execute(&self, args: &Args) -> Result<()> {
        match &self {
            Command::Build(cmd) => cmd.execute(args),
            Command::Init(cmd) => cmd.execute(args),
            Command::Serve(cmd) => cmd.execute(args),
        }
    }
}

pub trait Execute {
    fn execute(&self, args: &Args) -> Result<()>;
}

pub fn default_config_file() -> String {
    String::from("./tofu.toml")
}

pub fn default_dir() -> String {
    String::from(".")
}
