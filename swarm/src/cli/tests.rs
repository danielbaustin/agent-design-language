use super::commands::real_learn_export;
use super::demo_cmd::{is_ci_environment, real_demo};
use super::godel_cmd::{real_godel, real_godel_evaluate, real_godel_inspect, real_godel_run};
use super::open::{
    detect_platform, open_artifact, open_command_for, select_open_artifact, CommandRunner,
    OpenPlatform, RealCommandRunner,
};
use super::run::{enforce_signature_policy, now_ms};
use super::run_artifacts::{
    build_run_status, build_run_summary, build_scores_artifact, build_suggestions_artifact,
    classify_failure_kind, execution_plan_hash, load_resume_state, read_scores_if_present,
    resume_state_path_for_run_id, validate_pause_artifact_basic, write_run_state_artifacts,
    PauseStateArtifact, RunSummaryArtifact, RunSummaryCounts, RunSummaryLinks, RunSummaryPolicy,
    ScoresArtifact, ScoresGeneratedFrom, ScoresMetrics, ScoresSummary, StepStateArtifact,
    PAUSE_STATE_SCHEMA_VERSION,
};
use super::{real_instrument, real_keygen, real_learn, real_sign, real_verify, usage};
use ::adl::godel::experiment_record::{
    PersistedExperimentRecord, StageExperimentRecord, EXPERIMENT_RECORD_RUNTIME_SCHEMA,
};
use ::adl::godel::hypothesis::{PersistedHypothesisArtifact, HYPOTHESIS_ARTIFACT_VERSION};
use ::adl::godel::obsmem_index::{
    PersistedStageIndexEntry, StageIndexEntry, OBSMEM_INDEX_RUNTIME_SCHEMA,
};
use ::adl::godel::policy::{
    PersistedPolicyArtifact, PersistedPolicyComparisonArtifact, POLICY_ARTIFACT_VERSION,
    POLICY_COMPARISON_ARTIFACT_VERSION,
};
use ::adl::{adl, artifacts, execute, failure_taxonomy, instrumentation, resolve, signing, trace};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> MutexGuard<'static, ()> {
    match ENV_LOCK.get_or_init(|| Mutex::new(())).lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

struct EnvGuard {
    key: String,
    old: Option<OsString>,
    _lock: MutexGuard<'static, ()>,
}

impl EnvGuard {
    fn set(key: &str, value: &str) -> Self {
        let lock = env_lock();
        let old = std::env::var_os(key);
        unsafe {
            std::env::set_var(key, value);
        }
        Self {
            key: key.to_string(),
            old,
            _lock: lock,
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.old {
                Some(v) => std::env::set_var(&self.key, v),
                None => std::env::remove_var(&self.key),
            }
        }
    }
}

#[derive(Default)]
struct RecordingRunner {
    calls: std::sync::Mutex<Vec<(String, Vec<String>)>>,
    fail: bool,
}

impl CommandRunner for RecordingRunner {
    fn run(&self, program: &str, args: &[String]) -> anyhow::Result<()> {
        self.calls
            .lock()
            .expect("lock")
            .push((program.to_string(), args.to_vec()));
        if self.fail {
            Err(anyhow::anyhow!("runner failure"))
        } else {
            Ok(())
        }
    }
}

#[test]
fn select_open_artifact_prefers_first_html() {
    let artifacts = vec![
        PathBuf::from("out/one.txt"),
        PathBuf::from("out/two.html"),
        PathBuf::from("out/three.html"),
    ];
    let picked = select_open_artifact(&artifacts).unwrap();
    assert_eq!(picked, PathBuf::from("out/two.html"));
}

#[test]
fn open_command_selection_mac() {
    let (program, args) = open_command_for(OpenPlatform::Mac, Path::new("out/index.html"));
    assert_eq!(program, "open");
    assert_eq!(args, vec!["out/index.html".to_string()]);
}

#[test]
fn open_command_selection_linux() {
    let (program, args) = open_command_for(OpenPlatform::Linux, Path::new("out/index.html"));
    assert_eq!(program, "xdg-open");
    assert_eq!(args, vec!["out/index.html".to_string()]);
}

#[test]
fn open_command_selection_windows() {
    let (program, args) = open_command_for(OpenPlatform::Windows, Path::new("out/index.html"));
    assert_eq!(program, "cmd.exe");
    assert_eq!(
        args,
        vec![
            "/C".to_string(),
            "start".to_string(),
            "".to_string(),
            "out/index.html".to_string()
        ]
    );
}

#[test]
fn detect_platform_matches_current_target() {
    if cfg!(target_os = "macos") {
        assert_eq!(detect_platform(), OpenPlatform::Mac);
    } else if cfg!(target_os = "windows") {
        assert_eq!(detect_platform(), OpenPlatform::Windows);
    } else {
        assert_eq!(detect_platform(), OpenPlatform::Linux);
    }
}

#[test]
fn open_artifact_uses_runner_with_platform_command() {
    let runner = RecordingRunner::default();
    let path = Path::new("out/index.html");
    open_artifact(&runner, path).expect("open artifact");
    let calls = runner.calls.lock().expect("lock");
    assert_eq!(calls.len(), 1);
    let (program, args) = &calls[0];
    let (expected_program, expected_args) = open_command_for(detect_platform(), path);
    assert_eq!(program, &expected_program);
    assert_eq!(args, &expected_args);
}

#[test]
fn open_artifact_propagates_runner_failure() {
    let runner = RecordingRunner {
        fail: true,
        ..Default::default()
    };
    let err = open_artifact(&runner, Path::new("out/index.html")).expect_err("runner failure");
    assert!(err.to_string().contains("runner failure"));
}

