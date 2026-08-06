use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use csdlc_v2::cards::{PlanStep, ResourceProfile, StepStatus, ValidationLane};
use csdlc_v2::{BootstrapRequest, InitialCardInput, PlanningProfile};

fn command(root: &Path, program: &str, args: &[&str]) -> Output {
    Command::new(program)
        .current_dir(root)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run {program}: {error}"))
}

fn must_succeed(output: Output) -> String {
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("UTF-8 output")
}

fn git(root: &Path, args: &[&str]) -> String {
    must_succeed(command(root, "git", args))
}

fn request() -> BootstrapRequest {
    BootstrapRequest {
        issue: 42,
        repository: "example/repo".into(),
        actor: "test-operator".into(),
        design_path: "design/issue-42.md".into(),
        diagram_path: "design/issue-42.mmd".into(),
        design_reviewer: "reviewer".into(),
        design_approved: true,
        initial: InitialCardInput {
            title: "Claim-free issue workflow".into(),
            slug: "claim-free-issue-workflow".into(),
            version: "v0.92".into(),
            goal: "Prove the claim-free issue workflow.".into(),
            required_outcome: "Create, validate, diagnose, and bind one issue.".into(),
            declared_scope: vec!["claim-free workflow".into()],
            authority_boundary: vec!["local test repository".into()],
            operator_constraints: vec!["no network".into()],
            task_boundary: "Exercise only the focused binary path.".into(),
            deliverables: vec!["bound issue record".into()],
            acceptance_criteria: vec![
                "issue creation is claim-free".into(),
                "binding is atomic and idempotent".into(),
            ],
            dependencies: vec!["none".into()],
            repo_inputs: vec!["design/issue-42.md".into()],
            non_goals: vec!["publication".into()],
            plan_summary: "Create, validate, diagnose, and bind.".into(),
            steps: vec![PlanStep {
                id: "step-1".into(),
                action: "run the focused workflow".into(),
                acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                status: StepStatus::Pending,
            }],
            affected_areas: vec!["design/issue-42.md".into()],
            invariants: vec!["Git topology is binding authority".into()],
            risks: vec!["conflicting worktree".into()],
            planning_profile: PlanningProfile::Small,
            stop_conditions: vec!["topology conflict".into()],
            validation_lanes: vec![ValidationLane {
                lane: "focused".into(),
                proof_role: "actual binary workflow".into(),
                acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                deterministic: true,
                resource_profile: ResourceProfile::Small,
                budget_seconds: 120,
                budget_tokens: 1_000,
                argv: vec![
                    "cargo".into(),
                    "test".into(),
                    "--test".into(),
                    "gate2".into(),
                ],
                parallel_group: "local".into(),
                defer_reason: None,
            }],
            failure_policy: "Fail closed on invalid input or topology conflict.".into(),
            review_prompts: vec!["Review atomicity and idempotence.".into()],
            review_scope: "claim-free issue creation and binding".into(),
        },
    }
}

