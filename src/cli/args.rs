use super::{DesktopArgs, WebArgs};
use anyhow::{bail, Result};
use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
version = clap::crate_version ! (),
author = crate::author!()
)]
pub struct Args {
    /// project dir
    #[clap(long = "project-dir", short = 'd', value_hint = ValueHint::DirPath, default_value = "./")]
    pub project_dir: PathBuf,
    #[clap(subcommand)]
    pub sub_cmd: SubCommands,
}

#[derive(Parser)]
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
    pub fn as_web_mut(&mut self) -> Result<&mut WebArgs> {
        match self {
            SubCommands::Web(arg) => Ok(arg),
            _ => bail!("Expected subcommand to be web"),
        }
    }
    #[allow(dead_code)]
    pub fn as_desktop(&self) -> Result<&DesktopArgs> {
        match self {
            SubCommands::Desktop(arg) => Ok(arg),
            _ => bail!("Expected subcommand to be desktop"),
        }
    }
    #[allow(dead_code)]
    pub fn as_desktop_mut(&mut self) -> Result<&mut DesktopArgs> {
        match self {
            SubCommands::Desktop(arg) => Ok(arg),
            _ => bail!("Expected subcommand to be desktop"),
        }
    }
}
