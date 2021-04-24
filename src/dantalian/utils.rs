use anyhow::{anyhow, Result};
use std::path::Path;

pub fn path_str(path: &Path) -> Result<&str> {
    path.to_str().ok_or(anyhow!("path is not valid"))
}