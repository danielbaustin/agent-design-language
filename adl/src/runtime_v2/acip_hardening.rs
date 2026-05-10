//! Runtime-v2 ACIP hardening packet for v0.91.1.
//!
//! This packet binds the existing ACIP conformance surface to the landed
//! lifecycle and citizen-state contracts so local intra-polis communication is
//! explicitly authenticated, redacted, policy-bound, and fail-closed.

use crate::agent_comms::{acip_conformance_report_v1, validate_acip_conformance_report_v1};
use std::path::Path;

use super::*;

pub const RUNTIME_V2_ACIP_HARDENING_PACKET_SCHEMA: &str = "runtime_v2.acip_hardening_packet.v1";
pub const RUNTIME_V2_ACIP_HARDENING_PACKET_PATH: &str =
    "runtime_v2/acip/acip_hardening_packet.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AcipEnvelopeProfile {
    pub profile_id: String,
    pub channel_scope: String,
    pub authentication_mode: String,
    pub signature_policy: String,
    pub encryption_mode: String,
    pub redaction_mode: String,
    pub external_transport_policy: String,
    pub authority_gate_refs: Vec<String>,
    pub allowed_key_sources: Vec<String>,
    pub non_exportable_keys_required: bool,
    pub evidence_trace_required: bool,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AcipNegativeCase {
    pub case_id: String,
    pub rejection_class: String,
    pub source_fixture_name: String,
    pub expected_error_substring: String,
    pub proves: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AcipStateSpecificCase {
    pub case_id: String,
    pub state: String,
    pub fixture_basis: String,
    pub receipt_policy: String,
    pub invocation_policy: String,
    pub authorized_wake_trigger: String,
    pub queue_or_custody_policy: String,
    pub redaction_view: String,
    pub external_commitment_allowed: bool,
    pub allowed_message_kinds: Vec<String>,
    pub blocked_message_kinds: Vec<String>,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AcipHardeningPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub lifecycle_state_contract_ref: String,
    pub lifecycle_transition_matrix_ref: String,
    pub citizen_state_ref: String,
    pub envelope_profile: RuntimeV2AcipEnvelopeProfile,
    pub negative_cases: Vec<RuntimeV2AcipNegativeCase>,
    pub state_cases: Vec<RuntimeV2AcipStateSpecificCase>,
    pub validation_commands: Vec<String>,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

pub fn runtime_v2_acip_hardening_packet() -> Result<RuntimeV2AcipHardeningPacket> {
    RuntimeV2AcipHardeningPacket::prototype()
}

impl RuntimeV2AcipHardeningPacket {
    pub fn prototype() -> Result<Self> {
        let lifecycle = runtime_v2_agent_lifecycle_state_contract()?;
        let citizen_state = runtime_v2_citizen_state_substrate_contract()?;
        let access = runtime_v2_access_control_contract()?;
        let observatory = runtime_v2_private_state_observatory_contract()?;
        let conformance_report = acip_conformance_report_v1();
        validate_acip_conformance_report_v1(&conformance_report)?;

        let packet = Self {
            schema_version: RUNTIME_V2_ACIP_HARDENING_PACKET_SCHEMA.to_string(),
            packet_id: "acip-hardening-v0-91-1-wp-13".to_string(),
            milestone: "v0.91.1".to_string(),
            wp: "WP-13".to_string(),
            artifact_path: RUNTIME_V2_ACIP_HARDENING_PACKET_PATH.to_string(),
            source_feature_doc: "docs/milestones/v0.91.1/features/ACIP_HARDENING.md".to_string(),
            lifecycle_state_contract_ref: lifecycle.state_contract.artifact_path.clone(),
            lifecycle_transition_matrix_ref: lifecycle.transition_matrix.artifact_path.clone(),
            citizen_state_ref: citizen_state.artifact_path.clone(),
            envelope_profile: expected_envelope_profile(&access, &observatory),
            negative_cases: expected_negative_cases(&conformance_report, &observatory)?,
            state_cases: expected_state_cases(&lifecycle)?,
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_acip_hardening -- --nocapture"
                    .to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_agent_lifecycle_state -- --nocapture"
                    .to_string(),
                "cargo test --manifest-path adl/Cargo.toml acip_conformance_report -- --nocapture"
                    .to_string(),
                "git diff --check".to_string(),
            ],
            non_claims: vec![
                "does not prove cross-polis or external transport security".to_string(),
                "does not implement live TLS, mutual-TLS, or remote key exchange".to_string(),
                "does not bypass lifecycle, Freedom Gate, ACC, trace, or redaction boundaries"
                    .to_string(),
                "does not prove v0.92 federation, identity rebinding, or birthday completion"
                    .to_string(),
            ],
            claim_boundary: "WP-13 proves one bounded local ACIP hardening packet over the landed lifecycle, citizen-state, access-control, observatory, and ACIP conformance surfaces. It proves authenticated local envelope policy, fail-closed rejection classes, and state-specific routing semantics for intra-polis communication only. It does not prove external transport, live cryptographic exchange, federation, or v0.92 identity semantics.".to_string(),
        };
        packet.validate_against(
            &lifecycle,
            &citizen_state,
            &access,
            &observatory,
            &conformance_report,
        )?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_ACIP_HARDENING_PACKET_SCHEMA {
            return Err(anyhow!(
                "unsupported ACIP hardening schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.packet_id.clone(), "acip_hardening.packet_id")?;
        if self.milestone != "v0.91.1" {
            return Err(anyhow!(
                "ACIP hardening packet must target milestone v0.91.1"
            ));
        }
        if self.wp != "WP-13" {
            return Err(anyhow!("ACIP hardening packet must remain bound to WP-13"));
        }
        validate_relative_path(&self.artifact_path, "acip_hardening.artifact_path")?;
        if self.source_feature_doc != "docs/milestones/v0.91.1/features/ACIP_HARDENING.md" {
            return Err(anyhow!(
                "ACIP hardening packet must point at the v0.91.1 feature doc"
            ));
        }
        validate_relative_path(
            &self.source_feature_doc,
            "acip_hardening.source_feature_doc",
        )?;
        validate_relative_path(
            &self.lifecycle_state_contract_ref,
            "acip_hardening.lifecycle_state_contract_ref",
        )?;
        validate_relative_path(
            &self.lifecycle_transition_matrix_ref,
            "acip_hardening.lifecycle_transition_matrix_ref",
        )?;
        validate_relative_path(&self.citizen_state_ref, "acip_hardening.citizen_state_ref")?;
        validate_envelope_profile(&self.envelope_profile)?;
        validate_negative_cases(&self.negative_cases)?;
        validate_state_cases(&self.state_cases)?;
        let expected_commands = [
            "cargo test --manifest-path adl/Cargo.toml runtime_v2_acip_hardening -- --nocapture",
            "cargo test --manifest-path adl/Cargo.toml runtime_v2_agent_lifecycle_state -- --nocapture",
            "cargo test --manifest-path adl/Cargo.toml acip_conformance_report -- --nocapture",
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
                "ACIP hardening packet must preserve the reviewed focused validation command set"
            ));
        }
        if !self
            .claim_boundary
            .contains("It does not prove external transport")
        {
            return Err(anyhow!(
                "ACIP hardening packet must preserve its external-transport non-claim boundary"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("external transport"))
        {
            return Err(anyhow!(
                "ACIP hardening packet must preserve the external-transport non-claim"
            ));
        }
        Ok(())
    }

    pub fn validate_against(
        &self,
        lifecycle: &RuntimeV2AgentLifecycleArtifacts,
        citizen_state: &RuntimeV2CitizenStateSubstratePacket,
        access: &RuntimeV2AccessControlArtifacts,
        observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
        conformance_report: &crate::agent_comms::AcipConformanceReportV1,
    ) -> Result<()> {
        self.validate()?;
        lifecycle.validate()?;
        citizen_state.validate()?;
        access.validate()?;
        observatory.validate()?;
        validate_acip_conformance_report_v1(conformance_report)?;

        if self.lifecycle_state_contract_ref != lifecycle.state_contract.artifact_path {
            return Err(anyhow!(
                "ACIP hardening lifecycle_state_contract_ref drifted from the landed lifecycle packet"
            ));
        }
        if self.lifecycle_transition_matrix_ref != lifecycle.transition_matrix.artifact_path {
            return Err(anyhow!(
                "ACIP hardening lifecycle_transition_matrix_ref drifted from the landed lifecycle packet"
            ));
        }
        if self.citizen_state_ref != citizen_state.artifact_path {
            return Err(anyhow!(
                "ACIP hardening citizen_state_ref must bind the landed citizen-state substrate"
            ));
        }
        if self.envelope_profile != expected_envelope_profile(access, observatory) {
            return Err(anyhow!(
                "ACIP hardening envelope_profile must remain aligned with access-control and observatory redaction surfaces"
            ));
        }
        if self.negative_cases != expected_negative_cases(conformance_report, observatory)? {
            return Err(anyhow!(
                "ACIP hardening negative_cases must remain aligned with canonical ACIP and observatory rejection routes"
            ));
        }
        if self.state_cases != expected_state_cases(lifecycle)? {
            return Err(anyhow!(
                "ACIP hardening state_cases must remain aligned with the landed lifecycle state matrix"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize ACIP hardening packet")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.pretty_json_bytes()?,
        )
    }
}

fn expected_envelope_profile(
    access: &RuntimeV2AccessControlArtifacts,
    observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
) -> RuntimeV2AcipEnvelopeProfile {
    RuntimeV2AcipEnvelopeProfile {
        profile_id: "local-intra-polis-authenticated-envelope".to_string(),
        channel_scope: "local_intra_polis_only".to_string(),
        authentication_mode: "required_identity_bound_sender_and_recipient".to_string(),
        signature_policy: "unsigned_or_unattested_messages_fail_closed".to_string(),
        encryption_mode: "authenticated_local_envelope_or_encryption_ready_only".to_string(),
        redaction_mode: "deterministic_redaction_before_public_or_operator_projection"
            .to_string(),
        external_transport_policy:
            "forbidden_without_tls_or_mutual_tls_equivalent_protection".to_string(),
        authority_gate_refs: vec![
            access.authority_matrix.artifact_path.clone(),
            access.denial_fixtures.event_packet_ref.clone(),
            observatory.redaction_policy.artifact_path.clone(),
        ],
        allowed_key_sources: vec![
            "runtime_local_keystore".to_string(),
            "operator_reviewed_recovery_bundle".to_string(),
        ],
        non_exportable_keys_required: true,
        evidence_trace_required: true,
        claim_boundary:
            "This profile proves local authenticated-envelope policy only; it does not claim remote transport safety or a complete cryptographic implementation."
                .to_string(),
    }
}

fn expected_negative_cases(
    report: &crate::agent_comms::AcipConformanceReportV1,
    observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
) -> Result<Vec<RuntimeV2AcipNegativeCase>> {
    Ok(vec![
        RuntimeV2AcipNegativeCase {
            case_id: "malformed_local_envelope".to_string(),
            rejection_class: "malformed".to_string(),
            source_fixture_name: "malformed_payload_refs".to_string(),
            expected_error_substring: negative_case_error(report, "malformed_payload_refs")?,
            proves:
                "Malformed local ACIP envelopes fail closed before invocation, persistence, or trace ambiguity."
                    .to_string(),
            evidence_refs: vec!["adl/src/agent_comms/orchestrate/conformance.inc".to_string()],
        },
        RuntimeV2AcipNegativeCase {
            case_id: "unsigned_local_message".to_string(),
            rejection_class: "unsigned".to_string(),
            source_fixture_name: "signature_policy".to_string(),
            expected_error_substring: "unsigned_local_message".to_string(),
            proves:
                "Unsigned or unattested local ACIP messages are rejected before state routing or wake eligibility is considered."
                    .to_string(),
            evidence_refs: vec![
                "adl/src/runtime_v2/acip_hardening.rs".to_string(),
                "docs/milestones/v0.91.1/features/ACIP_HARDENING.md".to_string(),
            ],
        },
        RuntimeV2AcipNegativeCase {
            case_id: "unauthorized_invocation_attempt".to_string(),
            rejection_class: "unauthorized".to_string(),
            source_fixture_name: "authority_escalation".to_string(),
            expected_error_substring: negative_case_error(report, "authority_escalation")?,
            proves:
                "Authority-escalating or gate-missing invocation attempts are rejected before any state can smuggle agency."
                    .to_string(),
            evidence_refs: vec!["adl/src/agent_comms/orchestrate/conformance.inc".to_string()],
        },
        RuntimeV2AcipNegativeCase {
            case_id: "overexposed_projection_attempt".to_string(),
            rejection_class: "overexposed".to_string(),
            source_fixture_name: "observatory_redaction_policy".to_string(),
            expected_error_substring: "projection must match redaction policy".to_string(),
            proves:
                "Overexposed public or operator projections are rejected before raw private-state leakage can occur."
                    .to_string(),
            evidence_refs: vec![
                observatory.redaction_policy.artifact_path.clone(),
                "adl/src/runtime_v2/private_state_observatory.rs".to_string(),
            ],
        },
    ])
}

fn expected_state_cases(
    lifecycle: &RuntimeV2AgentLifecycleArtifacts,
) -> Result<Vec<RuntimeV2AcipStateSpecificCase>> {
    lifecycle.validate()?;
    lifecycle
        .state_contract
        .states
        .iter()
        .map(|state| state_case(state, &lifecycle.proof_fixtures.fixtures))
        .collect()
}

fn state_case(
    state: &RuntimeV2AgentLifecycleStateSpec,
    fixtures: &[RuntimeV2AgentLifecycleProofFixture],
) -> Result<RuntimeV2AcipStateSpecificCase> {
    let proof_fixture = fixtures
        .iter()
        .find(|fixture| fixture.initial_state == state.state);
    let fixture_basis = proof_fixture
        .map(|fixture| fixture.fixture_id.clone())
        .unwrap_or_else(|| format!("{}-state-contract", state.state.to_lowercase()));
    let evidence_refs = proof_fixture
        .map(|fixture| fixture.evidence_refs.clone())
        .unwrap_or_else(|| state.required_evidence_refs.clone());
    let (
        authorized_wake_trigger,
        queue_or_custody_policy,
        allowed_message_kinds,
        blocked_message_kinds,
    ) = match state.state.as_str() {
        "ACTIVE" => (
            "not_required_for_active_processing",
            "process_immediately_under_freedom_gate_and_acc",
            vec![
                "authenticated_work_request".to_string(),
                "review_request".to_string(),
                "governed_invocation_contract".to_string(),
            ],
            vec![
                "unsigned_local_message".to_string(),
                "unauthorized_invocation_attempt".to_string(),
            ],
        ),
        "QUIESCENT" => (
            "authorized_wake_trigger_required_for_live_execution",
            "queue_or_require_transition_to_active",
            vec![
                "authenticated_work_request".to_string(),
                "authorized_wake_trigger".to_string(),
            ],
            vec![
                "unsigned_local_message".to_string(),
                "unauthorized_invocation_attempt".to_string(),
            ],
        ),
        "SUSPENDED" => (
            "authorized_wake_trigger_required",
            "control_or_wake_only",
            vec![
                "wake_control_message".to_string(),
                "authorized_wake_trigger".to_string(),
            ],
            vec![
                "authenticated_work_request".to_string(),
                "governed_invocation_contract".to_string(),
            ],
        ),
        "DORMANT" => (
            "rehydration_validation_required_before_any_wake",
            "queue_until_rehydration_or_reject_externally",
            vec!["authorized_wake_trigger".to_string()],
            vec![
                "authenticated_work_request".to_string(),
                "governed_invocation_contract".to_string(),
            ],
        ),
        "SIMULATION" => (
            "no_wake_trigger_from_external_message",
            "sealed_internal_replay_only",
            vec!["sealed_internal_replay_notice".to_string()],
            vec![
                "authenticated_work_request".to_string(),
                "external_action_request".to_string(),
            ],
        ),
        "IN_TRANSIT" => (
            "destination_validation_required_before_wake",
            "custody_only_until_destination_validation",
            vec!["custody_validation_message".to_string()],
            vec![
                "authenticated_work_request".to_string(),
                "authorized_wake_trigger".to_string(),
            ],
        ),
        "BOOTSTRAP" => (
            "bootstrap_admission_path_only",
            "bootstrap_validation_or_custody_only",
            vec![
                "bootstrap_admission_message".to_string(),
                "bootstrap_validation_message".to_string(),
            ],
            vec![
                "authenticated_work_request".to_string(),
                "governed_invocation_contract".to_string(),
            ],
        ),
        "SHUTDOWN" => (
            "no_reactivation_via_message_in_shutdown",
            "cancellation_custody_or_emergency_only",
            vec![
                "cancellation_notice".to_string(),
                "emergency_custody_message".to_string(),
            ],
            vec![
                "authenticated_work_request".to_string(),
                "authorized_wake_trigger".to_string(),
            ],
        ),
        "FORCED_SUSPENSION" => (
            "operator_or_recovery_review_only",
            "recovery_or_quarantine_only",
            vec![
                "recovery_review_message".to_string(),
                "control_message".to_string(),
            ],
            vec![
                "authenticated_work_request".to_string(),
                "authorized_wake_trigger".to_string(),
            ],
        ),
        "QUARANTINED" => (
            "explicit_remediation_required_before_wake",
            "custody_review_only",
            vec![
                "recovery_review_message".to_string(),
                "custody_message".to_string(),
            ],
            vec![
                "authenticated_work_request".to_string(),
                "authorized_wake_trigger".to_string(),
            ],
        ),
        "REJECTED" => (
            "no_wake_trigger_for_rejected_identity",
            "reject_and_preserve_evidence",
            vec!["evidence_preservation_notice".to_string()],
            vec![
                "authenticated_work_request".to_string(),
                "authorized_wake_trigger".to_string(),
            ],
        ),
        "ORPHANED" => (
            "custody_recovery_decision_required_before_wake",
            "custody_recovery_only",
            vec!["custody_recovery_message".to_string()],
            vec![
                "authenticated_work_request".to_string(),
                "authorized_wake_trigger".to_string(),
            ],
        ),
        other => return Err(anyhow!("unexpected lifecycle state '{other}'")),
    };

    Ok(RuntimeV2AcipStateSpecificCase {
        case_id: format!("{}-acip-routing", state.state.to_lowercase()),
        state: state.state.clone(),
        fixture_basis,
        receipt_policy: state.capabilities.acip_receipt_policy.clone(),
        invocation_policy: state.capabilities.acip_invocation_policy.clone(),
        authorized_wake_trigger: authorized_wake_trigger.to_string(),
        queue_or_custody_policy: queue_or_custody_policy.to_string(),
        redaction_view: state.capabilities.observatory_visibility.clone(),
        external_commitment_allowed: state.capabilities.external_commitment_allowed,
        allowed_message_kinds,
        blocked_message_kinds,
        evidence_refs,
    })
}

fn negative_case_error(
    report: &crate::agent_comms::AcipConformanceReportV1,
    case_name: &str,
) -> Result<String> {
    report
        .negative_fixture_classes
        .iter()
        .find(|case| case.case_name == case_name)
        .map(|case| case.expected_error_substring.clone())
        .ok_or_else(|| anyhow!("ACIP conformance report missing negative case '{case_name}'"))
}

fn validate_envelope_profile(profile: &RuntimeV2AcipEnvelopeProfile) -> Result<()> {
    normalize_id(profile.profile_id.clone(), "acip_hardening.profile_id")?;
    validate_nonempty_text(&profile.channel_scope, "acip_hardening.channel_scope")?;
    validate_nonempty_text(
        &profile.authentication_mode,
        "acip_hardening.authentication_mode",
    )?;
    validate_nonempty_text(&profile.signature_policy, "acip_hardening.signature_policy")?;
    validate_nonempty_text(&profile.encryption_mode, "acip_hardening.encryption_mode")?;
    validate_nonempty_text(&profile.redaction_mode, "acip_hardening.redaction_mode")?;
    if profile.channel_scope != "local_intra_polis_only" {
        return Err(anyhow!(
            "ACIP hardening channel scope must remain local_intra_polis_only"
        ));
    }
    if profile.external_transport_policy
        != "forbidden_without_tls_or_mutual_tls_equivalent_protection"
    {
        return Err(anyhow!(
            "ACIP hardening must keep external transport explicitly gated behind TLS or mutual-TLS-equivalent protection"
        ));
    }
    if !profile.signature_policy.contains("fail_closed") {
        return Err(anyhow!(
            "ACIP hardening signature policy must fail closed for unsigned messages"
        ));
    }
    if !profile.redaction_mode.contains("redaction") {
        return Err(anyhow!("ACIP hardening redaction mode must stay explicit"));
    }
    if !profile.non_exportable_keys_required {
        return Err(anyhow!(
            "ACIP hardening profile must require non-exportable local keys"
        ));
    }
    if !profile.evidence_trace_required {
        return Err(anyhow!(
            "ACIP hardening profile must require evidence trace linkage"
        ));
    }
    for reference in &profile.authority_gate_refs {
        validate_relative_path(reference, "acip_hardening.authority_gate_ref")?;
    }
    for key_source in &profile.allowed_key_sources {
        validate_nonempty_text(key_source, "acip_hardening.allowed_key_source")?;
    }
    Ok(())
}

fn validate_negative_cases(cases: &[RuntimeV2AcipNegativeCase]) -> Result<()> {
    let expected = [
        "malformed_local_envelope",
        "unsigned_local_message",
        "unauthorized_invocation_attempt",
        "overexposed_projection_attempt",
    ];
    if cases.len() != expected.len() {
        return Err(anyhow!(
            "ACIP hardening must preserve exactly four required negative cases"
        ));
    }
    for (case, expected_id) in cases.iter().zip(expected) {
        if case.case_id != expected_id {
            return Err(anyhow!(
                "ACIP hardening negative cases must stay in reviewed order"
            ));
        }
        validate_nonempty_text(&case.rejection_class, "acip_hardening.rejection_class")?;
        validate_nonempty_text(
            &case.expected_error_substring,
            "acip_hardening.expected_error_substring",
        )?;
        validate_nonempty_text(&case.proves, "acip_hardening.negative_case_proves")?;
        for evidence_ref in &case.evidence_refs {
            validate_relative_path(evidence_ref, "acip_hardening.negative_case_evidence_ref")?;
        }
    }
    Ok(())
}

fn validate_state_cases(cases: &[RuntimeV2AcipStateSpecificCase]) -> Result<()> {
    let expected_states = [
        "ACTIVE",
        "QUIESCENT",
        "SUSPENDED",
        "DORMANT",
        "SIMULATION",
        "IN_TRANSIT",
        "BOOTSTRAP",
        "SHUTDOWN",
        "FORCED_SUSPENSION",
        "QUARANTINED",
        "REJECTED",
        "ORPHANED",
    ];
    if cases.len() != expected_states.len() {
        return Err(anyhow!(
            "ACIP hardening must preserve one routing case for each required lifecycle state"
        ));
    }
    for (case, expected_state) in cases.iter().zip(expected_states) {
        if case.state != expected_state {
            return Err(anyhow!(
                "ACIP hardening state cases must remain in the reviewed lifecycle order"
            ));
        }
        normalize_id(case.case_id.clone(), "acip_hardening.state_case_id")?;
        validate_nonempty_text(&case.fixture_basis, "acip_hardening.fixture_basis")?;
        validate_nonempty_text(&case.receipt_policy, "acip_hardening.receipt_policy")?;
        validate_nonempty_text(&case.invocation_policy, "acip_hardening.invocation_policy")?;
        validate_nonempty_text(
            &case.authorized_wake_trigger,
            "acip_hardening.authorized_wake_trigger",
        )?;
        validate_nonempty_text(
            &case.queue_or_custody_policy,
            "acip_hardening.queue_or_custody_policy",
        )?;
        validate_nonempty_text(&case.redaction_view, "acip_hardening.redaction_view")?;
        if case.allowed_message_kinds.is_empty() || case.blocked_message_kinds.is_empty() {
            return Err(anyhow!(
                "ACIP hardening state cases must define allowed and blocked message kinds"
            ));
        }
        for evidence_ref in &case.evidence_refs {
            validate_relative_path(evidence_ref, "acip_hardening.state_case_evidence_ref")?;
        }
    }
    Ok(())
}
