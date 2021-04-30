use clap::Clap;

use crate::cli::CliInterface;

mod cli;
mod pipelines;

fn main() {
    let cli_interface: CliInterface = CliInterface::parse();
    // cli_interface.
}
