use std::path::Path;
use std::process::Command;

use crate::cli::{DesktopArgs, CliInterface};
use crate::utils::CargoToml;

pub const DESKTOP_FEATURE: &str = "desktop";

/// desktop pipeline using gtk
pub fn desktop_pipeline(args: CliInterface, cargo_toml: &mut CargoToml) {
    cargo_toml.add_features(vec![DESKTOP_FEATURE.to_string()]);
    run(Path::new(&args.dir));
}

fn run(base_dir: &Path) {
    let out = Command::new("cargo")
        .args(&["run"])
        .current_dir(&base_dir)
        .output()
        .expect("failed to execute process");
}
