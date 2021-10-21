use clap::{Parser, ValueHint};
use std::path::PathBuf;

/// build for the web platform using wasm
#[derive(Parser)]
#[clap(
version = clap::crate_version!(),
author = crate::author!()
)]
pub struct WebArgs {
    /// start serving files at host:port
    #[clap(short, long, value_hint = ValueHint::Hostname)]
    pub serve: Option<String>,
    /// public dir containing static files eg. css served at /
    #[clap(long = "public-dir", short, value_hint = ValueHint::DirPath, requires = "serve")]
    pub public_dir: Option<PathBuf>,
    /// build on file change
    #[clap(short, long)]
    pub watch: bool,
    /// hot reload build files
    #[clap(
        short = 'r',
        long = "hot-reload",
        requires = "serve",
        requires = "watch"
    )]
    pub hot_reload: bool,
    /// production build
    #[clap(long)]
    pub release: bool,
    /// output dir to keep build artifacts
    #[clap(long = "output-dir", short, default_value = "target/.gxi", value_hint = ValueHint::DirPath)]
    pub output_dir: PathBuf,
}
