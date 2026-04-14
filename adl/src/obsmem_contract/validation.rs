use super::{ObsMemContractError, ObsMemContractErrorCode};

pub(super) fn validate_relative_path(path: &str) -> Result<(), ObsMemContractError> {
    if path.trim().is_empty() {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "relative path must be non-empty",
        ));
    }
    if path.starts_with('/') || path.contains(':') || path.contains('\\') || path.contains("..") {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "paths must be relative and must not escape",
        ));
    }
    Ok(())
}

pub(super) fn contains_disallowed_content(text: &str) -> bool {
    text.contains("/Users/")
        || text.contains("/home/")
        || text.contains("gho_")
        || text.contains("sk-")
}
