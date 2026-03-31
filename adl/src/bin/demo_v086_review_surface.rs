use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let out_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/v086/review_surface"));
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("create review surface demo dir '{}'", out_dir.display()))?;
    adl::demo::write_v086_review_surface_demo(&out_dir)?;
    println!("{}", out_dir.display());
    Ok(())
}
