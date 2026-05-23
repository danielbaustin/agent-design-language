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
    design_time_complete: bool,
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
        )
        .with_context(|| {
            format!(
                "doctor: failed to finalize closed/completed local bundle truth for issue #{}",
                issue_ref.issue_number()
            )
        })?;
        validate_closed_completed_ready_bundle(
            repo_root,
            issue_ref,
            &root_bundle_input,
            &root_bundle_output,
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
    if !root_stp.is_file() {
        bail!("doctor: missing root stp: {}", root_stp.display());
    }
    validate_bootstrap_stp(repo_root, &root_stp)?;
    validate_authored_prompt_surface("doctor", &root_stp, PromptSurfaceKind::Stp)?;
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

pub(super) fn ensure_pr_run_design_time_ready(
    repo_root: &Path,
    issue_ref: &IssueRef,
    _expected_branch: &str,
) -> Result<()> {
    let root_stp = issue_ref.task_bundle_stp_path(repo_root);
    let root_bundle_input = issue_ref.task_bundle_input_path(repo_root);
    let root_bundle_output = issue_ref.task_bundle_output_path(repo_root);
    let root_bundle_plan = issue_ref.task_bundle_plan_path(repo_root);
    let root_bundle_review_policy = issue_ref.task_bundle_review_policy_path(repo_root);
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
    let lifecycle = build_doctor_card_lifecycle(
        repo_root,
        &root_bundle_input,
        &root_stp,
        &root_bundle_plan,
        &root_bundle_review_policy,
        &root_bundle_output,
    );
    if lifecycle.pr_run_readiness == "ready" {
        return Ok(());
    }

    let blockers = lifecycle
        .stages
        .iter()
        .filter(|stage| ["SIP", "STP", "SPP", "SRP"].contains(&stage.stage))
        .filter(|stage| !stage.design_time_complete)
        .map(|stage| {
            format!(
                "- {}: {}{}",
                stage.stage,
                stage.detail,
                stage
                    .next_editor
                    .map(|editor| format!(" Route through `{editor}`."))
                    .unwrap_or_default()
            )
        })
        .collect::<Vec<_>>();
    bail!(
        "start: design-time card completion gate failed for issue #{} before worktree binding. Repair cards with editor skills before rerunning `pr run`:\n{}",
        issue_ref.issue_number(),
        blockers.join("\n")
    )
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
        .filter(|stage| ["SIP", "STP", "SPP", "SRP"].contains(&stage.stage))
        .all(|stage| stage.design_time_complete)
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
    if has_generic_sip_design_time_scaffold(&text) {
        return card_stage(
            repo_root,
            "SIP",
            path,
            stage_truth(
                "scaffold",
                false,
                false,
                Some("sip-editor"),
                "SIP is still generic bootstrap text and needs issue-specific design-time intent.",
            ),
        );
    }
    card_stage(
        repo_root,
        "SIP",
        path,
        stage_truth(
            "complete",
            true,
            false,
            None,
            "SIP has issue-specific design-time intent for execution planning.",
        ),
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
            stage_truth(
                "complete",
                true,
                false,
                None,
                "STP has the required task intent and acceptance surfaces.",
            ),
        );
    }
    card_stage(
        repo_root,
        "STP",
        path,
        stage_truth(
            "active",
            false,
            false,
            Some("stp-editor"),
            "STP exists but is not complete enough to anchor execution.",
        ),
    )
}

fn classify_spp_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "SPP", path, "spp-editor");
    };
    let status = line_value_after_prefix(&text, "status:").unwrap_or_default();
    if has_generic_spp_design_time_scaffold(&text) {
        return card_stage(
            repo_root,
            "SPP",
            path,
            stage_truth(
                "scaffold",
                false,
                false,
                Some("spp-editor"),
                "SPP is still generic or truncated bootstrap planning text and needs issue-specific plan truth.",
            ),
        );
    }
    if ["reviewed", "approved"].contains(&status.trim_matches('"')) {
        return card_stage(
            repo_root,
            "SPP",
            path,
            stage_truth(
                "complete",
                true,
                false,
                None,
                "SPP has reviewed or approved design-time planning state.",
            ),
        );
    }
    card_stage(
        repo_root,
        "SPP",
        path,
        stage_truth(
            "active",
            false,
            false,
            Some("spp-editor"),
            "SPP is branch-bound but not yet marked reviewed or approved.",
        ),
    )
}

