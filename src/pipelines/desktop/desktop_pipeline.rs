use crate::*;

use crate::cli::CliInterface;
use crate::utils::{CargoToml, exec_cmd};

pub const DESKTOP_FEATURE: &str = "desktop";

/// desktop pipeline using gtk
pub async fn desktop_pipeline(args: CliInterface, cargo_toml: &mut CargoToml) -> Result<()>{
    cargo_toml.add_features(vec![DESKTOP_FEATURE.to_string()]);
    exec_cmd("cargo", &["run"], Some(args.dir)).await?;
    Ok(())
}

