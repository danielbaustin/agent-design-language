use std::path::{Path, PathBuf};

use super::*;

pub(super) fn run_doctor_ready(
    repo_root: &Path,
    repo: &str,
    issue_ref: &IssueRef,
    branch: &str,
) -> Result<DoctorReadyResult> {
    let mut worktree_path = issue_ref.default_worktree_path(
        repo_root,
        std::env::var_os("ADL_WORKTREE_ROOT")
            .map(PathBuf::from)
            .as_deref(),
    );
    if let Ok(cwd) = std::env::current_dir() {
        if cwd != repo_root && cwd.join(".git").exists() {
            if let Ok(current_branch) = run_capture(
                "git",
                &["-C", path_str(&cwd)?, "rev-parse", "--abbrev-ref", "HEAD"],
            ) {
                if current_branch.trim() == branch {
                    worktree_path = cwd;
                }
            }
        }
    }
    let source_path = resolve_doctor_issue_prompt_path(repo_root, issue_ref)?;
    let root_stp = issue_ref.task_bundle_stp_path(repo_root);
    let wt_stp = issue_ref.task_bundle_stp_path(&worktree_path);
    let root_bundle_input = issue_ref.task_bundle_input_path(repo_root);
    let root_bundle_output = issue_ref.task_bundle_output_path(repo_root);
    let root_bundle_plan = issue_ref.task_bundle_plan_path(repo_root);
    let root_bundle_validation_plan = issue_ref.task_bundle_validation_plan_path(repo_root);
    let root_bundle_review_policy = issue_ref.task_bundle_review_policy_path(repo_root);
    let wt_bundle_input = issue_ref.task_bundle_input_path(&worktree_path);
    let wt_bundle_output = issue_ref.task_bundle_output_path(&worktree_path);
    let wt_bundle_plan = issue_ref.task_bundle_plan_path(&worktree_path);
    let wt_bundle_validation_plan = issue_ref.task_bundle_validation_plan_path(&worktree_path);
    let wt_bundle_review_policy = issue_ref.task_bundle_review_policy_path(&worktree_path);
    let closed_completed = crate::cli::pr_cmd::lifecycle::issue_is_closed_and_completed(
        issue_ref.issue_number(),
        repo,
    )?;

    validate_issue_prompt_exists(&source_path)?;
    validate_bootstrap_stp(repo_root, &source_path)?;
    validate_authored_prompt_surface("doctor", &source_path, PromptSurfaceKind::IssuePrompt)?;
    if closed_completed {
        validate_closed_completed_ready_bundle(
            repo_root,
            issue_ref,
            &root_bundle_input,
            &root_bundle_output,
            StructuredBundlePaths {
                plan_path: &root_bundle_plan,
                validation_plan_path: &root_bundle_validation_plan,
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
                &root_bundle_validation_plan,
                &root_bundle_review_policy,
                &root_bundle_output,
            ),
            status: "PASS",
        });
    }
    let root_bundle_complete = [
        &root_stp,
        &root_bundle_input,
        &root_bundle_output,
        &root_bundle_plan,
        &root_bundle_validation_plan,
        &root_bundle_review_policy,
    ]
    .iter()
    .all(|path| path.is_file());
    let wt_bundle_complete = [
        &wt_stp,
        &wt_bundle_input,
        &wt_bundle_output,
        &wt_bundle_plan,
        &wt_bundle_validation_plan,
        &wt_bundle_review_policy,
    ]
    .iter()
    .all(|path| path.is_file());
    let validation_repo_root =
        ready_validation_repo_root(repo_root, &worktree_path, wt_bundle_complete);
    if !root_bundle_complete && worktree_path.is_dir() && wt_bundle_complete {
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
        validate_bootstrap_stp(&worktree_path, &wt_stp)?;
        validate_authored_prompt_surface("doctor", &wt_stp, PromptSurfaceKind::Stp)?;
        validate_ready_cards(
            &worktree_path,
            issue_ref.issue_number(),
            issue_ref.slug(),
            wt_branch.trim(),
            &wt_bundle_input,
            &wt_bundle_output,
            StructuredBundlePaths {
                plan_path: &wt_bundle_plan,
                validation_plan_path: &wt_bundle_validation_plan,
                review_policy_path: &wt_bundle_review_policy,
            },
        )?;
        let card_lifecycle = build_doctor_card_lifecycle(
            repo_root,
            &wt_bundle_input,
            &wt_stp,
            &wt_bundle_plan,
            &wt_bundle_validation_plan,
            &wt_bundle_review_policy,
            &wt_bundle_output,
        );
        let status = doctor_ready_status_for(&card_lifecycle);
        return Ok(DoctorReadyResult {
            lifecycle_state: "run_bound",
            worktree: Some(path_relative_to_repo(repo_root, &worktree_path)),
            source: path_relative_to_repo(repo_root, &source_path),
            root_stp: path_relative_to_repo(repo_root, &root_stp),
            root_input: path_relative_to_repo(repo_root, &root_bundle_input),
            root_output: path_relative_to_repo(repo_root, &root_bundle_output),
            wt_stp: Some(path_relative_to_repo(repo_root, &wt_stp)),
            wt_input: Some(path_relative_to_repo(repo_root, &wt_bundle_input)),
            wt_output: Some(path_relative_to_repo(repo_root, &wt_bundle_output)),
            card_lifecycle,
            status,
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
        validation_repo_root,
        StructuredBundlePaths {
            plan_path: &root_bundle_plan,
            validation_plan_path: &root_bundle_validation_plan,
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
            let card_lifecycle = build_doctor_card_lifecycle(
                repo_root,
                &root_bundle_input,
                &root_stp,
                &root_bundle_plan,
                &root_bundle_validation_plan,
                &root_bundle_review_policy,
                &root_bundle_output,
            );
            let status = doctor_ready_status_for(&card_lifecycle);
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
                card_lifecycle,
                status,
            });
        }
        bail!("doctor: missing worktree: {}", worktree_path.display());
    }
    if root_indicates_pre_run
        && (!wt_stp.is_file() || !wt_bundle_input.is_file() || !wt_bundle_output.is_file())
    {
        let card_lifecycle = build_doctor_card_lifecycle(
            repo_root,
            &root_bundle_input,
            &root_stp,
            &root_bundle_plan,
            &root_bundle_validation_plan,
            &root_bundle_review_policy,
            &root_bundle_output,
        );
        let status = doctor_ready_status_for(&card_lifecycle);
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
            card_lifecycle,
            status,
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
        if stale_worktree_branch_mismatch_preserves_pre_run(root_indicates_pre_run, branch, wt_branch.trim()) {
            let card_lifecycle = build_doctor_card_lifecycle(
                repo_root,
                &root_bundle_input,
                &root_stp,
                &root_bundle_plan,
                &root_bundle_validation_plan,
                &root_bundle_review_policy,
                &root_bundle_output,
            );
            let status = doctor_ready_status_for(&card_lifecycle);
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
                card_lifecycle,
                status,
            });
        }
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
        validation_repo_root,
        issue_ref.issue_number(),
        issue_ref.slug(),
        wt_branch.trim(),
        &root_bundle_input,
        &root_bundle_output,
        StructuredBundlePaths {
            plan_path: &root_bundle_plan,
            validation_plan_path: &root_bundle_validation_plan,
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
            validation_plan_path: &wt_bundle_validation_plan,
            review_policy_path: &wt_bundle_review_policy,
        },
    )?;

    let card_lifecycle = build_doctor_card_lifecycle(
        repo_root,
        &wt_bundle_input,
        &wt_stp,
        &wt_bundle_plan,
        &wt_bundle_validation_plan,
        &wt_bundle_review_policy,
        &wt_bundle_output,
    );
    let status = doctor_ready_status_for(&card_lifecycle);

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
        card_lifecycle,
        status,
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
    let root_bundle_validation_plan = issue_ref.task_bundle_validation_plan_path(repo_root);
    let root_bundle_review_policy = issue_ref.task_bundle_review_policy_path(repo_root);
    validate_initialized_cards(
        issue_ref.issue_number(),
        issue_ref.slug(),
        &root_bundle_input,
        &root_bundle_output,
        repo_root,
        StructuredBundlePaths {
            plan_path: &root_bundle_plan,
            validation_plan_path: &root_bundle_validation_plan,
            review_policy_path: &root_bundle_review_policy,
        },
    )?;
    let lifecycle = build_doctor_card_lifecycle(
        repo_root,
        &root_bundle_input,
        &root_stp,
        &root_bundle_plan,
        &root_bundle_validation_plan,
        &root_bundle_review_policy,
        &root_bundle_output,
    );
    if lifecycle.pr_run_readiness == "ready" {
        return Ok(());
    }

    let blockers = lifecycle
        .stages
        .iter()
        .filter(|stage| ["SIP", "STP", "SPP", "VPP", "SRP"].contains(&stage.stage))
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

pub(super) fn ready_validation_repo_root<'a>(
    repo_root: &'a Path,
    worktree_path: &'a Path,
    wt_bundle_complete: bool,
) -> &'a Path {
    if worktree_path != repo_root && wt_bundle_complete {
        worktree_path
    } else {
        repo_root
    }
}

pub(super) fn stale_worktree_branch_mismatch_preserves_pre_run(
    root_indicates_pre_run: bool,
    expected_branch: &str,
    observed_branch: &str,
) -> bool {
    root_indicates_pre_run && observed_branch.trim() != expected_branch
}