#[test]
fn actual_binaries_create_validate_doctor_and_bind_without_claims() {
    let temp = tempfile::tempdir().expect("tempdir");
    let repo = temp.path().join("repo");
    let worktree = temp.path().join("worktrees/issue-42");
    let conflict = temp.path().join("worktrees/conflict");
    fs::create_dir_all(repo.join("docs/templates/prompts")).expect("registry directory");
    fs::create_dir_all(repo.join("csdlc-v2/operator")).expect("manifest directory");
    fs::create_dir_all(repo.join("design")).expect("design directory");
    fs::write(
        repo.join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry fixture");
    fs::write(
        repo.join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("shape fixture");
    fs::write(repo.join("design/issue-42.md"), "# Approved design\n").expect("design");
    fs::write(
        repo.join("design/issue-42.mmd"),
        "flowchart LR\n  Create --> Bind\n",
    )
    .expect("diagram");

    git(&repo, &["init", "-b", "main"]);
    git(&repo, &["config", "user.email", "test@example.com"]);
    git(&repo, &["config", "user.name", "C-SDLC Test"]);
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-m", "fixture"]);

    let invalid_create_request = temp.path().join("invalid-create.json");
    let mut invalid_create = request();
    invalid_create.issue = 40;
    invalid_create.design_path = "generated/invalid-design.md".into();
    invalid_create.diagram_path = "generated/invalid-diagram.mmd".into();
    invalid_create.initial.affected_areas.clear();
    fs::write(
        &invalid_create_request,
        serde_json::to_vec_pretty(&invalid_create).expect("serialize invalid create request"),
    )
    .expect("invalid create request");
    let repo_text = repo.to_string_lossy();
    let invalid_create_text = invalid_create_request.to_string_lossy();
    let invalid_created = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &invalid_create_text,
        ],
    );
    assert!(!invalid_created.status.success());
    assert!(!repo.join("generated/invalid-design.md").exists());
    assert!(!repo.join("generated/invalid-diagram.mmd").exists());

    let non_proving_request = temp.path().join("non-proving-create.json");
    let mut non_proving = request();
    non_proving.issue = 41;
    non_proving.design_path = "generated/non-proving-design.md".into();
    non_proving.diagram_path = "generated/non-proving-diagram.mmd".into();
    non_proving.initial.validation_lanes[0].argv = vec!["true".into()];
    fs::write(
        &non_proving_request,
        serde_json::to_vec_pretty(&non_proving).expect("serialize non-proving request"),
    )
    .expect("non-proving request");
    let non_proving_text = non_proving_request.to_string_lossy();
    let non_proving_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &non_proving_text,
        ],
    );
    assert!(!non_proving_result.status.success());
    assert!(!repo.join("generated/non-proving-design.md").exists());
    assert!(!repo.join("generated/non-proving-diagram.mmd").exists());

    let create_request = temp.path().join("create.json");
    let mut create = serde_json::to_value(request()).expect("serialize create request");
    create["claim"] = serde_json::json!({"id": "ignored-legacy-create-claim"});
    fs::write(
        &create_request,
        serde_json::to_vec_pretty(&create).expect("serialize create request"),
    )
    .expect("create request");
    let create_text = create_request.to_string_lossy();
    let created = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &["--root", &repo_text, "create", "--request", &create_text],
    ));
    assert!(!created.contains("claim"));

    let validated = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo_text, "issue", "--issue", "42"],
    ));
    assert!(validated.contains("\"status\":\"pass\""));
    let diagnosed = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &repo_text, "--issue", "42"],
    ));
    assert!(diagnosed.contains("\"ready\": true"));

    let duplicate_request = temp.path().join("duplicate-create.json");
    let mut duplicate = request();
    duplicate.design_path = "generated/duplicate-design.md".into();
    duplicate.diagram_path = "generated/duplicate-diagram.mmd".into();
    fs::write(
        &duplicate_request,
        serde_json::to_vec_pretty(&duplicate).expect("serialize duplicate request"),
    )
    .expect("duplicate request");
    let duplicate_text = duplicate_request.to_string_lossy();
    let duplicate_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &["--root", &repo_text, "create", "--request", &duplicate_text],
    );
    assert!(!duplicate_result.status.success());
    assert!(!repo.join("generated/duplicate-design.md").exists());
    assert!(!repo.join("generated/duplicate-diagram.mmd").exists());

    let bind_request = temp.path().join("bind.json");
    let bind = serde_json::json!({
        "issue": 42,
        "base_branch": "main",
        "branch": "issue-42",
        "worktree": worktree,
        "claim": {"id": "ignored-legacy-bind-claim"},
    });
    fs::write(
        &bind_request,
        serde_json::to_vec_pretty(&bind).expect("serialize bind request"),
    )
    .expect("bind request");
    let bind_text = bind_request.to_string_lossy();

    fs::write(repo.join("design/issue-42.md"), "# Stale design\n").expect("stale design");
    let invalid_validation = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo_text, "issue", "--issue", "42"],
    );
    assert!(!invalid_validation.status.success());
    let invalid_bind = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!invalid_bind.status.success());
    assert!(!worktree.exists());
    assert!(!git(&repo, &["branch", "--list", "issue-42"]).contains("issue-42"));
    fs::write(repo.join("design/issue-42.md"), "# Approved design\n").expect("restore design");

    let first = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    ));
    assert!(first.contains("\"created\":true"));
    let topology = git(&repo, &["worktree", "list", "--porcelain"]);
    assert!(topology.contains(&format!(
        "worktree {}",
        worktree.canonicalize().unwrap().display()
    )));
    assert!(topology.contains("branch refs/heads/issue-42"));

    let second = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    ));
    assert!(second.contains("\"created\":false"));

    let index = worktree.join(".csdlc/issues/42/index.json");
    let mut contradictory: serde_json::Value =
        serde_json::from_slice(&fs::read(&index).expect("bound index")).expect("index JSON");
    contradictory["branch"] = serde_json::json!("different-branch");
    fs::write(
        &index,
        serde_json::to_vec_pretty(&contradictory).expect("serialize contradictory index"),
    )
    .expect("contradictory index");
    let inconsistent = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!inconsistent.status.success());

    let conflict_request = temp.path().join("conflict.json");
    let conflict_bind = serde_json::json!({
        "issue": 42,
        "base_branch": "main",
        "branch": "issue-42-conflict",
        "worktree": conflict,
    });
    fs::write(
        &conflict_request,
        serde_json::to_vec_pretty(&conflict_bind).expect("serialize conflict request"),
    )
    .expect("conflict request");
    let conflict_text = conflict_request.to_string_lossy();
    let rejected = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &conflict_text],
    );
    assert!(!rejected.status.success());
    let rejection = String::from_utf8_lossy(&rejected.stdout);
    assert!(rejection.contains("reconciliation_required") || rejection.contains("corrupt_record"));

    git(
        &repo,
        &["worktree", "remove", "--force", &worktree.to_string_lossy()],
    );
}