fn has_generic_sip_design_time_scaffold(text: &str) -> bool {
    const MARKERS: &[&str] = &[
        "Prepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound.",
        "Keep the linked issue prompt, SIP, and SOR aligned for review.",
        "The linked source issue prompt is reviewable and structurally valid.",
        "files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt",
        "derive the exact command set from the linked issue prompt",
    ];
    MARKERS.iter().any(|marker| text.contains(marker))
}

fn has_generic_spp_design_time_scaffold(text: &str) -> bool {
    const MARKERS: &[&str] = &[
        "Bootstrap-generated SPP",
        "Design-time generated SPP; review before execution",
        "Review this SPP before execution; during runtime, update it before continuing if the actual execution sequence changes.",
        "generated from source issue prompt, STP/SIP surfaces",
    ];
    MARKERS.iter().any(|marker| text.contains(marker)) || has_truncation_sentinel_line(text)
}

fn has_truncation_sentinel_line(text: &str) -> bool {
    text.lines()
        .map(str::trim)
        .any(|line| matches!(line, "..." | "- ..." | "* ..." | "<...>"))
}

fn classify_srp_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "SRP", path, "srp-editor");
    };
    let has_review_results = srp_has_final_review_results(&text);
    let has_pre_review_absence_exception = text.contains("pre-execution review results are absent");
    let has_policy_exception = (text.contains("explicit policy exception")
        || text.contains("review_results_exception:")
        || text.contains("policy_exception:"))
        && !has_pre_review_absence_exception;
    let pre_execution_review_state = srp_looks_pre_execution_review_state(&text);
    let legacy_policy_only = text.contains("# Structured Review Policy")
        || text.contains("artifact_type: \"structured_review_policy\"");
    let structured_review_prompt = text.contains("# Structured Review Prompt")
        && text.contains("artifact_type: \"structured_review_prompt\"");
    if has_review_results || (has_policy_exception && !pre_execution_review_state) {
        return card_stage(
            repo_root,
            "SRP",
            path,
            stage_truth(
                "final",
                true,
                true,
                None,
                "SRP contains review results or an explicit policy exception for final review truth.",
            ),
        );
    }
    if structured_review_prompt && !legacy_policy_only {
        return card_stage(
            repo_root,
            "SRP",
            path,
            stage_truth(
                "pre_review",
                true,
                false,
                None,
                "SRP is a Structured Review Prompt ready for review; final review results are not recorded yet.",
            ),
        );
    }
    if legacy_policy_only {
        return card_stage(
            repo_root,
            "SRP",
            path,
            stage_truth(
                "legacy_compatible",
                false,
                false,
                Some("srp-editor"),
                "SRP validates as the legacy review-policy scaffold but is not final Structured Review Prompt truth.",
            ),
        );
    }
    card_stage(
        repo_root,
        "SRP",
        path,
        stage_truth(
            "active",
            false,
            false,
            Some("srp-editor"),
            "SRP exists but still needs review results or an explicit policy exception.",
        ),
    )
}

fn srp_looks_pre_execution_review_state(text: &str) -> bool {
    text.contains("pre-execution review results are absent")
        || text.contains("Review results are intentionally absent before implementation exists")
        || text.contains("Run the bounded issue review after implementation.")
        || text.contains("- Not run yet.")
        || text.contains("Not applicable until review runs.")
}

