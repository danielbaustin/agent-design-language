use std::path::{Component, Path, PathBuf};

/// Best-effort filesystem policy for sandbox path resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SandboxPathPolicy {
    pub allow_symlink_traversal: bool,
}

impl Default for SandboxPathPolicy {
    fn default() -> Self {
        Self {
            allow_symlink_traversal: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SandboxPathError {
    PathDenied {
        requested_path: String,
        reason: &'static str,
    },
    PathNotFound {
        requested_path: String,
    },
    PathNotCanonical {
        requested_path: String,
    },
    SymlinkDisallowed {
        requested_path: String,
        resolved_path: Option<String>,
    },
    EscapeAttempt {
        requested_path: String,
        resolved_path: Option<String>,
    },
    IoError {
        requested_path: String,
        operation: &'static str,
    },
}

impl SandboxPathError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::PathDenied { .. } => "sandbox_path_denied",
            Self::PathNotFound { .. } => "sandbox_path_not_found",
            Self::PathNotCanonical { .. } => "sandbox_path_not_canonical",
            Self::SymlinkDisallowed { .. } => "sandbox_symlink_disallowed",
            Self::EscapeAttempt { .. } => "sandbox_escape_attempt",
            Self::IoError { .. } => "sandbox_io_error",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::PathDenied { reason, .. } => {
                format!("sandbox path denied by policy ({reason})")
            }
            Self::PathNotFound { .. } => {
                "sandbox path not found; create the file/parent path inside sandbox root"
                    .to_string()
            }
            Self::PathNotCanonical { .. } => {
                "sandbox path could not be canonicalized deterministically".to_string()
            }
            Self::SymlinkDisallowed { .. } => {
                "sandbox path denied because symlink traversal is disabled by policy".to_string()
            }
            Self::EscapeAttempt { .. } => {
                "sandbox path denied because resolved path escapes sandbox root".to_string()
            }
            Self::IoError { operation, .. } => {
                format!("sandbox path validation IO error during {operation}")
            }
        }
    }

    pub fn requested_path(&self) -> Option<&str> {
        match self {
            Self::PathDenied { requested_path, .. }
            | Self::PathNotFound { requested_path }
            | Self::PathNotCanonical { requested_path }
            | Self::SymlinkDisallowed { requested_path, .. }
            | Self::EscapeAttempt { requested_path, .. }
            | Self::IoError { requested_path, .. } => Some(requested_path.as_str()),
        }
    }

    pub fn resolved_path(&self) -> Option<&str> {
        match self {
            Self::SymlinkDisallowed { resolved_path, .. }
            | Self::EscapeAttempt { resolved_path, .. } => resolved_path.as_deref(),
            _ => None,
        }
    }
}

fn normalize_rel_for_display(path: &Path) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => parts.push(part.to_string_lossy().to_string()),
            Component::ParentDir => parts.push("..".to_string()),
            Component::RootDir | Component::Prefix(_) => parts.push("<abs>".to_string()),
        }
    }
    if parts.is_empty() {
        "<empty>".to_string()
    } else {
        parts.join("/")
    }
}

fn sanitize_requested_path(path: &Path) -> String {
    if path.as_os_str().is_empty() {
        return "sandbox:/<empty>".to_string();
    }
    if path.is_absolute() {
        return "sandbox:/<absolute>".to_string();
    }
    format!("sandbox:/{}", normalize_rel_for_display(path))
}

impl std::fmt::Display for SandboxPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for SandboxPathError {}

fn sanitize_resolved_path(root_canon: &Path, resolved: &Path) -> String {
    match resolved.strip_prefix(root_canon) {
        Ok(rel) => format!("sandbox:/{}", normalize_rel_for_display(rel)),
        Err(_) => "sandbox:/<outside-root>".to_string(),
    }
}

fn classify_canonicalize_error(
    requested_path: &str,
    operation: &'static str,
    err: std::io::Error,
) -> SandboxPathError {
    match err.kind() {
        std::io::ErrorKind::NotFound => SandboxPathError::PathNotFound {
            requested_path: requested_path.to_string(),
        },
        std::io::ErrorKind::InvalidInput | std::io::ErrorKind::InvalidData => {
            SandboxPathError::PathNotCanonical {
                requested_path: requested_path.to_string(),
            }
        }
        _ => SandboxPathError::IoError {
            requested_path: requested_path.to_string(),
            operation,
        },
    }
}

fn classify_metadata_error(
    requested_path: &str,
    operation: &'static str,
    err: std::io::Error,
) -> SandboxPathError {
    match err.kind() {
        std::io::ErrorKind::NotFound => SandboxPathError::PathNotFound {
            requested_path: requested_path.to_string(),
        },
        _ => SandboxPathError::IoError {
            requested_path: requested_path.to_string(),
            operation,
        },
    }
}

