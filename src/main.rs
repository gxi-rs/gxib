use std::path::Path;

pub use anyhow::*;
use clap::Clap;
use tokio::fs::{read, write};

pub use crate::cli::*;
pub use crate::pipelines::*;
pub use crate::utils::*;

mod cli;
mod pipelines;
mod utils;

pub const CARGO_TOML: &str = "Cargo.toml";

#[tokio::main]
async fn main() -> Result<()> {
    // get the command line arguments
    let args: Args = Args::parse();
    // parse Cargo.toml
    let mut cargo_toml = CargoToml::from_file(Path::new(&args.dir).join(CARGO_TOML)).await?;
    // match sub commands
    match args.subcmd {
        //web
        SubCommands::Web(_) => WebPipeline {
            args: &args,
            cargo_toml: &mut cargo_toml,
        }
        .run()
        .await
        .with_context(|| "Error running web pipeline")?,
        //desktop
        _ => DesktopPipeline {
            args: &args,
            cargo_toml: &mut cargo_toml,
        }
        .run()
        .await
        .with_context(|| "Error running desktop pipeline")?,
    };
    Ok(())
}
