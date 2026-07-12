//! Runtime-v2 governed CSM shutdown DAG.
//!
//! Captures the ordered shutdown path as retained runtime evidence: quiesce
//! admission, drain work, flush durable state, close lifecycle evidence, drain
//! observability, send publishable notices, join component tasks, and retain a
//! final disposition.

use std::path::Path;

use super::*;

pub const RUNTIME_V2_CSM_SHUTDOWN_DAG_SCHEMA: &str = "runtime_v2.csm_shutdown_dag.v1";
pub const RUNTIME_V2_CSM_SHUTDOWN_DISPOSITION_SCHEMA: &str =
    "runtime_v2.csm_shutdown_disposition.v1";
pub const RUNTIME_V2_CSM_SHUTDOWN_DAG_ARTIFACT_PATH: &str =
    "runtime_v2/shutdown/governed_shutdown_dag.json";
pub const RUNTIME_V2_CSM_SHUTDOWN_DISPOSITION_PATH: &str =
    "runtime_v2/shutdown/final_shutdown_disposition.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShutdownPhase {
    QuiesceAdmission,
    DrainWork,
    FlushDurableState,
    CloseLifelog,
    DrainObservability,
    FinalCloudNotices,
    JoinComponents,
    RetainDisposition,
}

impl ShutdownPhase {
    fn as_str(self) -> &'static str {
        match self {
            Self::QuiesceAdmission => "quiesce_admission",
            Self::DrainWork => "drain_work",
            Self::FlushDurableState => "flush_durable_state",
            Self::CloseLifelog => "close_lifelog",
            Self::DrainObservability => "drain_observability",
            Self::FinalCloudNotices => "final_cloud_notices",
            Self::JoinComponents => "join_components",
            Self::RetainDisposition => "retain_disposition",
        }
    }
}

