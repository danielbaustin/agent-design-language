use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

use super::shared::{field_line_value, output_card_title_matches_slug, path_str, run_status};

pub(crate) fn validate_bootstrap_stp(repo_root: &Path, path: &Path) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    let output = Command::new("bash")
        .args([
            path_str(&validator)?,
            "--type",
            "stp",
            "--input",
            path_str(path)?,
        ])
        .output()
        .with_context(|| "failed to spawn 'bash'")?;
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
    _repo_root: &Path,
    issue: u32,
    slug: &str,
    actual_branch: &str,
    input_path: &Path,
    output_path: &Path,
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
) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    run_status(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "sip",
            "--phase",
            "bootstrap",
            "--input",
            path_str(input_path)?,
        ],
    )?;
    run_status(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "sor",
            "--phase",
            "bootstrap",
            "--input",
            path_str(output_path)?,
        ],
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
    Ok(())
}

pub(crate) fn validate_bootstrap_output_card(
    repo_root: &Path,
    issue: u32,
    slug: &str,
    branch: &str,
    output_path: &Path,
) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    run_status(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "sor",
            "--phase",
            "bootstrap",
            "--input",
            path_str(output_path)?,
        ],
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
