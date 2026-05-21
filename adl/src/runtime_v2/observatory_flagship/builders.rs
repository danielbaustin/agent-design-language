use super::*;
use std::collections::BTreeSet;
use std::path::Path;

impl RuntimeV2ObservatoryFlagshipArtifacts {
    pub fn prototype() -> Result<Self> {
        let challenge_artifacts = runtime_v2_continuity_challenge_contract()?;
        let operator_control_report = runtime_v2_operator_control_report_contract()?;
        let lifecycle_artifacts = runtime_v2_agent_lifecycle_state_contract()?;
        let acip_hardening_packet = runtime_v2_acip_hardening_contract()?;
        let a2a_adapter_boundary_packet = runtime_v2_a2a_adapter_boundary_contract()?;
        let runtime_inhabitant_integration = runtime_v2_runtime_inhabitant_integration_contract()?;
        let lens_sequence = observatory_flagship_walkthrough(
            &challenge_artifacts,
            &operator_control_report,
            &lifecycle_artifacts,
            &acip_hardening_packet,
            &a2a_adapter_boundary_packet,
            &runtime_inhabitant_integration,
        )?;
        let proof_packet = RuntimeV2ObservatoryFlagshipProofPacket::from_artifacts(
            &challenge_artifacts,
            &operator_control_report,
            &lifecycle_artifacts,
            &acip_hardening_packet,
            &a2a_adapter_boundary_packet,
            &runtime_inhabitant_integration,
            lens_sequence,
        )?;
        let operator_report_markdown =
            render_observatory_flagship_operator_report(&proof_packet, &challenge_artifacts)?;
        let artifacts = Self {
            challenge_artifacts,
            operator_control_report,
            lifecycle_artifacts,
            acip_hardening_packet,
            a2a_adapter_boundary_packet,
            runtime_inhabitant_integration,
            proof_packet,
            operator_report_markdown,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.proof_packet.validate_against(
            &self.challenge_artifacts,
            &self.operator_control_report,
            &self.lifecycle_artifacts,
            &self.acip_hardening_packet,
            &self.a2a_adapter_boundary_packet,
            &self.runtime_inhabitant_integration,
        )?;
        self.challenge_artifacts.validate()?;
        self.operator_control_report.validate()?;
        self.lifecycle_artifacts.validate()?;
        self.acip_hardening_packet.validate()?;
        self.a2a_adapter_boundary_packet.validate_against(
            &self.acip_hardening_packet,
            &crate::agent_comms::acip_a2a_fixture_set_v1(),
        )?;
        self.runtime_inhabitant_integration.validate_against(
            &runtime_v2_csm_integrated_run_contract()?,
            &runtime_v2_standing_contract()?,
            &runtime_v2_citizen_state_substrate_contract()?,
            &self.lifecycle_artifacts,
            &runtime_v2_memory_identity_architecture_contract()?,
            &runtime_v2_theory_of_mind_foundation_contract()?,
            &crate::capability_aptitude_testing::build_capability_aptitude_artifact_bundle(),
            &runtime_v2_intelligence_metric_architecture_contract()?,
            &runtime_v2_governed_learning_substrate_contract()?,
            &runtime_v2_access_control_contract()?,
            &self.acip_hardening_packet,
            &self.a2a_adapter_boundary_packet,
        )?;
        validate_flagship_operator_report(&self.proof_packet, &self.operator_report_markdown)
    }

    pub fn proof_packet_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.proof_packet.validate_against(
            &self.challenge_artifacts,
            &self.operator_control_report,
            &self.lifecycle_artifacts,
            &self.acip_hardening_packet,
            &self.a2a_adapter_boundary_packet,
            &self.runtime_inhabitant_integration,
        )?;
        serde_json::to_vec_pretty(&self.proof_packet)
            .context("serialize Runtime v2 Observatory flagship proof packet")
    }

    pub fn walkthrough_jsonl_bytes(&self) -> Result<Vec<u8>> {
        self.proof_packet.validate_against(
            &self.challenge_artifacts,
            &self.operator_control_report,
            &self.lifecycle_artifacts,
            &self.acip_hardening_packet,
            &self.a2a_adapter_boundary_packet,
            &self.runtime_inhabitant_integration,
        )?;
        let mut out = Vec::new();
        for step in &self.proof_packet.lens_sequence {
            serde_json::to_writer(&mut out, step)
                .context("serialize Runtime v2 Observatory flagship walkthrough step")?;
            out.push(b'\n');
        }
        Ok(out)
    }

