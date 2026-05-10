//! Runtime-v2 inhabitant integration packet for v0.91.1 WP-15.
//!
//! Lifts the already-landed standing, state, lifecycle, memory, access,
//! communication, and observatory surfaces into one bounded integration proof
//! packet and deterministic operator report ahead of the WP-16 flagship demo.

use std::path::Path;

use super::*;

pub const RUNTIME_V2_RUNTIME_INHABITANT_INTEGRATION_PACKET_SCHEMA: &str =
    "runtime_v2.runtime_inhabitant_integration_packet.v1";
pub const RUNTIME_V2_RUNTIME_INHABITANT_INTEGRATION_PACKET_PATH: &str =
    "runtime_v2/inhabitant/runtime_inhabitant_integration_packet.json";
pub const RUNTIME_V2_RUNTIME_INHABITANT_OPERATOR_REPORT_PATH: &str =
    "runtime_v2/inhabitant/runtime_inhabitant_operator_report.md";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2RuntimeInhabitantStageRef {
    pub sequence: u32,
    pub stage_id: String,
    pub artifact_ref: String,
    pub proves: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2RuntimeInhabitantIntegrationPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub integrated_run_ref: String,
    pub standing_transition_ref: String,
    pub standing_event_ref: String,
    pub citizen_state_ref: String,
    pub lifecycle_state_ref: String,
    pub memory_identity_ref: String,
    pub theory_of_mind_ref: String,
    pub intelligence_metric_ref: String,
    pub governed_learning_ref: String,
    pub access_control_ref: String,
    pub acip_hardening_ref: String,
    pub a2a_adapter_boundary_ref: String,
    pub observatory_packet_ref: String,
    pub observatory_operator_dependency_ref: String,
    pub operator_report_ref: String,
    pub trace_refs: Vec<String>,
    pub integration_stage_refs: Vec<RuntimeV2RuntimeInhabitantStageRef>,
    pub validation_commands: Vec<String>,
    pub proof_summary: String,
    pub proof_classification: String,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2RuntimeInhabitantIntegrationArtifacts {
    pub packet: RuntimeV2RuntimeInhabitantIntegrationPacket,
    pub operator_report_markdown: String,
}

impl RuntimeV2RuntimeInhabitantIntegrationArtifacts {
    pub fn prototype() -> Result<Self> {
        let integrated_run = runtime_v2_csm_integrated_run_contract()?;
        let standing = runtime_v2_standing_contract()?;
        let citizen_state = runtime_v2_citizen_state_substrate_contract()?;
        let lifecycle = runtime_v2_agent_lifecycle_state_contract()?;
        let memory = runtime_v2_memory_identity_architecture_contract()?;
        let tom = runtime_v2_theory_of_mind_foundation_contract()?;
        let intelligence = runtime_v2_intelligence_metric_architecture_contract()?;
        let learning = runtime_v2_governed_learning_substrate_contract()?;
        let access = runtime_v2_access_control_contract()?;
        let acip = runtime_v2_acip_hardening_contract()?;
        let a2a = runtime_v2_a2a_adapter_boundary_contract()?;

        let packet = RuntimeV2RuntimeInhabitantIntegrationPacket::prototype(
            &integrated_run,
            &standing,
            &citizen_state,
            &lifecycle,
            &memory,
            &tom,
            &intelligence,
            &learning,
            &access,
            &acip,
            &a2a,
        )?;
        let operator_report_markdown = render_runtime_inhabitant_operator_report(&packet);
        let artifacts = Self {
            packet,
            operator_report_markdown,
        };
        artifacts.validate_against(
            &integrated_run,
            &standing,
            &citizen_state,
            &lifecycle,
            &memory,
            &tom,
            &intelligence,
            &learning,
            &access,
            &acip,
            &a2a,
        )?;
        Ok(artifacts)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn validate_against(
        &self,
        integrated_run: &RuntimeV2CsmIntegratedRunArtifacts,
        standing: &RuntimeV2StandingArtifacts,
        citizen_state: &RuntimeV2CitizenStateSubstratePacket,
        lifecycle: &RuntimeV2AgentLifecycleArtifacts,
        memory: &RuntimeV2MemoryIdentityArchitecturePacket,
        tom: &RuntimeV2TheoryOfMindFoundationPacket,
        intelligence: &RuntimeV2IntelligenceMetricArchitecturePacket,
        learning: &RuntimeV2GovernedLearningSubstratePacket,
        access: &RuntimeV2AccessControlArtifacts,
        acip: &RuntimeV2AcipHardeningPacket,
        a2a: &RuntimeV2A2aAdapterBoundaryPacket,
    ) -> Result<()> {
        let boot = runtime_v2_csm_boot_admission_contract()?;
        let lineage = runtime_v2_private_state_lineage_contract()?;
        let witness = runtime_v2_private_state_witness_contract()?;
        let observatory = runtime_v2_private_state_observatory_contract()?;

        integrated_run.validate()?;
        standing.validate()?;
        citizen_state.validate()?;
        lifecycle.validate()?;
        memory.validate_against(citizen_state, &boot, &lineage, &witness, &observatory)?;
        tom.validate_against(citizen_state, memory)?;
        intelligence.validate_against(tom)?;
        learning.validate_against(intelligence, tom)?;
        access.validate()?;
        acip.validate()?;
        a2a.validate_against(acip, &crate::agent_comms::acip_a2a_fixture_set_v1())?;
        self.packet.validate_against(
            integrated_run,
            standing,
            citizen_state,
            lifecycle,
            memory,
            tom,
            intelligence,
            learning,
            access,
            acip,
            a2a,
        )?;
        validate_runtime_inhabitant_operator_report(&self.packet, &self.operator_report_markdown)
    }

    pub fn packet_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.packet.validate()?;
        serde_json::to_vec_pretty(&self.packet)
            .context("serialize runtime inhabitant integration packet")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_RUNTIME_INHABITANT_INTEGRATION_PACKET_PATH,
            self.packet_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_RUNTIME_INHABITANT_OPERATOR_REPORT_PATH,
            self.operator_report_markdown.as_bytes().to_vec(),
        )
    }
}