#[cfg(not(target_os = "windows"))]
#[test]
fn real_command_runner_surfaces_success_and_failure_status() {
    let runner = RealCommandRunner;
    runner.run("true", &[]).expect("true should succeed");
    let err = runner.run("false", &[]).expect_err("false should fail");
    assert!(err.to_string().contains("open command 'false' failed"));
}

#[test]
fn is_ci_environment_treats_falsey_values_as_false() {
    {
        let _guard = EnvGuard::set("CI", "false");
        assert!(!is_ci_environment());
    }
    {
        let _guard = EnvGuard::set("CI", "0");
        assert!(!is_ci_environment());
    }
    {
        let _guard = EnvGuard::set("CI", "true");
        assert!(is_ci_environment());
    }
}

#[test]
fn usage_mentions_v0_4_and_legacy_examples() {
    let text = usage();
    assert!(text.contains("Usage:"));
    assert!(text.contains("adl resume <run_id>"));
    assert!(text.contains("adl godel run"));
    assert!(text.contains("adl godel inspect"));
    assert!(text.contains("adl godel evaluate"));
    assert!(text.contains("Examples:"));
    assert!(text.contains("examples/v0-4-demo-fork-join-swarm.adl.yaml"));
    assert!(text.contains("examples/adl-0.1.yaml"));
    assert!(text.contains("--allow-unsigned"));
}

#[test]
fn real_godel_validates_subcommand_and_run_args() {
    let err = real_godel(&[]).expect_err("missing subcommand");
    assert!(err
        .to_string()
        .contains("supported: run, evaluate, inspect"));

    let err = real_godel(&["unknown".to_string()]).expect_err("unknown subcommand");
    assert!(err.to_string().contains("unknown godel subcommand"));

    let err = real_godel_run(&[]).expect_err("missing run-id");
    assert!(err.to_string().contains("requires --run-id"));

    let err = real_godel_run(&[
        "--run-id".to_string(),
        "run-745-a".to_string(),
        "--workflow-id".to_string(),
        "wf-godel-loop".to_string(),
        "--failure-code".to_string(),
        "tool_failure".to_string(),
        "--failure-summary".to_string(),
        "deterministic parse error".to_string(),
        "--evidence-ref".to_string(),
        "../bad.json".to_string(),
    ])
    .expect_err("unsafe evidence ref should fail");
    assert!(err.to_string().contains("GODEL_STAGE_LOOP_INVALID_INPUT"));
}

#[test]
fn real_godel_inspect_validates_args_and_missing_paths() {
    let err = real_godel_inspect(&[]).expect_err("missing run-id");
    assert!(err.to_string().contains("requires --run-id"));

    let err =
        real_godel_inspect(&["--bogus".to_string(), "x".to_string()]).expect_err("unknown arg");
    assert!(err.to_string().contains("unknown godel inspect arg"));

    let missing_root =
        std::env::temp_dir().join(format!("adl-godel-inspect-missing-{}", std::process::id()));
    let err = real_godel_inspect(&[
        "--run-id".to_string(),
        "run-745-a".to_string(),
        "--runs-dir".to_string(),
        missing_root.to_string_lossy().to_string(),
    ])
    .expect_err("missing artifacts");
    assert!(err.to_string().contains("GODEL_INSPECT_IO"));
}