    pub fn execution_summary(&self) -> Result<String> {
        let mut lines = vec![
            "D12 inhabited CSM Observatory flagship proof:".to_string(),
            format!("- proof packet: {}", self.proof_packet.artifact_path),
            format!(
                "- operator report: {}",
                self.proof_packet.operator_report_ref
            ),
            format!("- walkthrough: {}", self.proof_packet.walkthrough_ref),
        ];
        for step in &self.proof_packet.lens_sequence {
            lines.push(format!(
                "- {:02} {} :: {} -> {}",
                step.sequence, step.room, step.lens_or_memory_dot, step.artifact_ref
            ));
        }
        Ok(lines.join("\n"))
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();

        let access = &self.challenge_artifacts.access_control_artifacts;
        let observatory = &access.observatory_artifacts;
        let observatory_sanctuary = &observatory.sanctuary_artifacts;
        let challenge_sanctuary = &self.challenge_artifacts.sanctuary_artifacts;

        access.standing_artifacts.write_to_root(root)?;
        observatory.private_state_artifacts.write_to_root(root)?;
        observatory
            .witness_artifacts
            .lineage_artifacts
            .write_to_root(root)?;
        observatory.witness_artifacts.write_to_root(root)?;
        observatory_sanctuary
            .anti_equivocation_artifacts
            .witness_artifacts
            .lineage_artifacts
            .write_to_root(root)?;
        observatory_sanctuary
            .anti_equivocation_artifacts
            .witness_artifacts
            .write_to_root(root)?;
        observatory_sanctuary
            .anti_equivocation_artifacts
            .write_to_root(root)?;
        observatory_sanctuary.write_to_root(root)?;
        observatory.write_to_root(root)?;
        access.write_to_root(root)?;
        self.lifecycle_artifacts.write_to_root(root)?;
        self.acip_hardening_packet.write_to_root(root)?;
        self.a2a_adapter_boundary_packet.write_to_root(root)?;
        self.runtime_inhabitant_integration.write_to_root(root)?;

        challenge_sanctuary
            .anti_equivocation_artifacts
            .witness_artifacts
            .lineage_artifacts
            .write_to_root(root)?;
        challenge_sanctuary
            .anti_equivocation_artifacts
            .witness_artifacts
            .write_to_root(root)?;
        challenge_sanctuary
            .anti_equivocation_artifacts
            .write_to_root(root)?;
        challenge_sanctuary.write_to_root(root)?;
        self.challenge_artifacts.write_to_root(root)?;
        self.operator_control_report.write_to_root(root)?;

        let mut walkthrough = Vec::new();
        for step in &self.proof_packet.lens_sequence {
            serde_json::to_writer(&mut walkthrough, step)
                .context("serialize Runtime v2 Observatory flagship walkthrough step")?;
            walkthrough.push(b'\n');
        }
        write_relative(
            root,
            RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_PATH,
            walkthrough,
        )?;
        write_relative(
            root,
            RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH,
            self.operator_report_markdown.as_bytes().to_vec(),
        )?;
        write_relative(
            root,
            RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_PATH,
            serde_json::to_vec_pretty(&self.proof_packet)
                .context("serialize Runtime v2 Observatory flagship proof packet")?,
        )
    }
}

