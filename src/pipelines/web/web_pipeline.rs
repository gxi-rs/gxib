use crate::*;

pub const WEB_FEATURE: &str = "web";

/// web pipeline using wasm
pub async fn web_pipeline(args: CliInterface, cargo_toml: &mut CargoToml) -> Result<()> {
    cargo_toml.add_features(vec![WEB_FEATURE.to_string()]);
    let web_args = args.subcmd.as_web()?;

    println!("building web");
    Ok(())
}
