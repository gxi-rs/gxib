use clap::Parser;
use cli::SubCommands;
use log::info;
use simplelog::*;

use crate::pipelines::{DesktopPipeline, WebPipeline};
use anyhow::{Context, Result};
use cli::Args;

mod cli;
mod macros;
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
