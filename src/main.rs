use clap::Clap;

use crate::cli::CliInterface;
use crate::cli::SubCommands::Web;
use crate::pipelines::web::web_pipeline::web_pipeline;

mod cli;
mod pipelines;

fn main() {
    let cli_interface: CliInterface = CliInterface::parse();
    match cli_interface.subcmds {
        Web(web_cmd) => web_pipeline(web_cmd)
    }
}
