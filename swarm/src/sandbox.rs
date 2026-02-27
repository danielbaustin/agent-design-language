use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SandboxPathError {
    EmptyPath,
    AbsolutePath { path: String },
    ParentTraversal { path: String },
    RootCanonicalizeFailed { root: String },
    PathOutsideRoot { path: String },
    SymlinkEscape { path: String },
}

impl SandboxPathError {
    pub fn message(&self) -> String {
        match self {
            Self::EmptyPath => "path must be non-empty".to_string(),
            Self::AbsolutePath { path } => {
                format!("path must be relative (got absolute path '{}')", path)
            }
            Self::ParentTraversal { path } => {
                format!("path must not contain '..' traversal segments ('{}')", path)
            }
            Self::RootCanonicalizeFailed { root } => {
                format!("failed to canonicalize sandbox root '{}'", root)
            }
            Self::PathOutsideRoot { path } => {
                format!("path resolves outside sandbox root ('{}')", path)
            }
            Self::SymlinkEscape { path } => {
                format!(
                    "path contains a symlink escape outside sandbox root ('{}')",
                    path
                )
            }
        }
    }
}

impl std::fmt::Display for SandboxPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for SandboxPathError {}

fn validate_relative(path: &Path) -> Result<(), SandboxPathError> {
    let raw = path.display().to_string();
    if raw.trim().is_empty() {
        return Err(SandboxPathError::EmptyPath);
    }
    if path.is_absolute() {
        return Err(SandboxPathError::AbsolutePath { path: raw });
    }
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(SandboxPathError::ParentTraversal { path: raw });
    }
    Ok(())
}

pub fn resolve_existing_path_within_root(
    root: &Path,
    candidate: &Path,
) -> Result<PathBuf, SandboxPathError> {
    let root_canon = root
        .canonicalize()
        .map_err(|_| SandboxPathError::RootCanonicalizeFailed {
            root: root.display().to_string(),
        })?;

    // Absolute paths are allowed only if they still resolve under sandbox root.
    if !candidate.is_absolute() {
        validate_relative(candidate)?;
    }
    let joined = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(candidate)
    };
    let canonical = joined
        .canonicalize()
        .map_err(|_| SandboxPathError::SymlinkEscape {
            path: candidate.display().to_string(),
        })?;
    if !canonical.starts_with(&root_canon) {
        return Err(SandboxPathError::PathOutsideRoot {
            path: candidate.display().to_string(),
        });
    }
    Ok(canonical)
}

pub fn resolve_relative_path_for_write_within_root(
    root: &Path,
    rel: &Path,
) -> Result<PathBuf, SandboxPathError> {
    // Security note (TOCTOU): this resolver is best-effort path hardening, not
    // full OS-level isolation. It validates canonical ancestry at resolution
    // time, but cannot prevent all post-check filesystem races without kernel-
    // enforced sandboxing primitives.
    validate_relative(rel)?;
    let root_canon = root
        .canonicalize()
        .map_err(|_| SandboxPathError::RootCanonicalizeFailed {
            root: root.display().to_string(),
        })?;
    let candidate = root.join(rel);
    if candidate.exists() {
        let canonical = candidate
            .canonicalize()
            .map_err(|_| SandboxPathError::SymlinkEscape {
                path: rel.display().to_string(),
            })?;
        if !canonical.starts_with(&root_canon) {
            return Err(SandboxPathError::PathOutsideRoot {
                path: rel.display().to_string(),
            });
        }
        return Ok(candidate);
    }

    // Non-existent write targets are allowed if the nearest existing ancestor
    // canonicalizes under sandbox root.
    let mut ancestor = candidate.as_path();
    while !ancestor.exists() {
        ancestor = ancestor
            .parent()
            .ok_or_else(|| SandboxPathError::SymlinkEscape {
                path: rel.display().to_string(),
            })?;
    }
    let ancestor_canon = ancestor
        .canonicalize()
        .map_err(|_| SandboxPathError::SymlinkEscape {
            path: rel.display().to_string(),
        })?;
    if !ancestor_canon.starts_with(&root_canon) {
        return Err(SandboxPathError::SymlinkEscape {
            path: rel.display().to_string(),
        });
    }
    Ok(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{stamp}"))
    }

    #[test]
    fn resolve_existing_allows_in_root_paths() {
        let root = temp_dir("swarm-sandbox-root-ok");
        fs::create_dir_all(&root).expect("root");
        let file = root.join("in.txt");
        fs::write(&file, "ok").expect("write");
        let resolved =
            resolve_existing_path_within_root(&root, Path::new("in.txt")).expect("resolve");
        assert!(resolved.starts_with(root.canonicalize().expect("canon root")));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resolve_existing_rejects_parent_traversal() {
        let root = temp_dir("swarm-sandbox-parent");
        fs::create_dir_all(&root).expect("root");
        let err = resolve_existing_path_within_root(&root, Path::new("../escape.txt"))
            .expect_err("must reject parent traversal");
        assert_eq!(
            err,
            SandboxPathError::ParentTraversal {
                path: "../escape.txt".to_string()
            }
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resolve_for_write_rejects_absolute_paths_cross_platform() {
        let root = temp_dir("swarm-sandbox-abs");
        fs::create_dir_all(&root).expect("root");
        let abs = std::env::temp_dir().join("outside-target.txt");
        let err = resolve_relative_path_for_write_within_root(&root, &abs)
            .expect_err("must reject absolute");
        assert!(matches!(err, SandboxPathError::AbsolutePath { .. }));
        let _ = fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn resolve_for_write_rejects_symlink_escape() {
        use std::os::unix::fs as unix_fs;

        let base = temp_dir("swarm-sandbox-symlink");
        let root = base.join("root");
        let outside = base.join("outside");
        fs::create_dir_all(&root).expect("root");
        fs::create_dir_all(&outside).expect("outside");
        unix_fs::symlink(&outside, root.join("link")).expect("symlink");

        let err = resolve_relative_path_for_write_within_root(&root, Path::new("link/secret.txt"))
            .expect_err("symlink escape must fail");
        assert!(matches!(err, SandboxPathError::SymlinkEscape { .. }));

        let _ = fs::remove_dir_all(base);
    }
}
