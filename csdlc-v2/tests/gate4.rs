use std::time::Instant;

use csdlc_v2::pvf::*;
use csdlc_v2::{classify_schedule, classify_shepherd, execute, select, ErrorCode};

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
fn failed_lane_produces_failed_aggregate() {
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
    let trigger = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100));
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