impl RuntimeV2ObservatoryFlagshipProofPacket {
    pub fn from_artifacts(
        challenge: &RuntimeV2ContinuityChallengeArtifacts,
        operator: &RuntimeV2OperatorControlReport,
        lifecycle: &RuntimeV2AgentLifecycleArtifacts,
        acip: &RuntimeV2AcipHardeningPacket,
        a2a: &RuntimeV2A2aAdapterBoundaryPacket,
        runtime_inhabitant: &RuntimeV2RuntimeInhabitantIntegrationArtifacts,
        lens_sequence: Vec<RuntimeV2ObservatoryFlagshipWalkthroughStep>,
    ) -> Result<Self> {
        challenge.validate()?;
        operator.validate()?;
        lifecycle.validate()?;
        acip.validate()?;
        a2a.validate_against(acip, &crate::agent_comms::acip_a2a_fixture_set_v1())?;
        runtime_inhabitant.validate_against(
            &runtime_v2_csm_integrated_run_contract()?,
            &runtime_v2_standing_contract()?,
            &runtime_v2_citizen_state_substrate_contract()?,
            lifecycle,
            &runtime_v2_memory_identity_architecture_contract()?,
            &runtime_v2_theory_of_mind_foundation_contract()?,
            &crate::capability_aptitude_testing::build_capability_aptitude_artifact_bundle(),
            &runtime_v2_intelligence_metric_architecture_contract()?,
            &runtime_v2_governed_learning_substrate_contract()?,
            &runtime_v2_access_control_contract()?,
            acip,
            a2a,
        )?;
        validate_flagship_walkthrough(&lens_sequence)?;

        let access = &challenge.access_control_artifacts;
        let observatory = &access.observatory_artifacts;
        let standing = &access.standing_artifacts;
        let witness = &observatory.witness_artifacts;
        let sanctuary = &challenge.sanctuary_artifacts;
        let continuity_refs = vec![
            witness.witness_set.artifact_path.clone(),
            witness.receipt_set.artifact_path.clone(),
            witness.lineage_artifacts.ledger.artifact_path.clone(),
            witness
                .lineage_artifacts
                .materialized_head
                .artifact_path
                .clone(),
            sanctuary.quarantine_artifact.artifact_path.clone(),
        ];
        let observatory_refs = vec![
            observatory.redaction_policy.artifact_path.clone(),
            observatory.projection_packet.artifact_path.clone(),
            RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_REPORT_PATH.to_string(),
            RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PROOF_PATH.to_string(),
        ];
        let lifecycle_refs = vec![
            lifecycle.state_contract.artifact_path.clone(),
            lifecycle.transition_matrix.artifact_path.clone(),
            lifecycle.proof_fixtures.artifact_path.clone(),
        ];
        let standing_access_refs = vec![
            standing.policy.artifact_path.clone(),
            standing.event_packet.artifact_path.clone(),
            standing.communication_examples.artifact_path.clone(),
            RUNTIME_V2_STANDING_NEGATIVE_CASES_PATH.to_string(),
            access.authority_matrix.artifact_path.clone(),
            access.event_packet.artifact_path.clone(),
            RUNTIME_V2_ACCESS_DENIAL_FIXTURES_PATH.to_string(),
        ];
        let communication_boundary_refs = vec![
            acip.artifact_path.clone(),
            a2a.artifact_path.clone(),
            access.event_packet.artifact_path.clone(),
        ];
        let runtime_inhabitant_refs = vec![
            runtime_inhabitant.packet.artifact_path.clone(),
            runtime_inhabitant.packet.operator_report_ref.clone(),
        ];
        let challenge_refs = vec![
            challenge.challenge.artifact_path.clone(),
            challenge.freeze.artifact_path.clone(),
            challenge.appeal_review.artifact_path.clone(),
            challenge.threat_model.artifact_path.clone(),
            challenge.economics_placement.artifact_path.clone(),
            sanctuary.operator_report.artifact_path.clone(),
        ];
        let operator_report_refs = vec![
            operator.artifact_path.clone(),
            RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH.to_string(),
        ];
        let required_artifact_refs = required_artifact_refs(
            &continuity_refs,
            &observatory_refs,
            &lifecycle_refs,
            &standing_access_refs,
            &communication_boundary_refs,
            &runtime_inhabitant_refs,
            &challenge_refs,
            &operator_report_refs,
        );
        let feature_demo_coverage =
            feature_demo_coverage(&standing_access_refs, challenge, operator);

        let packet = Self {
            schema_version: RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_SCHEMA.to_string(),
            proof_id: "d12-inhabited-csm-observatory-flagship-proof-0001".to_string(),
            demo_id: "D12".to_string(),
            milestone: "v0.91.1".to_string(),
            artifact_path: RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_PATH.to_string(),
            operator_report_ref: RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH.to_string(),
            walkthrough_ref: RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_PATH.to_string(),
            source_docs: vec![
                "docs/milestones/v0.91.1/README.md".to_string(),
                "docs/milestones/v0.91.1/DEMO_MATRIX_v0.91.1.md".to_string(),
                "docs/milestones/v0.91.1/features/RUNTIME_INHABITANT_PROOF.md".to_string(),
                "docs/milestones/v0.91.1/RUNTIME_POLIS_ARCHITECTURE_PACKAGE_v0.91.1.md"
                    .to_string(),
            ],
            actor_roster: vec![
                RuntimeV2ObservatoryFlagshipActor {
                    actor_id: "proto-citizen-alpha".to_string(),
                    standing_class: "citizen".to_string(),
                    visible_role: "identity-bearing citizen with private state, continuity witness, and citizen receipt".to_string(),
                    evidence_refs: vec![
                        witness.receipt_set.artifact_path.clone(),
                        observatory.projection_packet.artifact_path.clone(),
                    ],
                    prohibited_claims: vec![
                        "personhood is not proven".to_string(),
                        "raw private state is not disclosed".to_string(),
                    ],
                },
                RuntimeV2ObservatoryFlagshipActor {
                    actor_id: "guest-operator-candidate".to_string(),
                    standing_class: "guest".to_string(),
                    visible_role: "bounded outside participant denied citizen-only continuity authority"
                        .to_string(),
                    evidence_refs: vec![
                        standing.event_packet.artifact_path.clone(),
                        access.denial_fixtures.event_packet_ref.clone(),
                    ],
                    prohibited_claims: vec![
                        "guest standing cannot silently become citizen standing".to_string(),
                    ],
                },
                RuntimeV2ObservatoryFlagshipActor {
                    actor_id: "operator.cli".to_string(),
                    standing_class: "operator".to_string(),
                    visible_role: "reviewer-visible operator using redacted Observatory surfaces"
                        .to_string(),
                    evidence_refs: vec![
                        operator.artifact_path.clone(),
                        RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH.to_string(),
                    ],
                    prohibited_claims: vec![
                        "operator view is not private-state authority".to_string(),
                    ],
                },
                RuntimeV2ObservatoryFlagshipActor {
                    actor_id: "flagship-service-witness".to_string(),
                    standing_class: "service".to_string(),
                    visible_role: "service actor that emits continuity evidence without acquiring citizen rights"
                        .to_string(),
                    evidence_refs: vec![
                        witness.witness_set.artifact_path.clone(),
                        challenge.challenge.artifact_path.clone(),
                    ],
                    prohibited_claims: vec![
                        "service witness cannot own or fork citizen state".to_string(),
                    ],
                },
            ],
            required_artifact_refs,
            continuity_refs,
            observatory_refs,
            lifecycle_refs,
            standing_access_refs,
            communication_boundary_refs,
            runtime_inhabitant_refs,
            challenge_refs,
            operator_report_refs,
            feature_demo_coverage,
            lens_sequence,
            reviewer_command:
                "adl runtime-v2 observatory-flagship-demo --out artifacts/v0911/demo-d12-observatory-flagship"
                    .to_string(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_observatory_flagship -- --nocapture"
                    .to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_observatory_flagship_demo -- --nocapture"
                    .to_string(),
                "adl runtime-v2 observatory-flagship-demo --out artifacts/v0911/demo-d12-observatory-flagship"
                    .to_string(),
                "git diff --check".to_string(),
            ],
            proof_summary:
                "D12 integrates the landed WP-03 lifecycle state model, WP-13 ACIP hardening packet, WP-14 A2A adapter boundary packet, WP-15 runtime inhabitant integration packet, and the observatory continuity/standing/access surfaces into one bounded v0.91.1 inhabited CSM Observatory proof while carrying an explicit feature-demo coverage roster for WP-02 through WP-16: citizen private-state continuity, witness and receipt evidence, redacted projection, lifecycle eligibility, authenticated local communication boundary, access-control denial, continuity challenge, sanctuary quarantine, runtime inhabitant integration, operator report, and room/lens walkthrough."
                    .to_string(),
            proof_classification: "proving".to_string(),
            non_claims: vec![
                "does not prove personhood".to_string(),
                "does not claim first true Godel-agent birthday".to_string(),
                "does not expose canonical private citizen state".to_string(),
                "does not implement unbounded live CSM execution".to_string(),
                "does not implement cross-polis federation or external transport semantics".to_string(),
            ],
            claim_boundary:
                "This packet proves the bounded local D12 v0.91.1 runtime inhabitant Observatory evidence package and that the artifact is reviewable as a unique continuation scenario; it does not prove personhood, a first true Godel-agent birthday, raw private-state inspection, or unbounded live Runtime v2 execution."
                    .to_string(),
        };
        packet.validate_against(
            challenge,
            operator,
            lifecycle,
            acip,
            a2a,
            runtime_inhabitant,
        )?;
        Ok(packet)
    }

