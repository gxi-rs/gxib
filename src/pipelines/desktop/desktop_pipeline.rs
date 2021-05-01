use crate::cli::Args;
use crate::utils::{exec_cmd, CargoToml};
use crate::*;

pub const DESKTOP_FEATURE: &str = "desktop";

/// desktop pipeline using gtk
pub struct DesktopPipeline<'a> {
    pub args: &'a Args,
    pub cargo_toml: &'a mut CargoToml,
}

impl DesktopPipeline<'_> {
    pub async fn run(&mut self) -> Result<()> {
        // write desktop feature
        {
            self.cargo_toml
                .add_features(vec![DESKTOP_FEATURE.to_string()]);
            self.cargo_toml.write_to_file().await?;
        }
        exec_cmd("cargo", &["run"], Some(&self.args.dir)).await?;
        Ok(())
    }
}
