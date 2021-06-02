use clap::{AppSettings, Clap};

/// build for the desktop platform using gtk
#[derive(Clap)]
#[clap(
version = clap::crate_version!(),
author = "aniketfuryrocks <prajapati.ani306@gmail.com>",
setting = AppSettings::ColoredHelp
)]
pub struct DesktopArgs {}
