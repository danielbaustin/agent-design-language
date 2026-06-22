use super::shared::{
    field_line_value, output_card_title_matches_slug, path_str, run_output_with_adl_tooling_bin,
};
use anyhow::{bail, Context, Result};
use serde_yaml::Value;
use std::path::Path;

pub(crate) struct StructuredBundlePaths<'a> {
    pub(crate) plan_path: &'a Path,
    pub(crate) validation_plan_path: &'a Path,
    pub(crate) review_policy_path: &'a Path,
}

pub(crate) fn validate_bootstrap_stp(repo_root: &Path, path: &Path) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    let output = run_output_with_adl_tooling_bin(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "stp",
            "--input",
            path_str(path)?,
        ],
    )?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let detail = if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            format!("bash failed with status {:?}", output.status.code())
        };
        bail!(
            "init: stp failed validation: {}: {}",
            path.display(),
            detail
        );
    }
    Ok(())
}

pub(crate) fn validate_ready_cards(
    repo_root: &Path,
    issue: u32,
    slug: &str,
    actual_branch: &str,
    input_path: &Path,
    output_path: &Path,
    structured_paths: StructuredBundlePaths<'_>,
) -> Result<()> {
    let expected = format!("issue-{:04}", issue);
    if field_line_value(input_path, "Task ID")? != expected {
        bail!("ready: input card Task ID mismatch");
    }
    if field_line_value(input_path, "Run ID")? != expected {
        bail!("ready: input card Run ID mismatch");
    }
    if field_line_value(output_path, "Task ID")? != expected {
        bail!("ready: output card Task ID mismatch");
    }
    if field_line_value(output_path, "Run ID")? != expected {
        bail!("ready: output card Run ID mismatch");
    }
    if !branch_matches_started_state(&field_line_value(input_path, "Branch")?, actual_branch) {
        bail!("ready: input card branch mismatch");
    }
    if !branch_matches_started_state(&field_line_value(output_path, "Branch")?, actual_branch) {
        bail!("ready: output card branch mismatch");
    }
    if !output_card_title_matches_slug(output_path, slug)? {
        bail!("ready: output card title mismatch");
    }
    validate_started_structured_artifact(
        repo_root,
        "ready",
        structured_paths.plan_path,
        "spp",
        actual_branch,
    )?;
    validate_started_structured_artifact(
        repo_root,
        "ready",
        structured_paths.validation_plan_path,
        "vpp",
        actual_branch,
    )?;
    validate_started_structured_artifact(
        repo_root,
        "ready",
        structured_paths.review_policy_path,
        "srp",
        actual_branch,
    )?;
    super::super::pr_cmd_validate::validate_authored_prompt_surface(
        "ready",
        input_path,
        super::super::pr_cmd_validate::PromptSurfaceKind::Sip,
    )?;
    Ok(())
}

pub(crate) fn validate_initialized_cards(
    issue: u32,
    slug: &str,
    input_path: &Path,
    output_path: &Path,
    repo_root: &Path,
    structured_paths: StructuredBundlePaths<'_>,
) -> Result<()> {
    let expected = format!("issue-{:04}", issue);
    if field_line_value(input_path, "Task ID")? != expected {
        bail!("doctor: input card Task ID mismatch");
    }
    if field_line_value(input_path, "Run ID")? != expected {
        bail!("doctor: input card Run ID mismatch");
    }
    if field_line_value(output_path, "Task ID")? != expected {
        bail!("doctor: output card Task ID mismatch");
    }
    if field_line_value(output_path, "Run ID")? != expected {
        bail!("doctor: output card Run ID mismatch");
    }
    if !output_card_title_matches_slug(output_path, slug)? {
        bail!("doctor: output card title mismatch");
    }
    validate_structured_artifact(repo_root, "doctor", structured_paths.plan_path, "spp")?;
    validate_structured_artifact(
        repo_root,
        "doctor",
        structured_paths.validation_plan_path,
        "vpp",
    )?;
    validate_structured_artifact(
        repo_root,
        "doctor",
        structured_paths.review_policy_path,
        "srp",
    )?;
    super::super::pr_cmd_validate::validate_authored_prompt_surface(
        "doctor",
        input_path,
        super::super::pr_cmd_validate::PromptSurfaceKind::Sip,
    )?;
    Ok(())
}

