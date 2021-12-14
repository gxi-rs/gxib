use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs::read;

pub async fn get_file_hash(path: impl AsRef<Path>) -> Result<String> {
    Ok(seahash::hash(&read(&path).await.with_context(|| {
        format!(
            "error while reading file {}",
            path.as_ref().to_str().unwrap()
        )
    })?)
    .to_string())
}
