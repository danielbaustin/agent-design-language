use super::*;
use crate::cli::pr_cmd_cards::validate_bootstrap_output_card;
use crate::cli::pr_cmd_cards::StructuredBundlePaths;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct DoctorPreflightJsonPullRequest {
    number: u32,
    head_ref_name: String,
    state: &'static str,
    queue: Option<String>,
    url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DoctorPreflightResult {
    target_queue: String,
    target_queue_source: &'static str,
    open_pr_count: usize,
    open_prs: Vec<DoctorPreflightJsonPullRequest>,
    status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DoctorReadyResult {
    pub(super) lifecycle_state: &'static str,
    worktree: Option<String>,
    source: String,
    root_stp: String,
    root_input: String,
    root_output: String,
    wt_stp: Option<String>,
    wt_input: Option<String>,
    wt_output: Option<String>,
    card_lifecycle: DoctorCardLifecycleJson,
    pub(super) status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct DoctorCardLifecycleJson {
    order: Vec<&'static str>,
    active_stage: &'static str,
    next_required_stage: Option<&'static str>,
    pr_run_readiness: &'static str,
    pr_finish_readiness: &'static str,
    stages: Vec<DoctorCardStageJson>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct DoctorCardStageJson {
    stage: &'static str,
    path: String,
    state: &'static str,
    complete: bool,
    final_ready: bool,
    next_editor: Option<&'static str>,
    detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct DoctorJsonOutput {
    schema: &'static str,
    issue: u32,
    version: String,
    slug: String,
    branch: String,
    mode: &'static str,
    target_queue: String,
    target_queue_source: &'static str,
    preflight_status: &'static str,
    open_pr_count: usize,
    open_prs: Vec<DoctorPreflightJsonPullRequest>,
    lifecycle_state: Option<&'static str>,
    ready_status: Option<&'static str>,
    worktree: Option<String>,
    source: Option<String>,
    root_stp: Option<String>,
    root_input: Option<String>,
    root_output: Option<String>,
    wt_stp: Option<String>,
    wt_input: Option<String>,
    wt_output: Option<String>,
    card_lifecycle: Option<DoctorCardLifecycleJson>,
    doctor_status: &'static str,
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
    let doctor_status = match parsed.mode {
        DoctorMode::Preflight => preflight.status,
        DoctorMode::Ready => ready.as_ref().map(|x| x.status).unwrap_or("BLOCK"),
        DoctorMode::Full => {
            if preflight.status == "PASS" && ready.as_ref().map(|x| x.status) == Some("PASS") {
                "PASS"
            } else {
                "BLOCK"
            }
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
            open_pr_count: preflight.open_pr_count,
            open_prs: preflight.open_prs,
            lifecycle_state: ready.as_ref().map(|x| x.lifecycle_state),
            ready_status: ready.as_ref().map(|x| x.status),
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

fn resolve_doctor_scope_and_slug(
    repo_root: &Path,
    parsed: &DoctorArgs,
    label: &str,
) -> Result<(String, String)> {
    if let (Some(version), Some(slug)) = (parsed.version.clone(), parsed.slug.clone()) {
        return Ok((version, slug));
    }
    let inferred = resolve_issue_scope_and_slug_from_local_state(repo_root, parsed.issue)?;
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

fn run_doctor_preflight(
    repo_root: &Path,
    repo: &str,
    version: &str,
    issue_ref: &IssueRef,
    branch: &str,
) -> Result<DoctorPreflightResult> {
    let source_path = resolve_issue_prompt_path(repo_root, issue_ref)?;
    let target_queue = resolve_issue_prompt_workflow_queue(&source_path)?;
    let unresolved =
        unresolved_milestone_pr_wave(repo, version, &target_queue.queue, Some(branch))?;
    let open_prs = unresolved
        .iter()
        .map(|pr| DoctorPreflightJsonPullRequest {
            number: pr.number,
            head_ref_name: pr.head_ref_name.clone(),
            state: if pr.is_draft { "draft" } else { "ready" },
            queue: pr.queue.clone(),
            url: pr.url.clone(),
        })
        .collect::<Vec<_>>();
    if open_prs.is_empty() {
        Ok(DoctorPreflightResult {
            target_queue: target_queue.queue,
            target_queue_source: target_queue.source,
            open_pr_count: 0,
            open_prs,
            status: "PASS",
        })
    } else {
        Ok(DoctorPreflightResult {
            target_queue: target_queue.queue,
            target_queue_source: target_queue.source,
            open_pr_count: open_prs.len(),
            open_prs,
            status: "BLOCK",
        })
    }
}

pub(super) fn run_doctor_ready(
    repo_root: &Path,
    repo: &str,
    issue_ref: &IssueRef,
    branch: &str,
) -> Result<DoctorReadyResult> {
    let worktree_path = issue_ref.default_worktree_path(
        repo_root,
        std::env::var_os("ADL_WORKTREE_ROOT")
            .map(PathBuf::from)
            .as_deref(),
    );
    let source_path = resolve_issue_prompt_path(repo_root, issue_ref)?;
    let root_stp = issue_ref.task_bundle_stp_path(repo_root);
    let wt_stp = issue_ref.task_bundle_stp_path(&worktree_path);
    let root_bundle_input = issue_ref.task_bundle_input_path(repo_root);
    let root_bundle_output = issue_ref.task_bundle_output_path(repo_root);
    let root_bundle_plan = issue_ref.task_bundle_plan_path(repo_root);
    let root_bundle_review_policy = issue_ref.task_bundle_review_policy_path(repo_root);
    let wt_bundle_input = issue_ref.task_bundle_input_path(&worktree_path);
    let wt_bundle_output = issue_ref.task_bundle_output_path(&worktree_path);
    let wt_bundle_plan = issue_ref.task_bundle_plan_path(&worktree_path);
    let wt_bundle_review_policy = issue_ref.task_bundle_review_policy_path(&worktree_path);
    let closed_completed =
        lifecycle::issue_is_closed_and_completed(issue_ref.issue_number(), repo)?;

    validate_issue_prompt_exists(&source_path)?;
    validate_bootstrap_stp(repo_root, &source_path)?;
    validate_authored_prompt_surface("doctor", &source_path, PromptSurfaceKind::IssuePrompt)?;
    if closed_completed {
        lifecycle::closeout_closed_completed_issue_bundle(
            repo_root,
            repo_root,
            issue_ref,
            &root_bundle_output,
        )?;
    }
    if !root_stp.is_file() {
        bail!("doctor: missing root stp: {}", root_stp.display());
    }
    validate_bootstrap_stp(repo_root, &root_stp)?;
    validate_authored_prompt_surface("doctor", &root_stp, PromptSurfaceKind::Stp)?;
    if closed_completed {
        validate_initialized_cards(
            issue_ref.issue_number(),
            issue_ref.slug(),
            &root_bundle_input,
            &root_bundle_output,
            repo_root,
            StructuredBundlePaths {
                plan_path: &root_bundle_plan,
                review_policy_path: &root_bundle_review_policy,
            },
        )?;
        return Ok(DoctorReadyResult {
            lifecycle_state: "closed",
            worktree: None,
            source: path_relative_to_repo(repo_root, &source_path),
            root_stp: path_relative_to_repo(repo_root, &root_stp),
            root_input: path_relative_to_repo(repo_root, &root_bundle_input),
            root_output: path_relative_to_repo(repo_root, &root_bundle_output),
            wt_stp: None,
            wt_input: None,
            wt_output: None,
            card_lifecycle: build_doctor_card_lifecycle(
                repo_root,
                &root_bundle_input,
                &root_stp,
                &root_bundle_plan,
                &root_bundle_review_policy,
                &root_bundle_output,
            ),
            status: "PASS",
        });
    }
    validate_initialized_cards(
        issue_ref.issue_number(),
        issue_ref.slug(),
        &root_bundle_input,
        &root_bundle_output,
        repo_root,
        StructuredBundlePaths {
            plan_path: &root_bundle_plan,
            review_policy_path: &root_bundle_review_policy,
        },
    )?;
    let root_input_body = fs::read_to_string(&root_bundle_input).with_context(|| {
        format!(
            "doctor: read root input card: {}",
            root_bundle_input.display()
        )
    })?;
    let root_branch = field_line_value(&root_bundle_input, "Branch")?;
    let root_indicates_pre_run = branch_indicates_unbound_state(&root_branch)
        || root_input_body.contains(
            "This issue is not started yet; do not assume a branch or worktree already exists.",
        )
        || root_input_body
            .contains("Do not assume a branch or worktree already exists before `pr run`.");
    if root_indicates_pre_run {
        validate_bootstrap_output_card(
            repo_root,
            issue_ref.issue_number(),
            issue_ref.slug(),
            &root_branch,
            &root_bundle_output,
        )?;
    }
    if !worktree_path.is_dir() {
        if root_indicates_pre_run {
            return Ok(DoctorReadyResult {
                lifecycle_state: "pre_run",
                worktree: None,
                source: path_relative_to_repo(repo_root, &source_path),
                root_stp: path_relative_to_repo(repo_root, &root_stp),
                root_input: path_relative_to_repo(repo_root, &root_bundle_input),
                root_output: path_relative_to_repo(repo_root, &root_bundle_output),
                wt_stp: None,
                wt_input: None,
                wt_output: None,
                card_lifecycle: build_doctor_card_lifecycle(
                    repo_root,
                    &root_bundle_input,
                    &root_stp,
                    &root_bundle_plan,
                    &root_bundle_review_policy,
                    &root_bundle_output,
                ),
                status: "PASS",
            });
        }
        bail!("doctor: missing worktree: {}", worktree_path.display());
    }
    if root_indicates_pre_run
        && (!wt_stp.is_file() || !wt_bundle_input.is_file() || !wt_bundle_output.is_file())
    {
        return Ok(DoctorReadyResult {
            lifecycle_state: "pre_run",
            worktree: None,
            source: path_relative_to_repo(repo_root, &source_path),
            root_stp: path_relative_to_repo(repo_root, &root_stp),
            root_input: path_relative_to_repo(repo_root, &root_bundle_input),
            root_output: path_relative_to_repo(repo_root, &root_bundle_output),
            wt_stp: None,
            wt_input: None,
            wt_output: None,
            card_lifecycle: build_doctor_card_lifecycle(
                repo_root,
                &root_bundle_input,
                &root_stp,
                &root_bundle_plan,
                &root_bundle_review_policy,
                &root_bundle_output,
            ),
            status: "PASS",
        });
    }
    let wt_branch = run_capture(
        "git",
        &[
            "-C",
            path_str(&worktree_path)?,
            "rev-parse",
            "--abbrev-ref",
            "HEAD",
        ],
    )?;
    if wt_branch.trim() != branch {
        bail!(
            "doctor: worktree branch mismatch for {}",
            worktree_path.display()
        );
    }
    if !wt_stp.is_file() {
        bail!("doctor: missing worktree stp: {}", wt_stp.display());
    }
    validate_bootstrap_stp(&worktree_path, &wt_stp)?;
    validate_authored_prompt_surface("doctor", &wt_stp, PromptSurfaceKind::Stp)?;
    validate_ready_cards(
        repo_root,
        issue_ref.issue_number(),
        issue_ref.slug(),
        wt_branch.trim(),
        &root_bundle_input,
        &root_bundle_output,
        StructuredBundlePaths {
            plan_path: &root_bundle_plan,
            review_policy_path: &root_bundle_review_policy,
        },
    )?;
    validate_ready_cards(
        &worktree_path,
        issue_ref.issue_number(),
        issue_ref.slug(),
        wt_branch.trim(),
        &wt_bundle_input,
        &wt_bundle_output,
        StructuredBundlePaths {
            plan_path: &wt_bundle_plan,
            review_policy_path: &wt_bundle_review_policy,
        },
    )?;

    Ok(DoctorReadyResult {
        lifecycle_state: "run_bound",
        worktree: Some(path_relative_to_repo(repo_root, &worktree_path)),
        source: path_relative_to_repo(repo_root, &source_path),
        root_stp: path_relative_to_repo(repo_root, &root_stp),
        root_input: path_relative_to_repo(repo_root, &root_bundle_input),
        root_output: path_relative_to_repo(repo_root, &root_bundle_output),
        wt_stp: Some(path_relative_to_repo(repo_root, &wt_stp)),
        wt_input: Some(path_relative_to_repo(repo_root, &wt_bundle_input)),
        wt_output: Some(path_relative_to_repo(repo_root, &wt_bundle_output)),
        card_lifecycle: build_doctor_card_lifecycle(
            repo_root,
            &wt_bundle_input,
            &wt_stp,
            &wt_bundle_plan,
            &wt_bundle_review_policy,
            &wt_bundle_output,
        ),
        status: "PASS",
    })
}

fn build_doctor_card_lifecycle(
    repo_root: &Path,
    sip_path: &Path,
    stp_path: &Path,
    spp_path: &Path,
    srp_path: &Path,
    sor_path: &Path,
) -> DoctorCardLifecycleJson {
    let stages = vec![
        classify_sip_stage(repo_root, sip_path),
        classify_stp_stage(repo_root, stp_path),
        classify_spp_stage(repo_root, spp_path),
        classify_srp_stage(repo_root, srp_path),
        classify_sor_stage(repo_root, sor_path),
    ];
    let next_required_stage = stages
        .iter()
        .find(|stage| !stage.complete)
        .map(|stage| stage.stage);
    let active_stage = next_required_stage.unwrap_or("SOR");
    let pr_run_readiness = if stages
        .iter()
        .filter(|stage| ["SIP", "STP", "SPP"].contains(&stage.stage))
        .all(|stage| stage.complete)
    {
        "ready"
    } else {
        "blocked"
    };
    let pr_finish_readiness = match (
        stages.iter().find(|stage| stage.stage == "SRP"),
        stages.iter().find(|stage| stage.stage == "SOR"),
    ) {
        (Some(srp), Some(sor)) if srp.final_ready && (sor.complete || sor.final_ready) => "ready",
        (Some(srp), Some(sor)) if srp.state == "legacy_compatible" || sor.state == "scaffold" => {
            "blocked"
        }
        _ => "blocked",
    };

    DoctorCardLifecycleJson {
        order: vec!["SIP", "STP", "SPP", "SRP", "SOR"],
        active_stage,
        next_required_stage,
        pr_run_readiness,
        pr_finish_readiness,
        stages,
    }
}

fn classify_sip_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "SIP", path, "sip-editor");
    };
    let branch = line_value_after_prefix(&text, "Branch:").unwrap_or_default();
    if branch_indicates_unbound_state(&branch) {
        return card_stage(
            repo_root,
            "SIP",
            path,
            "scaffold",
            false,
            false,
            Some("sip-editor"),
            "SIP is still a pre-run scaffold; branch/worktree execution has not been bound.",
        );
    }
    card_stage(
        repo_root,
        "SIP",
        path,
        "complete",
        true,
        false,
        None,
        "SIP is branch-bound and complete enough for execution planning.",
    )
}

fn classify_stp_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "STP", path, "stp-editor");
    };
    if text.contains("## Required Outcome") && text.contains("## Acceptance Criteria") {
        return card_stage(
            repo_root,
            "STP",
            path,
            "complete",
            true,
            false,
            None,
            "STP has the required task intent and acceptance surfaces.",
        );
    }
    card_stage(
        repo_root,
        "STP",
        path,
        "active",
        false,
        false,
        Some("stp-editor"),
        "STP exists but is not complete enough to anchor execution.",
    )
}

fn classify_spp_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "SPP", path, "spp-editor");
    };
    let status = line_value_after_prefix(&text, "status:").unwrap_or_default();
    let branch = line_value_after_prefix(&text, "branch:").unwrap_or_default();
    if text.contains("Bootstrap-generated SPP") || branch_indicates_unbound_state(&branch) {
        return card_stage(
            repo_root,
            "SPP",
            path,
            "scaffold",
            false,
            false,
            Some("spp-editor"),
            "SPP is still bootstrap/pre-run planning scaffold and needs issue-specific plan truth.",
        );
    }
    if ["reviewed", "approved"].contains(&status.trim_matches('"')) {
        return card_stage(
            repo_root,
            "SPP",
            path,
            "complete",
            true,
            false,
            None,
            "SPP has reviewed or approved planning state.",
        );
    }
    card_stage(
        repo_root,
        "SPP",
        path,
        "active",
        false,
        false,
        Some("spp-editor"),
        "SPP is branch-bound but not yet marked reviewed or approved.",
    )
}