#[test]
fn real_godel_inspect_reads_persisted_runtime_artifacts() {
    let base = std::env::temp_dir().join(format!("adl-godel-inspect-ok-{}", std::process::id()));
    let run_dir = base.join("run-745-a").join("godel");
    std::fs::create_dir_all(&run_dir).expect("create godel dir");

    let record = PersistedExperimentRecord {
        schema: EXPERIMENT_RECORD_RUNTIME_SCHEMA.to_string(),
        record: StageExperimentRecord {
            run_id: "run-745-a".to_string(),
            workflow_id: "wf-godel-loop".to_string(),
            failure_code: "tool_failure".to_string(),
            hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
            mutation_id: "mut:run-745-a:tool_failure:00".to_string(),
            mutation_target_surface: "workflow-step-config".to_string(),
            evaluation_decision: "adopt".to_string(),
            evaluation_rationale: "deterministic rationale".to_string(),
            improvement_delta: 1,
            evidence_refs: vec!["runs/run-745-a/run_status.json".to_string()],
        },
    };
    let index = PersistedStageIndexEntry {
        schema: OBSMEM_INDEX_RUNTIME_SCHEMA.to_string(),
        entry: StageIndexEntry {
            index_key: "tool_failure:hyp:run-745-a:tool_failure:00:adopt".to_string(),
            run_id: "run-745-a".to_string(),
            workflow_id: "wf-godel-loop".to_string(),
            failure_code: "tool_failure".to_string(),
            hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
            mutation_id: "mut:run-745-a:tool_failure:00".to_string(),
            experiment_outcome: "adopt".to_string(),
        },
    };
    let hypothesis = PersistedHypothesisArtifact {
        artifact_version: HYPOTHESIS_ARTIFACT_VERSION.to_string(),
        hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
        run_id: "run-745-a".to_string(),
        workflow_id: "wf-godel-loop".to_string(),
        failure_id: "failure:run-745-a:tool_failure".to_string(),
        failure_class: "tool_failure".to_string(),
        claim: "Primary hypothesis: failure_code=tool_failure indicates a bounded execution weakness derived from 'deterministic parse failure'.".to_string(),
        confidence: 0.67,
        evidence_refs: vec!["runs/run-745-a/run_status.json".to_string()],
        related_run_refs: vec!["run-745-a".to_string()],
    };
    let policy = PersistedPolicyArtifact {
        artifact_version: POLICY_ARTIFACT_VERSION.to_string(),
        policy_id: "policy:run-745-a:tool_failure".to_string(),
        run_id: "run-745-a".to_string(),
        workflow_id: "wf-godel-loop".to_string(),
        hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
        hypothesis_artifact_path: "runs/run-745-a/godel/godel_hypothesis.v1.json".to_string(),
        source_signal: "hypothesis:tool_failure:godel_hypothesis.v1".to_string(),
        selection_reason: "Deterministic policy update derived from hypothesis_id=hyp:run-745-a:tool_failure:00 and failure_class=tool_failure.".to_string(),
        before_policy: ::adl::godel::policy::PolicyState {
            retry_budget: 1,
            experiment_budget: 1,
            target_surface: "tool-invocation-config".to_string(),
            policy_mode: "baseline".to_string(),
        },
        after_policy: ::adl::godel::policy::PolicyState {
            retry_budget: 2,
            experiment_budget: 2,
            target_surface: "tool-invocation-config".to_string(),
            policy_mode: "adaptive_reviewed".to_string(),
        },
    };
    let comparison = PersistedPolicyComparisonArtifact {
        artifact_version: POLICY_COMPARISON_ARTIFACT_VERSION.to_string(),
        comparison_id: "cmp:run-745-a:tool_failure".to_string(),
        run_id: "run-745-a".to_string(),
        workflow_id: "wf-godel-loop".to_string(),
        policy_id: "policy:run-745-a:tool_failure".to_string(),
        hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
        changed_fields: vec![
            "experiment_budget".to_string(),
            "policy_mode".to_string(),
            "retry_budget".to_string(),
        ],
        deterministic_mapping:
            "stable failure_class -> baseline policy -> bounded policy adjustment".to_string(),
        before_policy: policy.before_policy.clone(),
        after_policy: policy.after_policy.clone(),
    };

    std::fs::write(
        run_dir.join("experiment_record.runtime.v1.json"),
        serde_json::to_string_pretty(&record).expect("record json"),
    )
    .expect("write record");
    std::fs::write(
        run_dir.join("godel_hypothesis.v1.json"),
        serde_json::to_string_pretty(&hypothesis).expect("hypothesis json"),
    )
    .expect("write hypothesis");
    std::fs::write(
        run_dir.join("godel_policy.v1.json"),
        serde_json::to_string_pretty(&policy).expect("policy json"),
    )
    .expect("write policy");
    std::fs::write(
        run_dir.join("godel_policy_comparison.v1.json"),
        serde_json::to_string_pretty(&comparison).expect("comparison json"),
    )
    .expect("write comparison");
    std::fs::write(
        run_dir.join("obsmem_index_entry.runtime.v1.json"),
        serde_json::to_string_pretty(&index).expect("index json"),
    )
    .expect("write index");

    real_godel_inspect(&[
        "--run-id".to_string(),
        "run-745-a".to_string(),
        "--runs-dir".to_string(),
        base.to_string_lossy().to_string(),
    ])
    .expect("inspect should succeed");

    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn real_godel_evaluate_validates_args_and_returns_summary() {
    let err = real_godel_evaluate(&[]).expect_err("missing failure-code");
    assert!(err.to_string().contains("requires --failure-code"));

    let err = real_godel_evaluate(&[
        "--failure-code".to_string(),
        "tool_failure".to_string(),
        "--experiment-result".to_string(),
        "mystery".to_string(),
        "--score-delta".to_string(),
        "1".to_string(),
    ])
    .expect_err("invalid experiment result");
    assert!(err.to_string().contains("<ok|blocked>"));

    let err = real_godel_evaluate(&[
        "--failure-code".to_string(),
        "tool_failure".to_string(),
        "--experiment-result".to_string(),
        "ok".to_string(),
        "--score-delta".to_string(),
        "nope".to_string(),
    ])
    .expect_err("invalid score delta");
    assert!(err.to_string().contains("valid i32"));

    real_godel_evaluate(&[
        "--failure-code".to_string(),
        "tool_failure".to_string(),
        "--experiment-result".to_string(),
        "blocked".to_string(),
        "--score-delta".to_string(),
        "0".to_string(),
    ])
    .expect("evaluate summary");
}

#[test]
fn select_open_artifact_returns_none_without_html() {
    let artifacts = vec![PathBuf::from("out/one.txt"), PathBuf::from("out/two.md")];
    assert!(select_open_artifact(&artifacts).is_none());
}

#[test]
fn run_artifacts_root_points_to_repo_adl_runs() {
    let root = artifacts::runs_root().expect("run artifacts root");
    let s = root.to_string_lossy();
    assert!(s.ends_with(".adl/runs"), "unexpected path: {s}");
}

#[test]
fn enforce_signature_policy_skips_when_not_running_or_not_v0_5() {
    let mk_doc = |version: &str| adl::AdlDoc {
        version: version.to_string(),
        providers: HashMap::new(),
        tools: HashMap::new(),
        agents: HashMap::new(),
        tasks: HashMap::new(),
        workflows: HashMap::new(),
        patterns: vec![],
        signature: None,
        run: adl::RunSpec {
            id: None,
            name: None,
            created_at: None,
            defaults: adl::RunDefaults::default(),
            workflow_ref: None,
            workflow: Some(adl::WorkflowSpec {
                id: None,
                kind: adl::WorkflowKind::Sequential,
                max_concurrency: None,
                steps: vec![],
            }),
            pattern_ref: None,
            inputs: HashMap::new(),
            placement: None,
            remote: None,
            delegation_policy: None,
        },
    };

    enforce_signature_policy(&mk_doc("0.5"), false, false).expect("do_run=false should skip");
    enforce_signature_policy(&mk_doc("0.4"), true, false).expect("v0.4 should skip");
    enforce_signature_policy(&mk_doc("0.5"), true, true).expect("allow_unsigned should skip");
}

fn minimal_resolved_for_artifacts(run_id: String) -> resolve::AdlResolved {
    resolve::AdlResolved {
        run_id,
        workflow_id: "wf".to_string(),
        steps: vec![resolve::ResolvedStep {
            id: "s1".to_string(),
            agent: Some("a1".to_string()),
            provider: Some("p1".to_string()),
            placement: None,
            task: Some("t1".to_string()),
            call: None,
            with: HashMap::new(),
            as_ns: None,
            delegation: None,
            prompt: Some(adl::PromptSpec {
                user: Some("u".to_string()),
                ..Default::default()
            }),
            inputs: HashMap::new(),
            guards: vec![],
            save_as: Some("s1_out".to_string()),
            write_to: Some("out/s1.txt".to_string()),
            on_error: None,
            retry: None,
        }],
        execution_plan: ::adl::execution_plan::ExecutionPlan {
            workflow_kind: adl::WorkflowKind::Sequential,
            nodes: vec![::adl::execution_plan::ExecutionNode {
                step_id: "s1".to_string(),
                depends_on: vec![],
                save_as: Some("s1_out".to_string()),
                delegation: None,
            }],
        },
        doc: adl::AdlDoc {
            version: "0.5".to_string(),
            providers: HashMap::new(),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: vec![],
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: Some("run".to_string()),
                created_at: None,
                defaults: adl::RunDefaults::default(),
                workflow_ref: None,
                workflow: Some(adl::WorkflowSpec {
                    id: Some("wf".to_string()),
                    kind: adl::WorkflowKind::Sequential,
                    max_concurrency: None,
                    steps: vec![],
                }),
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        },
    }
}

#[test]
fn write_run_state_and_load_resume_round_trip() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-out-{now}"));

    let mut tr = trace::Trace::new(run_id.clone(), "wf".to_string(), "0.5".to_string());
    tr.step_started("s1", "a1", "p1", "t1", None);
    tr.step_finished("s1", true);

    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: Some("review".to_string()),
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec![],
        saved_state: HashMap::from([(String::from("k"), String::from("v"))]),
        completed_outputs: HashMap::from([(String::from("s1_out"), String::from("done"))]),
    };

    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        100,
        150,
        "paused",
        Some(&pause),
        &[],
        None,
        None,
    )
    .expect("write run artifacts");

    assert!(
        run_dir.join("outputs").is_dir(),
        "artifact model v1 requires outputs/ directory"
    );
    assert!(
        run_dir.join("logs").is_dir(),
        "artifact model v1 requires logs/ directory"
    );
    assert!(
        run_dir.join("learning/overlays").is_dir(),
        "artifact model v1 requires learning/overlays directory"
    );
    assert!(
        run_dir.join("meta/ARTIFACT_MODEL.json").is_file(),
        "artifact model v1 requires version marker"
    );

    let resume =
        load_resume_state(&run_dir.join("run.json"), &resolved).expect("load resume state");
    assert!(resume.completed_step_ids.contains("s1"));
    assert_eq!(resume.saved_state.get("k").map(String::as_str), Some("v"));
    assert!(
        run_dir.join("pause_state.json").exists(),
        "paused runs must persist pause_state.json"
    );
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn load_resume_state_rejects_non_paused_status() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-nonpaused-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-nonpaused-{now}"));

    let tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        0,
        1,
        "success",
        None,
        &[],
        None,
        None,
    )
    .expect("write non-paused artifacts");
    let err = load_resume_state(&run_dir.join("run.json"), &resolved)
        .expect_err("non-paused run.json should fail for resume");
    assert!(err.to_string().contains("status='paused'"));
    assert!(err.to_string().contains("run_id='"));
    assert!(
        !run_dir.join("pause_state.json").exists(),
        "non-paused runs must not emit pause_state.json"
    );
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn load_resume_state_rejects_unknown_schema_version() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-schema-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-schema-{now}"));

    let mut tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    tr.step_started("s1", "a1", "p1", "t1", None);
    tr.step_finished("s1", true);
    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: Some("review".to_string()),
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec![],
        saved_state: HashMap::new(),
        completed_outputs: HashMap::new(),
    };
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        10,
        20,
        "paused",
        Some(&pause),
        &[],
        None,
        None,
    )
    .expect("write run artifacts");

    let run_json_path = run_dir.join("run.json");
    let mut run_json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&run_json_path).expect("read run.json"))
            .expect("parse run.json");
    run_json["schema_version"] = serde_json::Value::String("run_state.v0".to_string());
    artifacts::atomic_write(
        &run_json_path,
        serde_json::to_vec_pretty(&run_json)
            .expect("serialize modified run.json")
            .as_slice(),
    )
    .expect("rewrite run.json");

    let err = load_resume_state(&run_json_path, &resolved)
        .expect_err("schema mismatch should be rejected");
    assert!(err.to_string().contains("schema_version mismatch"));
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn load_resume_state_rejects_missing_pause_payload() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-missing-pause-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-missing-pause-{now}"));

    let tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        0,
        1,
        "paused",
        None,
        &[],
        None,
        None,
    )
    .expect("write paused artifacts");

    let err = load_resume_state(&run_dir.join("run.json"), &resolved)
        .expect_err("paused run.json without pause payload should fail");
    assert!(err.to_string().contains("missing pause payload"));
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn load_resume_state_rejects_workflow_mismatch() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-wf-mismatch-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-wf-mismatch-{now}"));

    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: None,
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec![],
        saved_state: HashMap::new(),
        completed_outputs: HashMap::new(),
    };
    let tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        0,
        1,
        "paused",
        Some(&pause),
        &[],
        None,
        None,
    )
    .expect("write paused artifacts");

    let mut mismatch = resolved.clone();
    mismatch.workflow_id = "wf-other".to_string();
    let err = load_resume_state(&run_dir.join("run.json"), &mismatch)
        .expect_err("workflow mismatch must fail");
    assert!(err.to_string().contains("workflow_id mismatch"));
    assert!(err.to_string().contains("state='wf'"));
    assert!(err.to_string().contains("current='wf-other'"));
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn load_resume_state_rejects_version_mismatch() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-version-mismatch-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-version-mismatch-{now}"));

    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: None,
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec![],
        saved_state: HashMap::new(),
        completed_outputs: HashMap::new(),
    };
    let tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        0,
        1,
        "paused",
        Some(&pause),
        &[],
        None,
        None,
    )
    .expect("write paused artifacts");

    let mut mismatch = resolved.clone();
    mismatch.doc.version = "0.6".to_string();
    let err = load_resume_state(&run_dir.join("run.json"), &mismatch)
        .expect_err("version mismatch must fail");
    assert!(err.to_string().contains("version mismatch"));
    assert!(err.to_string().contains("state='0.5'"));
    assert!(err.to_string().contains("current='0.6'"));
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn load_resume_state_rejects_execution_plan_mismatch() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-plan-mismatch-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-plan-mismatch-{now}"));

    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: None,
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec![],
        saved_state: HashMap::new(),
        completed_outputs: HashMap::new(),
    };
    let tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        0,
        1,
        "paused",
        Some(&pause),
        &[],
        None,
        None,
    )
    .expect("write paused artifacts");

    let run_json = run_dir.join("run.json");
    let raw = std::fs::read_to_string(&run_json).expect("read run.json");
    let mut value: serde_json::Value = serde_json::from_str(&raw).expect("parse run.json");
    value["execution_plan_hash"] = serde_json::Value::String("tampered-hash".to_string());
    std::fs::write(
        &run_json,
        serde_json::to_vec_pretty(&value).expect("serialize tampered run.json"),
    )
    .expect("write tampered run.json");

    let err = load_resume_state(&run_json, &resolved).expect_err("plan mismatch must fail");
    assert!(err.to_string().contains("execution plan mismatch"));
    assert!(err.to_string().contains("state plan != current plan"));
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn classify_failure_kind_handles_sandbox_and_io_causes() {
    let sandbox_err = anyhow::Error::new(::adl::sandbox::SandboxPathError::PathDenied {
        requested_path: "sandbox:/bad".to_string(),
        reason: "parent_traversal",
    });
    assert_eq!(classify_failure_kind(&sandbox_err), Some("sandbox_denied"));

    let io_err = anyhow::Error::new(std::io::Error::other("disk issue"));
    assert_eq!(classify_failure_kind(&io_err), Some("io_error"));
}

