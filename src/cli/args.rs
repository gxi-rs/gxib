use clap::{AppSettings, Clap, ValueHint};

use crate::*;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(
version = clap::crate_version ! (),
author = "aniketfuryrocks <prajapati.ani306@gmail.com>",
setting = AppSettings::ColoredHelp
)]
pub struct Args {
    /// project dir
    #[clap(long = "project-dir", short = 'd', value_hint = ValueHint::DirPath, default_value = "./")]
    pub project_dir: PathBuf,
    #[clap(subcommand)]
    pub sub_cmd: SubCommands,
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