fn classify_srp_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "SRP", path, "srp-editor");
    };
    let has_review_results = text.contains("review_results:")
        || text.contains("## Review Results")
        || text.contains("### Recommended Outcome");
    let has_policy_exception = text.contains("explicit policy exception")
        || text.contains("review_results_exception:")
        || text.contains("policy_exception:");
    let legacy_policy_only = text.contains("# Structured Review Policy")
        || text.contains("artifact_type: \"structured_review_policy\"");
    if has_review_results || has_policy_exception {
        return card_stage(
            repo_root,
            "SRP",
            path,
            "final",
            true,
            true,
            None,
            "SRP contains review results or an explicit policy exception for final review truth.",
        );
    }
    if legacy_policy_only {
        return card_stage(
            repo_root,
            "SRP",
            path,
            "legacy_compatible",
            false,
            false,
            Some("srp-editor"),
            "SRP validates as the legacy review-policy scaffold but is not final Structured Review Prompt truth.",
        );
    }
    card_stage(
        repo_root,
        "SRP",
        path,
        "active",
        false,
        false,
        Some("srp-editor"),
        "SRP exists but still needs review results or an explicit policy exception.",
    )
}

fn classify_sor_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "SOR", path, "sor-editor");
    };
    let status = line_value_after_prefix(&text, "Status:").unwrap_or_default();
    let integration_state =
        line_value_after_prefix(&text, "- Integration state:").unwrap_or_default();
    let result = line_value_after_prefix(&text, "- Result:").unwrap_or_default();
    let worktree_only =
        line_value_after_prefix(&text, "- Worktree-only paths remaining:").unwrap_or_default();
    if status == "NOT_STARTED" || text.contains("No implementation has started yet") {
        return card_stage(
            repo_root,
            "SOR",
            path,
            "scaffold",
            false,
            false,
            Some("sor-editor"),
            "SOR is still the pre-execution output scaffold.",
        );
    }
    if ["merged", "closed_no_pr"].contains(&integration_state.as_str())
        && matches!(
            (status.as_str(), result.as_str()),
            ("DONE", "PASS") | ("FAILED", "FAIL")
        )
        && worktree_only == "none"
        && text.contains("## Validation")
    {
        return card_stage(
            repo_root,
            "SOR",
            path,
            "final",
            true,
            true,
            None,
            "SOR records terminal integration, validation, closeout, and artifact truth.",
        );
    }
    if integration_state == "pr_open" && status == "DONE" && result == "PASS" {
        return card_stage(
            repo_root,
            "SOR",
            path,
            "complete",
            true,
            false,
            Some("sor-editor"),
            "SOR is complete enough for PR publication but is not terminal closeout truth.",
        );
    }
    card_stage(
        repo_root,
        "SOR",
        path,
        "active",
        false,
        false,
        Some("sor-editor"),
        "SOR exists but does not yet satisfy PR publication or terminal closeout readiness.",
    )
}