pub(crate) fn validate_bootstrap_cards(
    repo_root: &Path,
    issue: u32,
    slug: &str,
    branch: &str,
    input_path: &Path,
    output_path: &Path,
    structured_paths: StructuredBundlePaths<'_>,
) -> Result<()> {
    validate_structured_artifact_with_phase(
        repo_root,
        "start",
        input_path,
        "sip",
        Some("bootstrap"),
    )?;
    validate_structured_artifact_with_phase(
        repo_root,
        "start",
        output_path,
        "sor",
        Some("bootstrap"),
    )
    .with_context(|| {
        format!(
            "doctor: output card failed bootstrap validation: {}",
            output_path.display()
        )
    })?;
    let expected = format!("issue-{:04}", issue);
    if field_line_value(input_path, "Task ID")? != expected {
        bail!("start: input card Task ID mismatch");
    }
    if field_line_value(input_path, "Run ID")? != expected {
        bail!("start: input card Run ID mismatch");
    }
    if field_line_value(output_path, "Task ID")? != expected {
        bail!("start: output card Task ID mismatch");
    }
    if field_line_value(output_path, "Run ID")? != expected {
        bail!("start: output card Run ID mismatch");
    }
    if field_line_value(input_path, "Branch")? != branch {
        bail!("start: input card branch mismatch");
    }
    if field_line_value(output_path, "Branch")? != branch {
        bail!("start: output card branch mismatch");
    }
    if !output_card_title_matches_slug(output_path, slug)? {
        bail!("start: output card title mismatch");
    }
    validate_structured_artifact(repo_root, "start", structured_paths.plan_path, "spp")?;
    validate_structured_artifact(
        repo_root,
        "start",
        structured_paths.validation_plan_path,
        "vpp",
    )?;
    validate_structured_artifact(
        repo_root,
        "start",
        structured_paths.review_policy_path,
        "srp",
    )?;
    Ok(())
}

pub(crate) fn validate_bootstrap_output_card(
    repo_root: &Path,
    issue: u32,
    slug: &str,
    branch: &str,
    output_path: &Path,
) -> Result<()> {
    validate_structured_artifact_with_phase(
        repo_root,
        "doctor",
        output_path,
        "sor",
        Some("bootstrap"),
    )?;
    let expected = format!("issue-{:04}", issue);
    if field_line_value(output_path, "Task ID")? != expected {
        bail!("doctor: output card Task ID mismatch");
    }
    if field_line_value(output_path, "Run ID")? != expected {
        bail!("doctor: output card Run ID mismatch");
    }
    if field_line_value(output_path, "Branch")? != branch {
        bail!("doctor: output card branch mismatch");
    }
    if !output_card_title_matches_slug(output_path, slug)? {
        bail!("doctor: output card title mismatch");
    }
    Ok(())
}

fn branch_matches_started_state(recorded: &str, actual_branch: &str) -> bool {
    let recorded = recorded.trim();
    if recorded == actual_branch {
        return true;
    }
    recorded.starts_with("TBD (run pr.sh start ")
}

pub(crate) fn validate_structured_artifact(
    repo_root: &Path,
    phase: &str,
    path: &Path,
    kind: &str,
) -> Result<()> {
    validate_structured_artifact_with_phase(repo_root, phase, path, kind, None)
}

fn validate_structured_artifact_with_phase(
    repo_root: &Path,
    phase: &str,
    path: &Path,
    kind: &str,
    contract_phase: Option<&str>,
) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    let validator_path = path_str(&validator)?;
    let input_path = path_str(path)?;
    let mut args = vec!["bash", validator_path, "--type", kind];
    if let Some(contract_phase) = contract_phase {
        args.push("--phase");
        args.push(contract_phase);
    }
    args.push("--input");
    args.push(input_path);
    let output = run_output_with_adl_tooling_bin(args[0], &args[1..])?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let detail = if contract_phase == Some("bootstrap") {
            format!("bash failed with status {:?}", output.status.code())
        } else if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            format!("bash failed with status {:?}", output.status.code())
        };
        bail!(
            "{phase}: {kind} failed validation: {}: {detail}",
            path.display()
        );
    }
    Ok(())
}

fn validate_started_structured_artifact(
    repo_root: &Path,
    phase: &str,
    path: &Path,
    kind: &str,
    actual_branch: &str,
) -> Result<()> {
    validate_structured_artifact(repo_root, phase, path, kind)?;
    let text = std::fs::read_to_string(path)?;
    let front_matter = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n").map(|(fm, _)| fm))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "{phase}: {kind} missing YAML front matter: {}",
                path.display()
            )
        })?;
    let yaml: Value = serde_yaml::from_str(front_matter)?;
    let mapping = yaml.as_mapping().ok_or_else(|| {
        anyhow::anyhow!(
            "{phase}: {kind} front matter must be a mapping: {}",
            path.display()
        )
    })?;
    let branch = mapping
        .get(Value::String("branch".to_string()))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string();
    if branch != actual_branch {
        bail!(
            "{phase}: {kind} branch mismatch (expected {actual_branch}, found {})",
            if branch.is_empty() {
                "<empty>"
            } else {
                branch.as_str()
            }
        );
    }
    Ok(())
}
