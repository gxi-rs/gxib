use clap::{AppSettings, Clap, Subcommand};

use crate::cli::WebCmd;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "aniketfuryrocks <prajapati.ani306@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct CliInterface {
    #[clap(subcommand)]
    subcmds: SubCommands,
}

#[derive(Clap)]
pub enum SubCommands {
    Web(WebCmd),
}