fn missing_stage(
    repo_root: &Path,
    stage: &'static str,
    path: &Path,
    editor: &'static str,
) -> DoctorCardStageJson {
    card_stage(
        repo_root,
        stage,
        path,
        "missing",
        false,
        false,
        Some(editor),
        "Required lifecycle card is missing.",
    )
}

fn card_stage(
    repo_root: &Path,
    stage: &'static str,
    path: &Path,
    state: &'static str,
    complete: bool,
    final_ready: bool,
    next_editor: Option<&'static str>,
    detail: &str,
) -> DoctorCardStageJson {
    DoctorCardStageJson {
        stage,
        path: path_relative_to_repo(repo_root, path),
        state,
        complete,
        final_ready,
        next_editor,
        detail: detail.to_string(),
    }
}

fn read_card_text(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn line_value_after_prefix(text: &str, prefix: &str) -> Option<String> {
    text.lines()
        .find_map(|line| line.trim().strip_prefix(prefix))
        .map(|value| value.trim().trim_matches('"').to_string())
}

fn doctor_mode_name(mode: &DoctorMode) -> &'static str {
    match mode {
        DoctorMode::Full => "full",
        DoctorMode::Ready => "ready",
        DoctorMode::Preflight => "preflight",
    }
}

fn print_doctor_preflight_text(preflight: &DoctorPreflightResult) {
    println!("OPEN_PR_COUNT={}", preflight.open_pr_count);
    for pr in &preflight.open_prs {
        println!(
            "OPEN_PR=#{}|{}|{}|{}|{}",
            pr.number,
            pr.head_ref_name,
            pr.state,
            pr.queue.as_deref().unwrap_or("unknown"),
            pr.url
        );
    }
    println!("PREFLIGHT={}", preflight.status);
}