#[test]
fn classify_failure_kind_covers_verification_and_replay_invariant_failures() {
    let unsigned_doc = adl::AdlDoc {
        version: "0.5".to_string(),
        providers: HashMap::new(),
        tools: HashMap::new(),
        agents: HashMap::new(),
        tasks: HashMap::new(),
        workflows: HashMap::new(),
        patterns: vec![],
        signature: None,
        run: adl::RunSpec {
            id: None,
            name: None,
            created_at: None,
            defaults: adl::RunDefaults::default(),
            workflow_ref: None,
            workflow: Some(adl::WorkflowSpec {
                id: None,
                kind: adl::WorkflowKind::Sequential,
                max_concurrency: None,
                steps: vec![],
            }),
            pattern_ref: None,
            inputs: HashMap::new(),
            placement: None,
            remote: None,
            delegation_policy: None,
        },
    };
    let verify_err = signing::verify_doc(&unsigned_doc, None).expect_err("unsigned verify");
    assert_eq!(
        classify_failure_kind(&verify_err),
        Some("verification_failed")
    );

    let bad_trace_path = std::env::temp_dir().join(format!(
        "adl-main-replay-kind-{}-{}.json",
        now_ms(),
        std::process::id()
    ));
    std::fs::write(&bad_trace_path, "{\"activation_log_version\":1,\"ordering\":\"bad\",\"stable_ids\":{\"step_id\":\"x\",\"delegation_id\":\"x\",\"run_id\":\"x\"},\"events\":[]}")
        .expect("write bad replay file");
    let replay_err =
        instrumentation::load_trace_artifact(&bad_trace_path).expect_err("ordering mismatch");
    assert_eq!(
        classify_failure_kind(&replay_err),
        Some("replay_invariant_violation")
    );
    let _ = std::fs::remove_file(&bad_trace_path);
}

