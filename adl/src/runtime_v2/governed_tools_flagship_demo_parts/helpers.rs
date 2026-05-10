use std::collections::BTreeSet;

use super::*;

pub(crate) fn validate_relative_path_list(values: &[String], field: &str) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        validate_relative_path(value, field)?;
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate path {value}"));
        }
    }
    Ok(())
}

pub(crate) fn dedupe_paths(paths: &mut [String]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for path in paths.iter() {
        validate_relative_path(path, "governed_tools_flagship.required_artifact_ref")?;
        if !seen.insert(path.clone()) {
            return Err(anyhow!("duplicate path {}", path));
        }
    }
    Ok(())
}

pub(crate) fn require_case_value<T>(
    case: &RuntimeV2GovernedToolsFlagshipCase,
    field: &str,
    observed: T,
    expected: T,
) -> Result<()>
where
    T: PartialEq + std::fmt::Debug,
{
    if observed != expected {
        return Err(anyhow!(
            "{} {} must equal {:?}, found {:?}",
            case.case_id,
            field,
            expected,
            observed
        ));
    }
    Ok(())
}