fn validate_closed_completed_ready_bundle(
    repo_root: &Path,
    issue_ref: &IssueRef,
    root_bundle_input: &Path,
    root_bundle_output: &Path,
    bundle_paths: StructuredBundlePaths<'_>,
) -> Result<()> {
    let canonical_bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    let canonical_bundle_missing = !canonical_bundle_dir.is_dir()
        || !ensure_nonempty_file_path(root_bundle_input)?
        || !ensure_nonempty_file_path(root_bundle_output)?
        || !ensure_nonempty_file_path(bundle_paths.plan_path)?
        || !ensure_nonempty_file_path(bundle_paths.review_policy_path)?;
    if canonical_bundle_missing {
        lifecycle::reconcile_closed_completed_issue_bundle(
            repo_root,
            issue_ref,
            root_bundle_output,
        )
        .with_context(|| {
            format!(
                "doctor: failed to restore canonical closeout bundle for closed issue #{}",
                issue_ref.issue_number()
            )
        })?;
    }
    lifecycle::ensure_closed_completed_issue_bundle_truth(repo_root, issue_ref, root_bundle_output)
        .with_context(|| {
            format!(
                "doctor: closed issue #{} has stale canonical closeout truth; run the explicit closeout normalization path instead of doctor --mode ready",
                issue_ref.issue_number()
            )
        })?;
    validate_initialized_cards(
        issue_ref.issue_number(),
        issue_ref.slug(),
        root_bundle_input,
        root_bundle_output,
        repo_root,
        bundle_paths,
    )
}

fn srp_has_final_review_results(text: &str) -> bool {
    if !text.contains("review_results:") {
        return false;
    }
    let findings_status = line_value_after_prefix(text, "findings_status:");
    let recommended_outcome = line_value_after_prefix(text, "recommended_outcome:");
    matches_final_findings_status(findings_status.as_deref())
        && matches_final_recommended_outcome(recommended_outcome.as_deref())
}

fn normalized_review_value(value: Option<&str>) -> Option<String> {
    value.map(|value| {
        value
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .to_ascii_lowercase()
    })
}

fn matches_final_findings_status(value: Option<&str>) -> bool {
    matches!(
        normalized_review_value(value).as_deref(),
        Some("no_findings" | "findings_present")
    )
}

fn matches_final_recommended_outcome(value: Option<&str>) -> bool {
    matches!(
        normalized_review_value(value).as_deref(),
        Some("pass" | "block" | "needs_followup")
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
            stage_truth(
                "scaffold",
                false,
                false,
                Some("sor-editor"),
                "SOR is still the pre-execution output scaffold.",
            ),
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
            stage_truth(
                "final",
                true,
                true,
                None,
                "SOR records terminal integration, validation, closeout, and artifact truth.",
            ),
        );
    }
    if integration_state == "pr_open" && status == "DONE" && result == "PASS" {
        return card_stage(
            repo_root,
            "SOR",
            path,
            stage_truth(
                "complete",
                true,
                false,
                Some("sor-editor"),
                "SOR is complete enough for PR publication but is not terminal closeout truth.",
            ),
        );
    }
    card_stage(
        repo_root,
        "SOR",
        path,
        stage_truth(
            "active",
            false,
            false,
            Some("sor-editor"),
            "SOR exists but does not yet satisfy PR publication or terminal closeout readiness.",
        ),
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
        stage_truth(
            "missing",
            false,
            false,
            Some(editor),
            "Required lifecycle card is missing.",
        ),
    )
}

struct DoctorCardStageTruth {
    state: &'static str,
    complete: bool,
    final_ready: bool,
    next_editor: Option<&'static str>,
    detail: &'static str,
}

fn stage_truth(
    state: &'static str,
    complete: bool,
    final_ready: bool,
    next_editor: Option<&'static str>,
    detail: &'static str,
) -> DoctorCardStageTruth {
    DoctorCardStageTruth {
        state,
        complete,
        final_ready,
        next_editor,
        detail,
    }
}