    pub fn validate_against(
        &self,
        challenge: &RuntimeV2ContinuityChallengeArtifacts,
        operator: &RuntimeV2OperatorControlReport,
        lifecycle: &RuntimeV2AgentLifecycleArtifacts,
        acip: &RuntimeV2AcipHardeningPacket,
        a2a: &RuntimeV2A2aAdapterBoundaryPacket,
        runtime_inhabitant: &RuntimeV2RuntimeInhabitantIntegrationArtifacts,
    ) -> Result<()> {
        self.validate_shape()?;
        if !self.continuity_refs.iter().any(|artifact| {
            artifact
                == &challenge
                    .access_control_artifacts
                    .observatory_artifacts
                    .witness_artifacts
                    .witness_set
                    .artifact_path
        }) {
            return Err(anyhow!(
                "observatory flagship proof missing continuity witness ref"
            ));
        }
        if !self.continuity_refs.iter().any(|artifact| {
            artifact
                == &challenge
                    .access_control_artifacts
                    .observatory_artifacts
                    .witness_artifacts
                    .receipt_set
                    .artifact_path
        }) {
            return Err(anyhow!(
                "observatory flagship proof missing citizen receipt ref"
            ));
        }
        if !self.observatory_refs.iter().any(|artifact| {
            artifact
                == &challenge
                    .access_control_artifacts
                    .observatory_artifacts
                    .projection_packet
                    .artifact_path
        }) {
            return Err(anyhow!(
                "observatory flagship proof missing redacted projection packet ref"
            ));
        }
        if !self.standing_access_refs.iter().any(|artifact| {
            artifact
                == &challenge
                    .access_control_artifacts
                    .event_packet
                    .artifact_path
        }) {
            return Err(anyhow!(
                "observatory flagship proof missing access event ref"
            ));
        }
        if !self
            .lifecycle_refs
            .iter()
            .any(|artifact| artifact == &lifecycle.state_contract.artifact_path)
        {
            return Err(anyhow!(
                "observatory flagship proof missing lifecycle state contract ref"
            ));
        }
        if !self
            .lifecycle_refs
            .iter()
            .any(|artifact| artifact == &lifecycle.transition_matrix.artifact_path)
        {
            return Err(anyhow!(
                "observatory flagship proof missing lifecycle transition matrix ref"
            ));
        }
        if !self
            .communication_boundary_refs
            .iter()
            .any(|artifact| artifact == &acip.artifact_path)
        {
            return Err(anyhow!(
                "observatory flagship proof missing ACIP hardening ref"
            ));
        }
        if !self
            .communication_boundary_refs
            .iter()
            .any(|artifact| artifact == &a2a.artifact_path)
        {
            return Err(anyhow!(
                "observatory flagship proof missing A2A adapter boundary ref"
            ));
        }
        if !self
            .runtime_inhabitant_refs
            .iter()
            .any(|artifact| artifact == &runtime_inhabitant.packet.artifact_path)
        {
            return Err(anyhow!(
                "observatory flagship proof missing runtime inhabitant integration ref"
            ));
        }
        if !self
            .challenge_refs
            .iter()
            .any(|artifact| artifact == &challenge.challenge.artifact_path)
        {
            return Err(anyhow!(
                "observatory flagship proof missing continuity challenge ref"
            ));
        }
        if !self
            .operator_report_refs
            .iter()
            .any(|artifact| artifact == &operator.artifact_path)
        {
            return Err(anyhow!(
                "observatory flagship proof missing operator control report ref"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 Observatory flagship proof schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D12" {
            return Err(anyhow!(
                "observatory flagship proof must map to demo matrix row D12"
            ));
        }
        if self.milestone != "v0.91.1" {
            return Err(anyhow!("observatory flagship proof must target v0.91.1"));
        }
        normalize_id(self.proof_id.clone(), "observatory_flagship.proof_id")?;
        validate_relative_path(&self.artifact_path, "observatory_flagship.artifact_path")?;
        validate_relative_path(
            &self.operator_report_ref,
            "observatory_flagship.operator_report_ref",
        )?;
        validate_relative_path(
            &self.walkthrough_ref,
            "observatory_flagship.walkthrough_ref",
        )?;
        validate_relative_path_list(&self.source_docs, "observatory_flagship.source_docs")?;
        validate_relative_path_list(
            &self.required_artifact_refs,
            "observatory_flagship.required_artifact_refs",
        )?;
        validate_relative_path_list(
            &self.continuity_refs,
            "observatory_flagship.continuity_refs",
        )?;
        validate_relative_path_list(
            &self.observatory_refs,
            "observatory_flagship.observatory_refs",
        )?;
        validate_relative_path_list(&self.lifecycle_refs, "observatory_flagship.lifecycle_refs")?;
        validate_relative_path_list(
            &self.standing_access_refs,
            "observatory_flagship.standing_access_refs",
        )?;
        validate_relative_path_list(
            &self.communication_boundary_refs,
            "observatory_flagship.communication_boundary_refs",
        )?;
        validate_relative_path_list(
            &self.runtime_inhabitant_refs,
            "observatory_flagship.runtime_inhabitant_refs",
        )?;
        validate_relative_path_list(&self.challenge_refs, "observatory_flagship.challenge_refs")?;
        validate_relative_path_list(
            &self.operator_report_refs,
            "observatory_flagship.operator_report_refs",
        )?;
        validate_feature_demo_coverage(&self.feature_demo_coverage)?;
        if self.proof_classification != "proving" {
            return Err(anyhow!(
                "observatory flagship proof must be classified as proving"
            ));
        }
        validate_actor_roster(&self.actor_roster)?;
        validate_flagship_walkthrough(&self.lens_sequence)?;
        validate_required_flagship_refs(&self.required_artifact_refs)?;
        validate_nonempty_text(
            &self.reviewer_command,
            "observatory_flagship.reviewer_command",
        )?;
        if !self.reviewer_command.contains("observatory-flagship-demo") {
            return Err(anyhow!(
                "observatory flagship proof must include its runnable demo command"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_observatory_flagship"))
        {
            return Err(anyhow!(
                "observatory flagship proof must include the focused validation command"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("observatory-flagship-demo"))
        {
            return Err(anyhow!(
                "observatory flagship proof must include the runnable demo validation command"
            ));
        }
        for command in &self.validation_commands {
            validate_nonempty_text(command, "observatory_flagship.validation_commands")?;
        }
        validate_nonempty_text(&self.proof_summary, "observatory_flagship.proof_summary")?;
        for required_phrase in [
            "WP-03",
            "WP-13",
            "WP-14",
            "WP-16",
            "witness",
            "receipt",
            "redacted projection",
            "lifecycle",
            "communication boundary",
        ] {
            if !self.proof_summary.contains(required_phrase) {
                return Err(anyhow!(
                    "observatory flagship proof summary must mention {required_phrase}"
                ));
            }
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("personhood"))
        {
            return Err(anyhow!(
                "observatory flagship proof must preserve the personhood non-claim"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("first true Godel-agent birthday"))
        {
            return Err(anyhow!(
                "observatory flagship proof must preserve the first-birthday non-claim"
            ));
        }
        if !self
            .claim_boundary
            .contains("bounded local D12 v0.91.1 runtime inhabitant Observatory evidence package")
        {
            return Err(anyhow!(
                "observatory flagship proof must preserve its bounded D12 claim boundary"
            ));
        }
        Ok(())
    }
}

fn observatory_flagship_walkthrough(
    challenge: &RuntimeV2ContinuityChallengeArtifacts,
    operator: &RuntimeV2OperatorControlReport,
    lifecycle: &RuntimeV2AgentLifecycleArtifacts,
    acip: &RuntimeV2AcipHardeningPacket,
    a2a: &RuntimeV2A2aAdapterBoundaryPacket,
    runtime_inhabitant: &RuntimeV2RuntimeInhabitantIntegrationArtifacts,
) -> Result<Vec<RuntimeV2ObservatoryFlagshipWalkthroughStep>> {
    challenge.validate()?;
    operator.validate()?;
    lifecycle.validate()?;
    acip.validate()?;
    a2a.validate_against(acip, &crate::agent_comms::acip_a2a_fixture_set_v1())?;
    runtime_inhabitant.packet.validate()?;
    let access = &challenge.access_control_artifacts;
    let observatory = &access.observatory_artifacts;
    let witness = &observatory.witness_artifacts;
    Ok(vec![
        walkthrough_step(
            1,
            "World / Reality",
            "inhabited-roster",
            "citizen, guest, service, and operator roles are visible without raw private state",
            access
                .standing_artifacts
                .event_packet
                .artifact_path
                .as_str(),
            "who is present, and what standing do they have?",
            "standing is evidence, not identity transfer",
        ),
        walkthrough_step(
            2,
            "World / Reality",
            "continuity-witness",
            "single-lineage citizen state has witness and receipt evidence",
            witness.witness_set.artifact_path.as_str(),
            "why is proto-citizen-alpha still the same citizen?",
            "witnesses prove bounded transition continuity, not personhood",
        ),
        walkthrough_step(
            3,
            "Operator / Governance",
            "redacted-projection",
            "operator sees continuity state through a redacted Observatory packet",
            observatory.projection_packet.artifact_path.as_str(),
            "what may the operator inspect without violating private state?",
            "projection is not raw private-state authority",
        ),
        walkthrough_step(
            4,
            "Operator / Governance",
            "lifecycle-gate",
            "lifecycle state and transition evidence show whether ACIP messages may be received, queued, rejected, or invoked",
            lifecycle.state_contract.artifact_path.as_str(),
            "what lifecycle authority does this agent currently have?",
            "lifecycle evidence gates authority but does not prove consciousness or birthday",
        ),
        walkthrough_step(
            5,
            "Operator / Governance",
            "authenticated-local-comms",
            "authenticated local communication remains ACIP-bound and reviewable through the hardening packet",
            acip.artifact_path.as_str(),
            "how do we know local communication is authenticated and state-gated?",
            "ACIP packet proves bounded local comms policy, not external transport readiness",
        ),
        walkthrough_step(
            6,
            "Operator / Governance",
            "a2a-boundary",
            "A2A remains an adapter over ACIP rather than a second communication model",
            a2a.artifact_path.as_str(),
            "does A2A bypass the canonical local communication boundary?",
            "adapter evidence proves compatibility layering, not federation or new transport semantics",
        ),
        walkthrough_step(
            7,
            "Operator / Governance",
            "access-denial",
            "guest request for citizen-only authority is denied and evented",
            access.event_packet.artifact_path.as_str(),
            "can a guest silently acquire citizen rights?",
            "denial evidence proves refusal, not a general access-control system",
        ),
        walkthrough_step(
            8,
            "Operator / Governance",
            "sanctuary-quarantine",
            "ambiguous continuity is frozen into sanctuary/quarantine review",
            challenge
                .sanctuary_artifacts
                .quarantine_artifact
                .artifact_path
                .as_str(),
            "what happens when continuity is ambiguous?",
            "quarantine preserves evidence and blocks destructive transition",
        ),
        walkthrough_step(
            9,
            "Cognition / Internal State",
            "runtime-inhabitant-integration",
            "the integrated inhabitant packet binds standing, state, lifecycle, memory, capability, comms, learning, and observatory surfaces into one agent-shaped route",
            runtime_inhabitant.packet.artifact_path.as_str(),
            "where is the agent-shaped integration surface that pulls the earlier runtime proofs together?",
            "integration evidence proves bounded composition, not the full flagship by itself",
        ),
        walkthrough_step(
            10,
            "Cognition / Internal State",
            "challenge-and-appeal",
            "citizen challenge, freeze, and appeal remain reviewable without disclosure",
            challenge.challenge.artifact_path.as_str(),
            "how can a citizen contest continuity basis?",
            "challenge proof is procedural continuity evidence, not mind inspection",
        ),
        walkthrough_step(
            11,
            "Corporate Investor",
            "fallback-report",
            "non-specialist reviewer sees scope, risks, and non-claims in one report",
            RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH,
            "what does this demo prove and not prove?",
            "fallback report is a review surface, not runtime authority",
        ),
    ])
}

fn walkthrough_step(
    sequence: u32,
    room: &str,
    lens_or_memory_dot: &str,
    visible_surface: &str,
    artifact_ref: &str,
    continuity_question_answered: &str,
    proof_boundary: &str,
) -> RuntimeV2ObservatoryFlagshipWalkthroughStep {
    RuntimeV2ObservatoryFlagshipWalkthroughStep {
        schema_version: RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_SCHEMA.to_string(),
        sequence,
        room: room.to_string(),
        lens_or_memory_dot: lens_or_memory_dot.to_string(),
        visible_surface: visible_surface.to_string(),
        artifact_ref: artifact_ref.to_string(),
        continuity_question_answered: continuity_question_answered.to_string(),
        proof_boundary: proof_boundary.to_string(),
    }
}

#[allow(clippy::too_many_arguments)]
fn required_artifact_refs(
    continuity_refs: &[String],
    observatory_refs: &[String],
    lifecycle_refs: &[String],
    standing_access_refs: &[String],
    communication_boundary_refs: &[String],
    runtime_inhabitant_refs: &[String],
    challenge_refs: &[String],
    operator_report_refs: &[String],
) -> Vec<String> {
    let mut seen = BTreeSet::new();
    continuity_refs
        .iter()
        .chain(observatory_refs)
        .chain(lifecycle_refs)
        .chain(standing_access_refs)
        .chain(communication_boundary_refs)
        .chain(runtime_inhabitant_refs)
        .chain(challenge_refs)
        .chain(operator_report_refs)
        .filter(|artifact| seen.insert((*artifact).as_str().to_string()))
        .cloned()
        .collect()
}

fn feature_demo_coverage(
    standing_access_refs: &[String],
    challenge: &RuntimeV2ContinuityChallengeArtifacts,
    operator: &RuntimeV2OperatorControlReport,
) -> Vec<RuntimeV2ObservatoryFeatureDemoCoverage> {
    vec![
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "runtime-polis-architecture".to_string(),
            feature_name: "Runtime/polis architecture".to_string(),
            owning_wp: "WP-02".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/RUNTIME_POLIS_ARCHITECTURE.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "docs/milestones/v0.91.1/RUNTIME_POLIS_ARCHITECTURE_PACKAGE_v0.91.1.md".to_string(),
                "docs/milestones/v0.91.1/DEMO_MATRIX_v0.91.1.md".to_string(),
            ],
            coverage_summary: "Architecture inspection demo proves the runtime/polis package and artifact layout stay aligned.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "agent-lifecycle-state-model".to_string(),
            feature_name: "Agent lifecycle state model".to_string(),
            owning_wp: "WP-03".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/AGENT_LIFECYCLE_STATE_MODEL.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "runtime_v2/agent_lifecycle/state_contract.json".to_string(),
                "runtime_v2/agent_lifecycle/transition_matrix.json".to_string(),
            ],
            coverage_summary: "Lifecycle state demo proves receipt, queue, reject, and invoke eligibility remain explicit.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "csm-observatory-active-surface".to_string(),
            feature_name: "CSM observatory active surface".to_string(),
            owning_wp: "WP-04".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/CSM_OBSERVATORY_ACTIVE_SURFACE.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "runtime_v2/observatory/visibility_packet.json".to_string(),
                "runtime_v2/observatory/operator_report.md".to_string(),
            ],
            coverage_summary: "Observatory active packet demo proves operator-visible projection and redaction without raw state leakage.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "citizen-standing-model".to_string(),
            feature_name: "Citizen standing".to_string(),
            owning_wp: "WP-05".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/CITIZEN_STANDING_MODEL.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "runtime_v2/standing/standing_transitions.json".to_string(),
                "runtime_v2/standing/standing_events.json".to_string(),
            ],
            coverage_summary: "Standing demo proves mediated authority transitions and denied escalation paths.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "citizen-state-substrate".to_string(),
            feature_name: "Citizen state".to_string(),
            owning_wp: "WP-06".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/CITIZEN_STATE_SUBSTRATE.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "runtime_v2/citizen_state/citizen_state_substrate.json".to_string(),
                "runtime_v2/private_state/private_state_observatory_proof.json".to_string(),
            ],
            coverage_summary: "Citizen-state demo proves stale-state awareness, private-state boundaries, and safe projection.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "memory-identity-architecture".to_string(),
            feature_name: "Memory/identity architecture".to_string(),
            owning_wp: "WP-07".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/MEMORY_IDENTITY_ARCHITECTURE.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "runtime_v2/memory_identity/memory_identity_architecture.json".to_string(),
                "runtime_v2/private_state/continuity_witnesses.json".to_string(),
            ],
            coverage_summary: "Memory demo proves witness-backed continuity and observatory-linked identity state.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "theory-of-mind-foundation".to_string(),
            feature_name: "Theory of Mind foundation".to_string(),
            owning_wp: "WP-08".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/THEORY_OF_MIND_FOUNDATION.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "runtime_v2/theory_of_mind/theory_of_mind_foundation.json".to_string(),
                "runtime_v2/memory_identity/memory_identity_architecture.json".to_string(),
            ],
            coverage_summary: "ToM demo proves bounded agent-model updates from explicit evidence rather than spoofed mind-reading.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "capability-aptitude-testing".to_string(),
            feature_name: "Capability/aptitude testing".to_string(),
            owning_wp: "WP-09".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/CAPABILITY_APTITUDE_TESTING.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture/scorecard.json".to_string(),
                "docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture/final_report.md".to_string(),
            ],
            coverage_summary: "Capability demo proves fixture-mode execution and bounded internal evaluation with explicit limitations.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "intelligence-metric-architecture".to_string(),
            feature_name: "Intelligence metric architecture".to_string(),
            owning_wp: "WP-10".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/INTELLIGENCE_METRIC_ARCHITECTURE.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "runtime_v2/intelligence/intelligence_metric_architecture.json".to_string(),
                "docs/milestones/v0.91.1/review/intelligence_metric_architecture_fixture/scorecard.json".to_string(),
            ],
            coverage_summary: "Intelligence demo proves evidence-bound metrics layered over capability and ToM artifacts.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "governed-learning-substrate".to_string(),
            feature_name: "Governed learning substrate".to_string(),
            owning_wp: "WP-11".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/GOVERNED_LEARNING_SUBSTRATE.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "runtime_v2/learning/governed_learning_substrate.json".to_string(),
                "docs/milestones/v0.91.1/review/governed_learning_fixture/accepted_feedback_update.json".to_string(),
            ],
            coverage_summary: "Governed learning demo proves accepted, rejected, and rollback-aware update boundaries.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "anrm-gemma-placement".to_string(),
            feature_name: "ANRM/Gemma placement".to_string(),
            owning_wp: "WP-12".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/ANRM_GEMMA_PLACEMENT.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_dataset.json".to_string(),
                "docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_extractor_spec.json".to_string(),
            ],
            coverage_summary: "ANRM/Gemma demo proves deterministic trace extraction and dataset/spec parity.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "acip-hardening".to_string(),
            feature_name: "ACIP hardening".to_string(),
            owning_wp: "WP-13".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/ACIP_HARDENING.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "runtime_v2/acip/acip_hardening_packet.json".to_string(),
                "runtime_v2/access_control/access_events.json".to_string(),
            ],
            coverage_summary: "ACIP hardening demo proves authenticated local communication remains state-gated and reviewable.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "a2a-adapter-boundary".to_string(),
            feature_name: "A2A adapter boundary".to_string(),
            owning_wp: "WP-14".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/A2A_ADAPTER_BOUNDARY.md".to_string(),
            demo_mode: "dedicated_demo".to_string(),
            demo_surface_refs: vec![
                "runtime_v2/acip/a2a_adapter_boundary_packet.json".to_string(),
                "runtime_v2/acip/acip_hardening_packet.json".to_string(),
            ],
            coverage_summary: "A2A adapter demo proves compatibility stays layered over ACIP rather than becoming a second transport model.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "runtime-inhabitant-proof".to_string(),
            feature_name: "Runtime inhabitant proof".to_string(),
            owning_wp: "WP-15".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/RUNTIME_INHABITANT_PROOF.md".to_string(),
            demo_mode: "integrated_demo_dependency".to_string(),
            demo_surface_refs: vec![
                "runtime_v2/inhabitant/runtime_inhabitant_integration_packet.json".to_string(),
                "runtime_v2/inhabitant/runtime_inhabitant_operator_report.md".to_string(),
            ],
            coverage_summary: "WP-15 integrates standing, state, lifecycle, memory, capability, learning, access, and comms into one agent-shaped proof surface.".to_string(),
        },
        RuntimeV2ObservatoryFeatureDemoCoverage {
            feature_id: "observatory-visible-flagship-demo".to_string(),
            feature_name: "Observatory-visible flagship demo".to_string(),
            owning_wp: "WP-16".to_string(),
            feature_doc_ref: "docs/milestones/v0.91.1/features/RUNTIME_INHABITANT_PROOF.md".to_string(),
            demo_mode: "flagship_demo".to_string(),
            demo_surface_refs: vec![
                RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_PATH.to_string(),
                RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH.to_string(),
                RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_PATH.to_string(),
                challenge.challenge.artifact_path.clone(),
                operator.artifact_path.clone(),
            ],
            coverage_summary: format!(
                "D12 flagship demo proves the inhabited observatory route and explicitly aggregates the earlier feature demos through {} standing/access refs.",
                standing_access_refs.len()
            ),
        },
    ]
}
