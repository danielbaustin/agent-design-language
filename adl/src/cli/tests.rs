use super::artifact_cmd::real_artifact;
use super::commands::real_learn_export;
use super::demo_cmd::{is_ci_environment, real_demo};
use super::godel_cmd::{
    real_godel, real_godel_affect_slice, real_godel_evaluate, real_godel_inspect, real_godel_run,
};
use super::open::{
    detect_platform, open_artifact, open_command_for, select_open_artifact, CommandRunner,
    OpenPlatform, RealCommandRunner,
};
use super::provider_cmd::real_provider;
use super::run::{enforce_signature_policy, now_ms};
use super::run_artifacts::{
    build_aee_decision_artifact, build_run_status, build_run_summary, build_scores_artifact,
    build_suggestions_artifact, classify_failure_kind, execution_plan_hash, load_resume_state,
    load_steering_patch, read_scores_if_present, resume_state_path_for_run_id,
    validate_pause_artifact_basic, validate_pause_artifact_for_resume, write_run_state_artifacts,
    PauseStateArtifact, RunSummaryArtifact, RunSummaryCounts, RunSummaryLinks, RunSummaryPolicy,
    ScoresArtifact, ScoresGeneratedFrom, ScoresMetrics, ScoresSummary, StepStateArtifact,
    AEE_DECISION_VERSION, PAUSE_STATE_SCHEMA_VERSION,
};
use super::{
    csdlc_issue_to_pr_args, csdlc_usage, dispatch_args, dispatch_csdlc_args, dispatch_review_args,
    dispatch_runtime_args, looks_like_adl_workflow_path, looks_like_issue_ref, real_instrument,
    real_keygen, real_learn, real_sign, real_verify, reject_csdlc_runtime_run,
    review_to_tooling_args, review_usage, runtime_usage, usage, version_text,
};
use ::adl::godel::cross_workflow::{
    DownstreamWorkflowDecision, PersistedCrossWorkflowArtifact, CROSS_WORKFLOW_ARTIFACT_VERSION,
};
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
use ::adl::godel::prioritization::{
    PersistedPrioritizationArtifact, PRIORITIZATION_ARTIFACT_VERSION,
};
use ::adl::godel::promotion::{
    PersistedEvalReportArtifact, PersistedPromotionDecisionArtifact, EVAL_REPORT_ARTIFACT_VERSION,
    PROMOTION_DECISION_ARTIFACT_VERSION,
};
use ::adl::{adl, artifacts, execute, instrumentation, resolve, signing, trace};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

pub(crate) fn env_lock() -> MutexGuard<'static, ()> {
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

