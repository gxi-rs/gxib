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

#[tokio::main]
async fn main() -> Result<()> {
    // get the command line arguments
    let cli_interface: CliInterface = CliInterface::parse();
    // parse cargo.toml
    let cargo_toml_path = &format!("{}/Cargo.toml", cli_interface.dir)[..];
    let cargo_toml_path = Path::new(cargo_toml_path);
    // parse Cargo.toml
    let mut cargo_toml = CargoToml::new(
        &read(&cargo_toml_path)
            .await
            .with_context(|| "Error reading Cargo.toml file")?,
    ).with_context(|| "Error parsing Cargo.toml file")?;
    // match sub commands
    match cli_interface.subcmd {
        SubCommands::Web(_) => web_pipeline(cli_interface, &mut cargo_toml)
            .await
            .with_context(|| "Error running web pipeline")?,
        _ => desktop_pipeline(cli_interface, &mut cargo_toml)
            .await
            .with_context(|| "Error running desktop pipeline")?,
    };
    // write to Cargo.toml file
    write(&cargo_toml_path, cargo_toml.to_string())
        .await
        .with_context(|| "Error writing to Cargo.toml file")?;
    Ok(())
}
