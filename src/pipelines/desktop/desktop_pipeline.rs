use crate::cli::Args;
use crate::utils::exec_cmd;
use crate::*;

/// desktop pipeline using gtk
pub struct DesktopPipeline<'a> {
    pub args: &'a Args,
}

impl DesktopPipeline<'_> {
    pub async fn run(&mut self) -> Result<()> {
        exec_cmd("cargo", &["run"], Some(&self.args.project_dir), None).await?;
        Ok(())
    }
}
