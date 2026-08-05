use crate::{ConvergenceError, ConvergenceErrorCode};
use std::collections::BTreeSet;
use std::path::{Component, Path};

pub(crate) fn normalize_paths(paths: &[String]) -> Result<Vec<String>, ConvergenceError> {
    let mut normalized = BTreeSet::new();
    for value in paths {
        normalized.insert(normalize_path(value)?);
    }
    Ok(normalized.into_iter().collect())
}

pub(crate) fn normalize_path(value: &str) -> Result<String, ConvergenceError> {
    reject_secret(value)?;
    let path = Path::new(value);
    if path.is_absolute() || value.trim().is_empty() {
        return Err(invalid_path(value));
    }
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            _ => return Err(invalid_path(value)),
        }
    }
    if parts.is_empty() {
        return Err(invalid_path(value));
    }
    Ok(parts.join("/"))
}

pub(crate) fn validate_revision(value: &str) -> Result<(), ConvergenceError> {
    if value.len() != 40 || !value.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(ConvergenceError::new(
            ConvergenceErrorCode::InvalidInput,
            "revision must be a 40-character hex digest",
        ));
    }
    Ok(())
}

pub(crate) fn validate_digest(value: &str) -> Result<(), ConvergenceError> {
    if value.len() != 64 || !value.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(ConvergenceError::new(
            ConvergenceErrorCode::InvalidInput,
            "artifact digest must be a 64-character hex digest",
        ));
    }
    Ok(())
}

pub(crate) fn reject_secret(value: &str) -> Result<(), ConvergenceError> {
    let lower = value.to_ascii_lowercase();
    if ["secret", "token", "password", "private_key"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        return Err(ConvergenceError::new(
            ConvergenceErrorCode::InvalidInput,
            "secret-bearing values are not accepted by convergence records",
        ));
    }
    Ok(())
}

pub(crate) fn path_contains(parent: &str, child: &str) -> bool {
    let parent: Vec<_> = parent.split('/').collect();
    let child: Vec<_> = child.split('/').collect();
    child.starts_with(&parent)
}

pub(crate) fn paths_overlap(left: &str, right: &str) -> bool {
    let left: Vec<_> = left.split('/').collect();
    let right: Vec<_> = right.split('/').collect();
    left == right || left.starts_with(&right) || right.starts_with(&left)
}

fn invalid_path(value: &str) -> ConvergenceError {
    ConvergenceError::new(
        ConvergenceErrorCode::InvalidPath,
        format!("path `{value}` is not repository-relative and normalized"),
    )
}
