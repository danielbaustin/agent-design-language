use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

pub(super) fn collect_files(root: &Path) -> Result<Vec<PathBuf>> {
    fn visit(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        let mut entries = fs::read_dir(dir)
            .with_context(|| format!("failed to read demo artifact dir '{}'", dir.display()))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let path = entry.path();
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                visit(root, &path, files)?;
            } else if file_type.is_file() {
                files.push(path.strip_prefix(root)?.to_path_buf());
            }
        }
        Ok(())
    }

    let mut files = Vec::new();
    visit(root, root, &mut files)?;
    files.sort();
    Ok(files)
}

pub(super) fn collect_existing_files(root: &Path) -> Result<Vec<PathBuf>> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    Ok(collect_files(root)?
        .into_iter()
        .map(|rel| root.join(rel))
        .collect())
}
