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
    // match sub commands
    match args.sub_cmd {
        //web
        SubCommands::Web(_) => {
            info!("Building Web App");
            let web_pipeline = WebPipeline::new(args).await?;
            WebPipeline::run(web_pipeline)
                .await
                .with_context(|| "Error running web pipeline")?;
        }
        //desktop
        _ => {
            info!("Building Desktop App");
            DesktopPipeline { args: &args }
                .run()
                .await
                .with_context(|| "Error running desktop pipeline")?;
        }
    };
    Ok(())
}
