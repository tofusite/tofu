use std::path::Path;

use self::config::Config;

pub mod config;

pub fn build<P: AsRef<Path>>(context: P, config: Config) {
    println!("{:?}", config);
}
