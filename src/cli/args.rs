use clap::{AppSettings, Clap};

use crate::*;

#[derive(Clap)]
#[clap(
    version = VERSION,
    author = "aniketfuryrocks <prajapati.ani306@gmail.com>",
    setting = AppSettings::ColoredHelp
)]
pub struct Args {
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
    pub fn as_web(&self) -> Result<&WebArgs> {
        match self {
            SubCommands::Web(arg) => Ok(arg),
            _ => bail!("Expected subcommand to be web"),
        }
    }
    pub fn as_desktop(&self) -> Result<&DesktopArgs> {
        match self {
            SubCommands::Desktop(arg) => Ok(arg),
            _ => bail!("Expected subcommand to be web"),
        }
    }
    pub fn as_web_mut(&mut self) -> Result<&mut WebArgs> {
        match self {
            SubCommands::Web(arg) => Ok(arg),
            _ => bail!("Expected subcommand to be web"),
        }
    }
    pub fn as_desktop_mut(&mut self) -> Result<&mut DesktopArgs> {
        match self {
            SubCommands::Desktop(arg) => Ok(arg),
            _ => bail!("Expected subcommand to be web"),
        }
    }
}