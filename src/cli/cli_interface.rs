use clap::{AppSettings, Clap};

use crate::*;

#[derive(Clap)]
#[clap(
version = "0.1.0",
author = "aniketfuryrocks <prajapati.ani306@gmail.com>"
)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct CliInterface {
    #[clap(short, long, default_value = "./")]
    pub dir: String,
    #[clap(subcommand)]
    pub subcmd: SubCommands,
}

#[derive(Clap)]
pub enum SubCommands {
    Web(WebArgs),
    Desktop(DesktopArgs),
}

impl SubCommands {
    pub fn as_web_mut(&mut self) -> Result<&mut WebArgs> {
        match self {
            SubCommands::Web(arg) => Ok(arg),
            _ => bail!("Expected subcommand to be web")
        }
    }
    pub fn as_desktop_mut(&mut self) -> Result<&mut DesktopArgs> {
        match self {
            SubCommands::Desktop(arg) => Ok(arg),
            _ => bail!("Expected subcommand to be web")
        }
    }
}
