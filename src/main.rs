use std::fs::{read, write};
use std::path::Path;

use clap::Clap;

use crate::cli::{CliInterface, SubCommands::*};
use crate::pipelines::desktop_pipeline;
use crate::pipelines::web_pipeline;
use crate::utils::CargoToml;

mod cli;
mod pipelines;
mod utils;

fn main() {
    // get the command line arguments
    let cli_interface: CliInterface = CliInterface::parse();
    // parse cargo.toml
    let cargo_toml_path = &format!("{}/Cargo.toml", cli_interface.dir)[..];
    let cargo_toml_path = Path::new(cargo_toml_path);
    // parse Cargo.toml
    let mut cargo_toml = { CargoToml::new(&read(&cargo_toml_path).expect("Error reading Cargo.toml file")) };
    // match sub commands
    match cli_interface.subcmds {
        Web(web_cmd) => web_pipeline(web_cmd, &mut cargo_toml),
        Desktop(desktop_args) => desktop_pipeline(desktop_args, &mut cargo_toml),
    }
    // write to Cargo.toml file
    write(&cargo_toml_path, cargo_toml.to_string()).expect("Error writing to Cargo.toml file");
}
