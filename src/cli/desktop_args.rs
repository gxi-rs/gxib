use clap::{AppSettings, Clap};

/// build for the desktop platform using gtk
#[derive(Clap)]
#[clap(
version = crate::VERSION,
author = "aniketfuryrocks <prajapati.ani306@gmail.com>",
setting = AppSettings::ColoredHelp
)]
pub struct DesktopArgs {}
