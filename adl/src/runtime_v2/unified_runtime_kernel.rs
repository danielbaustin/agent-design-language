//! Unified v0.91.7 runtime-kernel integration path.
//!
//! This module composes the live CSM/long-lived-agent substrate with the
//! Runtime v2 contract surfaces that are already authoritative for lifecycle,
//! standing, scheduler/provider, ACIP, memory, continuity, resilience, and
//! observability proof.

use chrono::Utc;
use serde_json::json;
use std::{collections::BTreeSet, fs, path::Path};

use super::*;
use crate::long_lived_agent::{self, RunOptions};

pub const RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_SCHEMA: &str = "runtime_v2.unified_runtime_kernel.v1";
pub const RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_SUMMARY: &str =
    "issue_5097/unified_runtime_kernel_summary.json";
pub const RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_EVENTS: &str =
    "issue_5097/unified_runtime_kernel_events.jsonl";
pub const RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_NEGATIVE_CASES: &str =
    "issue_5097/unified_runtime_kernel_negative_cases.json";
pub const RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_AWS_SIGNAL_CONFIG: &str =
    "issue_5097/aws_signal_config_disabled.json";
pub const RUNTIME_V2_UNIFIED_CURRENT_RUNTIME_ROOT: &str = "issue_5097/current_runtime";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2UnifiedKernelParticipant {
    pub participant_id: String,
    pub boundary: String,
    pub authority: String,
    pub artifact_refs: Vec<String>,
    pub correlation_id: String,
    pub local_proof_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2UnifiedKernelEvent {
    pub sequence: u32,
    pub correlation_id: String,
    pub participant_id: String,
    pub phase: String,
    pub outcome: String,
    pub artifact_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2UnifiedKernelNegativeCase {
    pub case_id: String,
    pub boundary: String,
    pub injected_condition: String,
    pub expected_outcome: String,
    pub retained_evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2UnifiedRuntimeKernelSummary {
    pub schema_version: String,
    pub issue: u32,
    pub milestone: String,
    pub proof_id: String,
    pub entrypoint: String,
    pub kernel_boundary: String,
    pub deterministic_boundary: String,
    pub live_runtime_refs: Vec<String>,
    pub runtime_v2_contract_refs: Vec<String>,
    pub participants: Vec<RuntimeV2UnifiedKernelParticipant>,
    pub retained_evidence_refs: Vec<String>,
    pub negative_case_refs: Vec<String>,
    pub validation_commands: Vec<String>,
    pub local_vs_external_claim: String,
    pub downstream_consumer: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2UnifiedRuntimeKernelArtifacts {
    pub integrated_run: RuntimeV2CsmIntegratedRunArtifacts,
    pub standing: RuntimeV2StandingArtifacts,
    pub aee_obsmem: RuntimeV2AeeObsMemPvfTraceHandoffArtifacts,
    pub acip: RuntimeV2AcipHardeningPacket,
    pub memory_identity: RuntimeV2MemoryIdentityArchitecturePacket,
    pub godel_runtime: RuntimeV2GodelAgentRuntimePacket,
    pub summary: RuntimeV2UnifiedRuntimeKernelSummary,
    pub events: Vec<RuntimeV2UnifiedKernelEvent>,
    pub negative_cases: Vec<RuntimeV2UnifiedKernelNegativeCase>,
}

impl RuntimeV2UnifiedRuntimeKernelArtifacts {
    pub fn prototype() -> Result<Self> {
        let integrated_run = runtime_v2_csm_integrated_run_contract()?;
        let standing = runtime_v2_standing_contract()?;
        let aee_obsmem = runtime_v2_aee_obsmem_pvf_trace_handoff_contract()?;
        let acip = runtime_v2_acip_hardening_contract()?;
        let memory_identity = runtime_v2_memory_identity_architecture_contract()?;
        let godel_runtime = RuntimeV2GodelAgentRuntimePacket::prototype(10)?;
        let participants = unified_kernel_participants(
            &integrated_run,
            &standing,
            &aee_obsmem,
            &acip,
            &memory_identity,
            &godel_runtime,
        );
        let events = unified_kernel_events(&participants);
        let negative_cases = unified_kernel_negative_cases();
        let summary = RuntimeV2UnifiedRuntimeKernelSummary {
            schema_version: RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_SCHEMA.to_string(),
            issue: 5097,
            milestone: "v0.91.7".to_string(),
            proof_id: "issue-5097-unified-runtime-kernel-0001".to_string(),
            entrypoint: "adl runtime-v2 unified-runtime-kernel --out docs/milestones/v0.91.7/review/runtime/unified_kernel_5097/evidence".to_string(),
            kernel_boundary: "CSM daemon/tick owns the authoritative runtime flow; Runtime v2 contracts are consumed as typed services and proof obligations instead of parallel architecture.".to_string(),
            deterministic_boundary: "deterministic core records scheduler/provider, lifecycle/standing, memory, ACIP, resilience, and observability decisions; cloud/provider transports remain explicit nondeterministic shell inputs unless configured.".to_string(),
            live_runtime_refs: vec![
                format!("{RUNTIME_V2_UNIFIED_CURRENT_RUNTIME_ROOT}/agent.yaml"),
                format!("{RUNTIME_V2_UNIFIED_CURRENT_RUNTIME_ROOT}/initial_status.json"),
                format!("{RUNTIME_V2_UNIFIED_CURRENT_RUNTIME_ROOT}/run_status.json"),
                format!("{RUNTIME_V2_UNIFIED_CURRENT_RUNTIME_ROOT}/stop_status.json"),
                format!("{RUNTIME_V2_UNIFIED_CURRENT_RUNTIME_ROOT}/final_status.json"),
            ],
            runtime_v2_contract_refs: vec![
                integrated_run.proof_packet.artifact_path.clone(),
                RUNTIME_V2_STANDING_POLICY_PATH.to_string(),
                RUNTIME_V2_AEE_OBSMEM_PVF_HANDOFF_PACKET.to_string(),
                RUNTIME_V2_ACIP_HARDENING_PACKET_PATH.to_string(),
                memory_identity.artifact_path.clone(),
                RUNTIME_V2_GODEL_AGENT_RUNTIME_PATH.to_string(),
            ],
            participants,
            retained_evidence_refs: vec![
                RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_SUMMARY.to_string(),
                RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_EVENTS.to_string(),
                RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_NEGATIVE_CASES.to_string(),
                RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_AWS_SIGNAL_CONFIG.to_string(),
                "runtime_v2/csm_run/integrated_first_run_proof_packet.json".to_string(),
                "runtime_v2/csm_run/integrated_first_run_transcript.jsonl".to_string(),
                "runtime_v2/recovery/safe_resume_decision.json".to_string(),
                RUNTIME_V2_STANDING_POLICY_PATH.to_string(),
                RUNTIME_V2_STANDING_NEGATIVE_CASES_PATH.to_string(),
                RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_WRITE.to_string(),
                RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_ACK.to_string(),
                RUNTIME_V2_AEE_OBSMEM_PVF_RETRIEVAL.to_string(),
                RUNTIME_V2_ACIP_HARDENING_PACKET_PATH.to_string(),
                memory_identity.artifact_path.clone(),
                RUNTIME_V2_GODEL_AGENT_RUNTIME_PATH.to_string(),
            ],
            negative_case_refs: vec![RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_NEGATIVE_CASES.to_string()],
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_unified_runtime_kernel -- --nocapture".to_string(),
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 unified-runtime-kernel --out docs/milestones/v0.91.7/review/runtime/unified_kernel_5097/evidence".to_string(),
                "git diff --check".to_string(),
            ],
            local_vs_external_claim: "This proof runs a bounded local current-runtime tick and materializes local contract evidence. It does not require or claim live AWS credentials, paid cloud resources, remote providers, or long soak completion.".to_string(),
            downstream_consumer: "#5096 can consume the scheduler/provider, reasoning/loop, lifecycle/standing, and continuity boundaries without redefining runtime architecture.".to_string(),
        };
        let artifacts = Self {
            integrated_run,
            standing,
            aee_obsmem,
            acip,
            memory_identity,
            godel_runtime,
            summary,
            events,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.integrated_run.validate()?;
        self.standing.validate()?;
        self.aee_obsmem.validate()?;
        self.acip.validate()?;
        self.memory_identity.validate()?;
        self.godel_runtime.validate()?;
        self.summary.validate_against(self)?;
        validate_unified_kernel_events(&self.events, &self.summary.participants)?;
        validate_unified_kernel_negative_cases(&self.negative_cases, &self.summary)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.validate()?;
        self.integrated_run.write_to_root(root)?;
        self.standing.write_to_root(root)?;
        self.aee_obsmem.write_to_root(root)?;
        self.acip.write_to_root(root)?;
        self.memory_identity.write_to_root(root)?;
        self.godel_runtime
            .write_to_path(root.join(RUNTIME_V2_GODEL_AGENT_RUNTIME_PATH))?;
        write_unified_current_runtime(root)?;
        write_relative(
            root,
            RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_AWS_SIGNAL_CONFIG,
            serde_json::to_vec_pretty(&json!({
                "schema_version": "runtime_v2.unified_runtime_kernel.external_signal_config.v1",
                "aws_signal_status": "disabled_for_local_deterministic_proof",
                "eventbridge": "not_configured",
                "cloudwatch": "not_configured",
                "sns_sqs": "not_configured",
                "expected_behavior": "runtime records disabled external signal configuration without attempting live cloud publish",
                "negative_case_ref": "missing_disabled_external_signal_config"
            }))?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_EVENTS,
            self.events_jsonl_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_NEGATIVE_CASES,
            serde_json::to_vec_pretty(&self.negative_cases)?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_SUMMARY,
            serde_json::to_vec_pretty(&self.summary)?,
        )
    }

    fn events_jsonl_bytes(&self) -> Result<Vec<u8>> {
        let mut out = Vec::new();
        for event in &self.events {
            serde_json::to_writer(&mut out, event).context("serialize unified kernel event")?;
            out.push(b'\n');
        }
        Ok(out)
    }
}

impl RuntimeV2UnifiedRuntimeKernelSummary {
    pub fn validate_against(
        &self,
        artifacts: &RuntimeV2UnifiedRuntimeKernelArtifacts,
    ) -> Result<()> {
        require_exact(
            &self.schema_version,
            RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_SCHEMA,
            "unified_runtime_kernel.schema_version",
        )?;
        if self.issue != 5097 {
            return Err(anyhow!(
                "unified runtime kernel proof must remain bound to #5097"
            ));
        }
        require_exact(
            &self.milestone,
            "v0.91.7",
            "unified_runtime_kernel.milestone",
        )?;
        normalize_id(self.proof_id.clone(), "unified_runtime_kernel.proof_id")?;
        validate_nonempty_text(&self.entrypoint, "unified_runtime_kernel.entrypoint")?;
        validate_nonempty_text(
            &self.kernel_boundary,
            "unified_runtime_kernel.kernel_boundary",
        )?;
        validate_nonempty_text(
            &self.deterministic_boundary,
            "unified_runtime_kernel.deterministic_boundary",
        )?;
        validate_relative_path_list(
            &self.live_runtime_refs,
            "unified_runtime_kernel.live_runtime_refs",
        )?;
        validate_relative_path_list(
            &self.runtime_v2_contract_refs,
            "unified_runtime_kernel.runtime_v2_contract_refs",
        )?;
        validate_relative_path_list(
            &self.retained_evidence_refs,
            "unified_runtime_kernel.retained_evidence_refs",
        )?;
        validate_relative_path_list(
            &self.negative_case_refs,
            "unified_runtime_kernel.negative_case_refs",
        )?;
        for required in [
            RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_SUMMARY,
            RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_EVENTS,
            RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_NEGATIVE_CASES,
            RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_AWS_SIGNAL_CONFIG,
            artifacts.integrated_run.proof_packet.artifact_path.as_str(),
            RUNTIME_V2_STANDING_POLICY_PATH,
            RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_ACK,
            RUNTIME_V2_ACIP_HARDENING_PACKET_PATH,
            artifacts.memory_identity.artifact_path.as_str(),
            RUNTIME_V2_GODEL_AGENT_RUNTIME_PATH,
        ] {
            if !self
                .retained_evidence_refs
                .iter()
                .any(|value| value == required)
            {
                return Err(anyhow!(
                    "unified runtime kernel summary missing retained evidence ref '{required}'"
                ));
            }
        }
        let required_participants = [
            "daemon_tick",
            "lifecycle_standing",
            "scheduler_provider",
            "memory_obsmem",
            "acip_boundary",
            "resilience_continuity",
            "observability",
            "external_signals",
        ];
        for required in required_participants {
            if !self
                .participants
                .iter()
                .any(|participant| participant.participant_id == required)
            {
                return Err(anyhow!(
                    "unified runtime kernel missing participant '{required}'"
                ));
            }
        }
        for participant in &self.participants {
            participant.validate()?;
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("unified-runtime-kernel"))
        {
            return Err(anyhow!(
                "unified runtime kernel validation commands must include the CLI proof"
            ));
        }
        validate_nonempty_text(
            &self.local_vs_external_claim,
            "unified_runtime_kernel.local_vs_external_claim",
        )?;
        validate_nonempty_text(
            &self.downstream_consumer,
            "unified_runtime_kernel.downstream_consumer",
        )
    }
}

impl RuntimeV2UnifiedKernelParticipant {
    fn validate(&self) -> Result<()> {
        normalize_id(self.participant_id.clone(), "unified_kernel.participant_id")?;
        validate_nonempty_text(&self.boundary, "unified_kernel.participant.boundary")?;
        validate_nonempty_text(&self.authority, "unified_kernel.participant.authority")?;
        validate_relative_path_list(
            &self.artifact_refs,
            "unified_kernel.participant.artifact_refs",
        )?;
        normalize_id(
            self.correlation_id.clone(),
            "unified_kernel.participant.correlation_id",
        )?;
        match self.local_proof_status.as_str() {
            "local_executed" | "contract_consumed" | "local_disabled_recorded" => Ok(()),
            other => Err(anyhow!(
                "unsupported unified kernel participant local proof status '{other}'"
            )),
        }
    }
}

fn unified_kernel_participants(
    integrated_run: &RuntimeV2CsmIntegratedRunArtifacts,
    standing: &RuntimeV2StandingArtifacts,
    aee_obsmem: &RuntimeV2AeeObsMemPvfTraceHandoffArtifacts,
    acip: &RuntimeV2AcipHardeningPacket,
    memory_identity: &RuntimeV2MemoryIdentityArchitecturePacket,
    _godel_runtime: &RuntimeV2GodelAgentRuntimePacket,
) -> Vec<RuntimeV2UnifiedKernelParticipant> {
    vec![
        participant(
            "daemon_tick",
            "CSM daemon/tick current-runtime substrate",
            "live current runtime owns bounded tick execution",
            &[
                &format!("{RUNTIME_V2_UNIFIED_CURRENT_RUNTIME_ROOT}/run_status.json"),
                &format!("{RUNTIME_V2_UNIFIED_CURRENT_RUNTIME_ROOT}/final_status.json"),
            ],
            "local_executed",
        ),
        participant(
            "lifecycle_standing",
            "Runtime v2 lifecycle and standing policy visible to the live path",
            "standing policy gates rights and lifecycle transitions",
            &[&standing.policy.artifact_path, RUNTIME_V2_STANDING_NEGATIVE_CASES_PATH],
            "contract_consumed",
        ),
        participant(
            "scheduler_provider",
            "scheduler/provider/local-agent selection",
            "Godel runtime provider registry and launch plan bind scheduler authority",
            &[
                RUNTIME_V2_GODEL_AGENT_RUNTIME_PATH,
                &integrated_run.governed_episode.scheduling_decision.artifact_path,
            ],
            "contract_consumed",
        ),
        participant(
            "memory_obsmem",
            "AEE observation to ObsMem write and retrieval",
            "AEE emits reviewable trace handoff; ObsMem adapter owns persisted memory write",
            &[
                &aee_obsmem.packet.obsmem_write_ref,
                &aee_obsmem.packet.obsmem_ack_ref,
                &aee_obsmem.packet.obsmem_retrieval_ref,
                &memory_identity.artifact_path,
            ],
            "contract_consumed",
        ),
        participant(
            "acip_boundary",
            "ACIP runtime stream readiness",
            "ACIP hardening packet owns authenticated communication boundary",
            &[&acip.artifact_path],
            "contract_consumed",
        ),
        participant(
            "resilience_continuity",
            "stop, lease, failed-cycle, checkpoint, recovery, replay",
            "integrated run owns resilience, continuity, recovery, quarantine, and hardening proof refs",
            &[
                &integrated_run.wake_continuity.wake_continuity_proof.artifact_path,
                &integrated_run.recovery.safe_resume_decision.artifact_path,
                &integrated_run.quarantine.quarantine_artifact.artifact_path,
                &integrated_run.hardening.proof_packet.artifact_path,
            ],
            "contract_consumed",
        ),
        participant(
            "observability",
            "synthetic runtime observability correlation index",
            "unified kernel records one correlation id per participant and links each id to the participant's first retained evidence reference",
            &[RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_EVENTS, &integrated_run.observatory.visibility_packet_path],
            "contract_consumed",
        ),
        participant(
            "external_signals",
            "AWS signal shell boundary",
            "disabled local config is recorded fail-closed; live cloud publish is not claimed",
            &[RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_AWS_SIGNAL_CONFIG],
            "local_disabled_recorded",
        ),
    ]
}

fn participant(
    participant_id: &str,
    boundary: &str,
    authority: &str,
    artifact_refs: &[&str],
    local_proof_status: &str,
) -> RuntimeV2UnifiedKernelParticipant {
    RuntimeV2UnifiedKernelParticipant {
        participant_id: participant_id.to_string(),
        boundary: boundary.to_string(),
        authority: authority.to_string(),
        artifact_refs: artifact_refs
            .iter()
            .map(|value| value.to_string())
            .collect(),
        correlation_id: format!("corr-issue-5097-{participant_id}"),
        local_proof_status: local_proof_status.to_string(),
    }
}

fn unified_kernel_events(
    participants: &[RuntimeV2UnifiedKernelParticipant],
) -> Vec<RuntimeV2UnifiedKernelEvent> {
    participants
        .iter()
        .enumerate()
        .map(|(index, participant)| RuntimeV2UnifiedKernelEvent {
            sequence: index as u32 + 1,
            correlation_id: participant.correlation_id.clone(),
            participant_id: participant.participant_id.clone(),
            phase: "unified_kernel_local_proof".to_string(),
            outcome: participant.local_proof_status.clone(),
            artifact_ref: participant
                .artifact_refs
                .first()
                .cloned()
                .expect("participant artifact refs are populated"),
        })
        .collect()
}

fn unified_kernel_negative_cases() -> Vec<RuntimeV2UnifiedKernelNegativeCase> {
    vec![
        negative_case(
            "invalid_lifecycle_standing_transition",
            "lifecycle_standing",
            "guest tries to acquire citizen and continuity rights without identity binding",
            "standing policy rejects transition and preserves review gate",
            RUNTIME_V2_STANDING_NEGATIVE_CASES_PATH,
        ),
        negative_case(
            "provider_scheduler_mismatch",
            "scheduler_provider",
            "scheduled agent count or provider binding diverges from launch plan",
            "Godel runtime validation rejects provider/scheduler mismatch",
            RUNTIME_V2_GODEL_AGENT_RUNTIME_PATH,
        ),
        negative_case(
            "failed_tick_recoverable_cycle",
            "resilience_continuity",
            "cycle fails after lease/checkpoint boundary",
            "recovery/quarantine evidence preserves resumable or custody-safe state",
            "runtime_v2/recovery/safe_resume_decision.json",
        ),
        negative_case(
            "stop_request",
            "daemon_tick",
            "operator stop request enters the live current-runtime path",
            "bounded local runtime emits stop status and final status artifacts",
            &format!("{RUNTIME_V2_UNIFIED_CURRENT_RUNTIME_ROOT}/stop_status.json"),
        ),
        negative_case(
            "missing_disabled_external_signal_config",
            "external_signals",
            "AWS/EventBridge/CloudWatch/SNS config is absent in local proof",
            "runtime records disabled external signal config without live publish",
            RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_AWS_SIGNAL_CONFIG,
        ),
    ]
}

fn negative_case(
    case_id: &str,
    boundary: &str,
    injected_condition: &str,
    expected_outcome: &str,
    retained_evidence_ref: &str,
) -> RuntimeV2UnifiedKernelNegativeCase {
    RuntimeV2UnifiedKernelNegativeCase {
        case_id: case_id.to_string(),
        boundary: boundary.to_string(),
        injected_condition: injected_condition.to_string(),
        expected_outcome: expected_outcome.to_string(),
        retained_evidence_ref: retained_evidence_ref.to_string(),
    }
}

fn validate_unified_kernel_events(
    events: &[RuntimeV2UnifiedKernelEvent],
    participants: &[RuntimeV2UnifiedKernelParticipant],
) -> Result<()> {
    if events.len() != participants.len() {
        return Err(anyhow!(
            "unified kernel events must contain one event per participant"
        ));
    }
    let participant_ids = participants
        .iter()
        .map(|participant| participant.participant_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut observed_ids = BTreeSet::new();
    for (expected_sequence, event) in (1u32..).zip(events.iter()) {
        if event.sequence != expected_sequence {
            return Err(anyhow!(
                "unified kernel events must be contiguous and ordered"
            ));
        }
        normalize_id(
            event.correlation_id.clone(),
            "unified_kernel_event.correlation_id",
        )?;
        normalize_id(
            event.participant_id.clone(),
            "unified_kernel_event.participant_id",
        )?;
        validate_nonempty_text(&event.phase, "unified_kernel_event.phase")?;
        validate_nonempty_text(&event.outcome, "unified_kernel_event.outcome")?;
        validate_relative_path(&event.artifact_ref, "unified_kernel_event.artifact_ref")?;
        let participant = participants
            .iter()
            .find(|participant| participant.participant_id == event.participant_id)
            .ok_or_else(|| anyhow!("unified kernel event references unknown participant"))?;
        if !observed_ids.insert(event.participant_id.as_str()) {
            return Err(anyhow!(
                "unified kernel events contain duplicate participant '{}'",
                event.participant_id
            ));
        }
        if participant.correlation_id != event.correlation_id {
            return Err(anyhow!(
                "unified kernel event correlation id must match participant"
            ));
        }
    }
    if observed_ids != participant_ids {
        return Err(anyhow!(
            "unified kernel events must cover exactly the participant set"
        ));
    }
    Ok(())
}

fn validate_unified_kernel_negative_cases(
    cases: &[RuntimeV2UnifiedKernelNegativeCase],
    summary: &RuntimeV2UnifiedRuntimeKernelSummary,
) -> Result<()> {
    let required = [
        "invalid_lifecycle_standing_transition",
        "provider_scheduler_mismatch",
        "failed_tick_recoverable_cycle",
        "stop_request",
        "missing_disabled_external_signal_config",
    ];
    for required_case in required {
        if !cases.iter().any(|case| case.case_id == required_case) {
            return Err(anyhow!(
                "unified runtime kernel missing negative case '{required_case}'"
            ));
        }
    }
    for case in cases {
        normalize_id(case.case_id.clone(), "unified_kernel_negative.case_id")?;
        normalize_id(case.boundary.clone(), "unified_kernel_negative.boundary")?;
        validate_nonempty_text(
            &case.injected_condition,
            "unified_kernel_negative.injected_condition",
        )?;
        validate_nonempty_text(
            &case.expected_outcome,
            "unified_kernel_negative.expected_outcome",
        )?;
        validate_relative_path(
            &case.retained_evidence_ref,
            "unified_kernel_negative.retained_evidence_ref",
        )?;
        if !summary
            .retained_evidence_refs
            .iter()
            .any(|reference| reference == &case.retained_evidence_ref)
            && !summary
                .live_runtime_refs
                .iter()
                .any(|reference| reference == &case.retained_evidence_ref)
        {
            return Err(anyhow!(
                "unified kernel negative case '{}' references evidence not retained by summary: {}",
                case.case_id,
                case.retained_evidence_ref
            ));
        }
    }
    Ok(())
}

fn write_unified_current_runtime(root: &Path) -> Result<()> {
    let current_root = root.join(RUNTIME_V2_UNIFIED_CURRENT_RUNTIME_ROOT);
    fs::create_dir_all(&current_root).with_context(|| {
        format!(
            "create unified current-runtime root {}",
            current_root.display()
        )
    })?;
    let spec_path = current_root.join("agent.yaml");
    fs::write(
        &spec_path,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: issue-5097-unified-runtime-kernel
display_name: Issue 5097 Unified Runtime Kernel Proof
state_root: state
workflow:
  kind: demo_adapter
  name: unified_runtime_kernel_local_tick
  run_args:
    provider_id: local_deterministic
    model: bounded-local-proof
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: runtime-v2/unified-runtime-kernel
  write_policy: append_only
"#,
    )
    .with_context(|| format!("write unified runtime spec {}", spec_path.display()))?;

    let initial_status = long_lived_agent::status(&spec_path)?;
    let run_status = long_lived_agent::run(
        &spec_path,
        RunOptions {
            max_cycles: 1,
            interval_secs: Some(0),
            no_sleep: true,
            recover_stale_lease: false,
        },
    )?;
    let stopped = long_lived_agent::stop(
        &spec_path,
        "bounded #5097 unified runtime kernel local proof stop request",
    )?;
    let final_status = long_lived_agent::status(&spec_path)?;

    write_json(&current_root.join("initial_status.json"), &initial_status)?;
    write_json(&current_root.join("run_status.json"), &run_status)?;
    write_json(&current_root.join("stop_status.json"), &stopped)?;
    write_json(
        &current_root.join("final_status.json"),
        &json!({
            "generated_at": Utc::now(),
            "status": final_status,
            "classification": "bounded_local_current_runtime_tick"
        }),
    )
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create output directory {}", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(value)
        .with_context(|| format!("serialize json artifact {}", path.display()))?;
    fs::write(path, bytes).with_context(|| format!("write json artifact {}", path.display()))
}

fn validate_relative_path_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for value in values {
        validate_relative_path(value, field)?;
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate artifact ref"));
        }
    }
    Ok(())
}

fn require_exact(actual: &str, expected: &str, field: &str) -> Result<()> {
    if actual != expected {
        return Err(anyhow!("{field} must equal '{expected}', got '{actual}'"));
    }
    Ok(())
}