fn unique_temp_dir(label: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("{label}-{now}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
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

mod artifact_builders;
mod godel;
mod internal_commands;
mod open_usage;
mod run_state;

#[test]
fn top_level_version_flag_is_handled_before_workflow_dispatch() {
    dispatch_args(&["--version".to_string()]).expect("version flag should succeed");
    dispatch_args(&["-V".to_string()]).expect("short version flag should succeed");
    assert_eq!(version_text(), env!("CARGO_PKG_VERSION"));
}

#[test]
fn top_level_dispatch_routes_process_and_public_help_paths() {
    dispatch_args(&["--help".to_string()]).expect("top-level help should succeed");

    dispatch_args(&["process".to_string(), "--help".to_string()])
        .expect("process help should route through dispatch");

    dispatch_args(&[
        "process".to_string(),
        "status".to_string(),
        "--pid".to_string(),
        std::process::id().to_string(),
        "--json".to_string(),
    ])
    .expect("process status should route through top-level dispatch");

    let ledger = unique_temp_dir("session-ledger-dispatch").join("ledger.json");
    dispatch_args(&[
        "session".to_string(),
        "status".to_string(),
        "--ledger".to_string(),
        ledger.display().to_string(),
        "--json".to_string(),
    ])
    .expect("session status should route through top-level dispatch");
}

#[test]
fn top_level_dispatch_routes_safe_help_branches_without_workflow_execution() {
    for command in [
        "provider",
        "runtime-v2",
        "keygen",
        "tooling",
        "resume",
        "process",
        "session",
    ] {
        dispatch_args(&[command.to_string(), "--help".to_string()])
            .expect("top-level help branch should route without workflow execution");
    }
}

#[test]
fn csdlc_dispatch_exposes_help_and_version_without_runtime_dispatch() {
    dispatch_csdlc_args(&["--help".to_string()]).expect("csdlc help should succeed");
    dispatch_csdlc_args(&["-h".to_string()]).expect("csdlc short help should succeed");
    dispatch_csdlc_args(&["help".to_string()]).expect("csdlc help alias should succeed");
    dispatch_csdlc_args(&["--version".to_string()]).expect("csdlc version should succeed");
    dispatch_csdlc_args(&["-V".to_string()]).expect("csdlc short version should succeed");

    let usage = csdlc_usage();
    assert!(usage.contains("adl-csdlc issue run <issue>"));
    assert!(usage.contains("adl/tools/pr.sh remains the canonical agent-facing issue wrapper"));
    assert!(usage.contains("shared pr control-plane client layer"));
    assert!(usage.contains("adl-runtime run <adl.yaml>"));
}

#[test]
fn runtime_dispatch_exposes_help_and_version_without_csdlc_dispatch() {
    dispatch_runtime_args(&["--help".to_string()]).expect("runtime help should succeed");
    dispatch_runtime_args(&["-h".to_string()]).expect("runtime short help should succeed");
    dispatch_runtime_args(&["help".to_string()]).expect("runtime help alias should succeed");
    dispatch_runtime_args(&["--version".to_string()]).expect("runtime version should succeed");
    dispatch_runtime_args(&["-V".to_string()]).expect("runtime short version should succeed");

    let usage = runtime_usage();
    assert!(usage.contains("adl-runtime run <adl.yaml>"));
    assert!(usage.contains("adl-runtime scheduler plan --input <bundle.json> [--out <path>] [--json]"));
    assert!(usage.contains("adl <adl.yaml> remains available as a compatibility shortcut"));
    assert!(usage.contains("C-SDLC issue work belongs to adl/tools/pr.sh run <issue>"));
}

#[test]
fn review_dispatch_exposes_help_and_version_without_runtime_or_csdlc_dispatch() {
    dispatch_review_args(&["--help".to_string()]).expect("review help should succeed");
    dispatch_review_args(&["-h".to_string()]).expect("review short help should succeed");
    dispatch_review_args(&["help".to_string()]).expect("review help alias should succeed");
    dispatch_review_args(&["--version".to_string()]).expect("review version should succeed");
    dispatch_review_args(&["-V".to_string()]).expect("review short version should succeed");

    let usage = review_usage();
    assert!(usage.contains("adl-review - ADL review tooling compatibility binary"));
    assert!(usage.contains("adl-review code-review --out <dir>"));
    assert!(usage.contains("adl tooling code-review"));
    assert!(usage.contains("adl-runtime run <adl.yaml>"));
}

#[test]
fn review_dispatch_maps_review_commands_and_rejects_other_families() {
    let mapped = review_to_tooling_args(
        "verify-repo-review-contract",
        &["--review".to_string(), "r.md".to_string()],
    )
    .expect("review args should map");
    assert_eq!(
        mapped,
        vec![
            "verify-repo-review-contract".to_string(),
            "--review".to_string(),
            "r.md".to_string()
        ]
    );

    let help_mapped = review_to_tooling_args("code-review", &["--help".to_string()])
        .expect("review help should map to common tooling help");
    assert_eq!(help_mapped, vec!["help".to_string()]);

    dispatch_review_args(&["code-review".to_string(), "--help".to_string()])
        .expect("code-review help should route through existing tooling help");
    dispatch_review_args(&["card-surface".to_string(), "--help".to_string()])
        .expect("card-surface help should route through existing tooling help");

    let missing = dispatch_review_args(&[]).expect_err("missing review command should fail");
    assert!(missing
        .to_string()
        .contains("adl-review requires a command"));

    let pr_err = dispatch_review_args(&["pr".to_string(), "run".to_string(), "3599".to_string()])
        .expect_err("review binary must not own issue work");
    assert!(pr_err.to_string().contains("review tooling only"));

    let runtime_err = dispatch_review_args(&["run".to_string(), "workflow.adl.yaml".to_string()])
        .expect_err("review binary must not own runtime workflows");
    assert!(runtime_err
        .to_string()
        .contains("does not run ADL runtime commands"));
}

#[test]
fn runtime_dispatch_rejects_csdlc_and_issue_run_inputs() {
    dispatch_runtime_args(&["run".to_string(), "--help".to_string()])
        .expect("runtime run help should succeed");

    let issue_err = dispatch_runtime_args(&["run".to_string(), "3598".to_string()])
        .expect_err("runtime run must reject numeric issue ids");
    assert!(issue_err
        .to_string()
        .contains("adl/tools/pr.sh run <issue>"));

    let hash_issue_err = dispatch_runtime_args(&["run".to_string(), "#3598".to_string()])
        .expect_err("runtime run must reject hash-prefixed issue ids");
    assert!(hash_issue_err
        .to_string()
        .contains("adl/tools/pr.sh run <issue>"));
    assert!(looks_like_issue_ref("3598"));
    assert!(looks_like_issue_ref("#3598"));
    assert!(!looks_like_issue_ref("workflow.adl.yaml"));
    assert!(!looks_like_issue_ref("#not-an-issue"));

    let pr_err = dispatch_runtime_args(&["pr".to_string(), "run".to_string(), "3598".to_string()])
        .expect_err("runtime must not own pr commands");
    assert!(pr_err
        .to_string()
        .contains("does not own C-SDLC workflow commands"));

    let tooling_err =
        dispatch_runtime_args(&["tooling".to_string(), "prompt-template".to_string()])
            .expect_err("runtime must not own tooling commands");
    assert!(tooling_err
        .to_string()
        .contains("does not own C-SDLC workflow commands"));
}

#[test]
fn csdlc_dispatch_routes_tooling_and_pr_errors_to_existing_surfaces() {
    dispatch_csdlc_args(&["tooling".to_string(), "help".to_string()])
        .expect("tooling help should route through existing tooling");
    let missing_command = dispatch_csdlc_args(&[]).expect_err("missing command should fail closed");
    assert!(missing_command
        .to_string()
        .contains("adl-csdlc requires a command"));
    let unknown_command =
        dispatch_csdlc_args(&["frobnicate".to_string()]).expect_err("unknown command should fail");
    assert!(unknown_command
        .to_string()
        .contains("unknown adl-csdlc command"));
    let pr_err = dispatch_csdlc_args(&["pr".to_string()])
        .expect_err("empty pr command should route to existing pr validation");
    assert!(pr_err.to_string().contains("pr requires a subcommand"));
    let issue_err = dispatch_csdlc_args(&["issue".to_string()])
        .expect_err("empty issue alias should fail before behavior changes");
    assert!(issue_err
        .to_string()
        .contains("adl-csdlc issue requires a pr-compatible subcommand"));
}

#[test]
fn csdlc_issue_run_rejects_runtime_yaml_and_non_numeric_operands() {
    let missing_issue_err = dispatch_csdlc_args(&["issue".to_string(), "run".to_string()])
        .expect_err("issue run should require an issue id");
    assert!(missing_issue_err
        .to_string()
        .contains("requires a numeric issue id"));

    let yaml_err = dispatch_csdlc_args(&[
        "issue".to_string(),
        "run".to_string(),
        "workflow.adl.yaml".to_string(),
    ])
    .expect_err("runtime YAML must not route through adl-csdlc issue run");
    assert!(yaml_err
        .to_string()
        .contains("Use adl-runtime run <adl.yaml>"));

    let non_numeric_err = dispatch_csdlc_args(&[
        "issue".to_string(),
        "run".to_string(),
        "not-an-issue".to_string(),
    ])
    .expect_err("issue run should require numeric issue ids");
    assert!(non_numeric_err
        .to_string()
        .contains("expects a numeric issue id"));

    assert!(looks_like_adl_workflow_path("workflow.adl.yaml"));
    assert!(looks_like_adl_workflow_path("workflow.adl.yml"));
    assert!(!looks_like_adl_workflow_path("3596"));
    reject_csdlc_runtime_run("adl-csdlc issue", &["run".to_string()])
        .expect("run without operand is left to downstream issue validation");
    reject_csdlc_runtime_run(
        "adl-csdlc issue",
        &["doctor".to_string(), "3596".to_string()],
    )
    .expect("non-run issue subcommands should not be rejected");
}

#[test]
fn csdlc_issue_run_maps_to_existing_pr_start_command() {
    let mapped = csdlc_issue_to_pr_args(&[
        "run".to_string(),
        "3596".to_string(),
        "--slug".to_string(),
        "example".to_string(),
    ])
    .expect("numeric issue run should map to existing pr start command");
    assert_eq!(
        mapped,
        vec![
            "start".to_string(),
            "3596".to_string(),
            "--slug".to_string(),
            "example".to_string()
        ]
    );

    let doctor = csdlc_issue_to_pr_args(&["doctor".to_string(), "3596".to_string()])
        .expect("non-run issue subcommands should preserve pr args");
    assert_eq!(doctor, vec!["doctor".to_string(), "3596".to_string()]);
}

#[test]
fn csdlc_github_client_boundary_doc_records_shared_ownership() {
    let doc = include_str!("../../../docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md");

    assert!(doc.contains("adl/tools/pr.sh"));
    assert!(doc.contains("remains the canonical"));
    assert!(doc.contains("adl-csdlc"));
    assert!(doc.contains("Rust-owned C-SDLC compatibility binary"));
    assert!(doc.contains("adl/src/cli/pr_cmd/github_client.rs"));
    assert!(doc.contains("adl/src/cli/pr_cmd/github.rs"));
    assert!(doc.contains("ADL_GITHUB_CLIENT"));
    assert!(doc.contains("ADL_GITHUB_DISABLE_GH_FALLBACK"));
    assert!(doc.contains("Do not duplicate GitHub issue or PR metadata interpretation"));
}

#[test]
fn csdlc_pr_and_top_level_run_reject_runtime_workflow_execution() {
    let pr_yaml_err = dispatch_csdlc_args(&[
        "pr".to_string(),
        "run".to_string(),
        "workflow.adl.yml".to_string(),
    ])
    .expect_err("runtime YAML must not route through adl-csdlc pr run");
    assert!(pr_yaml_err
        .to_string()
        .contains("cannot execute ADL workflow YAML"));

    let top_level_run_err =
        dispatch_csdlc_args(&["run".to_string(), "workflow.adl.yaml".to_string()])
            .expect_err("top-level csdlc run is ambiguous");
    assert!(top_level_run_err
        .to_string()
        .contains("does not run ADL workflow YAML"));
}

#[test]
fn provider_setup_dispatch_path_succeeds() {
    let _lock = env_lock();
    let temp = unique_temp_dir("provider-setup-dispatch");
    let prev_dir = std::env::current_dir().expect("cwd");
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("adl crate lives under repo root");
    std::env::set_current_dir(repo_root).expect("chdir repo root");
    let result = real_provider(&[
        "setup".to_string(),
        "chatgpt".to_string(),
        "--out".to_string(),
        temp.display().to_string(),
        "--force".to_string(),
    ]);
    std::env::set_current_dir(prev_dir).expect("restore cwd");
    result.expect("provider setup dispatch should succeed");
    assert!(temp.join("provider.adl.yaml").exists());
    assert!(temp.join("env.example").exists());
    assert!(temp.join("README.md").exists());
}
