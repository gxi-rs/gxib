use std::path::Path;

use clap::Clap;
use serde;

use crate::cli::{CliInterface, SubCommands::{*}};
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
    let toml_parse = parse_cargo_toml(Path::new(&format!("{}/Cargo.toml", cli_interface.dir)[..]));
    match cli_interface.subcmds {
        Web(web_cmd) => web_pipeline(web_cmd),
        Desktop(desktop_args) => desktop_pipeline(desktop_args)
    }
    println!("{}", toml::to_string_pretty(&toml_parse).unwrap());
}
