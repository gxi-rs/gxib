use clap::Clap;

/// build for the web platform
#[derive(Clap)]
pub struct WebArgs {
    #[clap(short, long, default_value = true)]
    serve: bool
}