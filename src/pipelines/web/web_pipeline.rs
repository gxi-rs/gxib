use crate::*;

pub const WEB_FEATURE: &str = "web";

/// web pipeline using wasm
pub async fn web_pipeline(_args: CliInterface, cargo_toml: &mut CargoToml) -> Result<()> {
    cargo_toml.add_features(vec![WEB_FEATURE.to_string()]);
    //TODO:
    // * Use webpack to serve
    // * Compile to wasm32-unknown-unknown
    // * use `wasm-opt -Oz` to shrink file size
    // * use --release and lto = true for release
    // * use wasm-pack
    println!("building web");
    Ok(())
}