impl RuntimeV2RuntimeInhabitantIntegrationPacket {
    #[allow(clippy::too_many_arguments)]
    pub fn prototype(
        integrated_run: &RuntimeV2CsmIntegratedRunArtifacts,
        standing: &RuntimeV2StandingArtifacts,
        citizen_state: &RuntimeV2CitizenStateSubstratePacket,
        lifecycle: &RuntimeV2AgentLifecycleArtifacts,
        memory: &RuntimeV2MemoryIdentityArchitecturePacket,
        tom: &RuntimeV2TheoryOfMindFoundationPacket,
        intelligence: &RuntimeV2IntelligenceMetricArchitecturePacket,
        learning: &RuntimeV2GovernedLearningSubstratePacket,
        access: &RuntimeV2AccessControlArtifacts,
        acip: &RuntimeV2AcipHardeningPacket,
        a2a: &RuntimeV2A2aAdapterBoundaryPacket,
    ) -> Result<Self> {
        let packet = Self {
            schema_version: RUNTIME_V2_RUNTIME_INHABITANT_INTEGRATION_PACKET_SCHEMA.to_string(),
            packet_id: "runtime-inhabitant-integration-v0-91-1-wp-15".to_string(),
            milestone: "v0.91.1".to_string(),
            wp: "WP-15".to_string(),
            artifact_path: RUNTIME_V2_RUNTIME_INHABITANT_INTEGRATION_PACKET_PATH.to_string(),
            source_feature_doc: "docs/milestones/v0.91.1/features/RUNTIME_INHABITANT_PROOF.md"
                .to_string(),
            integrated_run_ref: integrated_run.proof_packet.artifact_path.clone(),
            standing_transition_ref: standing.transition_packet.artifact_path.clone(),
            standing_event_ref: standing.event_packet.artifact_path.clone(),
            citizen_state_ref: citizen_state.artifact_path.clone(),
            lifecycle_state_ref: lifecycle.state_contract.artifact_path.clone(),
            memory_identity_ref: memory.artifact_path.clone(),
            theory_of_mind_ref: tom.artifact_path.clone(),
            intelligence_metric_ref: intelligence.artifact_path.clone(),
            governed_learning_ref: learning.artifact_path.clone(),
            access_control_ref: access.event_packet.artifact_path.clone(),
            acip_hardening_ref: acip.artifact_path.clone(),
            a2a_adapter_boundary_ref: a2a.artifact_path.clone(),
            observatory_packet_ref: integrated_run.proof_packet.observatory_packet_ref.clone(),
            observatory_operator_dependency_ref: integrated_run
                .proof_packet
                .operator_report_ref
                .clone(),
            operator_report_ref: RUNTIME_V2_RUNTIME_INHABITANT_OPERATOR_REPORT_PATH.to_string(),
            trace_refs: expected_trace_refs(integrated_run, standing, access),
            integration_stage_refs: expected_stage_refs(
                integrated_run,
                standing,
                citizen_state,
                lifecycle,
                memory,
                tom,
                intelligence,
                learning,
                access,
                acip,
                a2a,
            ),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_runtime_inhabitant_integration -- --nocapture"
                    .to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_integrated_run -- --nocapture"
                    .to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture"
                    .to_string(),
                "git diff --check".to_string(),
            ],
            proof_summary: "WP-15 integrates standing, citizen state, lifecycle, memory identity, theory-of-mind, intelligence metric, governed learning, access control, ACIP/A2A communication boundary, and the bounded integrated CSM run into one agent-shaped runtime proof packet and deterministic operator report.".to_string(),
            proof_classification: "proving".to_string(),
            non_claims: vec![
                "does not claim a first true birthday or personhood event".to_string(),
                "does not claim autonomous federation or external communication readiness"
                    .to_string(),
                "does not bypass lifecycle state, Freedom Gate, ACC, trace, redaction, or review boundaries".to_string(),
                "does not replace the WP-16 observatory-visible flagship demo".to_string(),
            ],
            claim_boundary: "This packet proves one bounded v0.91.1 integrated agent-shaped runtime surface by connecting the already-landed standing, state, lifecycle, memory, communication, access, observatory, and trace artifacts into one deterministic proof/report route. It does not prove birthday completion, personhood, unbounded autonomy, or the full WP-16 flagship demo.".to_string(),
        };
        packet.validate_against(
            integrated_run,
            standing,
            citizen_state,
            lifecycle,
            memory,
            tom,
            intelligence,
            learning,
            access,
            acip,
            a2a,
        )?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_RUNTIME_INHABITANT_INTEGRATION_PACKET_SCHEMA {
            return Err(anyhow!(
                "unsupported runtime inhabitant integration schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.packet_id.clone(),
            "runtime_inhabitant_integration.packet_id",
        )?;
        if self.milestone != "v0.91.1" {
            return Err(anyhow!(
                "runtime inhabitant integration packet must target milestone v0.91.1"
            ));
        }
        if self.wp != "WP-15" {
            return Err(anyhow!(
                "runtime inhabitant integration packet must remain bound to WP-15"
            ));
        }
        validate_relative_path(
            &self.artifact_path,
            "runtime_inhabitant_integration.artifact_path",
        )?;
        if self.source_feature_doc != "docs/milestones/v0.91.1/features/RUNTIME_INHABITANT_PROOF.md"
        {
            return Err(anyhow!(
                "runtime inhabitant integration packet must point at the v0.91.1 runtime inhabitant feature doc"
            ));
        }
        validate_relative_path(
            &self.source_feature_doc,
            "runtime_inhabitant_integration.source_feature_doc",
        )?;
        validate_relative_path(
            &self.integrated_run_ref,
            "runtime_inhabitant_integration.integrated_run_ref",
        )?;
        validate_relative_path(
            &self.standing_transition_ref,
            "runtime_inhabitant_integration.standing_transition_ref",
        )?;
        validate_relative_path(
            &self.standing_event_ref,
            "runtime_inhabitant_integration.standing_event_ref",
        )?;
        validate_relative_path(
            &self.citizen_state_ref,
            "runtime_inhabitant_integration.citizen_state_ref",
        )?;
        validate_relative_path(
            &self.lifecycle_state_ref,
            "runtime_inhabitant_integration.lifecycle_state_ref",
        )?;
        validate_relative_path(
            &self.memory_identity_ref,
            "runtime_inhabitant_integration.memory_identity_ref",
        )?;
        validate_relative_path(
            &self.theory_of_mind_ref,
            "runtime_inhabitant_integration.theory_of_mind_ref",
        )?;
        validate_relative_path(
            &self.intelligence_metric_ref,
            "runtime_inhabitant_integration.intelligence_metric_ref",
        )?;
        validate_relative_path(
            &self.governed_learning_ref,
            "runtime_inhabitant_integration.governed_learning_ref",
        )?;
        validate_relative_path(
            &self.access_control_ref,
            "runtime_inhabitant_integration.access_control_ref",
        )?;
        validate_relative_path(
            &self.acip_hardening_ref,
            "runtime_inhabitant_integration.acip_hardening_ref",
        )?;
        validate_relative_path(
            &self.a2a_adapter_boundary_ref,
            "runtime_inhabitant_integration.a2a_adapter_boundary_ref",
        )?;
        validate_relative_path(
            &self.observatory_packet_ref,
            "runtime_inhabitant_integration.observatory_packet_ref",
        )?;
        validate_relative_path(
            &self.observatory_operator_dependency_ref,
            "runtime_inhabitant_integration.observatory_operator_dependency_ref",
        )?;
        if self.operator_report_ref != RUNTIME_V2_RUNTIME_INHABITANT_OPERATOR_REPORT_PATH {
            return Err(anyhow!(
                "runtime inhabitant integration packet must preserve the deterministic operator report path"
            ));
        }
        validate_relative_path(
            &self.operator_report_ref,
            "runtime_inhabitant_integration.operator_report_ref",
        )?;
        validate_runtime_inhabitant_trace_refs(&self.trace_refs)?;
        validate_runtime_inhabitant_stage_refs(&self.integration_stage_refs)?;
        let expected_commands = [
            "cargo test --manifest-path adl/Cargo.toml runtime_v2_runtime_inhabitant_integration -- --nocapture",
            "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_integrated_run -- --nocapture",
            "cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture",
            "git diff --check",
        ];
        if self.validation_commands.len() != expected_commands.len()
            || self
                .validation_commands
                .iter()
                .map(String::as_str)
                .ne(expected_commands)
        {
            return Err(anyhow!(
                "runtime inhabitant integration packet must preserve the reviewed validation command set"
            ));
        }
        if self.proof_classification != "proving" {
            return Err(anyhow!(
                "runtime inhabitant integration packet must remain a proving packet"
            ));
        }
        if !self
            .claim_boundary
            .contains("does not prove birthday completion")
        {
            return Err(anyhow!(
                "runtime inhabitant integration packet must preserve the birthday/autonomy claim boundary"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("WP-16 observatory-visible flagship demo"))
        {
            return Err(anyhow!(
                "runtime inhabitant integration packet must preserve the WP-16 non-claim boundary"
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn validate_against(
        &self,
        integrated_run: &RuntimeV2CsmIntegratedRunArtifacts,
        standing: &RuntimeV2StandingArtifacts,
        citizen_state: &RuntimeV2CitizenStateSubstratePacket,
        lifecycle: &RuntimeV2AgentLifecycleArtifacts,
        memory: &RuntimeV2MemoryIdentityArchitecturePacket,
        tom: &RuntimeV2TheoryOfMindFoundationPacket,
        intelligence: &RuntimeV2IntelligenceMetricArchitecturePacket,
        learning: &RuntimeV2GovernedLearningSubstratePacket,
        access: &RuntimeV2AccessControlArtifacts,
        acip: &RuntimeV2AcipHardeningPacket,
        a2a: &RuntimeV2A2aAdapterBoundaryPacket,
    ) -> Result<()> {
        self.validate()?;
        if self.integrated_run_ref != integrated_run.proof_packet.artifact_path {
            return Err(anyhow!(
                "runtime inhabitant integration packet must remain bound to the integrated CSM run proof packet"
            ));
        }
        if self.standing_transition_ref != standing.transition_packet.artifact_path
            || self.standing_event_ref != standing.event_packet.artifact_path
        {
            return Err(anyhow!(
                "runtime inhabitant integration packet must remain bound to the reviewed standing surfaces"
            ));
        }
        if self.citizen_state_ref != citizen_state.artifact_path
            || self.lifecycle_state_ref != lifecycle.state_contract.artifact_path
            || self.memory_identity_ref != memory.artifact_path
            || self.theory_of_mind_ref != tom.artifact_path
            || self.intelligence_metric_ref != intelligence.artifact_path
            || self.governed_learning_ref != learning.artifact_path
        {
            return Err(anyhow!(
                "runtime inhabitant integration packet dependency refs drifted from the reviewed state/cognition surfaces"
            ));
        }
        if self.access_control_ref != access.event_packet.artifact_path
            || self.acip_hardening_ref != acip.artifact_path
            || self.a2a_adapter_boundary_ref != a2a.artifact_path
        {
            return Err(anyhow!(
                "runtime inhabitant integration packet dependency refs drifted from the reviewed access/comms surfaces"
            ));
        }
        if self.observatory_packet_ref != integrated_run.proof_packet.observatory_packet_ref
            || self.observatory_operator_dependency_ref
                != integrated_run.proof_packet.operator_report_ref
        {
            return Err(anyhow!(
                "runtime inhabitant integration packet must preserve observatory packet and operator dependency refs"
            ));
        }
        let expected_trace = expected_trace_refs(integrated_run, standing, access);
        if self.trace_refs != expected_trace {
            return Err(anyhow!(
                "runtime inhabitant integration packet trace refs drifted from the reviewed run/standing/access evidence"
            ));
        }
        let expected_stages = expected_stage_refs(
            integrated_run,
            standing,
            citizen_state,
            lifecycle,
            memory,
            tom,
            intelligence,
            learning,
            access,
            acip,
            a2a,
        );
        if self.integration_stage_refs != expected_stages {
            return Err(anyhow!(
                "runtime inhabitant integration packet integration_stage_refs drifted from the reviewed runtime spine"
            ));
        }
        Ok(())
    }
}

fn expected_trace_refs(
    integrated_run: &RuntimeV2CsmIntegratedRunArtifacts,
    standing: &RuntimeV2StandingArtifacts,
    access: &RuntimeV2AccessControlArtifacts,
) -> Vec<String> {
    vec![
        standing.event_packet.artifact_path.clone(),
        integrated_run.proof_packet.trace_ref.clone(),
        integrated_run.proof_packet.execution_transcript_ref.clone(),
        access.event_packet.artifact_path.clone(),
    ]
}

#[allow(clippy::too_many_arguments)]
fn expected_stage_refs(
    integrated_run: &RuntimeV2CsmIntegratedRunArtifacts,
    standing: &RuntimeV2StandingArtifacts,
    citizen_state: &RuntimeV2CitizenStateSubstratePacket,
    lifecycle: &RuntimeV2AgentLifecycleArtifacts,
    memory: &RuntimeV2MemoryIdentityArchitecturePacket,
    tom: &RuntimeV2TheoryOfMindFoundationPacket,
    intelligence: &RuntimeV2IntelligenceMetricArchitecturePacket,
    learning: &RuntimeV2GovernedLearningSubstratePacket,
    access: &RuntimeV2AccessControlArtifacts,
    acip: &RuntimeV2AcipHardeningPacket,
    a2a: &RuntimeV2A2aAdapterBoundaryPacket,
) -> Vec<RuntimeV2RuntimeInhabitantStageRef> {
    vec![
        RuntimeV2RuntimeInhabitantStageRef {
            sequence: 1,
            stage_id: "standing-mediated-action".to_string(),
            artifact_ref: standing.transition_packet.artifact_path.clone(),
            proves: "Standing remains mediated, trace-bound, and authority-preserving for the inhabitant runtime.".to_string(),
        },
        RuntimeV2RuntimeInhabitantStageRef {
            sequence: 2,
            stage_id: "citizen-state-substrate".to_string(),
            artifact_ref: citizen_state.artifact_path.clone(),
            proves: "Citizen state remains private, stale-state aware, and projection-bounded.".to_string(),
        },
        RuntimeV2RuntimeInhabitantStageRef {
            sequence: 3,
            stage_id: "lifecycle-binding".to_string(),
            artifact_ref: lifecycle.state_contract.artifact_path.clone(),
            proves: "Lifecycle state remains explicit and reviewable before inhabitant execution claims.".to_string(),
        },
        RuntimeV2RuntimeInhabitantStageRef {
            sequence: 4,
            stage_id: "memory-identity-binding".to_string(),
            artifact_ref: memory.artifact_path.clone(),
            proves: "Memory and identity stay witness-backed and observatory-linked instead of theatrical.".to_string(),
        },
        RuntimeV2RuntimeInhabitantStageRef {
            sequence: 5,
            stage_id: "theory-of-mind-boundary".to_string(),
            artifact_ref: tom.artifact_path.clone(),
            proves: "Theory-of-mind reasoning remains bounded to reviewable, non-spoofed surfaces.".to_string(),
        },
        RuntimeV2RuntimeInhabitantStageRef {
            sequence: 6,
            stage_id: "intelligence-metric-surface".to_string(),
            artifact_ref: intelligence.artifact_path.clone(),
            proves: "Runtime capability claims remain tied to reviewed intelligence metrics rather than self-assertion.".to_string(),
        },
        RuntimeV2RuntimeInhabitantStageRef {
            sequence: 7,
            stage_id: "governed-learning-surface".to_string(),
            artifact_ref: learning.artifact_path.clone(),
            proves: "Learning remains governed, rollback-aware, and dependent on reviewed cognition surfaces.".to_string(),
        },
        RuntimeV2RuntimeInhabitantStageRef {
            sequence: 8,
            stage_id: "access-and-observatory-projection".to_string(),
            artifact_ref: access.event_packet.artifact_path.clone(),
            proves: "Access control and observatory projection remain redacted, authority-scoped, and reviewable.".to_string(),
        },
        RuntimeV2RuntimeInhabitantStageRef {
            sequence: 9,
            stage_id: "acip-hardening".to_string(),
            artifact_ref: acip.artifact_path.clone(),
            proves: "Communication remains bound to ACIP hardening and local authenticated-envelope policy.".to_string(),
        },
        RuntimeV2RuntimeInhabitantStageRef {
            sequence: 10,
            stage_id: "a2a-adapter-boundary".to_string(),
            artifact_ref: a2a.artifact_path.clone(),
            proves: "A2A remains an adapter over ACIP rather than a second communication model.".to_string(),
        },
        RuntimeV2RuntimeInhabitantStageRef {
            sequence: 11,
            stage_id: "integrated-csm-run-spine".to_string(),
            artifact_ref: integrated_run.proof_packet.execution_transcript_ref.clone(),
            proves: "The bounded integrated CSM run provides the deterministic agent-shaped execution spine for the inhabitant proof.".to_string(),
        },
    ]
}

fn validate_runtime_inhabitant_trace_refs(trace_refs: &[String]) -> Result<()> {
    if trace_refs.len() != 4 {
        return Err(anyhow!(
            "runtime inhabitant integration packet must preserve exactly four reviewed trace refs"
        ));
    }
    for trace_ref in trace_refs {
        validate_relative_path(trace_ref, "runtime_inhabitant_integration.trace_refs[]")?;
    }
    Ok(())
}

fn validate_runtime_inhabitant_stage_refs(
    stage_refs: &[RuntimeV2RuntimeInhabitantStageRef],
) -> Result<()> {
    if stage_refs.len() != 11 {
        return Err(anyhow!(
            "runtime inhabitant integration packet must preserve exactly eleven reviewed stage refs"
        ));
    }
    for (idx, stage) in stage_refs.iter().enumerate() {
        if stage.sequence != (idx as u32) + 1 {
            return Err(anyhow!(
                "runtime inhabitant integration stage refs must remain in reviewed sequence order"
            ));
        }
        normalize_id(
            stage.stage_id.clone(),
            "runtime_inhabitant_integration.integration_stage_refs[].stage_id",
        )?;
        validate_relative_path(
            &stage.artifact_ref,
            "runtime_inhabitant_integration.integration_stage_refs[].artifact_ref",
        )?;
        validate_nonempty_text(
            &stage.proves,
            "runtime_inhabitant_integration.integration_stage_refs[].proves",
        )?;
    }
    Ok(())
}

fn render_runtime_inhabitant_operator_report(
    packet: &RuntimeV2RuntimeInhabitantIntegrationPacket,
) -> String {
    let mut lines = vec![
        "# Runtime Inhabitant Integration Operator Report".to_string(),
        "".to_string(),
        format!("Proof packet: `{}`", packet.artifact_path),
        format!("Classification: `{}`", packet.proof_classification),
        "".to_string(),
        packet.proof_summary.clone(),
        "".to_string(),
        "Integrated stage spine:".to_string(),
    ];
    for stage in &packet.integration_stage_refs {
        lines.push(format!(
            "- {:02} `{}` -> `{}`",
            stage.sequence, stage.stage_id, stage.artifact_ref
        ));
        lines.push(format!("  proves: {}", stage.proves));
    }
    lines.push(String::new());
    lines.push("Trace evidence:".to_string());
    for trace_ref in &packet.trace_refs {
        lines.push(format!("- `{trace_ref}`"));
    }
    lines.push(String::new());
    lines.push("Non-claims:".to_string());
    for claim in &packet.non_claims {
        lines.push(format!("- {claim}"));
    }
    lines.join("\n")
}

fn validate_runtime_inhabitant_operator_report(
    packet: &RuntimeV2RuntimeInhabitantIntegrationPacket,
    report: &str,
) -> Result<()> {
    if !report.contains("# Runtime Inhabitant Integration Operator Report") {
        return Err(anyhow!(
            "runtime inhabitant operator report must keep its canonical heading"
        ));
    }
    if !report.contains(&packet.artifact_path) {
        return Err(anyhow!(
            "runtime inhabitant operator report must cite the packet artifact path"
        ));
    }
    for stage in &packet.integration_stage_refs {
        if !report.contains(&stage.stage_id) || !report.contains(&stage.artifact_ref) {
            return Err(anyhow!(
                "runtime inhabitant operator report must include every reviewed stage ref"
            ));
        }
    }
    for trace_ref in &packet.trace_refs {
        if !report.contains(trace_ref) {
            return Err(anyhow!(
                "runtime inhabitant operator report must include every reviewed trace ref"
            ));
        }
    }
    Ok(())
}
