use crate::cli::{CliInterface};
use crate::utils::CargoToml;

pub const WEB_FEATURE: &str = "web";

/// web pipeline using wasm
pub fn web_pipeline(_args: CliInterface, cargo_toml: &mut CargoToml) {
    cargo_toml.add_features(vec![WEB_FEATURE.to_string()]);
    ///TODO:
    /// * Use webpack to serve
    /// * Compile to wasm32-unknown-unknown
    /// * use `wasm-opt -Oz` to shrink file size
    /// * use --release and lto = true for release
    /// * use wasm-pack
    println!("building web");
}
