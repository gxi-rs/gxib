use crate::cli::DesktopArgs;
use crate::utils::CargoToml;

pub const DESKTOP_FEATURE: &str = "desktop";

/// desktop pipeline using gtk
pub fn desktop_pipeline(_args: DesktopArgs, cargo_toml: &mut CargoToml) {
    cargo_toml.add_features(vec![DESKTOP_FEATURE.to_string()]);
    println!("building desktop");
}