fn print_doctor_ready_text(ready: &DoctorReadyResult) {
    println!("LIFECYCLE_STATE={}", ready.lifecycle_state);
    if let Some(worktree) = &ready.worktree {
        println!("WORKTREE={worktree}");
    }
    println!("SOURCE={}", ready.source);
    println!("ROOT_STP={}", ready.root_stp);
    println!("ROOT_INPUT={}", ready.root_input);
    println!("ROOT_OUTPUT={}", ready.root_output);
    if let Some(wt_stp) = &ready.wt_stp {
        println!("WT_STP={wt_stp}");
    }
    if let Some(wt_input) = &ready.wt_input {
        println!("WT_INPUT={wt_input}");
    }
    if let Some(wt_output) = &ready.wt_output {
        println!("WT_OUTPUT={wt_output}");
    }
    println!("READY={}", ready.status);
}

fn print_doctor_card_lifecycle_text(card_lifecycle: &DoctorCardLifecycleJson) {
    println!("CARD_LIFECYCLE_ORDER={}", card_lifecycle.order.join("->"));
    println!(
        "CARD_LIFECYCLE_ACTIVE_STAGE={}",
        card_lifecycle.active_stage
    );
    println!(
        "CARD_LIFECYCLE_NEXT_REQUIRED_STAGE={}",
        card_lifecycle.next_required_stage.unwrap_or("none")
    );
    println!(
        "CARD_LIFECYCLE_PR_RUN_READINESS={}",
        card_lifecycle.pr_run_readiness
    );
    println!(
        "CARD_LIFECYCLE_PR_FINISH_READINESS={}",
        card_lifecycle.pr_finish_readiness
    );
    for stage in &card_lifecycle.stages {
        println!(
            "CARD_STAGE={}|{}|complete={}|final={}|editor={}|{}",
            stage.stage,
            stage.state,
            stage.complete,
            stage.final_ready,
            stage.next_editor.unwrap_or("none"),
            stage.path
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn card_lifecycle_marks_legacy_srp_policy_as_not_finish_ready() {
        let repo = lifecycle_temp_repo("legacy-srp-policy");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"reviewed\"\n---\n",
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Policy\n\n## Review Summary\n\npolicy only\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.active_stage, "SRP");
        assert_eq!(lifecycle.next_required_stage, Some("SRP"));
        assert_eq!(lifecycle.pr_run_readiness, "ready");
        assert_eq!(lifecycle.pr_finish_readiness, "blocked");
        assert_stage(&lifecycle, "SRP", "legacy_compatible", false, false);
        assert_stage(&lifecycle, "SOR", "complete", true, false);
    }

    #[test]
    fn card_lifecycle_distinguishes_active_plan_from_scaffold_output() {
        let repo = lifecycle_temp_repo("active-spp-scaffold-sor");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\ncodex_plan:\n  - step: \"implement\"\n    status: \"in_progress\"\n",
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Policy\n",
                sor: "Branch: codex/3065-test\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.active_stage, "SPP");
        assert_eq!(lifecycle.next_required_stage, Some("SPP"));
        assert_eq!(lifecycle.pr_run_readiness, "blocked");
        assert_stage(&lifecycle, "SPP", "active", false, false);
        assert_stage(&lifecycle, "SOR", "scaffold", false, false);
    }

    #[test]
    fn card_lifecycle_blocks_run_readiness_for_incomplete_active_stp() {
        let repo = lifecycle_temp_repo("incomplete-active-stp");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"reviewed\"\n---\n",
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Policy\n",
                sor: "Branch: codex/3065-test\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.active_stage, "STP");
        assert_eq!(lifecycle.next_required_stage, Some("STP"));
        assert_eq!(lifecycle.pr_run_readiness, "blocked");
        assert_stage(&lifecycle, "STP", "active", false, false);
    }

    #[test]
    fn card_lifecycle_reports_final_review_and_output_truth() {
        let repo = lifecycle_temp_repo("final-srp-sor");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\n---\n",
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nreview_results:\n  findings_status: \"no_findings\"\n  recommended_outcome: \"pass\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\n### Recommended Outcome\n\n- pass\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: none\n- Integration state: merged\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.active_stage, "SOR");
        assert_eq!(lifecycle.next_required_stage, None);
        assert_eq!(lifecycle.pr_finish_readiness, "ready");
        assert_stage(&lifecycle, "SRP", "final", true, true);
        assert_stage(&lifecycle, "SOR", "final", true, true);
    }

    #[test]
    fn card_lifecycle_blocks_final_sor_with_contradictory_status_and_result() {
        let repo = lifecycle_temp_repo("contradictory-sor-status-result");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\n---\n",
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nreview_results:\n  findings_status: \"no_findings\"\n  recommended_outcome: \"pass\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\n### Recommended Outcome\n\n- pass\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: none\n- Integration state: merged\n- Result: FAIL\n\n## Validation\n- focused validation failed\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.pr_finish_readiness, "blocked");
        assert_stage(&lifecycle, "SOR", "active", false, false);
    }

    struct LifecycleFixture<'a> {
        sip: &'a str,
        stp: &'a str,
        spp: &'a str,
        srp: &'a str,
        sor: &'a str,
    }

    struct LifecycleFixturePaths {
        sip: PathBuf,
        stp: PathBuf,
        spp: PathBuf,
        srp: PathBuf,
        sor: PathBuf,
    }

    fn lifecycle_temp_repo(label: &str) -> PathBuf {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let repo = std::env::temp_dir().join(format!(
            "adl-doctor-card-lifecycle-{label}-{now}-{}",
            std::process::id()
        ));
        fs::create_dir_all(&repo).expect("create lifecycle temp repo");
        repo
    }

    fn write_lifecycle_fixture(
        repo: &Path,
        fixture: LifecycleFixture<'_>,
    ) -> LifecycleFixturePaths {
        let bundle = repo.join(".adl/v0.91.2/tasks/issue-3065__fixture");
        fs::create_dir_all(&bundle).expect("create lifecycle fixture bundle");
        let paths = LifecycleFixturePaths {
            sip: bundle.join("sip.md"),
            stp: bundle.join("stp.md"),
            spp: bundle.join("spp.md"),
            srp: bundle.join("srp.md"),
            sor: bundle.join("sor.md"),
        };
        fs::write(&paths.sip, fixture.sip).expect("write sip");
        fs::write(&paths.stp, fixture.stp).expect("write stp");
        fs::write(&paths.spp, fixture.spp).expect("write spp");
        fs::write(&paths.srp, fixture.srp).expect("write srp");
        fs::write(&paths.sor, fixture.sor).expect("write sor");
        paths
    }

    fn assert_stage(
        lifecycle: &DoctorCardLifecycleJson,
        stage: &str,
        state: &str,
        complete: bool,
        final_ready: bool,
    ) {
        let stage = lifecycle
            .stages
            .iter()
            .find(|candidate| candidate.stage == stage)
            .expect("stage exists");
        assert_eq!(stage.state, state);
        assert_eq!(stage.complete, complete);
        assert_eq!(stage.final_ready, final_ready);
    }
}
