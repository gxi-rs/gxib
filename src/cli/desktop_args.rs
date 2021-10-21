use clap::Parser;

/// build for the desktop platform using gtk
#[derive(Parser)]
#[clap(
version = clap::crate_version!(),
author = crate::author!()
)]
pub struct DesktopArgs {}