fn validate_relative(path: &Path) -> Result<(), SandboxPathError> {
    let requested = sanitize_requested_path(path);
    if requested == "sandbox:/<empty>" {
        return Err(SandboxPathError::PathDenied {
            requested_path: requested,
            reason: "empty_path",
        });
    }
    if path.is_absolute() {
        return Err(SandboxPathError::PathDenied {
            requested_path: requested,
            reason: "absolute_path",
        });
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(SandboxPathError::PathDenied {
            requested_path: requested,
            reason: "parent_traversal",
        });
    }
    Ok(())
}

fn path_traverses_symlink(
    root_canon: &Path,
    target: &Path,
    requested: &str,
) -> Result<bool, SandboxPathError> {
    let rel = if target.is_absolute() {
        match target.strip_prefix(root_canon) {
            Ok(rel) => rel,
            Err(_) => return Ok(false),
        }
    } else {
        target
    };

    let mut current = root_canon.to_path_buf();
    for component in rel.components() {
        match component {
            Component::CurDir => continue,
            Component::Normal(part) => current.push(part),
            Component::ParentDir => {
                return Err(SandboxPathError::PathDenied {
                    requested_path: requested.to_string(),
                    reason: "parent_traversal",
                });
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(SandboxPathError::PathDenied {
                    requested_path: requested.to_string(),
                    reason: "absolute_path",
                });
            }
        }

        if !current.exists() {
            break;
        }

        let meta = std::fs::symlink_metadata(&current)
            .map_err(|err| classify_metadata_error(requested, "symlink_metadata", err))?;
        if meta.file_type().is_symlink() {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn resolve_existing_path_within_root(
    root: &Path,
    candidate: &Path,
) -> Result<PathBuf, SandboxPathError> {
    resolve_existing_path_within_root_with_policy(root, candidate, SandboxPathPolicy::default())
}

pub fn resolve_existing_path_within_root_with_policy(
    root: &Path,
    candidate: &Path,
    policy: SandboxPathPolicy,
) -> Result<PathBuf, SandboxPathError> {
    let requested = sanitize_requested_path(candidate);
    let root_canon = root
        .canonicalize()
        .map_err(|err| classify_canonicalize_error("sandbox:/<root>", "canonicalize_root", err))?;

    if !candidate.is_absolute() {
        validate_relative(candidate)?;
    }
    let joined = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root_canon.join(candidate)
    };

    if !policy.allow_symlink_traversal && path_traverses_symlink(&root_canon, &joined, &requested)?
    {
        return Err(SandboxPathError::SymlinkDisallowed {
            requested_path: requested,
            resolved_path: None,
        });
    }

    let canonical = joined
        .canonicalize()
        .map_err(|err| classify_canonicalize_error(&requested, "canonicalize_candidate", err))?;
    if !canonical.starts_with(&root_canon) {
        return Err(SandboxPathError::EscapeAttempt {
            requested_path: requested,
            resolved_path: Some(sanitize_resolved_path(&root_canon, &canonical)),
        });
    }
    Ok(canonical)
}

pub fn resolve_relative_path_for_write_within_root(
    root: &Path,
    rel: &Path,
) -> Result<PathBuf, SandboxPathError> {
    resolve_relative_path_for_write_within_root_with_policy(root, rel, SandboxPathPolicy::default())
}

pub fn resolve_relative_path_for_write_within_root_with_policy(
    root: &Path,
    rel: &Path,
    policy: SandboxPathPolicy,
) -> Result<PathBuf, SandboxPathError> {
    // Security note (TOCTOU): this resolver is best-effort path hardening, not
    // full OS-level isolation. It validates canonical ancestry at resolution
    // time, but cannot prevent all post-check filesystem races without kernel-
    // enforced sandboxing primitives.
    validate_relative(rel)?;
    let requested = sanitize_requested_path(rel);
    let root_canon = root
        .canonicalize()
        .map_err(|err| classify_canonicalize_error("sandbox:/<root>", "canonicalize_root", err))?;

    let candidate = root_canon.join(rel);
    if !policy.allow_symlink_traversal
        && path_traverses_symlink(&root_canon, &candidate, &requested)?
    {
        return Err(SandboxPathError::SymlinkDisallowed {
            requested_path: requested,
            resolved_path: None,
        });
    }

    if candidate.exists() {
        let canonical = candidate.canonicalize().map_err(|err| {
            classify_canonicalize_error(&requested, "canonicalize_existing_target", err)
        })?;
        if !canonical.starts_with(&root_canon) {
            return Err(SandboxPathError::EscapeAttempt {
                requested_path: requested,
                resolved_path: Some(sanitize_resolved_path(&root_canon, &canonical)),
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
            .ok_or_else(|| SandboxPathError::PathNotFound {
                requested_path: requested.clone(),
            })?;
    }

    if !policy.allow_symlink_traversal && path_traverses_symlink(&root_canon, ancestor, &requested)?
    {
        return Err(SandboxPathError::SymlinkDisallowed {
            requested_path: requested,
            resolved_path: None,
        });
    }

    let ancestor_canon = ancestor.canonicalize().map_err(|err| {
        classify_canonicalize_error(&requested, "canonicalize_existing_ancestor", err)
    })?;
    if !ancestor_canon.starts_with(&root_canon) {
        return Err(SandboxPathError::EscapeAttempt {
            requested_path: requested,
            resolved_path: Some(sanitize_resolved_path(&root_canon, &ancestor_canon)),
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
    fn resolve_existing_rejects_parent_traversal_with_stable_code() {
        let root = temp_dir("swarm-sandbox-parent");
        fs::create_dir_all(&root).expect("root");
        let err = resolve_existing_path_within_root(&root, Path::new("../escape.txt"))
            .expect_err("must reject parent traversal");
        assert_eq!(err.code(), "sandbox_path_denied");
        assert_eq!(err.requested_path(), Some("sandbox:/../escape.txt"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resolve_existing_missing_file_is_not_found_code() {
        let root = temp_dir("swarm-sandbox-missing");
        fs::create_dir_all(&root).expect("root");
        let err = resolve_existing_path_within_root(&root, Path::new("missing.txt"))
            .expect_err("missing file should fail");
        assert_eq!(err.code(), "sandbox_path_not_found");
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resolve_for_write_rejects_absolute_paths_cross_platform() {
        let root = temp_dir("swarm-sandbox-abs");
        fs::create_dir_all(&root).expect("root");
        let abs = std::env::temp_dir().join("outside-target.txt");
        let err = resolve_relative_path_for_write_within_root(&root, &abs)
            .expect_err("must reject absolute");
        assert_eq!(err.code(), "sandbox_path_denied");
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
        assert_eq!(err.code(), "sandbox_escape_attempt");

        let _ = fs::remove_dir_all(base);
    }

    #[cfg(unix)]
    #[test]
    fn resolve_existing_allows_symlink_inside_root_when_policy_allows() {
        use std::os::unix::fs as unix_fs;

        let base = temp_dir("swarm-sandbox-symlink-inside-allow");
        let root = base.join("root");
        let target_dir = root.join("dir");
        fs::create_dir_all(&target_dir).expect("target");
        fs::write(target_dir.join("ok.txt"), "ok").expect("write");
        unix_fs::symlink(&target_dir, root.join("link")).expect("symlink");

        let resolved = resolve_existing_path_within_root_with_policy(
            &root,
            Path::new("link/ok.txt"),
            SandboxPathPolicy {
                allow_symlink_traversal: true,
            },
        )
        .expect("symlink traversal should be allowed");
        assert!(resolved.starts_with(root.canonicalize().expect("root canon")));

        let _ = fs::remove_dir_all(base);
    }

    #[cfg(unix)]
    #[test]
    fn resolve_existing_rejects_symlink_when_policy_disallows() {
        use std::os::unix::fs as unix_fs;

        let base = temp_dir("swarm-sandbox-symlink-disallow");
        let root = base.join("root");
        let target_dir = root.join("dir");
        fs::create_dir_all(&target_dir).expect("target");
        fs::write(target_dir.join("ok.txt"), "ok").expect("write");
        unix_fs::symlink(&target_dir, root.join("link")).expect("symlink");

        let err = resolve_existing_path_within_root_with_policy(
            &root,
            Path::new("link/ok.txt"),
            SandboxPathPolicy {
                allow_symlink_traversal: false,
            },
        )
        .expect_err("symlink traversal should be denied");
        assert_eq!(err.code(), "sandbox_symlink_disallowed");

        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn sandbox_error_message_redacts_absolute_host_paths() {
        let abs = std::env::temp_dir().join("sensitive-host-path.txt");
        let root = temp_dir("swarm-sandbox-redaction");
        fs::create_dir_all(&root).expect("root");

        let err = resolve_relative_path_for_write_within_root(&root, &abs)
            .expect_err("absolute path must be denied");
        let msg = err.message();
        assert!(!msg.contains(&abs.display().to_string()));
        assert_eq!(err.requested_path(), Some("sandbox:/<absolute>"));

        let _ = fs::remove_dir_all(root);
    }

    #[cfg(windows)]
    #[test]
    fn sandbox_error_message_redacts_windows_drive_letter_paths() {
        let root = temp_dir("swarm-sandbox-windows-redaction");
        fs::create_dir_all(&root).expect("root");

        let win_abs = PathBuf::from(r"C:\Users\alice\secrets.txt");
        let err = resolve_relative_path_for_write_within_root(&root, &win_abs)
            .expect_err("windows absolute path must be denied");
        let msg = err.message();
        assert_eq!(err.code(), "sandbox_path_denied");
        assert_eq!(err.requested_path(), Some("sandbox:/<absolute>"));
        assert!(!msg.contains(r"C:\Users\alice\secrets.txt"));
        assert!(!msg.contains(r"C:\"));

        let _ = fs::remove_dir_all(root);
    }
}
