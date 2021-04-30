use std::path::Path;
use std::process::{Command, Stdio};

use crate::cli::CliInterface;
use crate::utils::CargoToml;

pub const DESKTOP_FEATURE: &str = "desktop";

/// desktop pipeline using gtk
pub fn desktop_pipeline(args: CliInterface, cargo_toml: &mut CargoToml) {
    cargo_toml.add_features(vec![DESKTOP_FEATURE.to_string()]);
    run(Path::new(&args.dir));
}

fn run(base_dir: &Path) {
    Command::new("cargo")
        .args(&["run"])
        .current_dir(&base_dir.canonicalize().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to run cargo. Make sure cargo is installed and is available in path");
}
