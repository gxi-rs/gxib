use std::path::Path;

use tokio::process::Command;

use crate::*;

pub async fn exec_cmd(
    name: &str,
    args: &[&str],
    current_dir: Option<impl AsRef<Path>>,
) -> Result<()> {
    let mut cmd = Command::new(name);
    if let Some(dir) = current_dir {
        cmd.current_dir(dir);
    }
    let child = cmd
        .args(args)
        .spawn()
        .with_context(|| format!("error running `{} {:?}`", name, args))?
        .wait()
        .await
        .with_context(|| format!("error waiting for `{} {:?}` to end", name, args))?;
    if !child.success() {
        bail!("`{}{:?}` didn't exit gracefully", name, args)
    }
    Ok(())
}
