use clap::{AppSettings, Clap};

/// build for the web platform using wasm
#[derive(Clap)]
#[clap(
version = crate::VERSION,
author = "aniketfuryrocks <prajapati.ani306@gmail.com>",
setting = AppSettings::ColoredHelp
)]
pub struct WebArgs {
    /// start development server
    #[clap(short, long)]
    pub serve: bool,
    /// build on file change
    #[clap(short,long)]
    pub watch: bool,
    /// hot reload build files
    #[clap(short='r',long="hot-reload")]
    pub hot_reload: bool,
    /// production build
    #[clap(long)]
    pub release: bool,
    /// target dir for cargo builds.
    #[clap(long = "target-dir", default_value = "target")]
    pub target_dir: String,
    /// output dir for cargo builds.
    #[clap(long = "output-dir", short, default_value = "target/.gxi")]
    pub output_dir: String, 
}
