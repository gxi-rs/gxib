use crate::cli::{WebArgs, CliInterface};
use crate::utils::CargoToml;

pub const WEB_FEATURE: &str = "web";

/// web pipeline using wasm
pub fn web_pipeline(_args: CliInterface, cargo_toml: &mut CargoToml) {
    cargo_toml.add_features(vec![WEB_FEATURE.to_string()]);
    println!("building web");
}
