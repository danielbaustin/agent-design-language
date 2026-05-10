//! Runtime-v2 A2A adapter boundary packet for v0.91.1.
//!
//! This packet lifts the existing ACIP A2A adapter fixture surface into a
//! bounded runtime-facing proof artifact so Sprint 3 can prove that A2A remains
//! an adapter over ACIP rather than a second communication model.

use crate::agent_comms::{
    acip_a2a_fixture_set_v1, validate_acip_a2a_fixture_set_v1, AcipA2aFixtureSetV1,
    AcipA2aTrustClassV1,
};
use std::path::Path;

use super::*;

pub const RUNTIME_V2_A2A_ADAPTER_BOUNDARY_PACKET_SCHEMA: &str =
    "runtime_v2.a2a_adapter_boundary_packet.v1";
pub const RUNTIME_V2_A2A_ADAPTER_BOUNDARY_PACKET_PATH: &str =
    "runtime_v2/acip/a2a_adapter_boundary_packet.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2A2aAdapterLane {
    pub adapter_id: String,
    pub trust_classification: String,
    pub agent_card_ref: String,
    pub external_capability_claim: String,
    pub adl_capability: String,
    pub policy_basis_ref: String,
    pub required_entrypoint: String,
    pub invocation_contract_schema_ref: String,
    pub trace_bundle_schema_ref: String,
    pub local_scope_only: bool,
    pub external_transport_status: String,
    pub refusal_mode: String,
    pub trace_evidence_refs: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2A2aNegativeCase {
    pub case_id: String,
    pub expected_error_substring: String,
    pub proves: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2A2aAdapterBoundaryPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub acip_hardening_ref: String,
    pub adapter_fixture_schema_ref: String,
    pub invocation_contract_schema_ref: String,
    pub trace_bundle_schema_ref: String,
    pub adapter_lanes: Vec<RuntimeV2A2aAdapterLane>,
    pub negative_cases: Vec<RuntimeV2A2aNegativeCase>,
    pub validation_commands: Vec<String>,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

pub fn runtime_v2_a2a_adapter_boundary_packet() -> Result<RuntimeV2A2aAdapterBoundaryPacket> {
    RuntimeV2A2aAdapterBoundaryPacket::prototype()
}

impl RuntimeV2A2aAdapterBoundaryPacket {
    pub fn prototype() -> Result<Self> {
        let acip = runtime_v2_acip_hardening_contract()?;
        let fixtures = acip_a2a_fixture_set_v1();
        validate_acip_a2a_fixture_set_v1(&fixtures)?;

        let packet = Self {
            schema_version: RUNTIME_V2_A2A_ADAPTER_BOUNDARY_PACKET_SCHEMA.to_string(),
            packet_id: "a2a-adapter-boundary-v0-91-1-wp-14".to_string(),
            milestone: "v0.91.1".to_string(),
            wp: "WP-14".to_string(),
            artifact_path: RUNTIME_V2_A2A_ADAPTER_BOUNDARY_PACKET_PATH.to_string(),
            source_feature_doc: "docs/milestones/v0.91.1/features/A2A_ADAPTER_BOUNDARY.md"
                .to_string(),
            acip_hardening_ref: acip.artifact_path.clone(),
            adapter_fixture_schema_ref: "acip.a2a.fixture.v1".to_string(),
            invocation_contract_schema_ref: "acip.invocation.contract.v1".to_string(),
            trace_bundle_schema_ref: "acip.trace.bundle.v1".to_string(),
            adapter_lanes: expected_adapter_lanes(&fixtures)?,
            negative_cases: expected_negative_cases(&fixtures)?,
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_a2a_adapter_boundary -- --nocapture"
                    .to_string(),
                "cargo test --manifest-path adl/Cargo.toml acip_a2a_adapter -- --nocapture"
                    .to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_acip_hardening -- --nocapture"
                    .to_string(),
                "git diff --check".to_string(),
            ],
            non_claims: vec![
                "does not prove external federation or cross-polis routing".to_string(),
                "does not create a second canonical internal communication model".to_string(),
                "does not grant execution authority from Agent Card claims".to_string(),
                "does not bypass ACIP, ACC, lifecycle, trace, or redaction boundaries"
                    .to_string(),
            ],
            claim_boundary: "WP-14 proves one bounded A2A-over-ACIP adapter boundary packet over the landed WP-13 ACIP hardening surface and the existing ACIP A2A fixture layer. It proves that A2A capability claims stay identity-bound, route through agent.invoke, preserve local-only transport, and remain denied for direct execution or external transport in v0.91.1. It does not prove external federation, transport readiness, or a second canonical communication architecture.".to_string(),
        };
        packet.validate_against(&acip, &fixtures)?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_A2A_ADAPTER_BOUNDARY_PACKET_SCHEMA {
            return Err(anyhow!(
                "unsupported A2A adapter boundary schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.packet_id.clone(), "a2a_adapter_boundary.packet_id")?;
        if self.milestone != "v0.91.1" {
            return Err(anyhow!(
                "A2A adapter boundary packet must target milestone v0.91.1"
            ));
        }
        if self.wp != "WP-14" {
            return Err(anyhow!(
                "A2A adapter boundary packet must remain bound to WP-14"
            ));
        }
        validate_relative_path(&self.artifact_path, "a2a_adapter_boundary.artifact_path")?;
        if self.source_feature_doc != "docs/milestones/v0.91.1/features/A2A_ADAPTER_BOUNDARY.md" {
            return Err(anyhow!(
                "A2A adapter boundary packet must point at the v0.91.1 feature doc"
            ));
        }
        validate_relative_path(
            &self.source_feature_doc,
            "a2a_adapter_boundary.source_feature_doc",
        )?;
        if self.acip_hardening_ref != RUNTIME_V2_ACIP_HARDENING_PACKET_PATH {
            return Err(anyhow!(
                "A2A adapter boundary packet must bind the landed WP-13 ACIP hardening packet"
            ));
        }
        if self.adapter_fixture_schema_ref != "acip.a2a.fixture.v1" {
            return Err(anyhow!(
                "A2A adapter boundary packet must preserve the ACIP A2A fixture schema binding"
            ));
        }
        if self.invocation_contract_schema_ref != "acip.invocation.contract.v1" {
            return Err(anyhow!(
                "A2A adapter boundary packet must preserve the ACIP invocation schema binding"
            ));
        }
        if self.trace_bundle_schema_ref != "acip.trace.bundle.v1" {
            return Err(anyhow!(
                "A2A adapter boundary packet must preserve the ACIP trace bundle binding"
            ));
        }
        validate_adapter_lanes(&self.adapter_lanes)?;
        validate_a2a_negative_cases(&self.negative_cases)?;
        let expected_commands = [
            "cargo test --manifest-path adl/Cargo.toml runtime_v2_a2a_adapter_boundary -- --nocapture",
            "cargo test --manifest-path adl/Cargo.toml acip_a2a_adapter -- --nocapture",
            "cargo test --manifest-path adl/Cargo.toml runtime_v2_acip_hardening -- --nocapture",
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
                "A2A adapter boundary packet must preserve the reviewed focused validation command set"
            ));
        }
        if !self
            .claim_boundary
            .contains("does not prove external federation")
        {
            return Err(anyhow!(
                "A2A adapter boundary packet must preserve its external-federation non-claim boundary"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("second canonical internal communication model"))
        {
            return Err(anyhow!(
                "A2A adapter boundary packet must preserve the one-communication-model non-claim"
            ));
        }
        Ok(())
    }

    pub fn validate_against(
        &self,
        acip: &RuntimeV2AcipHardeningPacket,
        fixtures: &AcipA2aFixtureSetV1,
    ) -> Result<()> {
        self.validate()?;
        acip.validate()?;
        validate_acip_a2a_fixture_set_v1(fixtures)?;

        if self.acip_hardening_ref != acip.artifact_path {
            return Err(anyhow!(
                "A2A adapter boundary packet must remain bound to the landed ACIP hardening packet"
            ));
        }
        if self.adapter_lanes != expected_adapter_lanes(fixtures)? {
            return Err(anyhow!(
                "A2A adapter boundary packet adapter_lanes drifted from the reviewed ACIP A2A fixture set"
            ));
        }
        if self.negative_cases != expected_negative_cases(fixtures)? {
            return Err(anyhow!(
                "A2A adapter boundary packet negative_cases drifted from the reviewed ACIP A2A fixture set"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize A2A adapter boundary packet")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let output = root
            .as_ref()
            .join(RUNTIME_V2_A2A_ADAPTER_BOUNDARY_PACKET_PATH);
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create {}", parent.display()))?;
        }
        std::fs::write(&output, self.pretty_json_bytes()?)
            .with_context(|| format!("write {}", output.display()))?;
        Ok(())
    }
}

fn expected_adapter_lanes(fixtures: &AcipA2aFixtureSetV1) -> Result<Vec<RuntimeV2A2aAdapterLane>> {
    let mut lanes = Vec::with_capacity(fixtures.valid_adapters.len());
    for adapter in &fixtures.valid_adapters {
        let mapping = adapter
            .capability_mappings
            .first()
            .ok_or_else(|| anyhow!("A2A valid adapter must contain one capability mapping"))?;
        lanes.push(RuntimeV2A2aAdapterLane {
            adapter_id: adapter.adapter_id.clone(),
            trust_classification: trust_class_name(&adapter.trust_classification.classification),
            agent_card_ref: adapter.identity_claim.agent_card_ref.clone(),
            external_capability_claim: mapping.external_capability_claim.clone(),
            adl_capability: mapping.adl_capability.clone(),
            policy_basis_ref: mapping.policy_basis_ref.clone(),
            required_entrypoint: adapter.invoke_boundary.required_entrypoint.clone(),
            invocation_contract_schema_ref: adapter
                .invoke_boundary
                .invocation_contract_schema_ref
                .clone(),
            trace_bundle_schema_ref: adapter.invoke_boundary.trace_bundle_schema_ref.clone(),
            local_scope_only: adapter.transport_boundary.local_scope_only,
            external_transport_status: "deferred_until_tls_equivalent".to_string(),
            refusal_mode: adapter.transport_boundary.refusal_mode.clone(),
            trace_evidence_refs: adapter.trace_evidence_refs.clone(),
            non_claims: adapter.non_claims.clone(),
        });
    }
    Ok(lanes)
}

fn expected_negative_cases(
    fixtures: &AcipA2aFixtureSetV1,
) -> Result<Vec<RuntimeV2A2aNegativeCase>> {
    let proves_map = [
        (
            "parallel_authority_model",
            "A2A identity or capability claims cannot create parallel execution authority.",
        ),
        (
            "direct_execution_grant",
            "A2A capability claims must route through agent.invoke and cannot directly execute work.",
        ),
        (
            "missing_agent_invoke_boundary",
            "A2A adapters must preserve the agent.invoke boundary over ACIP invocation contracts.",
        ),
        (
            "external_transport_without_local_gate",
            "A2A external transport remains explicitly deferred until TLS-equivalent protection exists.",
        ),
    ];
    fixtures
        .negative_cases
        .iter()
        .map(|case| {
            let proves = proves_map
                .iter()
                .find(|(name, _)| *name == case.name)
                .map(|(_, proves)| proves.to_string())
                .ok_or_else(|| anyhow!("unexpected A2A negative case '{}'", case.name))?;
            Ok(RuntimeV2A2aNegativeCase {
                case_id: case.name.clone(),
                expected_error_substring: case.expected_error_substring.clone(),
                proves,
            })
        })
        .collect()
}

fn validate_adapter_lanes(lanes: &[RuntimeV2A2aAdapterLane]) -> Result<()> {
    if lanes.len() != 3 {
        return Err(anyhow!(
            "A2A adapter boundary packet must preserve exactly three reviewed adapter lanes"
        ));
    }
    let expected_trust = ["naked", "guest", "citizen"];
    for (lane, expected) in lanes.iter().zip(expected_trust) {
        normalize_id(
            lane.adapter_id.clone(),
            "a2a_adapter_boundary.adapter_lanes[].adapter_id",
        )?;
        if lane.trust_classification != expected {
            return Err(anyhow!(
                "A2A adapter boundary packet adapter_lanes must remain in reviewed trust-class order"
            ));
        }
        validate_relative_path(
            &lane.agent_card_ref,
            "a2a_adapter_boundary.adapter_lanes[].agent_card_ref",
        )?;
        normalize_id(
            lane.external_capability_claim.clone(),
            "a2a_adapter_boundary.adapter_lanes[].external_capability_claim",
        )?;
        normalize_id(
            lane.adl_capability.clone(),
            "a2a_adapter_boundary.adapter_lanes[].adl_capability",
        )?;
        validate_relative_path(
            &lane.policy_basis_ref,
            "a2a_adapter_boundary.adapter_lanes[].policy_basis_ref",
        )?;
        if lane.required_entrypoint != "agent.invoke" {
            return Err(anyhow!("A2A adapter lanes must route through agent.invoke"));
        }
        if lane.invocation_contract_schema_ref != "acip.invocation.contract.v1" {
            return Err(anyhow!(
                "A2A adapter lanes must preserve ACIP invocation schema binding"
            ));
        }
        if lane.trace_bundle_schema_ref != "acip.trace.bundle.v1" {
            return Err(anyhow!(
                "A2A adapter lanes must preserve ACIP trace bundle schema binding"
            ));
        }
        if !lane.local_scope_only {
            return Err(anyhow!(
                "A2A adapter lanes must remain local-scope only in v0.91.1"
            ));
        }
        if lane.external_transport_status != "deferred_until_tls_equivalent" {
            return Err(anyhow!(
                "A2A adapter lanes must keep external transport deferred until TLS-equivalent protection exists"
            ));
        }
        if lane.refusal_mode != "refuse" {
            return Err(anyhow!(
                "A2A adapter lanes must fail closed for unsupported external transport"
            ));
        }
        if lane.trace_evidence_refs.is_empty() {
            return Err(anyhow!(
                "A2A adapter lanes must preserve trace evidence references"
            ));
        }
        for evidence_ref in &lane.trace_evidence_refs {
            validate_relative_path(
                evidence_ref,
                "a2a_adapter_boundary.adapter_lanes[].trace_evidence_refs[]",
            )?;
        }
        if lane.non_claims.len() < 2 {
            return Err(anyhow!(
                "A2A adapter lanes must preserve non-claim coverage"
            ));
        }
    }
    Ok(())
}

fn validate_a2a_negative_cases(cases: &[RuntimeV2A2aNegativeCase]) -> Result<()> {
    let expected = [
        "parallel_authority_model",
        "direct_execution_grant",
        "missing_agent_invoke_boundary",
        "external_transport_without_local_gate",
    ];
    if cases.len() != expected.len() {
        return Err(anyhow!(
            "A2A adapter boundary packet must preserve exactly four reviewed negative cases"
        ));
    }
    for (case, expected_name) in cases.iter().zip(expected) {
        normalize_id(
            case.case_id.clone(),
            "a2a_adapter_boundary.negative_cases[].case_id",
        )?;
        if case.case_id != expected_name {
            return Err(anyhow!(
                "A2A adapter boundary packet negative cases must remain in reviewed order"
            ));
        }
        validate_nonempty_text(
            &case.expected_error_substring,
            "a2a_adapter_boundary.negative_cases[].expected_error_substring",
        )?;
        validate_nonempty_text(&case.proves, "a2a_adapter_boundary.negative_cases[].proves")?;
    }
    Ok(())
}

fn trust_class_name(trust: &AcipA2aTrustClassV1) -> String {
    match trust {
        AcipA2aTrustClassV1::Naked => "naked".to_string(),
        AcipA2aTrustClassV1::Guest => "guest".to_string(),
        AcipA2aTrustClassV1::Citizen => "citizen".to_string(),
    }
}
