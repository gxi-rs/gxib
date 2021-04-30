use clap::Clap;

/// build for the web platform using wasm
#[derive(Clap)]
pub struct WebArgs {
    #[clap(short, long)]
    pub serve: bool,
}
