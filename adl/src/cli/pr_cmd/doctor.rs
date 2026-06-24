use super::*;
use crate::cli::pr_cmd_cards::{validate_bootstrap_output_card, StructuredBundlePaths};
use std::fs;
use std::path::{Path, PathBuf};

mod card_lifecycle;
mod preflight;
mod printing;
mod ready;
#[cfg(test)]
mod tests;
mod types;

use card_lifecycle::{
    build_doctor_card_lifecycle, doctor_ready_status_for, validate_closed_completed_ready_bundle,
};
use preflight::run_doctor_preflight;
use printing::{
    print_doctor_card_lifecycle_text, print_doctor_preflight_text, print_doctor_ready_text,
};
use ready::run_doctor_ready as internal_run_doctor_ready;
use types::{
    DoctorCardLifecycleJson, DoctorCardStageJson, DoctorJsonOutput, DoctorPreflightJsonPullRequest,
    DoctorPreflightResult, DoctorReadyResult,
};

pub(super) fn run_doctor_ready(
    repo_root: &Path,
    repo: &str,
    issue_ref: &IssueRef,
    branch: &str,
) -> Result<DoctorReadyResult> {
    internal_run_doctor_ready(repo_root, repo, issue_ref, branch)
}

pub(super) fn ensure_pr_run_design_time_ready(
    repo_root: &Path,
    issue_ref: &IssueRef,
    expected_branch: &str,
) -> Result<()> {
    ready::ensure_pr_run_design_time_ready(repo_root, issue_ref, expected_branch)
}

fn doctor_mode_name(mode: &DoctorMode) -> &'static str {
    match mode {
        DoctorMode::Full => "full",
        DoctorMode::Ready => "ready",
        DoctorMode::Preflight => "preflight",
    }
}

pub(super) fn resolve_doctor_issue_prompt_path(
    repo_root: &Path,
    issue_ref: &IssueRef,
) -> Result<PathBuf> {
    let mut roots = vec![repo_root.to_path_buf()];
    let worktree_path = issue_ref.default_worktree_path(
        repo_root,
        std::env::var_os("ADL_WORKTREE_ROOT")
            .map(PathBuf::from)
            .as_deref(),
    );
    if !roots.iter().any(|root| root == &worktree_path) {
        roots.push(worktree_path);
    }
    if let Ok(cwd) = std::env::current_dir() {
        if !roots.iter().any(|root| root == &cwd) {
            roots.push(cwd);
        }
    }
    for root in roots {
        if let Ok(path) = resolve_issue_prompt_path(&root, issue_ref) {
            return Ok(path);
        }
    }
    resolve_issue_prompt_path(repo_root, issue_ref)
}

pub(super) fn run_doctor(parsed: DoctorArgs, label: &str) -> Result<()> {
    let repo_root = primary_checkout_root()?;
    let repo = default_repo(&repo_root)?;
    let (version, slug) = resolve_doctor_scope_and_slug(&repo_root, &parsed, label)?;
    let issue_ref = IssueRef::new(parsed.issue, version.clone(), slug.clone())?;
    let branch = issue_ref.branch_name("codex");

    let preflight = run_doctor_preflight(&repo_root, &repo, &version, &issue_ref, &branch)?;
    let ready = match parsed.mode {
        DoctorMode::Preflight => None,
        DoctorMode::Ready | DoctorMode::Full => {
            Some(run_doctor_ready(&repo_root, &repo, &issue_ref, &branch)?)
        }
    };
    let mode = doctor_mode_name(&parsed.mode);
    let ready_status = ready.as_ref().map(|x| x.status);
    let doctor_status = match parsed.mode {
        DoctorMode::Preflight => preflight.status,
        DoctorMode::Ready => ready_status.unwrap_or("BLOCK"),
        DoctorMode::Full => {
            doctor_full_status(preflight.status, preflight.block_kind, ready_status)
        }
    };

    if parsed.json {
        print_json(&DoctorJsonOutput {
            schema: "adl.pr.doctor.v1",
            issue: parsed.issue,
            version,
            slug,
            branch,
            mode,
            target_queue: preflight.target_queue.clone(),
            target_queue_source: preflight.target_queue_source,
            preflight_status: preflight.status,
            preflight_block_kind: preflight.block_kind,
            preflight_guidance: preflight.guidance,
            open_pr_count: preflight.open_pr_count,
            open_prs: preflight.open_prs,
            lifecycle_state: ready.as_ref().map(|x| x.lifecycle_state),
            ready_status,
            worktree: ready.as_ref().and_then(|x| x.worktree.clone()),
            source: ready.as_ref().map(|x| x.source.clone()),
            root_stp: ready.as_ref().map(|x| x.root_stp.clone()),
            root_input: ready.as_ref().map(|x| x.root_input.clone()),
            root_output: ready.as_ref().map(|x| x.root_output.clone()),
            wt_stp: ready.as_ref().and_then(|x| x.wt_stp.clone()),
            wt_input: ready.as_ref().and_then(|x| x.wt_input.clone()),
            wt_output: ready.as_ref().and_then(|x| x.wt_output.clone()),
            card_lifecycle: ready.as_ref().map(|x| x.card_lifecycle.clone()),
            doctor_status,
        })?;
    } else {
        println!("ISSUE={}", parsed.issue);
        println!("VERSION={version}");
        println!("SLUG={slug}");
        println!("BRANCH={branch}");
        println!("TARGET_QUEUE={}", preflight.target_queue);
        println!("TARGET_QUEUE_SOURCE={}", preflight.target_queue_source);
        print_doctor_preflight_text(&preflight);
        if let Some(ready) = &ready {
            print_doctor_ready_text(ready);
            print_doctor_card_lifecycle_text(&ready.card_lifecycle);
        }
        println!("DOCTOR_MODE={mode}");
        println!("DOCTOR_STATUS={doctor_status}");
    }
    Ok(())
}

fn doctor_full_status(
    preflight_status: &'static str,
    preflight_block_kind: &'static str,
    ready_status: Option<&'static str>,
) -> &'static str {
    match (preflight_status, preflight_block_kind, ready_status) {
        ("PASS", _, Some("PASS")) => "PASS",
        ("BLOCK", "open_pr_wave", Some("PASS")) => "WARN",
        _ => "BLOCK",
    }
}

fn resolve_doctor_scope_and_slug(
    repo_root: &Path,
    parsed: &DoctorArgs,
    label: &str,
) -> Result<(String, String)> {
    if let (Some(version), Some(slug)) = (parsed.version.clone(), parsed.slug.clone()) {
        return Ok((version, slug));
    }
    let inferred =
        resolve_issue_scope_and_slug_from_available_local_state(repo_root, parsed.issue)?;
    let version = parsed
        .version
        .clone()
        .or(inferred.as_ref().map(|x| x.0.clone()))
        .unwrap_or_else(|| DEFAULT_VERSION.to_string());
    let slug = match parsed.mode {
        DoctorMode::Preflight => parsed
            .slug
            .clone()
            .or(inferred.map(|x| x.1))
            .unwrap_or_else(|| format!("issue-{}", parsed.issue)),
        DoctorMode::Ready | DoctorMode::Full => {
            parsed.slug.clone().or(inferred.map(|x| x.1)).ok_or_else(|| {
                if label == "ready" {
                    anyhow!("ready: could not infer slug; pass --slug or run start first")
                } else {
                    anyhow!(
                        "doctor: could not infer slug for readiness check; pass --slug or create the execution context first"
                    )
                }
            })?
        }
    };
    Ok((version, slug))
}