fn card_stage(
    repo_root: &Path,
    stage: &'static str,
    path: &Path,
    truth: DoctorCardStageTruth,
) -> DoctorCardStageJson {
    DoctorCardStageJson {
        stage,
        path: path_relative_to_repo(repo_root, path),
        state: truth.state,
        complete: truth.complete,
        design_time_complete: matches!(stage, "SIP" | "STP" | "SPP" | "SRP") && truth.complete,
        final_ready: truth.final_ready,
        next_editor: truth.next_editor,
        detail: truth.detail.to_string(),
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
            "CARD_STAGE={}|{}|complete={}|design_time={}|final={}|editor={}|{}",
            stage.stage,
            stage.state,
            stage.complete,
            stage.design_time_complete,
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
        assert_eq!(lifecycle.pr_run_readiness, "blocked");
        assert_eq!(lifecycle.pr_finish_readiness, "blocked");
        assert_stage(&lifecycle, "SRP", "legacy_compatible", false, false);
        assert_stage(&lifecycle, "SOR", "complete", true, false);
    }

    #[test]
    fn card_lifecycle_does_not_treat_placeholder_srp_results_as_final() {
        let repo = lifecycle_temp_repo("placeholder-srp-results");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"reviewed\"\n---\n",
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\nreview_results:\n  findings_status: \"not_run | findings_present | no_findings\"\n  recommended_outcome: \"pass | block | needs_followup | not_run\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\n### Recommended Outcome\n\n- <pass, block, needs_followup, or not_run>\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.active_stage, "SRP");
        assert_eq!(lifecycle.next_required_stage, Some("SRP"));
        assert_eq!(lifecycle.pr_finish_readiness, "blocked");
        assert_stage(&lifecycle, "SRP", "legacy_compatible", false, false);
    }

    #[test]
    fn card_lifecycle_does_not_treat_unknown_srp_result_values_as_final() {
        let repo = lifecycle_temp_repo("unknown-srp-results");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"reviewed\"\n---\n",
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nreview_results:\n  findings_status: \"todo\"\n  recommended_outcome: \"ship_it\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\n### Recommended Outcome\n\n- ship_it\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.active_stage, "SRP");
        assert_eq!(lifecycle.pr_finish_readiness, "blocked");
        assert_stage(&lifecycle, "SRP", "legacy_compatible", false, false);
    }

    #[test]
    fn card_lifecycle_allows_explicit_srp_policy_exception() {
        let repo = lifecycle_temp_repo("srp-policy-exception");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"reviewed\"\n---\n",
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nreview_results_exception: \"explicit policy exception: docs-only no-op review\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\nexplicit policy exception recorded\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_stage(&lifecycle, "SRP", "final", true, true);
        assert_eq!(lifecycle.pr_finish_readiness, "ready");
    }

    #[test]
    fn card_lifecycle_accepts_pre_review_srp_prompt_without_final_results() {
        let repo = lifecycle_temp_repo("pre-review-srp-prompt");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"reviewed\"\n---\n",
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Prompt\n\n## Review Instructions\n\nRun the bounded issue review after implementation.\n",
                sor: "Branch: not bound yet\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.pr_run_readiness, "ready");
        assert_eq!(lifecycle.pr_finish_readiness, "blocked");
        assert_stage(&lifecycle, "SRP", "pre_review", true, false);
        let srp = lifecycle
            .stages
            .iter()
            .find(|stage| stage.stage == "SRP")
            .expect("srp stage exists");
        assert!(srp.design_time_complete);
        assert_eq!(srp.next_editor, None);
    }

    #[test]
    fn card_lifecycle_does_not_treat_pre_execution_srp_absence_as_final_exception() {
        let repo = lifecycle_temp_repo("pre-review-srp-absence-exception");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"reviewed\"\n---\n",
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent until implementation exists\"\n---\n\n# Structured Review Prompt\n\n## Review Instructions\n\nRun the bounded issue review after implementation.\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.pr_run_readiness, "ready");
        assert_eq!(lifecycle.pr_finish_readiness, "blocked");
        assert_stage(&lifecycle, "SRP", "pre_review", true, false);
    }

    #[test]
    fn card_lifecycle_accepts_terminal_structured_review_prompt_exception() {
        let repo = lifecycle_temp_repo("srp-prompt-exception-final");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"reviewed\"\n---\n",
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nreview_results_exception: \"explicit policy exception: docs-only no-op review\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\nexplicit policy exception recorded\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.pr_finish_readiness, "ready");
        assert_stage(&lifecycle, "SRP", "final", true, true);
    }

    #[test]
    fn closed_ready_validation_is_read_only_and_reports_truth_drift() {
        let repo = lifecycle_temp_repo("closed-ready-read-only");
        let issue_ref = IssueRef::new(1410, "v0.91.2", "fixture").expect("issue ref");
        let bundle = issue_ref.task_bundle_dir_path(&repo);
        fs::create_dir_all(&bundle).expect("create bundle");

        let sip = bundle.join("sip.md");
        let stp = bundle.join("stp.md");
        let spp = bundle.join("spp.md");
        let srp = bundle.join("srp.md");
        let sor = bundle.join("sor.md");

        let sip_text = format!(
            "# ADL Input Card\n\nTask ID: {task_id}\nRun ID: {task_id}\nVersion: v0.91.2\nTitle: Fixture\nBranch: codex/1410-fixture\n\nContext:\n- Issue: https://github.com/example/repo/issues/{issue}\n- PR: https://github.com/example/repo/pull/{issue}\n- Source Issue Prompt: .adl/v0.91.2/bodies/issue-1410-fixture.md\n- Docs: none\n- Other: none\n\n## Agent Execution Rules\n- Do not run `pr start`; the branch and worktree already exist.\n- Only modify files required for the issue.\n\n## Prompt Spec\n```yaml\nprompt_schema: adl.v1\nactor:\n  role: execution_agent\n  name: codex\nmodel:\n  id: gpt-5-codex\n  determinism_mode: stable\ninputs:\n  sections:\n    - goal\n    - required_outcome\n    - acceptance_criteria\n    - target_files_surfaces\n    - validation_plan\noutputs:\n  output_card: .adl/v0.91.2/tasks/{bundle}/sor.md\nconstraints:\n  disallow_secrets: true\n  disallow_absolute_host_paths: true\n```\n\n## Goal\n\nKeep the closed-ready doctor path read-only when closeout truth is stale.\n\n## Required Outcome\n\n- The issue must refuse stale closed-issue truth without mutating the bundle.\n\n## Acceptance Criteria\n\n- closed-ready validation reports stale truth\n- the stale SOR remains byte-identical after validation fails\n\n## Target Files / Surfaces\n- adl/src/cli/pr_cmd/doctor.rs\n\n## Validation Plan\n- `cargo test --manifest-path adl/Cargo.toml closed_ready_validation_is_read_only_and_reports_truth_drift -- --nocapture`\n\n## Demo / Proof Requirements\n- none\n\n## Non-goals / Out of scope\n- runtime closeout mutation\n",
            task_id = issue_ref.task_issue_id(),
            issue = issue_ref.issue_number(),
            bundle = issue_ref.task_bundle_dir_name(),
        );
        let stp_text = "## Required Outcome\n\n- closed-ready validation stays read-only when canonical closeout truth is stale\n\n## Acceptance Criteria\n\n- stale closeout truth causes a blocking validation error\n- no bundle files are mutated on failure\n";
        let spp_text = format!(
            "---\nschema_version: \"0.1\"\nartifact_type: \"structured_planning_prompt\"\nname: \"fixture-plan\"\nissue: {issue}\ntask_id: \"{task_id}\"\nrun_id: \"{task_id}\"\nversion: v0.91.2\ntitle: \"Fixture\"\nbranch: \"codex/1410-fixture\"\nstatus: \"reviewed\"\nactivation_state: \"reviewed\"\nplan_revision: 1\nsource_refs:\n  - kind: \"issue\"\n    ref: \"https://github.com/example/repo/issues/{issue}\"\nscope:\n  files:\n    - \".adl/v0.91.2/tasks/{bundle}/sip.md\"\nconstraints:\n  - \"read_only_until_execution_is_approved\"\nconfidence: \"medium\"\nplan_summary: \"Fixture plan for closed-ready validation.\"\nassumptions:\n  - \"The canonical bundle already exists.\"\nproposed_steps:\n  - id: \"step-1\"\n    description: \"Validate closed-ready truth without mutation.\"\n    expected_output: \".adl/v0.91.2/tasks/{bundle}/spp.md\"\n    allowed_mode: \"execution_after_approval\"\ncodex_plan:\n  - step: \"Validate closed-ready truth without mutation.\"\n    status: \"pending\"\naffected_areas:\n  - \"doctor\"\ninvariants_to_preserve:\n  - \"Do not mutate stale closeout truth during validation.\"\nrisks_and_edge_cases:\n  - \"Closed issue bundles can still drift.\"\ntest_strategy:\n  - \"Run the focused doctor regression test.\"\nexecution_handoff: \"Use this artifact as the durable plan-of-record before execution.\"\nrequired_permissions:\n  - \"workspace-write after execution approval\"\nstop_conditions:\n  - \"Stop if validation would mutate the stale bundle.\"\nalternatives_considered:\n  - description: \"Use transient planning only.\"\n    reason_not_chosen: \"That would not leave durable reviewable plan truth.\"\nreview_hooks:\n  - \"Check read-only behavior.\"\nnotes: \"fixture\"\n---\n\n# Structured Plan Prompt\n\n## Plan Summary\n\nFixture plan.\n\n## Codex Plan\n\n1. [pending] Validate closed-ready truth without mutation.\n",
            issue = issue_ref.issue_number(),
            task_id = issue_ref.task_issue_id(),
            bundle = issue_ref.task_bundle_dir_name(),
        );
        let srp_text = format!(
            "---\nschema_version: \"0.1\"\nartifact_type: \"structured_review_prompt\"\nname: \"fixture-review\"\nissue: {issue}\ntask_id: \"{task_id}\"\nversion: v0.91.2\ntitle: \"Fixture\"\nbranch: \"codex/1410-fixture\"\nstatus: \"draft\"\nsource_refs:\n  - kind: \"issue\"\n    ref: \"https://github.com/example/repo/issues/{issue}\"\nreview_mode: \"pre_pr_independent_review\"\ntiming: \"before_pr_open\"\nscope_basis:\n  - \".adl/v0.91.2/tasks/{bundle}/sip.md\"\nin_scope_surfaces:\n  - \"tracked changes for this issue branch\"\nevidence_policy:\n  - \"Use repository evidence and issue-local validation only.\"\nvalidation_inputs:\n  - \"Issue-local proofs recorded in the SOR.\"\nallowed_dispositions:\n  - \"PASS\"\n  - \"BLOCK\"\nreviewer_constraints:\n  - \"Do not widen issue scope.\"\nrefusal_policy:\n  - \"Refuse unsupported claims.\"\nfollow_up_routing:\n  - \"Route findings back to the issue branch.\"\nnon_claims:\n  - \"This prompt does not claim review has already run.\"\npolicy_refs:\n  - \".adl/v0.91.2/tasks/{bundle}/spp.md\"\nfindings_status: \"not_run\"\nrecommended_outcome: \"not_applicable\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent until implementation exists\"\n---\n\n# Structured Review Prompt\n\n## Review Instructions\n\nRun the bounded issue review after implementation.\n",
            issue = issue_ref.issue_number(),
            task_id = issue_ref.task_issue_id(),
            bundle = issue_ref.task_bundle_dir_name(),
        );
        fs::write(&sip, sip_text).expect("write sip");
        fs::write(&stp, stp_text).expect("write stp");
        fs::write(&spp, spp_text).expect("write spp");
        fs::write(&srp, srp_text).expect("write srp");
        let stale_sor = "# issue-1410-fixture\n\nTask ID: issue-1410\nRun ID: issue-1410\nVersion: v0.91.2\nTitle: Fixture\nBranch: codex/1410-fixture\nStatus: IN_PROGRESS\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: adl/src/foo.rs\n- Integration state: pr_open\n- Verification scope: worktree\n- Result: PASS\n";
        fs::write(&sor, stale_sor).expect("write stale sor");

        let err = validate_closed_completed_ready_bundle(
            &repo,
            &issue_ref,
            &sip,
            &sor,
            StructuredBundlePaths {
                plan_path: &spp,
                review_policy_path: &srp,
            },
        )
        .expect_err("stale closeout truth should fail");

        let _ = err;
        assert_eq!(fs::read_to_string(&sor).expect("read sor"), stale_sor);
    }

    #[test]
    fn card_lifecycle_allows_ellipsis_in_reviewed_spp_prose() {
        let repo = lifecycle_temp_repo("spp-ellipsis-prose");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"reviewed\"\n---\n\n# Structured Plan Prompt\n\n## Validation\n\nInspect provider output like `downloading... done` without treating it as truncation.\n",
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Prompt\n",
                sor: "Branch: not bound yet\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.pr_run_readiness, "ready");
        assert_stage(&lifecycle, "SPP", "complete", true, false);
    }

    #[test]
    fn card_lifecycle_blocks_generic_pre_run_spp_before_execution() {
        let repo = lifecycle_temp_repo("generic-pre-run-spp");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: not bound yet\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"not bound yet\"\nstatus: \"draft\"\n---\n\nBootstrap-generated SPP; revise before use if planning review is required.\n",
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"not bound yet\"\nstatus: \"draft\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent\"\n---\n\n# Structured Review Prompt\n",
                sor: "Branch: not bound yet\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.pr_run_readiness, "blocked");
        assert_stage(&lifecycle, "SIP", "complete", true, false);
        assert_stage(&lifecycle, "SPP", "scaffold", false, false);
        assert_eq!(
            lifecycle
                .stages
                .iter()
                .find(|stage| stage.stage == "SIP")
                .and_then(|stage| stage.next_editor),
            None
        );
        assert_eq!(
            lifecycle
                .stages
                .iter()
                .find(|stage| stage.stage == "SPP")
                .and_then(|stage| stage.next_editor),
            Some("spp-editor")
        );
    }

    #[test]
    fn card_lifecycle_blocks_generic_sip_before_execution() {
        let repo = lifecycle_temp_repo("generic-sip");
        let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: not bound yet\n\n## Goal\n\nPrepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound.\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: "---\nbranch: \"not bound yet\"\nstatus: \"approved\"\n---\n\n# Structured Plan Prompt\n",
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"not bound yet\"\nstatus: \"draft\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent\"\n---\n\n# Structured Review Prompt\n",
                sor: "Branch: not bound yet\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

        let lifecycle = build_doctor_card_lifecycle(
            &repo, &paths.sip, &paths.stp, &paths.spp, &paths.srp, &paths.sor,
        );

        assert_eq!(lifecycle.pr_run_readiness, "blocked");
        assert_stage(&lifecycle, "SIP", "scaffold", false, false);
        assert_eq!(
            lifecycle
                .stages
                .iter()
                .find(|stage| stage.stage == "SIP")
                .and_then(|stage| stage.next_editor),
            Some("sip-editor")
        );
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

    #[test]
    fn card_lifecycle_accepts_tracked_csdlc_bundle() {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("adl crate lives under repo root")
            .to_path_buf();
        let bundle =
            repo_root.join("workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards");
        let lifecycle = build_doctor_card_lifecycle(
            &repo_root,
            &bundle.join("sip.md"),
            &bundle.join("stp.md"),
            &bundle.join("spp.md"),
            &bundle.join("srp.md"),
            &bundle.join("sor.md"),
        );

        assert_eq!(lifecycle.active_stage, "SOR");
        assert_eq!(lifecycle.next_required_stage, None);
        assert_eq!(lifecycle.pr_run_readiness, "ready");
        assert_eq!(lifecycle.pr_finish_readiness, "ready");
        assert_stage(&lifecycle, "SIP", "complete", true, false);
        assert_stage(&lifecycle, "STP", "complete", true, false);
        assert_stage(&lifecycle, "SPP", "complete", true, false);
        assert_stage(&lifecycle, "SRP", "final", true, true);
        assert_stage(&lifecycle, "SOR", "final", true, true);
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
