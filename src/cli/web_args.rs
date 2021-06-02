use clap::{AppSettings, Clap, ValueHint};
use std::path::PathBuf;
use std::str::FromStr;
use crate::*;

/// build for the web platform using wasm
#[derive(Clap)]
#[clap(
version = crate::VERSION,
author = "aniketfuryrocks <prajapati.ani306@gmail.com>",
setting = AppSettings::ColoredHelp
)]
pub struct WebArgs {
    /// start development server at host:port Syntax -> host:port where port is of type u32. Eg -> localhost:8080
    /// value "." corresponds to localhost:8080
    #[clap(short, long, value_hint = ValueHint::Hostname)]
    pub serve: Option<WebServeArgs>,
    /// build on file change
    #[clap(short, long)]
    pub watch: bool,
    /// hot reload build files
    #[clap(short = 'r', long = "hot-reload", requires = "serve", requires = "watch")]
    pub hot_reload: bool,
    /// production build
    #[clap(long)]
    pub release: bool,
    /// target dir for cargo builds.
    #[clap(long = "target-dir", default_value = "target", value_hint = ValueHint::DirPath)]
    pub target_dir: PathBuf,
    /// output dir for cargo builds.
    #[clap(long = "output-dir", short, default_value = "target/.gxi", value_hint = ValueHint::DirPath)]
    pub output_dir: PathBuf,
}

#[derive(Clap, Debug)]
pub struct WebServeArgs {
    pub hostname: String,
    pub port: u32,
}

impl FromStr for WebServeArgs {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "." {
            return Ok(Self {
                hostname: "localhost".into(),
                port: 8080
            })
        }
        let split = input.split(":").collect::<Vec<&str>>();
        if split.len() != 2 {
            anyhow::bail!("Syntax -> host:port where port is of type u32.")
        }
        Ok(Self {
            hostname: split[0].into(),
            port: u32::from_str(split[1])
                .with_context(|| "port should be of type u32")?,
        })
    }
}
