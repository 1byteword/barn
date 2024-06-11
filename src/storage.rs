use std::fs::{self};
use std::path::Path;

pub fn ensure_dir_exists(path: &str) -> std::io::Result<()> {
    if !Path::new(path).exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}