#[test]
fn taxonomy_category_mapping_is_stable_for_core_codes() {
    assert_eq!(
        failure_taxonomy::category_for_code("policy_denied"),
        failure_taxonomy::POLICY_DENIED
    );
    assert_eq!(
        failure_taxonomy::category_for_code("verification_failed"),
        failure_taxonomy::VERIFICATION_FAILED
    );
    assert_eq!(
        failure_taxonomy::category_for_code("replay_invariant_violation"),
        failure_taxonomy::REPLAY_INVARIANT_VIOLATION
    );
    assert_eq!(
        failure_taxonomy::category_for_code("provider_error"),
        failure_taxonomy::TOOL_FAILURE
    );
}

#[test]
fn classify_failure_kind_returns_none_for_unclassified_errors() {
    let generic = anyhow::anyhow!("generic failure");
    assert_eq!(classify_failure_kind(&generic), None);
}

#[test]
fn execution_plan_hash_is_deterministic_for_same_plan() {
    let resolved = minimal_resolved_for_artifacts("hash-run".to_string());
    let a = execution_plan_hash(&resolved.execution_plan).expect("hash a");
    let b = execution_plan_hash(&resolved.execution_plan).expect("hash b");
    assert_eq!(a, b);
    assert_eq!(a.len(), 16, "fnv-1a hex length should be stable");
}

