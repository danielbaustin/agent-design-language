use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let out_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/v086/candidate_selection"));
    fs::create_dir_all(&out_dir).with_context(|| {
        format!(
            "create candidate selection demo dir '{}'",
            out_dir.display()
        )
    })?;
    adl::demo::write_v086_candidate_selection_demo(&out_dir)?;
    println!("{}", out_dir.display());
    Ok(())
}
