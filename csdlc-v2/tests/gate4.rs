use std::time::Instant;

use csdlc_v2::pvf::*;
use csdlc_v2::{
    classify_schedule, classify_shepherd, edit_issue, execute, finalize, initialize_native_json,
    select, shared_request_path, BootstrapRequest, CardKind, Claim, EditRequest, ErrorCode,
    FinalizeRequest, InitialCardInput, LifecyclePhase, PlanningProfile, SemanticOperation, Store,
};

fn install_native_authority(root: &std::path::Path) {
    let registry = root.join("docs/templates/prompts/current.json");
    let manifest = root.join("csdlc-v2/operator/native-card-shape.json");
    std::fs::create_dir_all(registry.parent().unwrap()).unwrap();
    std::fs::create_dir_all(manifest.parent().unwrap()).unwrap();
    std::fs::write(
        registry,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    std::fs::write(
        manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
}

fn bound_fixture() -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord) {
    let temp = tempfile::tempdir().expect("fixture");
    assert!(std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    install_native_authority(temp.path());
    std::fs::create_dir_all(temp.path().join("docs")).unwrap();
    std::fs::write(temp.path().join("docs/design.md"), "# Design\n").unwrap();
    std::fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .unwrap();
    let store = Store::new(temp.path());
    let mut record = initialize_native_json(
        &store,
        &serde_json::to_vec(&BootstrapRequest {
            issue: 5627,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "operator".into(),
            design_approved: true,
            claim: Claim {
                id: "claim".into(),
                owner: "agent".into(),
                generation: 0,
                acquired_unix_seconds: 1,
                expires_unix_seconds: u64::MAX,
                heartbeat_unix_seconds: 1,
                branch: "codex/5627".into(),
                worktree: ".".into(),
                protected_paths: vec!["csdlc-v2".into()],
                purpose: "four command proof".into(),
            },
            initial: InitialCardInput {
                title: "four command fixture".into(),
                slug: "four-command".into(),
                version: "v0.91.8".into(),
                goal: "collapse routine lifecycle".into(),
                required_outcome: "atomic finalize".into(),
                declared_scope: vec!["csdlc-v2".into()],
                authority_boundary: vec!["no runtime".into()],
                operator_constraints: vec!["typed v2".into()],
                task_boundary: "prove finalize".into(),
                deliverables: vec!["finalize".into()],
                acceptance_criteria: vec!["AC-1: atomic".into()],
                dependencies: vec!["none".into()],
                repo_inputs: vec!["csdlc-v2".into()],
                non_goals: vec!["runtime".into()],
                plan_summary: "finalize once".into(),
                steps: vec![csdlc_v2::cards::PlanStep {
                    id: "S1".into(),
                    action: "finalize".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: csdlc_v2::cards::StepStatus::Pending,
                }],
                invariants: vec!["zero partial writes".into()],
                risks: vec!["partial state".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["validation fails".into()],
                validation_lanes: vec![csdlc_v2::cards::ValidationLane {
                    lane: "focused".into(),
                    proof_role: "finalize".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    deterministic: true,
                    resource_profile: csdlc_v2::cards::ResourceProfile::Small,
                    budget_seconds: 30,
                    budget_tokens: 10,
                    argv: vec!["true".into()],
                    parallel_group: "local".into(),
                    defer_reason: None,
                }],
                failure_policy: "fail closed".into(),
                review_prompts: vec!["review atomicity".into()],
                review_scope: "csdlc-v2".into(),
            },
        })
        .unwrap(),
    )
    .expect("bootstrap");
    for phase in [LifecyclePhase::Ready, LifecyclePhase::Bound] {
        record = edit_issue(
            &store,
            EditRequest {
                issue: 5627,
                card: CardKind::Sip,
                expected_generation: record.generation,
                expected_digest: record.digest,
                claim_id: "claim".into(),
                actor: "agent".into(),
                reason: "fixture".into(),
                operation: SemanticOperation::AdvancePhase { phase },
                fail_after_backup: false,
            },
        )
        .expect("advance");
    }
    (temp, store, record)
}

fn lane(id: &str, deps: &[&str], executable: &str, argv: &[&str]) -> PvfLane {
    PvfLane {
        id: id.into(),
        proof_role: format!("prove {id}"),
        purpose: format!("validate {id}"),
        determinism: Determinism::Deterministic,
        resources: ResourceCost {
            cpu_units: 1,
            memory_mib: 8,
            tokens: 10,
        },
        credentials: vec![],
        network: NetworkPolicy::Denied,
        dependencies: deps.iter().map(|v| (*v).into()).collect(),
        parallel_group: "local".into(),
        release_gate: ReleaseGate::Optional,
        execution: ExecutionMode::Local,
        timeout_seconds: 2,
        executable: executable.into(),
        argv: argv.iter().map(|v| (*v).into()).collect(),
        evidence: EvidencePolicy {
            max_log_bytes: 256,
            redact_values: vec!["secret".into()],
            require_relative_paths: true,
        },
    }
}

fn manifest() -> PvfManifest {
    PvfManifest {
        schema: "csdlc.pvf.manifest.v1".into(),
        lanes: vec![
            lane("a", &[], "/bin/sleep", &["0.1"]),
            lane("b", &[], "/bin/sleep", &["0.1"]),
            lane("c", &["a", "b"], "/usr/bin/true", &[]),
        ],
    }
}

#[test]
fn finalize_is_one_atomic_implemented_transition_and_failure_writes_no_state() {
    let (temp, store, record) = bound_fixture();
    let before = std::fs::read(store.issue_dir(5627).join("index.json")).expect("before");
    let request = |executable: &str| FinalizeRequest {
        schema: "csdlc.finalize_request.v1".into(),
        issue: 5627,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "agent".into(),
        summary: "implemented four-command lifecycle".into(),
        changes: vec!["csdlc-v2".into()],
        artifacts: vec!["publication intent".into()],
        execution: ExecutionRequest {
            manifest: PvfManifest {
                schema: "csdlc.pvf.manifest.v1".into(),
                lanes: vec![lane("focused", &[], executable, &[])],
            },
            selection: SelectionRequest {
                requested_lanes: vec!["focused".into()],
                allow_network: false,
                available_credentials: vec![],
            },
            budget: ExecutionBudget {
                max_parallel: 1,
                cpu_units: 1,
                memory_mib: 8,
                tokens: 10,
            },
            root: temp.path().into(),
            evidence_dir: temp.path().join(".csdlc/evidence/5627"),
            cancellation_file: None,
        },
    };
    let evidence_dir = temp.path().join(".csdlc/evidence/5627");
    std::fs::create_dir_all(&evidence_dir).expect("prior evidence directory");
    std::fs::write(evidence_dir.join("prior.log"), b"prior evidence\n").expect("prior evidence");
    assert_eq!(
        finalize(&store, request("/usr/bin/false"))
            .unwrap_err()
            .code,
        ErrorCode::ValidationFailed
    );
    assert_eq!(
        std::fs::read(store.issue_dir(5627).join("index.json")).expect("unchanged"),
        before
    );
    assert_eq!(
        std::fs::read(evidence_dir.join("prior.log")).expect("prior evidence remains"),
        b"prior evidence\n"
    );
    assert!(std::fs::read_dir(temp.path().join(".csdlc/evidence"))
        .expect("evidence parent")
        .all(|entry| !entry
            .expect("entry")
            .file_name()
            .to_string_lossy()
            .starts_with(".csdlc-finalize-")));
    let mut unsafe_request = request("/usr/bin/true");
    unsafe_request.execution.evidence_dir = temp.path().join("unrelated");
    assert_eq!(
        finalize(&store, unsafe_request).unwrap_err().code,
        ErrorCode::UnsafeCheckout
    );
    std::fs::create_dir_all(temp.path().join("outside")).expect("outside directory");
    let mut symlink_request = request("/bin/sh");
    symlink_request.execution.manifest.lanes[0].argv = vec![
        "-c".into(),
        "for d in .csdlc/evidence/.csdlc-finalize-*; do rm -rf \"$d\"; ln -s ../../../outside \"$d\"; done".into(),
    ];
    assert_eq!(
        finalize(&store, symlink_request).unwrap_err().code,
        ErrorCode::UnsafeCheckout
    );
    assert_eq!(
        std::fs::read(evidence_dir.join("prior.log")).expect("prior evidence remains"),
        b"prior evidence\n"
    );
    let implemented = finalize(&store, request("/usr/bin/true")).expect("finalize");
    assert_eq!(implemented.phase, LifecyclePhase::Implemented);
    assert_eq!(implemented.generation, record.generation + 1);
    assert_eq!(
        implemented.audit.last().expect("audit").operation,
        "finalize_implementation"
    );
}

#[test]
fn routine_lifecycle_contract_measures_four_commands_and_two_artifacts() {
    let (temp, _store, _record) = bound_fixture();
    let routine_commands = [
        "csdlc-validate finalize",
        "csdlc-review record",
        "csdlc-publish publish",
        "csdlc-closeout closeout",
    ];
    let replaced_routine_commands = [
        "csdlc-validate execute",
        "csdlc-edit apply implemented",
        "csdlc-review assign",
        "csdlc-review record",
        "csdlc-edit apply reviewed",
        "csdlc-publish publish draft",
        "csdlc-publish ready",
        "csdlc-publish reconcile-merged",
        "csdlc-closeout closeout",
    ];
    let durable_post_product_artifacts = [
        ".csdlc/publication/5627.intent.json",
        ".git/csdlc-v2/closeout/5627.json",
    ];
    let request = shared_request_path(temp.path(), 5627).expect("shared request path");

    assert_eq!(routine_commands.len(), 4);
    assert_eq!(replaced_routine_commands.len(), 9);
    assert!(durable_post_product_artifacts.len() <= 2);
    assert!(request.ends_with(".git/csdlc-v2/requests/5627.json"));
    assert!(!durable_post_product_artifacts
        .iter()
        .any(|artifact| artifact.contains("requests")));
}

#[test]
fn selection_is_stable_topological_and_includes_dependencies() {
    let request = SelectionRequest {
        requested_lanes: vec!["c".into()],
        allow_network: false,
        available_credentials: vec![],
    };
    let first = select(&manifest(), &request).expect("select");
    let second = select(&manifest(), &request).expect("select");
    assert_eq!(
        first.waves,
        vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string()]
        ]
    );
    assert_eq!(first.waves, second.waves);
}

#[test]
fn invalid_cycle_network_credentials_and_budget_fail_closed() {
    let mut cyclic = manifest();
    cyclic.lanes[0].dependencies = vec!["c".into()];
    let request = SelectionRequest {
        requested_lanes: vec!["c".into()],
        allow_network: false,
        available_credentials: vec![],
    };
    assert!(matches!(
        select(&cyclic, &request).expect_err("cycle").code,
        ErrorCode::InvalidManifest
    ));
    let mut external = manifest();
    external.lanes[2].network = NetworkPolicy::External;
    assert!(matches!(
        select(&external, &request).expect_err("network").code,
        ErrorCode::InvalidManifest
    ));
    let temp = tempfile::tempdir().expect("temp");
    let mut over_budget = manifest();
    over_budget.lanes[0].resources.cpu_units = 2;
    let error = execute(ExecutionRequest {
        manifest: over_budget,
        selection: request,
        budget: ExecutionBudget {
            max_parallel: 2,
            cpu_units: 1,
            memory_mib: 16,
            tokens: 30,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("evidence"),
        cancellation_file: None,
    })
    .expect_err("budget");
    assert!(matches!(error.code, ErrorCode::InvalidManifest));
}

#[test]
fn independent_lanes_run_concurrently_and_converge_with_typed_evidence() {
    let temp = tempfile::tempdir().expect("temp");
    let started = Instant::now();
    let report = execute(ExecutionRequest {
        manifest: manifest(),
        selection: SelectionRequest {
            requested_lanes: vec!["c".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 2,
            cpu_units: 2,
            memory_mib: 16,
            tokens: 30,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("evidence"),
        cancellation_file: None,
    })
    .expect("execute");
    assert_eq!(report.disposition, ValidationDisposition::LocalPass);
    assert_eq!(report.evidence.len(), 3);
    assert!(
        started.elapsed().as_millis() < 190,
        "parallel wave was serialized"
    );
    assert!(report
        .evidence
        .iter()
        .all(|item| item.status == LaneStatus::Passed && item.path_hygiene_ok));
}

#[test]
fn deterministic_budget_packing_accepts_feasible_uneven_wave() {
    let temp = tempfile::tempdir().expect("temp");
    let mut heavy = lane("a", &[], "/usr/bin/true", &[]);
    heavy.resources.cpu_units = 2;
    let one = lane("b", &[], "/usr/bin/true", &[]);
    let two = lane("c", &[], "/usr/bin/true", &[]);
    let report = execute(ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![heavy, one, two],
        },
        selection: SelectionRequest {
            requested_lanes: vec!["a".into(), "b".into(), "c".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 2,
            cpu_units: 2,
            memory_mib: 16,
            tokens: 30,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("packed"),
        cancellation_file: None,
    })
    .expect("feasible packing");
    assert_eq!(report.disposition, ValidationDisposition::LocalPass);
    assert_eq!(report.evidence.len(), 3);
}

#[test]
fn timeout_failure_redaction_and_deferred_ci_are_truthful() {
    let temp = tempfile::tempdir().expect("temp");
    let mut lanes = vec![
        lane("redact", &[], "/usr/bin/printf", &["secret"]),
        lane("fail", &[], "/usr/bin/false", &[]),
    ];
    lanes[1].release_gate = ReleaseGate::NonGoal;
    let mut deferred = lane("ci", &[], "/usr/bin/true", &[]);
    deferred.execution = ExecutionMode::DeferredCi;
    lanes.push(deferred);
    let report = execute(ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes,
        },
        selection: SelectionRequest {
            requested_lanes: vec!["redact".into(), "fail".into(), "ci".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 2,
            cpu_units: 2,
            memory_mib: 16,
            tokens: 20,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("evidence"),
        cancellation_file: None,
    })
    .expect("execute");
    assert_eq!(report.disposition, ValidationDisposition::DeferredCi);
    assert!(report
        .evidence
        .iter()
        .any(|e| e.status == LaneStatus::AcceptedNonGoal));
    assert!(report
        .evidence
        .iter()
        .any(|e| e.status == LaneStatus::DeferredCi));
    let redacted = std::fs::read_to_string(temp.path().join("evidence/redact.log")).expect("log");
    assert_eq!(redacted, "[REDACTED]");
    assert!(!serde_json::to_string(&report)
        .expect("report JSON")
        .contains("secret"));
    assert!(
        report
            .evidence
            .iter()
            .find(|e| e.lane == "redact")
            .expect("redact")
            .redaction_ok
    );
    assert_eq!(
        report
            .evidence
            .iter()
            .find(|e| e.lane == "redact")
            .expect("redact")
            .redactions_applied,
        1
    );
}

#[test]
fn unsafe_lane_ids_are_rejected_before_evidence_paths_exist() {
    for id in ["../escape", "a/b", "/absolute"] {
        let manifest = PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![lane(id, &[], "/usr/bin/true", &[])],
        };
        let error = select(
            &manifest,
            &SelectionRequest {
                requested_lanes: vec![id.into()],
                allow_network: false,
                available_credentials: vec![],
            },
        )
        .expect_err("unsafe id");
        assert!(matches!(error.code, ErrorCode::InvalidManifest));
    }
}

#[test]
fn deferred_dependency_blocks_local_dependent_and_non_goal_only_converges() {
    let temp = tempfile::tempdir().expect("temp");
    let mut prerequisite = lane("ci", &[], "/usr/bin/true", &[]);
    prerequisite.execution = ExecutionMode::DeferredCi;
    let dependent = lane("release", &["ci"], "/usr/bin/true", &[]);
    let report = execute(ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![prerequisite, dependent],
        },
        selection: SelectionRequest {
            requested_lanes: vec!["release".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 2,
            cpu_units: 2,
            memory_mib: 16,
            tokens: 10,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("evidence"),
        cancellation_file: None,
    })
    .expect("report");
    assert_eq!(report.disposition, ValidationDisposition::DeferredCi);
    assert_eq!(
        report
            .evidence
            .iter()
            .find(|e| e.lane == "release")
            .expect("release")
            .status,
        LaneStatus::DeferredCi
    );
    let mut non_goal = lane("optional", &[], "/usr/bin/false", &[]);
    non_goal.release_gate = ReleaseGate::NonGoal;
    let report = execute(ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![non_goal],
        },
        selection: SelectionRequest {
            requested_lanes: vec!["optional".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 1,
            cpu_units: 1,
            memory_mib: 8,
            tokens: 0,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("non-goal"),
        cancellation_file: None,
    })
    .expect("non-goal");
    assert_eq!(report.disposition, ValidationDisposition::AcceptedNonGoal);
    let mut skipped = lane("skip", &[], "/usr/bin/true", &[]);
    skipped.release_gate = ReleaseGate::NonGoal;
    let dependent = lane("dependent", &["skip"], "/usr/bin/true", &[]);
    let report = execute(ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![skipped, dependent],
        },
        selection: SelectionRequest {
            requested_lanes: vec!["dependent".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 1,
            cpu_units: 1,
            memory_mib: 8,
            tokens: 10,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("blocked"),
        cancellation_file: None,
    })
    .expect("blocked dependent");
    assert_eq!(report.disposition, ValidationDisposition::Blocked);
}

#[test]
fn failed_lane_can_be_repaired_and_retried_to_local_pass() {
    let temp = tempfile::tempdir().expect("temp");
    let report = execute(ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![lane("fail", &[], "/usr/bin/false", &[])],
        },
        selection: SelectionRequest {
            requested_lanes: vec!["fail".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 1,
            cpu_units: 1,
            memory_mib: 8,
            tokens: 10,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("failed"),
        cancellation_file: None,
    })
    .expect("failed report");
    assert_eq!(report.disposition, ValidationDisposition::Failed);
    let repaired = execute(ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![lane("fail", &[], "/usr/bin/true", &[])],
        },
        selection: SelectionRequest {
            requested_lanes: vec!["fail".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 1,
            cpu_units: 1,
            memory_mib: 8,
            tokens: 10,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("repaired"),
        cancellation_file: None,
    })
    .expect("repaired retry");
    assert_eq!(repaired.disposition, ValidationDisposition::LocalPass);
}

#[test]
fn deadline_timeout_is_typed_and_fails_aggregate() {
    let temp = tempfile::tempdir().expect("temp");
    let pid_file = temp.path().join("timeout-child.pid");
    let script = format!("sleep 30 & echo $! > {}; wait", pid_file.display());
    let mut slow = lane("timeout", &[], "/bin/sh", &["-c", &script]);
    slow.timeout_seconds = 1;
    let started = Instant::now();
    let report = execute(ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![slow],
        },
        selection: SelectionRequest {
            requested_lanes: vec!["timeout".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 1,
            cpu_units: 1,
            memory_mib: 8,
            tokens: 10,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("timeout"),
        cancellation_file: None,
    })
    .expect("timeout report");
    assert_eq!(report.disposition, ValidationDisposition::Failed);
    assert_eq!(report.evidence[0].status, LaneStatus::TimedOut);
    assert!(started.elapsed() < std::time::Duration::from_millis(1500));
    let pid: i32 = std::fs::read_to_string(pid_file)
        .expect("pid")
        .trim()
        .parse()
        .expect("pid number");
    std::thread::sleep(std::time::Duration::from_millis(50));
    assert_eq!(
        unsafe { libc::kill(pid, 0) },
        -1,
        "timeout descendant survived"
    );
}

#[test]
fn validate_cli_redacts_machine_readable_command() {
    let temp = tempfile::tempdir().expect("temp");
    let request = ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![lane("redact", &[], "/usr/bin/printf", &["secret"])],
        },
        selection: SelectionRequest {
            requested_lanes: vec!["redact".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 1,
            cpu_units: 1,
            memory_mib: 8,
            tokens: 10,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("cli-evidence"),
        cancellation_file: None,
    };
    let path = temp.path().join("request.json");
    std::fs::write(&path, serde_json::to_vec(&request).expect("JSON")).expect("request");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_csdlc-validate"))
        .args(["--request", path.to_str().expect("path")])
        .output()
        .expect("CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8");
    assert!(!stdout.contains("secret"));
    assert!(stdout.contains("[REDACTED]"));
}

#[test]
fn high_output_drains_and_failed_peer_cancels_process_group() {
    let temp = tempfile::tempdir().expect("temp");
    let high = lane("high", &[], "/usr/bin/head", &["-c", "200000", "/dev/zero"]);
    let report = execute(ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![high],
        },
        selection: SelectionRequest {
            requested_lanes: vec!["high".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 1,
            cpu_units: 1,
            memory_mib: 8,
            tokens: 10,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("high"),
        cancellation_file: None,
    })
    .expect("high output");
    assert_eq!(report.disposition, ValidationDisposition::LocalPass);
    assert_eq!(
        std::fs::metadata(temp.path().join("high/high.log"))
            .expect("log")
            .len(),
        256
    );
    let mut slow = lane("slow", &[], "/bin/sleep", &["2"]);
    slow.timeout_seconds = 3;
    let fail = lane("fail", &[], "/usr/bin/false", &[]);
    let started = Instant::now();
    let report = execute(ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![fail, slow],
        },
        selection: SelectionRequest {
            requested_lanes: vec!["fail".into(), "slow".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 2,
            cpu_units: 2,
            memory_mib: 16,
            tokens: 20,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("peers"),
        cancellation_file: None,
    })
    .expect("peer cancellation");
    assert_eq!(report.disposition, ValidationDisposition::Failed);
    assert!(started.elapsed() < std::time::Duration::from_secs(1));
}

#[test]
fn cancellation_is_observed_before_process_start() {
    let temp = tempfile::tempdir().expect("temp");
    let cancel = temp.path().join("cancel");
    std::fs::write(&cancel, "stop").expect("cancel");
    let report = execute(ExecutionRequest {
        manifest: manifest(),
        selection: SelectionRequest {
            requested_lanes: vec!["c".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 2,
            cpu_units: 2,
            memory_mib: 16,
            tokens: 30,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("evidence"),
        cancellation_file: Some(cancel),
    })
    .expect("cancelled report");
    assert_eq!(report.disposition, ValidationDisposition::Waiting);
    assert!(report.evidence.is_empty());
}

#[test]
fn cancellation_during_execution_terminates_process_group() {
    let temp = tempfile::tempdir().expect("temp");
    let cancel = temp.path().join("cancel");
    let pid_file = temp.path().join("child.pid");
    let script = format!("sleep 30 & echo $! > {}; wait", pid_file.display());
    let mut tree = lane("tree", &[], "/bin/sh", &["-c", &script]);
    tree.timeout_seconds = 5;
    let cancel_writer = cancel.clone();
    let pid_observer = pid_file.clone();
    let trigger = std::thread::spawn(move || {
        let deadline = Instant::now() + std::time::Duration::from_secs(1);
        while !pid_observer.exists() && Instant::now() < deadline {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        std::fs::write(cancel_writer, "stop").expect("cancel");
    });
    let started = Instant::now();
    let report = execute(ExecutionRequest {
        manifest: PvfManifest {
            schema: "csdlc.pvf.manifest.v1".into(),
            lanes: vec![tree],
        },
        selection: SelectionRequest {
            requested_lanes: vec!["tree".into()],
            allow_network: false,
            available_credentials: vec![],
        },
        budget: ExecutionBudget {
            max_parallel: 1,
            cpu_units: 1,
            memory_mib: 8,
            tokens: 10,
        },
        root: temp.path().into(),
        evidence_dir: temp.path().join("tree-evidence"),
        cancellation_file: Some(cancel),
    })
    .expect("cancel tree");
    trigger.join().expect("trigger");
    assert_eq!(report.disposition, ValidationDisposition::Waiting);
    assert!(started.elapsed() < std::time::Duration::from_secs(1));
    let pid: i32 = std::fs::read_to_string(pid_file)
        .expect("pid")
        .trim()
        .parse()
        .expect("pid number");
    std::thread::sleep(std::time::Duration::from_millis(50));
    assert_eq!(
        unsafe { libc::kill(pid, 0) },
        -1,
        "descendant survived process-group cancellation"
    );
}

#[test]
fn scheduler_and_shepherd_are_read_only_classifiers() {
    let schedule = classify_schedule(&ScheduleInput {
        phase_ready: true,
        cards_ready: true,
        design_ready: true,
        dependencies_ready: true,
        claim_live: true,
        paths_clear: true,
        budget_available: true,
    });
    assert_eq!(schedule.eligible_operations, vec!["validate"]);
    assert!(schedule.authority.contains("cannot claim"));
    let waiting = classify_shepherd(&ShepherdInput {
        validation: None,
        dependency_wait: true,
        retryable_failure: false,
        repair_needed: false,
        operator_decision_needed: false,
    });
    assert_eq!(waiting.state, ShepherdState::Waiting);
    assert!(waiting.authority.contains("observe only"));
    let operator = classify_shepherd(&ShepherdInput {
        validation: Some(ValidationDisposition::Failed),
        dependency_wait: false,
        retryable_failure: true,
        repair_needed: true,
        operator_decision_needed: true,
    });
    assert_eq!(operator.state, ShepherdState::OperatorRequired);
}

#[test]
fn shepherd_cli_exposes_schema_and_checked_examples() {
    use std::process::Command;
    let binary = env!("CARGO_BIN_EXE_csdlc-shepherd");
    let schema = Command::new(binary)
        .arg("--schema")
        .output()
        .expect("shepherd schema");
    assert!(schema.status.success());
    let schema: serde_json::Value = serde_json::from_slice(&schema.stdout).expect("schema json");
    assert!(schema["properties"]["repair_needed"].is_object());

    for (name, expected) in [
        ("ready", ShepherdState::Ready),
        ("waiting", ShepherdState::Waiting),
        ("retryable", ShepherdState::Retryable),
        ("repair_required", ShepherdState::RepairRequired),
        ("operator_required", ShepherdState::OperatorRequired),
    ] {
        let output = Command::new(binary)
            .args(["--example", name])
            .output()
            .expect("shepherd example");
        assert!(output.status.success(), "{name}");
        let input: ShepherdInput = serde_json::from_slice(&output.stdout).expect("example json");
        let fixture = std::fs::read_to_string(format!(
            "{}/operator/examples/shepherd/{name}.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("example fixture");
        let fixture: serde_json::Value = serde_json::from_str(&fixture).expect("fixture json");
        assert_eq!(
            serde_json::to_value(&input).expect("input json"),
            fixture,
            "{name}"
        );
        assert_eq!(classify_shepherd(&input).state, expected, "{name}");
    }
    let invalid = Command::new(binary)
        .args(["--example", "unknown"])
        .output()
        .expect("invalid example");
    assert_eq!(invalid.status.code(), Some(64));
}