#[test]
fn build_run_summary_sorts_remote_policy_and_tracks_denials() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("summary-{now}-{}", std::process::id());
    let mut resolved = minimal_resolved_for_artifacts(run_id);
    resolved.steps.push(resolve::ResolvedStep {
        id: "s2".to_string(),
        agent: Some("a1".to_string()),
        provider: Some("p1".to_string()),
        placement: None,
        task: Some("t1".to_string()),
        call: None,
        with: HashMap::new(),
        as_ns: None,
        delegation: Some(adl::DelegationSpec {
            role: Some("reviewer".to_string()),
            requires_verification: Some(true),
            escalation_target: None,
            tags: vec!["b".to_string(), "a".to_string()],
        }),
        prompt: Some(adl::PromptSpec {
            user: Some("u".to_string()),
            ..Default::default()
        }),
        inputs: HashMap::new(),
        guards: vec![],
        save_as: Some("s2_out".to_string()),
        write_to: Some("out/s2.txt".to_string()),
        on_error: None,
        retry: None,
    });
    resolved.doc.run.remote = Some(adl::RunRemoteSpec {
        endpoint: "http://127.0.0.1:8787".to_string(),
        timeout_ms: Some(30_000),
        require_signed_requests: true,
        require_key_id: true,
        verify_allowed_algs: vec!["rsa".to_string(), "ed25519".to_string(), "rsa".to_string()],
        verify_allowed_key_sources: vec![
            "embedded".to_string(),
            "kms".to_string(),
            "embedded".to_string(),
        ],
    });

    let run_paths = artifacts::RunArtifactPaths::for_run(&resolved.run_id).expect("paths");
    run_paths.ensure_layout().expect("layout");
    run_paths.write_model_marker().expect("marker");
    artifacts::atomic_write(&run_paths.scores_json(), b"{}").expect("scores");
    artifacts::atomic_write(&run_paths.suggestions_json(), b"{}").expect("suggestions");

    let steps = vec![
        StepStateArtifact {
            step_id: "s1".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            status: "success".to_string(),
            output_artifact_path: Some("out/s1.txt".to_string()),
        },
        StepStateArtifact {
            step_id: "s2".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            status: "failure".to_string(),
            output_artifact_path: None,
        },
        StepStateArtifact {
            step_id: "s3".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            status: "not_run".to_string(),
            output_artifact_path: None,
        },
    ];
    let failure = anyhow::Error::new(::adl::sandbox::SandboxPathError::PathDenied {
        requested_path: "sandbox:/bad".to_string(),
        reason: "parent_traversal",
    });
    let summary = build_run_summary(
        &resolved,
        "failure",
        None,
        &steps,
        2,
        Some(&failure),
        &run_paths,
    );

    assert!(summary.policy.security_envelope_enabled);
    assert_eq!(summary.policy.verify_allowed_algs, vec!["ed25519", "rsa"]);
    assert_eq!(
        summary.policy.verify_allowed_key_sources,
        vec!["embedded", "kms"]
    );
    assert_eq!(
        summary
            .policy
            .security_denials_by_code
            .get("sandbox_denied"),
        Some(&1)
    );
    assert_eq!(summary.counts.total_steps, 2);
    assert_eq!(summary.counts.completed_steps, 2);
    assert_eq!(summary.counts.failed_steps, 1);
    assert_eq!(summary.counts.delegation_steps, 1);
    assert_eq!(summary.counts.delegation_requires_verification_steps, 1);
    assert_eq!(
        summary.links.scores_json.as_deref(),
        Some("learning/scores.json")
    );
    assert_eq!(
        summary.links.suggestions_json.as_deref(),
        Some("learning/suggestions.json")
    );

    let _ = std::fs::remove_dir_all(run_paths.run_dir());
}

#[test]
fn build_run_status_tracks_attempts_and_resume_completed_steps() {
    let resolved = minimal_resolved_for_artifacts("status-run".to_string());
    let mut tr = trace::Trace::new(
        "status-run".to_string(),
        "wf".to_string(),
        "0.5".to_string(),
    );
    tr.step_started("s1", "a1", "p1", "t1", None);
    tr.step_finished("s1", true);
    tr.step_started("s2", "a1", "p1", "t1", None);
    tr.step_started("s2", "a1", "p1", "t1", None);
    tr.step_finished("s2", false);

    let steps = vec![
        StepStateArtifact {
            step_id: "s1".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            status: "success".to_string(),
            output_artifact_path: Some("out/s1.txt".to_string()),
        },
        StepStateArtifact {
            step_id: "s2".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            status: "failure".to_string(),
            output_artifact_path: None,
        },
        StepStateArtifact {
            step_id: "s3".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            status: "not_run".to_string(),
            output_artifact_path: None,
        },
    ];
    let resume_completed = BTreeSet::from(["s0".to_string()]);
    let status = build_run_status(&resolved, &tr, "failed", &steps, None, &resume_completed);

    assert_eq!(
        status.completed_steps,
        vec!["s0".to_string(), "s1".to_string()]
    );
    assert_eq!(
        status.pending_steps,
        vec!["s2".to_string(), "s3".to_string()]
    );
    assert_eq!(status.failed_step_id.as_deref(), Some("s2"));
    assert_eq!(status.attempt_counts_by_step.get("s0"), Some(&0));
    assert_eq!(status.attempt_counts_by_step.get("s1"), Some(&1));
    assert_eq!(status.attempt_counts_by_step.get("s2"), Some(&2));
    assert_eq!(status.started_steps.as_ref().map(|v| v.len()), Some(2));
    assert!(
        status.effective_max_concurrency.is_none() || status.effective_max_concurrency == Some(4)
    );
}

