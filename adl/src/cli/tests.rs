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
use super::run::{enforce_signature_policy, now_ms};
use super::run_artifacts::{
    build_aee_decision_artifact, build_run_status, build_run_summary, build_scores_artifact,
    build_suggestions_artifact, classify_failure_kind, execution_plan_hash, load_resume_state,
    read_scores_if_present, resume_state_path_for_run_id, validate_pause_artifact_basic,
    write_run_state_artifacts, PauseStateArtifact, RunSummaryArtifact, RunSummaryCounts,
    RunSummaryLinks, RunSummaryPolicy, ScoresArtifact, ScoresGeneratedFrom, ScoresMetrics,
    ScoresSummary, StepStateArtifact, AEE_DECISION_VERSION, PAUSE_STATE_SCHEMA_VERSION,
};
use super::{real_instrument, real_keygen, real_learn, real_sign, real_verify, usage};
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