const ORDERED_PHASES: [ShutdownPhase; 8] = [
    ShutdownPhase::QuiesceAdmission,
    ShutdownPhase::DrainWork,
    ShutdownPhase::FlushDurableState,
    ShutdownPhase::CloseLifelog,
    ShutdownPhase::DrainObservability,
    ShutdownPhase::FinalCloudNotices,
    ShutdownPhase::JoinComponents,
    ShutdownPhase::RetainDisposition,
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmShutdownStep {
    pub step_id: String,
    pub sequence: u32,
    pub phase: String,
    pub component: String,
    pub required_before: Vec<String>,
    pub action: String,
    pub expected_outcome: String,
    pub retained_evidence_ref: String,
    pub failure_classification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmShutdownDag {
    pub schema_version: String,
    pub dag_id: String,
    pub issue: u64,
    pub milestone: String,
    pub artifact_path: String,
    pub source_refs: Vec<String>,
    pub steps: Vec<RuntimeV2CsmShutdownStep>,
    pub forced_shutdown_policy: String,
    pub partial_classification_policy: String,
    pub cloud_notice_policy: String,
    pub final_disposition_ref: String,
    pub validation_commands: Vec<String>,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmShutdownComponentOutcome {
    pub component: String,
    pub phase: String,
    pub outcome: String,
    pub evidence_ref: String,
    pub recoverable_partial: bool,
    pub operator_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmShutdownCloudNotice {
    pub notice_id: String,
    pub publishable: bool,
    pub sent: bool,
    pub blocked_reason: Option<String>,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmShutdownDisposition {
    pub schema_version: String,
    pub disposition_id: String,
    pub dag_ref: String,
    pub artifact_path: String,
    pub shutdown_kind: String,
    pub forced_shutdown_explicit: bool,
    pub final_state: String,
    pub component_outcomes: Vec<RuntimeV2CsmShutdownComponentOutcome>,
    pub cloud_notices: Vec<RuntimeV2CsmShutdownCloudNotice>,
    pub retained_evidence_refs: Vec<String>,
    pub safe_fail_serialization_ref: String,
    pub observability_drain_ref: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmShutdownDagArtifacts {
    pub dag: RuntimeV2CsmShutdownDag,
    pub normal_disposition: RuntimeV2CsmShutdownDisposition,
    pub forced_disposition: RuntimeV2CsmShutdownDisposition,
    pub publish_blocked_disposition: RuntimeV2CsmShutdownDisposition,
}

impl RuntimeV2CsmShutdownDagArtifacts {
    pub fn prototype() -> Result<Self> {
        let dag = RuntimeV2CsmShutdownDag {
            schema_version: RUNTIME_V2_CSM_SHUTDOWN_DAG_SCHEMA.to_string(),
            dag_id: "proto-csm-01-governed-shutdown-dag-0001".to_string(),
            issue: 5114,
            milestone: "v0.91.7".to_string(),
            artifact_path: RUNTIME_V2_CSM_SHUTDOWN_DAG_ARTIFACT_PATH.to_string(),
            source_refs: vec![
                "docs/milestones/v0.91.7/review/runtime/csm_runtime_rearchitecture_5068.md"
                    .to_string(),
                "runtime_v2/csm_run/integrated_first_run_proof_packet.json".to_string(),
                "runtime_v2/reasoning_graph/reasoning_graph.json".to_string(),
                "runtime_v2/aee_obsmem_pvf_trace_handoff/aee_obsmem_pvf_trace_handoff.json"
                    .to_string(),
            ],
            steps: governed_shutdown_steps(),
            forced_shutdown_policy:
                "forced shutdown is allowed only as an explicit operator-visible mode after quiesce; unfinished reasoning_runtime or AEE work must be completed or serialized as recoverable partials"
                    .to_string(),
            partial_classification_policy:
                "scheduler, reasoning_runtime, and AEE work may exit only as completed or recoverable_partial with safe-fail serialization evidence"
                    .to_string(),
            cloud_notice_policy:
                "cloud_bridge notices are sent after observability drains and only when final disposition is publishable"
                    .to_string(),
            final_disposition_ref: RUNTIME_V2_CSM_SHUTDOWN_DISPOSITION_PATH.to_string(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_shutdown_dag -- --nocapture"
                    .to_string(),
                "cargo fmt --manifest-path adl/Cargo.toml --all -- --check".to_string(),
                "git diff --check".to_string(),
            ],
            non_claims: vec![
                "does not implement sibling #5112 supervision policy".to_string(),
                "does not claim integrated CSM soak completion; #5120 owns that proof".to_string(),
                "does not send live cloud notices".to_string(),
            ],
            claim_boundary:
                "This artifact proves the governed shutdown DAG ordering and retained disposition contract for WP-07 #5114; it does not implement sibling supervision policy, live cloud publication, or integrated soak completion."
                    .to_string(),
        };

        let normal_disposition = disposition(
            "proto-csm-01-normal-shutdown-0001",
            "operator_requested_graceful",
            false,
            "shutdown_complete_publishable",
            normal_component_outcomes(),
            vec![cloud_notice(
                "cloud-notice-normal-final-0001",
                true,
                true,
                None,
            )],
        );
        let forced_disposition = disposition(
            "proto-csm-01-forced-shutdown-0001",
            "operator_requested_forced",
            true,
            "shutdown_complete_with_recoverable_partials",
            forced_component_outcomes(),
            vec![cloud_notice(
                "cloud-notice-forced-final-0001",
                true,
                true,
                None,
            )],
        );
        let publish_blocked_disposition = disposition(
            "proto-csm-01-publish-blocked-shutdown-0001",
            "operator_requested_graceful",
            false,
            "shutdown_complete_notice_blocked",
            publish_blocked_component_outcomes(),
            vec![cloud_notice(
                "cloud-notice-publish-blocked-0001",
                false,
                false,
                Some("final_disposition_not_publishable"),
            )],
        );

        let artifacts = Self {
            dag,
            normal_disposition,
            forced_disposition,
            publish_blocked_disposition,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.dag.validate()?;
        self.normal_disposition.validate_against_dag(&self.dag)?;
        self.forced_disposition.validate_against_dag(&self.dag)?;
        self.publish_blocked_disposition
            .validate_against_dag(&self.dag)?;
        if self.normal_disposition.forced_shutdown_explicit {
            return Err(anyhow!("normal shutdown must not be classified as forced"));
        }
        if !self.forced_disposition.forced_shutdown_explicit {
            return Err(anyhow!("forced shutdown must be explicit and observable"));
        }
        if !self
            .forced_disposition
            .component_outcomes
            .iter()
            .any(|outcome| outcome.recoverable_partial)
        {
            return Err(anyhow!(
                "forced shutdown must classify unfinished work as recoverable partials"
            ));
        }
        if self
            .publish_blocked_disposition
            .cloud_notices
            .iter()
            .any(|notice| notice.sent)
        {
            return Err(anyhow!(
                "publish-blocked shutdown must not send cloud notices"
            ));
        }
        Ok(())
    }

    pub fn dag_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.dag.validate()?;
        serde_json::to_vec_pretty(&self.dag).context("serialize Runtime v2 CSM shutdown DAG")
    }

    pub fn normal_disposition_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.normal_disposition.validate_against_dag(&self.dag)?;
        serde_json::to_vec_pretty(&self.normal_disposition)
            .context("serialize Runtime v2 normal CSM shutdown disposition")
    }

    pub fn forced_disposition_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.forced_disposition.validate_against_dag(&self.dag)?;
        serde_json::to_vec_pretty(&self.forced_disposition)
            .context("serialize Runtime v2 forced CSM shutdown disposition")
    }

    pub fn publish_blocked_disposition_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.publish_blocked_disposition
            .validate_against_dag(&self.dag)?;
        serde_json::to_vec_pretty(&self.publish_blocked_disposition)
            .context("serialize Runtime v2 publish-blocked CSM shutdown disposition")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        write_relative(root, &self.dag.artifact_path, self.dag_pretty_json_bytes()?)?;
        write_relative(
            root,
            &self.normal_disposition.artifact_path,
            self.normal_disposition_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            "runtime_v2/shutdown/forced_shutdown_disposition.json",
            self.forced_disposition_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            "runtime_v2/shutdown/publish_blocked_shutdown_disposition.json",
            self.publish_blocked_disposition_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmShutdownDag {
    pub fn validate(&self) -> Result<()> {
        require_exact(
            &self.schema_version,
            RUNTIME_V2_CSM_SHUTDOWN_DAG_SCHEMA,
            "shutdown_dag.schema_version",
        )?;
        normalize_id(self.dag_id.clone(), "shutdown_dag.dag_id")?;
        if self.issue != 5114 {
            return Err(anyhow!("shutdown_dag.issue must stay bound to issue #5114"));
        }
        require_exact(&self.milestone, "v0.91.7", "shutdown_dag.milestone")?;
        validate_relative_path(&self.artifact_path, "shutdown_dag.artifact_path")?;
        validate_relative_path(
            &self.final_disposition_ref,
            "shutdown_dag.final_disposition_ref",
        )?;
        validate_requirement_list(&self.source_refs, "shutdown_dag.source_refs")?;
        for source_ref in &self.source_refs {
            validate_relative_path(source_ref, "shutdown_dag.source_refs")?;
        }
        validate_shutdown_steps(&self.steps)?;
        validate_nonempty_text(
            &self.forced_shutdown_policy,
            "shutdown_dag.forced_shutdown_policy",
        )?;
        validate_contains(
            &self.forced_shutdown_policy,
            "explicit operator-visible",
            "shutdown forced policy must require explicit observability",
        )?;
        validate_contains(
            &self.partial_classification_policy,
            "recoverable_partial",
            "shutdown partial classification policy must name recoverable partials",
        )?;
        validate_contains(
            &self.cloud_notice_policy,
            "only when final disposition is publishable",
            "shutdown cloud notice policy must fail closed when not publishable",
        )?;
        validate_requirement_list(
            &self.validation_commands,
            "shutdown_dag.validation_commands",
        )?;
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_csm_shutdown_dag"))
        {
            return Err(anyhow!(
                "shutdown DAG validation commands must include the focused test marker"
            ));
        }
        validate_requirement_list(&self.non_claims, "shutdown_dag.non_claims")?;
        validate_contains(
            &self.claim_boundary,
            "does not implement sibling supervision policy",
            "shutdown DAG claim boundary must avoid #5112 supervision policy scope",
        )
    }
}

impl RuntimeV2CsmShutdownDisposition {
    pub fn validate_against_dag(&self, dag: &RuntimeV2CsmShutdownDag) -> Result<()> {
        require_exact(
            &self.schema_version,
            RUNTIME_V2_CSM_SHUTDOWN_DISPOSITION_SCHEMA,
            "shutdown_disposition.schema_version",
        )?;
        normalize_id(
            self.disposition_id.clone(),
            "shutdown_disposition.disposition_id",
        )?;
        require_exact(
            &self.dag_ref,
            &dag.artifact_path,
            "shutdown_disposition.dag_ref",
        )?;
        validate_relative_path(&self.artifact_path, "shutdown_disposition.artifact_path")?;
        match self.shutdown_kind.as_str() {
            "operator_requested_graceful" | "operator_requested_forced" => {}
            other => return Err(anyhow!("unsupported shutdown kind '{other}'")),
        }
        if self.shutdown_kind == "operator_requested_forced" && !self.forced_shutdown_explicit {
            return Err(anyhow!("forced shutdown must be explicit and observable"));
        }
        validate_final_state(&self.final_state)?;
        validate_component_outcomes(&self.component_outcomes)?;
        validate_cloud_notices(&self.cloud_notices, &self.final_state)?;
        validate_relative_path(
            &self.safe_fail_serialization_ref,
            "shutdown_disposition.safe_fail_serialization_ref",
        )?;
        validate_relative_path(
            &self.observability_drain_ref,
            "shutdown_disposition.observability_drain_ref",
        )?;
        if self.retained_evidence_refs.is_empty() {
            return Err(anyhow!(
                "shutdown disposition must retain ordered component evidence"
            ));
        }
        for evidence_ref in &self.retained_evidence_refs {
            validate_relative_path(evidence_ref, "shutdown_disposition.retained_evidence_refs")?;
        }
        if !self
            .retained_evidence_refs
            .iter()
            .any(|reference| reference == &self.safe_fail_serialization_ref)
        {
            return Err(anyhow!(
                "shutdown disposition must retain safe-fail serialization evidence"
            ));
        }
        if !self
            .retained_evidence_refs
            .iter()
            .any(|reference| reference == &self.observability_drain_ref)
        {
            return Err(anyhow!(
                "shutdown disposition must retain observability drain evidence"
            ));
        }
        validate_contains(
            &self.claim_boundary,
            "does not send live cloud notices",
            "shutdown disposition claim boundary must preserve live-cloud non-claim",
        )
    }
}

pub fn runtime_v2_csm_shutdown_dag_contract() -> Result<RuntimeV2CsmShutdownDagArtifacts> {
    RuntimeV2CsmShutdownDagArtifacts::prototype()
}

fn governed_shutdown_steps() -> Vec<RuntimeV2CsmShutdownStep> {
    vec![
        step(
            "shutdown-quiesce-runtime-api",
            1,
            ShutdownPhase::QuiesceAdmission,
            "runtime_api",
            &[],
            "reject_new_mutating_admission_and_record_shutdown_intent",
            "mutating_admission_quiesced",
            "runtime_v2/shutdown/runtime_api_quiesce.json",
            "blocking_if_unavailable",
        ),
        step(
            "shutdown-quiesce-scheduler-intake",
            2,
            ShutdownPhase::QuiesceAdmission,
            "scheduler",
            &["shutdown-quiesce-runtime-api"],
            "close_scheduler_intake_queue",
            "scheduler_intake_quiesced",
            "runtime_v2/shutdown/scheduler_intake_quiesce.json",
            "blocking_if_unavailable",
        ),
        step(
            "shutdown-drain-scheduler",
            3,
            ShutdownPhase::DrainWork,
            "scheduler",
            &["shutdown-quiesce-scheduler-intake"],
            "drain_ready_and_running_work",
            "scheduler_work_drained_or_recoverable",
            "runtime_v2/shutdown/scheduler_drain.json",
            "recoverable_partial_if_incomplete",
        ),
        step(
            "shutdown-drain-reasoning-runtime",
            4,
            ShutdownPhase::DrainWork,
            "reasoning_runtime",
            &["shutdown-drain-scheduler"],
            "complete_in_flight_reasoning_or_serialize_recoverable_partial",
            "reasoning_runtime_completed_or_recoverable_partial",
            "runtime_v2/shutdown/reasoning_runtime_drain.json",
            "recoverable_partial_if_incomplete",
        ),
        step(
            "shutdown-drain-aee",
            5,
            ShutdownPhase::DrainWork,
            "aee",
            &["shutdown-drain-reasoning-runtime"],
            "complete_aee_memory_work_or_serialize_recoverable_partial",
            "aee_completed_or_recoverable_partial",
            "runtime_v2/shutdown/aee_drain.json",
            "recoverable_partial_if_incomplete",
        ),
        step(
            "shutdown-flush-checkpoint",
            6,
            ShutdownPhase::FlushDurableState,
            "checkpoint",
            &["shutdown-drain-aee"],
            "flush_continuity_checkpoint",
            "checkpoint_serialized_before_finalization",
            "runtime_v2/shutdown/continuity_checkpoint_flush.json",
            "safe_fail_serialization_if_failed",
        ),
        step(
            "shutdown-flush-safe-fail",
            7,
            ShutdownPhase::FlushDurableState,
            "safe_fail_serialization",
            &["shutdown-flush-checkpoint"],
            "serialize_recoverable_partials_and_failure_context",
            "safe_fail_bundle_retained",
            "runtime_v2/shutdown/safe_fail_bundle.json",
            "blocking_if_unavailable",
        ),
        step(
            "shutdown-close-lifelog",
            8,
            ShutdownPhase::CloseLifelog,
            "lifelog",
            &["shutdown-flush-safe-fail"],
            "append_lifecycle_close_event",
            "lifelog_closed_after_checkpoint",
            "runtime_v2/shutdown/lifelog_close.json",
            "safe_fail_serialization_if_failed",
        ),
        step(
            "shutdown-drain-observability",
            9,
            ShutdownPhase::DrainObservability,
            "observability",
            &["shutdown-close-lifelog"],
            "drain_shutdown_events_and_retained_disposition_probe",
            "observability_drained_after_lifelog",
            "runtime_v2/shutdown/observability_drain.json",
            "blocking_if_unavailable",
        ),
        step(
            "shutdown-cloud-notices",
            10,
            ShutdownPhase::FinalCloudNotices,
            "cloud_bridge",
            &["shutdown-drain-observability"],
            "send_final_notice_only_when_disposition_publishable",
            "cloud_notice_sent_or_blocked_without_false_progress",
            "runtime_v2/shutdown/cloud_notice_decision.json",
            "publish_blocked_if_not_publishable",
        ),
        step(
            "shutdown-join-component-tasks",
            11,
            ShutdownPhase::JoinComponents,
            "component_tasks",
            &["shutdown-cloud-notices"],
            "join_component_tasks_after_evidence_finalization",
            "component_tasks_joined",
            "runtime_v2/shutdown/component_task_joins.json",
            "recoverable_partial_if_incomplete",
        ),
        step(
            "shutdown-retain-final-disposition",
            12,
            ShutdownPhase::RetainDisposition,
            "shutdown_disposition",
            &["shutdown-join-component-tasks"],
            "retain_ordered_component_outcomes",
            "final_disposition_retained",
            RUNTIME_V2_CSM_SHUTDOWN_DISPOSITION_PATH,
            "blocking_if_unavailable",
        ),
    ]
}

fn normal_component_outcomes() -> Vec<RuntimeV2CsmShutdownComponentOutcome> {
    vec![
        outcome("runtime_api", "quiesce_admission", "quiesced", false),
        outcome("scheduler", "drain_work", "completed", false),
        outcome("reasoning_runtime", "drain_work", "completed", false),
        outcome("aee", "drain_work", "completed", false),
        outcome("checkpoint", "flush_durable_state", "flushed", false),
        outcome(
            "safe_fail_serialization",
            "flush_durable_state",
            "retained_empty_bundle",
            false,
        ),
        outcome("lifelog", "close_lifelog", "closed", false),
        outcome("observability", "drain_observability", "drained", false),
        outcome("cloud_bridge", "final_cloud_notices", "notice_sent", false),
        outcome("component_tasks", "join_components", "joined", false),
    ]
}

fn forced_component_outcomes() -> Vec<RuntimeV2CsmShutdownComponentOutcome> {
    vec![
        outcome("runtime_api", "quiesce_admission", "quiesced", false),
        outcome("scheduler", "drain_work", "recoverable_partial", true),
        outcome(
            "reasoning_runtime",
            "drain_work",
            "recoverable_partial",
            true,
        ),
        outcome("aee", "drain_work", "recoverable_partial", true),
        outcome("checkpoint", "flush_durable_state", "flushed", false),
        outcome(
            "safe_fail_serialization",
            "flush_durable_state",
            "serialized_partials",
            false,
        ),
        outcome("lifelog", "close_lifelog", "closed_forced", false),
        outcome("observability", "drain_observability", "drained", false),
        outcome("cloud_bridge", "final_cloud_notices", "notice_sent", false),
        outcome("component_tasks", "join_components", "joined", false),
    ]
}

fn publish_blocked_component_outcomes() -> Vec<RuntimeV2CsmShutdownComponentOutcome> {
    let mut outcomes = normal_component_outcomes();
    for outcome in &mut outcomes {
        if outcome.component == "cloud_bridge" {
            outcome.outcome = "notice_blocked_not_publishable".to_string();
        }
    }
    outcomes
}

fn disposition(
    disposition_id: &str,
    shutdown_kind: &str,
    forced_shutdown_explicit: bool,
    final_state: &str,
    component_outcomes: Vec<RuntimeV2CsmShutdownComponentOutcome>,
    cloud_notices: Vec<RuntimeV2CsmShutdownCloudNotice>,
) -> RuntimeV2CsmShutdownDisposition {
    RuntimeV2CsmShutdownDisposition {
        schema_version: RUNTIME_V2_CSM_SHUTDOWN_DISPOSITION_SCHEMA.to_string(),
        disposition_id: disposition_id.to_string(),
        dag_ref: RUNTIME_V2_CSM_SHUTDOWN_DAG_ARTIFACT_PATH.to_string(),
        artifact_path: RUNTIME_V2_CSM_SHUTDOWN_DISPOSITION_PATH.to_string(),
        shutdown_kind: shutdown_kind.to_string(),
        forced_shutdown_explicit,
        final_state: final_state.to_string(),
        component_outcomes,
        cloud_notices,
        retained_evidence_refs: vec![
            "runtime_v2/shutdown/runtime_api_quiesce.json".to_string(),
            "runtime_v2/shutdown/scheduler_drain.json".to_string(),
            "runtime_v2/shutdown/reasoning_runtime_drain.json".to_string(),
            "runtime_v2/shutdown/aee_drain.json".to_string(),
            "runtime_v2/shutdown/continuity_checkpoint_flush.json".to_string(),
            "runtime_v2/shutdown/safe_fail_bundle.json".to_string(),
            "runtime_v2/shutdown/lifelog_close.json".to_string(),
            "runtime_v2/shutdown/observability_drain.json".to_string(),
            "runtime_v2/shutdown/cloud_notice_decision.json".to_string(),
            "runtime_v2/shutdown/component_task_joins.json".to_string(),
        ],
        safe_fail_serialization_ref: "runtime_v2/shutdown/safe_fail_bundle.json".to_string(),
        observability_drain_ref: "runtime_v2/shutdown/observability_drain.json".to_string(),
        claim_boundary:
            "This retained shutdown disposition records governed shutdown evidence and does not send live cloud notices, implement sibling supervision policy, or claim integrated soak completion."
                .to_string(),
    }
}

#[allow(clippy::too_many_arguments)]
fn step(
    step_id: &str,
    sequence: u32,
    phase: ShutdownPhase,
    component: &str,
    required_before: &[&str],
    action: &str,
    expected_outcome: &str,
    retained_evidence_ref: &str,
    failure_classification: &str,
) -> RuntimeV2CsmShutdownStep {
    RuntimeV2CsmShutdownStep {
        step_id: step_id.to_string(),
        sequence,
        phase: phase.as_str().to_string(),
        component: component.to_string(),
        required_before: required_before
            .iter()
            .map(|value| value.to_string())
            .collect(),
        action: action.to_string(),
        expected_outcome: expected_outcome.to_string(),
        retained_evidence_ref: retained_evidence_ref.to_string(),
        failure_classification: failure_classification.to_string(),
    }
}

fn outcome(
    component: &str,
    phase: &str,
    outcome: &str,
    recoverable_partial: bool,
) -> RuntimeV2CsmShutdownComponentOutcome {
    RuntimeV2CsmShutdownComponentOutcome {
        component: component.to_string(),
        phase: phase.to_string(),
        outcome: outcome.to_string(),
        evidence_ref: format!("runtime_v2/shutdown/{component}_{phase}.json"),
        recoverable_partial,
        operator_visible: true,
    }
}

fn cloud_notice(
    notice_id: &str,
    publishable: bool,
    sent: bool,
    blocked_reason: Option<&str>,
) -> RuntimeV2CsmShutdownCloudNotice {
    RuntimeV2CsmShutdownCloudNotice {
        notice_id: notice_id.to_string(),
        publishable,
        sent,
        blocked_reason: blocked_reason.map(str::to_string),
        evidence_ref: "runtime_v2/shutdown/cloud_notice_decision.json".to_string(),
    }
}

fn validate_shutdown_steps(steps: &[RuntimeV2CsmShutdownStep]) -> Result<()> {
    if steps.len() != 12 {
        return Err(anyhow!(
            "shutdown DAG must contain the governed 12-step path"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut last_sequence = 0;
    let mut last_phase_index = 0;
    for step in steps {
        normalize_id(step.step_id.clone(), "shutdown_step.step_id")?;
        if !seen.insert(step.step_id.clone()) {
            return Err(anyhow!("shutdown DAG contains duplicate step id"));
        }
        if step.sequence <= last_sequence {
            return Err(anyhow!("shutdown DAG steps must preserve sequence order"));
        }
        last_sequence = step.sequence;
        let phase_index = ORDERED_PHASES
            .iter()
            .position(|phase| phase.as_str() == step.phase)
            .ok_or_else(|| anyhow!("unsupported shutdown phase '{}'", step.phase))?;
        if phase_index < last_phase_index {
            return Err(anyhow!(
                "shutdown DAG phases must not finalize before prior drains and flushes"
            ));
        }
        last_phase_index = phase_index;
        validate_nonempty_text(&step.component, "shutdown_step.component")?;
        validate_nonempty_text(&step.action, "shutdown_step.action")?;
        validate_nonempty_text(&step.expected_outcome, "shutdown_step.expected_outcome")?;
        validate_relative_path(
            &step.retained_evidence_ref,
            "shutdown_step.retained_evidence_ref",
        )?;
        validate_failure_classification(&step.failure_classification)?;
        for predecessor in &step.required_before {
            if !seen.contains(predecessor) {
                return Err(anyhow!(
                    "shutdown step '{}' names missing or later predecessor '{}'",
                    step.step_id,
                    predecessor
                ));
            }
        }
    }
    require_phase_before_component(steps, "checkpoint", "observability")?;
    require_phase_before_component(steps, "checkpoint", "cloud_bridge")?;
    require_phase_before_component(steps, "lifelog", "observability")?;
    require_phase_before_component(steps, "observability", "cloud_bridge")?;
    require_phase_before_component(steps, "cloud_bridge", "component_tasks")?;
    Ok(())
}

fn validate_component_outcomes(outcomes: &[RuntimeV2CsmShutdownComponentOutcome]) -> Result<()> {
    let required = [
        "runtime_api",
        "scheduler",
        "reasoning_runtime",
        "aee",
        "checkpoint",
        "safe_fail_serialization",
        "lifelog",
        "observability",
        "cloud_bridge",
        "component_tasks",
    ];
    if outcomes.len() != required.len() {
        return Err(anyhow!(
            "shutdown disposition must record every ordered component outcome"
        ));
    }
    for (expected, outcome) in required.iter().zip(outcomes) {
        require_exact(
            &outcome.component,
            expected,
            "shutdown_disposition.component_outcome.component",
        )?;
        validate_relative_path(
            &outcome.evidence_ref,
            "shutdown_disposition.component_outcome.evidence_ref",
        )?;
        validate_nonempty_text(
            &outcome.phase,
            "shutdown_disposition.component_outcome.phase",
        )?;
        validate_nonempty_text(
            &outcome.outcome,
            "shutdown_disposition.component_outcome.outcome",
        )?;
        if !outcome.operator_visible {
            return Err(anyhow!(
                "shutdown component outcomes must be operator-visible"
            ));
        }
    }
    for component in ["scheduler", "reasoning_runtime", "aee"] {
        let outcome = outcomes
            .iter()
            .find(|outcome| outcome.component == component)
            .ok_or_else(|| anyhow!("missing shutdown outcome for {component}"))?;
        if outcome.outcome != "completed" && !outcome.recoverable_partial {
            return Err(anyhow!(
                "in-flight {component} work must complete or be recoverable_partial"
            ));
        }
    }
    Ok(())
}

fn validate_cloud_notices(
    notices: &[RuntimeV2CsmShutdownCloudNotice],
    final_state: &str,
) -> Result<()> {
    if notices.is_empty() {
        return Err(anyhow!(
            "shutdown disposition must record cloud notice decisions"
        ));
    }
    let publishable = final_state != "shutdown_complete_notice_blocked";
    for notice in notices {
        normalize_id(notice.notice_id.clone(), "shutdown_cloud_notice.notice_id")?;
        validate_relative_path(&notice.evidence_ref, "shutdown_cloud_notice.evidence_ref")?;
        if notice.publishable != publishable {
            return Err(anyhow!(
                "cloud notice publishable flag must match final disposition publishability"
            ));
        }
        if notice.sent && !notice.publishable {
            return Err(anyhow!(
                "cloud notices must not be sent when disposition is not publishable"
            ));
        }
        if !notice.publishable && notice.blocked_reason.is_none() {
            return Err(anyhow!(
                "publish-blocked cloud notice must retain a blocked reason"
            ));
        }
    }
    Ok(())
}

fn validate_final_state(value: &str) -> Result<()> {
    match value {
        "shutdown_complete_publishable"
        | "shutdown_complete_with_recoverable_partials"
        | "shutdown_complete_notice_blocked" => Ok(()),
        other => Err(anyhow!("unsupported shutdown final_state '{other}'")),
    }
}

fn validate_failure_classification(value: &str) -> Result<()> {
    match value {
        "blocking_if_unavailable"
        | "recoverable_partial_if_incomplete"
        | "safe_fail_serialization_if_failed"
        | "publish_blocked_if_not_publishable" => Ok(()),
        other => Err(anyhow!(
            "unsupported shutdown failure_classification '{other}'"
        )),
    }
}

fn require_phase_before_component(
    steps: &[RuntimeV2CsmShutdownStep],
    before_component: &str,
    after_component: &str,
) -> Result<()> {
    let before = steps
        .iter()
        .find(|step| step.component == before_component)
        .ok_or_else(|| anyhow!("shutdown DAG missing {before_component} step"))?;
    let after = steps
        .iter()
        .find(|step| step.component == after_component)
        .ok_or_else(|| anyhow!("shutdown DAG missing {after_component} step"))?;
    if before.sequence >= after.sequence {
        return Err(anyhow!(
            "shutdown DAG must run {before_component} before {after_component}"
        ));
    }
    Ok(())
}

fn require_exact(value: &str, expected: &str, field: &str) -> Result<()> {
    if value != expected {
        return Err(anyhow!("{field} must be '{expected}'"));
    }
    Ok(())
}

fn validate_requirement_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}

fn validate_contains(value: &str, needle: &str, message: &str) -> Result<()> {
    if !value.contains(needle) {
        return Err(anyhow!("{message}"));
    }
    Ok(())
}
