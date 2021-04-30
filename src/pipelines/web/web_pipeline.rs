use crate::cli::WebArgs;
use crate::utils::CargoToml;

pub const WEB_FEATURE: &str = "web";

/// web pipeline using wasm
pub fn web_pipeline(_args: WebArgs, cargo_toml: &mut CargoToml) {
    cargo_toml.add_features(vec![WEB_FEATURE.to_string()]);
    println!("building web");
}
