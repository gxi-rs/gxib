use std::fs::{read, write};
use std::path::Path;

use clap::Clap;

use crate::cli::{CliInterface, SubCommands::*};
use crate::pipelines::desktop_pipeline;
use crate::pipelines::web_pipeline;
use crate::utils::parse_cargo_toml;

mod cli;
mod pipelines;
mod utils;

fn main() {
    // get the command line arguments
    let cli_interface: CliInterface = CliInterface::parse();
    // parse cargo.toml
    let cargo_toml_path = &format!("{}/Cargo.toml", cli_interface.dir)[..];
    let cargo_toml_path = Path::new(cargo_toml_path);

    let toml_parse = {
        parse_cargo_toml(&read(&cargo_toml_path).unwrap())
    };
    match cli_interface.subcmds {
        Web(web_cmd) => web_pipeline(web_cmd),
        Desktop(desktop_args) => desktop_pipeline(desktop_args),
    }
    // write to Cargo.toml file
    write(&cargo_toml_path, toml_parse.to_formatted_string());
}