#[test]
fn build_scores_and_suggestions_artifacts_are_deterministic() {
    let run_summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "run-demo".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.5".to_string(),
        swarm_version: env!("CARGO_PKG_VERSION").to_string(),
        status: "failure".to_string(),
        error_kind: Some("sandbox_denied".to_string()),
        counts: RunSummaryCounts {
            total_steps: 4,
            completed_steps: 3,
            failed_steps: 1,
            provider_call_count: 4,
            delegation_steps: 1,
            delegation_requires_verification_steps: 1,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled: true,
            signing_required: true,
            key_id_required: true,
            verify_allowed_algs: vec!["ed25519".to_string()],
            verify_allowed_key_sources: vec!["embedded".to_string()],
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code: BTreeMap::from([
                ("DELEGATION_DENIED".to_string(), 2usize),
                ("sandbox_denied".to_string(), 1usize),
            ]),
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: None,
            outputs_dir: "outputs".to_string(),
            logs_dir: "logs".to_string(),
            learning_dir: "learning".to_string(),
            scores_json: None,
            suggestions_json: None,
            overlays_dir: "learning/overlays".to_string(),
            cluster_groundwork_json: None,
            trace_json: None,
        },
    };
    let mut tr = trace::Trace::new("run-demo".to_string(), "wf".to_string(), "0.5".to_string());
    tr.step_started("a", "a1", "p1", "t1", None);
    tr.step_started("b", "a1", "p1", "t1", None);
    tr.step_finished("a", true);
    tr.step_started("b", "a1", "p1", "t1", None);
    tr.step_finished("b", false);

    let scores = build_scores_artifact(&run_summary, &tr);
    assert_eq!(scores.summary.failure_count, 1);
    assert_eq!(scores.summary.retry_count, 1);
    assert_eq!(scores.summary.delegation_denied_count, 2);
    assert_eq!(scores.summary.security_denied_count, 3);
    assert_eq!(scores.summary.success_ratio, 0.5);
    assert_eq!(scores.metrics.scheduler_max_parallel_observed, 2);

    let with_scores = build_suggestions_artifact(&run_summary, Some(&scores));
    let without_scores = build_suggestions_artifact(&run_summary, None);
    assert_eq!(with_scores.suggestions_version, 1);
    assert_eq!(
        with_scores.suggestions.first().map(|s| s.id.as_str()),
        Some("sug-001")
    );
    assert!(with_scores
        .suggestions
        .windows(2)
        .all(|pair| pair[0].id < pair[1].id));
    assert_eq!(with_scores.generated_from.scores_version, Some(1));
    assert_eq!(without_scores.generated_from.scores_version, None);
}

#[test]
fn read_scores_if_present_handles_valid_and_invalid_json() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("scores-read-{now}-{}", std::process::id());
    let run_paths = artifacts::RunArtifactPaths::for_run(&run_id).expect("paths");
    run_paths.ensure_layout().expect("layout");

    artifacts::atomic_write(&run_paths.scores_json(), b"{not-json").expect("write invalid");
    assert!(read_scores_if_present(&run_paths).is_none());

    let valid = serde_json::to_vec_pretty(&ScoresArtifact {
        scores_version: 1,
        run_id: run_id.clone(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 1.0,
            failure_count: 0,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    })
    .expect("serialize");
    artifacts::atomic_write(&run_paths.scores_json(), &valid).expect("write valid");
    let parsed = read_scores_if_present(&run_paths).expect("should parse valid score file");
    assert_eq!(parsed.run_id, run_id);

    let _ = std::fs::remove_dir_all(run_paths.run_dir());
}

#[test]
fn real_learn_validates_subcommand_and_export_args() {
    let err = real_learn(&[]).expect_err("missing subcommand");
    assert!(err.to_string().contains("supported: export"));

    let err = real_learn(&["unknown".to_string()]).expect_err("unknown subcommand");
    assert!(err.to_string().contains("unknown learn subcommand"));

    let err = real_learn_export(&[
        "--format".to_string(),
        "csv".to_string(),
        "--out".to_string(),
        "/tmp/out".to_string(),
    ])
    .expect_err("unsupported format");
    assert!(err.to_string().contains("unsupported learn export format"));

    let err =
        real_learn_export(&["--format".to_string(), "jsonl".to_string()]).expect_err("missing out");
    assert!(err.to_string().contains("requires --out"));

    let err =
        real_learn_export(&["--bogus".to_string(), "x".to_string()]).expect_err("unknown arg");
    assert!(err.to_string().contains("unknown learn export arg"));
}

