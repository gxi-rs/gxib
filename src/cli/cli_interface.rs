use clap::{AppSettings, Clap};

use crate::cli::desktop_args::DesktopArgs;
use crate::cli::WebArgs;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "aniketfuryrocks <prajapati.ani306@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct CliInterface {
    #[clap(subcommand)]
    pub subcmds: SubCommands,
}

#[derive(Clap)]
pub enum SubCommands {
    Web(WebArgs),
    Desktop(DesktopArgs),
}
