use std::path::Path;

use clap::Clap;
use simplelog::*;
use tokio::fs::{read, write};

pub use crate::cli::*;
pub use crate::pipelines::*;
pub use crate::utils::*;
pub use anyhow::*;
pub use log::*;

mod cli;
mod pipelines;
mod utils;

pub const CARGO_TOML: &str = "Cargo.toml";

#[tokio::main]
async fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    // get the command line arguments
    let args: Args = Args::parse();
    // parse Cargo.toml
    let mut cargo_toml = CargoToml::from_file(Path::new(&args.dir).join(CARGO_TOML)).await?;
    // match sub commands
    match args.subcmd {
        //web
        SubCommands::Web(_) => {
            info!("Building Web App");
            let web_pipeline = WebPipeline::new(args, cargo_toml).await?;
            WebPipeline::run(web_pipeline)
                .await
                .with_context(|| "Error running web pipeline")?;
        }
        //desktop
        _ => {
            info!("Building Desktop App");
            DesktopPipeline {
                args: &args,
                cargo_toml: &mut cargo_toml,
            }
            .run()
            .await
            .with_context(|| "Error running desktop pipeline")?;
        }
    };
    Ok(())
}
