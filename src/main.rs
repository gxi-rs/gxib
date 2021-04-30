use clap::Clap;

use crate::cli::{CliInterface, SubCommands::{*}};
use crate::pipelines::desktop_pipeline;
use crate::pipelines::web_pipeline;

mod cli;
mod pipelines;

fn main() {
    let cli_interface: CliInterface = CliInterface::parse();
    match cli_interface.subcmds {
        Web(web_cmd) => web_pipeline(web_cmd),
        Desktop(desktop_args) => desktop_pipeline(desktop_args)
    }
}
