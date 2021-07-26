use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use anyhow::{Context, Result};
use argh::FromArgs;
use tokio::runtime::Runtime;
use warp::Filter;

use crate::Args;

use super::Execute;

const OUT_DIR: &str = "./.tofu/serve_output";

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "serve",
    description = "run a tofu development server"
)]
pub struct Serve {
    #[argh(
        option,
        description = "the config file to use",
        default = "String::from(\"./tofu.toml\")"
    )]
    config: String,
    #[argh(
        option,
        description = "the directory to build",
        default = "String::from(\".\")"
    )]
    dir: String,
    #[argh(
        option,
        description = "the host to bind the development server to",
        default = "String::from(\"0.0.0.0\")"
    )]
    host: String,
    #[argh(
        option,
        description = "the port to run a development server on",
        default = "8080"
    )]
    port: u16,
}

impl Execute for Serve {
    fn execute(&self, _args: &Args) -> Result<()> {
        let out = PathBuf::from(OUT_DIR);
        let _dir = Path::new(&self.dir);
        let _config = &self.config;

        let host = &self.host;
        let port = self.port;

        let bind_string = format!("{}:{}", host, port);
        let bind = bind_string
            .parse()
            .with_context(|| format!("Invalid bind address format: {}", bind_string))?;

        create_http_server(bind, out);

        loop {
            std::thread::sleep(Duration::from_millis(1000));
        }
    }
}

/// Create the static file server used to host files for the dev server.
fn create_http_server(bind: SocketAddr, dir: PathBuf) {
    thread::spawn(move || {
        // This Filter listens on "/" and serves files from the output directory
        let root = warp::get().and(warp::fs::dir(dir));

        // Spawn a tokio runtime and block on the warp server
        let rt = Runtime::new().expect("Unable to spawn tokio runtime");
        rt.block_on(warp::serve(root).run(bind));
    });
}