#[test]
fn cli_internal_keygen_sign_verify_roundtrip_succeeds() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!("adl-main-keygen-{now}"));
    let key_dir = base.join("keys");
    std::fs::create_dir_all(&base).expect("create base dir");
    real_keygen(&[
        "--out-dir".to_string(),
        key_dir.to_string_lossy().to_string(),
    ])
    .expect("keygen should succeed");

    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/v0-5-pattern-linear.adl.yaml");
    let signed = base.join("signed.adl.yaml");
    real_sign(&[
        source.to_string_lossy().to_string(),
        "--key".to_string(),
        key_dir
            .join("ed25519-private.b64")
            .to_string_lossy()
            .to_string(),
        "--key-id".to_string(),
        "test-main".to_string(),
        "--out".to_string(),
        signed.to_string_lossy().to_string(),
    ])
    .expect("sign should succeed");

    real_verify(&[
        signed.to_string_lossy().to_string(),
        "--key".to_string(),
        key_dir
            .join("ed25519-public.b64")
            .to_string_lossy()
            .to_string(),
    ])
    .expect("verify should succeed");

    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn cli_internal_instrument_variants_succeed() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/v0-5-pattern-fork-join.adl.yaml");
    real_instrument(&[
        "graph".to_string(),
        fixture.to_string_lossy().to_string(),
        "--format".to_string(),
        "json".to_string(),
    ])
    .expect("graph json");
    real_instrument(&[
        "graph".to_string(),
        fixture.to_string_lossy().to_string(),
        "--format".to_string(),
        "dot".to_string(),
    ])
    .expect("graph dot");
    real_instrument(&[
        "diff-plan".to_string(),
        fixture.to_string_lossy().to_string(),
        fixture.to_string_lossy().to_string(),
    ])
    .expect("diff-plan");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!("adl-main-instrument-{now}"));
    std::fs::create_dir_all(&base).expect("create base dir");
    let left = base.join("left.trace.json");
    let right = base.join("right.trace.json");
    std::fs::write(&left, "[]").expect("write left trace");
    std::fs::write(&right, "[]").expect("write right trace");
    real_instrument(&["replay".to_string(), left.to_string_lossy().to_string()]).expect("replay");
    real_instrument(&[
        "diff-trace".to_string(),
        left.to_string_lossy().to_string(),
        right.to_string_lossy().to_string(),
    ])
    .expect("diff-trace");
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn cli_internal_learn_export_writes_jsonl() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!("adl-main-learn-{now}"));
    let runs_dir = base.join("runs");
    std::fs::create_dir_all(&runs_dir).expect("create runs dir");
    let out = base.join("learning.jsonl");
    real_learn_export(&[
        "--format".to_string(),
        "jsonl".to_string(),
        "--runs-dir".to_string(),
        runs_dir.to_string_lossy().to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
    ])
    .expect("learn export");
    assert!(out.exists(), "learn export should emit output file");
    let tool_result = base.join("learning.jsonl.tool_result.v1.json");
    assert!(
        tool_result.exists(),
        "learn export should emit tool_result sidecar"
    );
    let tool_result_json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&tool_result).expect("read tool_result"))
            .expect("parse tool_result");
    assert_eq!(
        tool_result_json
            .get("schema_version")
            .and_then(|v| v.as_str()),
        Some("tool_result.v1")
    );
    assert_eq!(
        tool_result_json.get("tool_name").and_then(|v| v.as_str()),
        Some("adl.learn.export")
    );
    assert_eq!(
        tool_result_json.get("status").and_then(|v| v.as_str()),
        Some("success")
    );
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn cli_internal_demo_print_plan_path_succeeds() {
    real_demo(&["demo-a-say-mcp".to_string(), "--print-plan".to_string()])
        .expect("known demo should succeed");
}

#[test]
fn cli_internal_demo_trace_only_path_succeeds() {
    real_demo(&["demo-a-say-mcp".to_string(), "--trace".to_string()])
        .expect("trace-only dry run should succeed");
}

#[test]
fn cli_internal_demo_run_no_open_path_succeeds() {
    real_demo(&[
        "demo-b-one-command".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
    ])
    .expect("demo run with explicit no-open should succeed");
}

#[test]
fn cli_internal_demo_help_path_succeeds() {
    real_demo(&["demo-a-say-mcp".to_string(), "--help".to_string()])
        .expect("help path should succeed");
}

#[test]
fn cli_internal_demo_defaults_to_run_when_no_mode_flag_is_given() {
    real_demo(&["demo-a-say-mcp".to_string(), "--no-open".to_string()])
        .expect("default demo invocation should run");
}

#[test]
fn cli_internal_demo_run_trace_and_out_path_succeeds() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let out_dir = std::env::temp_dir().join(format!("adl-demo-out-{now}"));
    real_demo(&[
        "demo-a-say-mcp".to_string(),
        "--run".to_string(),
        "--trace".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("demo run with trace and explicit out dir should succeed");
    assert!(out_dir.join("demo-a-say-mcp").exists());
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn validate_pause_artifact_basic_rejects_mismatches() {
    let mk = || PauseStateArtifact {
        schema_version: PAUSE_STATE_SCHEMA_VERSION.to_string(),
        run_id: "run-1".to_string(),
        workflow_id: "wf".to_string(),
        version: "0.5".to_string(),
        status: "paused".to_string(),
        adl_path: "swarm/examples/v0-6-hitl-pause-resume.adl.yaml".to_string(),
        execution_plan_hash: "abc".to_string(),
        steering_history: Vec::new(),
        pause: execute::PauseState {
            paused_step_id: "s1".to_string(),
            reason: None,
            completed_step_ids: vec!["s1".to_string()],
            remaining_step_ids: vec![],
            saved_state: HashMap::new(),
            completed_outputs: HashMap::new(),
        },
    };

    let mut wrong_schema = mk();
    wrong_schema.schema_version = "pause_state.v0".to_string();
    assert!(validate_pause_artifact_basic(&wrong_schema, "run-1").is_err());

    let mut wrong_status = mk();
    wrong_status.status = "success".to_string();
    assert!(validate_pause_artifact_basic(&wrong_status, "run-1").is_err());

    let mut wrong_run = mk();
    wrong_run.run_id = "run-2".to_string();
    assert!(validate_pause_artifact_basic(&wrong_run, "run-1").is_err());
}

#[test]
fn resume_state_path_for_run_id_targets_pause_state_json() {
    let path = resume_state_path_for_run_id("demo-run").expect("path");
    let s = path.to_string_lossy();
    assert!(
        s.ends_with(".adl/runs/demo-run/pause_state.json"),
        "path={s}"
    );
}
