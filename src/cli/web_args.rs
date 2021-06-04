use clap::{AppSettings, Clap, ValueHint};
use std::path::PathBuf;

/// build for the web platform using wasm
#[derive(Clap)]
#[clap(
version = clap::crate_version!(),
author = "aniketfuryrocks <prajapati.ani306@gmail.com>",
setting = AppSettings::ColoredHelp
)]
pub struct WebArgs {
    /// start serving files at host:port
    #[clap(short, long, value_hint = ValueHint::Hostname)]
    pub serve: Option<String>,
    /// public dir containing static files eg. css served at /
    #[clap(lang = "public-dir", short, value_hint = ValueHint::DirPath, requires = "serve")]
    pub public_dir: Option<PathBuf>,
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
    #[clap(long = "target-dir", short, default_value = "target", value_hint = ValueHint::DirPath)]
    pub target_dir: PathBuf,
    /// output dir for cargo builds.
    #[clap(long = "output-dir", short, default_value = "target/.gxi", value_hint = ValueHint::DirPath)]
    pub output_dir: PathBuf,
}