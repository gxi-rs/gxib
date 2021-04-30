use crate::cli::DesktopArgs;
use crate::utils::CargoToml;

/// desktop pipeline using gtk
pub fn desktop_pipeline(_args: DesktopArgs, cargo_toml: &mut CargoToml) {
    println!("building desktop");
}
