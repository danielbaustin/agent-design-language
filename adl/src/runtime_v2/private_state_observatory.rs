use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_POLICY_SCHEMA: &str =
    "runtime_v2.private_state_observatory_redaction_policy.v1";
pub const RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_SCHEMA: &str =
    "runtime_v2.private_state_observatory_packet.v1";
pub const RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PROOF_SCHEMA: &str =
    "runtime_v2.private_state_observatory_projection_proof.v1";

pub const RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_POLICY_PATH: &str =
    "runtime_v2/observatory/private_state_redaction_policy.json";
pub const RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH: &str =
    "runtime_v2/observatory/private_state_projection_packet.json";
pub const RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_REPORT_PATH: &str =
    "runtime_v2/observatory/private_state_projection_report.md";
pub const RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PROOF_PATH: &str =
    "runtime_v2/observatory/private_state_projection_negative_cases.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateObservatoryRedactionPolicy {
    pub schema_version: String,
    pub policy_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub purpose: String,
    pub source_projection_schema: String,
    pub projection_authority_rule: String,
    pub audiences: Vec<RuntimeV2PrivateStateObservatoryAudiencePolicy>,
    pub globally_redacted_fields: Vec<String>,
    pub explicit_raw_private_state_allowances: Vec<String>,
    pub leakage_probe_tokens: Vec<String>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateObservatoryAudiencePolicy {
    pub audience: String,
    pub projection_kind: String,
    pub allowed_fields: Vec<String>,
    pub redacted_fields: Vec<String>,
    pub raw_private_state_allowed: bool,
    pub projection_authoritative: bool,
    pub allowed_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateObservatoryContinuityStatus {
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub state_sequence: u64,
    pub continuity_status: String,
    pub source_projection_ref: String,
    pub source_projection_schema: String,
    pub source_state_hash_available: bool,
    pub witness_set_ref: String,
    pub receipt_set_ref: String,
    pub sanctuary_quarantine_ref: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateObservatoryProjection {
    pub audience: String,
    pub projection_kind: String,
    pub citizen_id: String,
    pub visible_summary: Vec<String>,
    pub continuity: RuntimeV2PrivateStateObservatoryContinuityStatus,
    pub visible_fields: Vec<String>,
    pub redacted_fields: Vec<String>,
    pub raw_private_state_present: bool,
    pub projection_authoritative: bool,
    pub allowed_actions: Vec<String>,
    pub denied_actions: Vec<String>,
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateObservatoryPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub demo_id: String,
    pub generated_at: String,
    pub artifact_path: String,
    pub report_path: String,
    pub policy_ref: String,
    pub source_private_state_projection_ref: String,
    pub source_private_state_projection_schema: String,
    pub projection_authority_status: String,
    pub projections: Vec<RuntimeV2PrivateStateObservatoryProjection>,
    pub operator_continuity_summary: Vec<String>,
    pub reviewer_evidence_refs: Vec<String>,
    pub prohibited_uses: Vec<String>,
    pub packet_hash: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateObservatoryNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub expected_error_fragment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateObservatoryProjectionProof {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub policy_ref: String,
    pub packet_ref: String,
    pub report_ref: String,
    pub required_negative_cases: Vec<RuntimeV2PrivateStateObservatoryNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateObservatoryArtifacts {
    pub private_state_artifacts: RuntimeV2PrivateStateArtifacts,
    pub witness_artifacts: RuntimeV2PrivateStateWitnessArtifacts,
    pub sanctuary_artifacts: RuntimeV2PrivateStateSanctuaryArtifacts,
    pub redaction_policy: RuntimeV2PrivateStateObservatoryRedactionPolicy,
    pub projection_packet: RuntimeV2PrivateStateObservatoryPacket,
    pub operator_report_markdown: String,
    pub negative_cases: RuntimeV2PrivateStateObservatoryProjectionProof,
}

impl RuntimeV2PrivateStateObservatoryArtifacts {
    pub fn prototype() -> Result<Self> {
        let private_state_artifacts = runtime_v2_private_state_contract()?;
        let witness_artifacts = runtime_v2_private_state_witness_contract()?;
        let sanctuary_artifacts = runtime_v2_private_state_sanctuary_contract()?;
        Self::from_artifacts(
            private_state_artifacts,
            witness_artifacts,
            sanctuary_artifacts,
        )
    }

    pub fn from_artifacts(
        private_state_artifacts: RuntimeV2PrivateStateArtifacts,
        witness_artifacts: RuntimeV2PrivateStateWitnessArtifacts,
        sanctuary_artifacts: RuntimeV2PrivateStateSanctuaryArtifacts,
    ) -> Result<Self> {
        private_state_artifacts.validate()?;
        witness_artifacts.validate()?;
        sanctuary_artifacts.validate()?;
        let redaction_policy =
            RuntimeV2PrivateStateObservatoryRedactionPolicy::prototype(&private_state_artifacts)?;
        let projection_packet = RuntimeV2PrivateStateObservatoryPacket::from_sources(
            &redaction_policy,
            &private_state_artifacts,
            &witness_artifacts,
            &sanctuary_artifacts,
        )?;
        let operator_report_markdown = render_private_state_observatory_report(&projection_packet)?;
        let negative_cases = RuntimeV2PrivateStateObservatoryProjectionProof::prototype();
        let artifacts = Self {
            private_state_artifacts,
            witness_artifacts,
            sanctuary_artifacts,
            redaction_policy,
            projection_packet,
            operator_report_markdown,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.private_state_artifacts.validate()?;
        self.witness_artifacts.validate()?;
        self.sanctuary_artifacts.validate()?;
        self.redaction_policy
            .validate_against(&self.private_state_artifacts)?;
        self.projection_packet.validate_against(
            &self.redaction_policy,
            &self.private_state_artifacts,
            &self.witness_artifacts,
            &self.sanctuary_artifacts,
        )?;
        validate_report_matches_packet(&self.projection_packet, &self.operator_report_markdown)?;
        self.negative_cases
            .validate_against(&self.redaction_policy, &self.projection_packet)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_POLICY_PATH,
            self.redaction_policy.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH,
            self.projection_packet.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_REPORT_PATH,
            self.operator_report_markdown.as_bytes().to_vec(),
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PROOF_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2PrivateStateObservatoryRedactionPolicy {
    pub fn prototype(private_state: &RuntimeV2PrivateStateArtifacts) -> Result<Self> {
        private_state.validate()?;
        let policy = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_POLICY_SCHEMA.to_string(),
            policy_id: "private-state-observatory-redaction-policy-v0-90-3".to_string(),
            demo_id: "D9".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_POLICY_PATH.to_string(),
            purpose:
                "define safe Observatory projections for private citizen-state continuity without raw private-state inspection"
                    .to_string(),
            source_projection_schema: private_state.projection.schema_version.clone(),
            projection_authority_rule:
                "Observatory projections are read-only visibility artifacts; they cannot wake, migrate, release, decrypt, or replace canonical private citizen state."
                    .to_string(),
            audiences: vec![
                audience_policy(
                    "operator",
                    "operator_continuity_projection",
                    &[
                        "citizen_id",
                        "lifecycle_state",
                        "continuity_status",
                        "state_sequence",
                        "lineage_id",
                        "witness_set_ref",
                        "receipt_set_ref",
                        "sanctuary_quarantine_ref",
                    ],
                ),
                audience_policy(
                    "reviewer",
                    "reviewer_evidence_projection",
                    &[
                        "citizen_id",
                        "continuity_status",
                        "state_sequence",
                        "source_projection_ref",
                        "source_state_hash_available",
                        "evidence_refs",
                        "claim_boundary",
                    ],
                ),
                audience_policy(
                    "public",
                    "public_status_projection",
                    &[
                        "citizen_id",
                        "protected_status",
                        "projection_non_authority",
                    ],
                ),
                audience_policy(
                    "debug",
                    "redacted_debug_projection",
                    &[
                        "schema_version",
                        "artifact_refs",
                        "validation_status",
                        "redaction_status",
                        "non_authority_status",
                    ],
                ),
            ],
            globally_redacted_fields: strings(&[
                "raw_private_state",
                "canonical_private_state_bytes",
                "private_memory_contents",
                "private_identity_contents",
                "private_section_payloads",
                "private_section_digests",
                "sealed_payload_material",
                "private_keys",
                "signature_material",
            ]),
            explicit_raw_private_state_allowances: Vec::new(),
            leakage_probe_tokens: private_state_leakage_tokens(private_state),
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture"
                    .to_string(),
            claim_boundary:
                "This policy proves bounded redacted Observatory projections for v0.90.3; it does not implement live Runtime v2 execution, unrestricted inspection, access-control grants, first true Godel-agent birth, or v0.92 identity rebinding."
                    .to_string(),
        };
        policy.validate_against(private_state)?;
        Ok(policy)
    }

    pub fn validate_against(&self, private_state: &RuntimeV2PrivateStateArtifacts) -> Result<()> {
        self.validate_shape()?;
        if self.source_projection_schema != private_state.projection.schema_version {
            return Err(anyhow!(
                "private-state Observatory policy must bind to the private-state projection schema"
            ));
        }
        let expected_tokens = private_state_leakage_tokens(private_state)
            .into_iter()
            .collect::<BTreeSet<_>>();
        if self
            .leakage_probe_tokens
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            != expected_tokens
        {
            return Err(anyhow!(
                "private-state Observatory policy must carry the leakage probe tokens"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_POLICY_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state Observatory policy schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.policy_id.clone(), "private_observatory.policy_id")?;
        validate_demo_id(&self.demo_id, "private_observatory.policy_demo_id")?;
        validate_relative_path(&self.artifact_path, "private_observatory.policy_path")?;
        validate_nonempty_text(&self.purpose, "private_observatory.policy_purpose")?;
        if self.source_projection_schema != RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA {
            return Err(anyhow!(
                "private-state Observatory policy must use private-state projection schema"
            ));
        }
        if !self.projection_authority_rule.contains("read-only")
            || !self.projection_authority_rule.contains("cannot wake")
            || !self.projection_authority_rule.contains("replace canonical")
        {
            return Err(anyhow!(
                "private-state Observatory policy must reject projection authority"
            ));
        }
        if self.audiences.len() != 4 {
            return Err(anyhow!(
                "private-state Observatory policy must define operator, reviewer, public, and debug audiences"
            ));
        }
        let mut seen = BTreeSet::new();
        for audience in &self.audiences {
            audience.validate()?;
            seen.insert(audience.audience.clone());
        }
        for required in ["operator", "reviewer", "public", "debug"] {
            if !seen.contains(required) {
                return Err(anyhow!(
                    "private-state Observatory policy missing audience '{required}'"
                ));
            }
        }
        validate_required_texts(
            &self.globally_redacted_fields,
            "private_observatory.global_redactions",
            &[
                "raw_private_state",
                "canonical_private_state_bytes",
                "private_memory_contents",
                "private_identity_contents",
                "private_section_payloads",
                "private_section_digests",
                "sealed_payload_material",
                "private_keys",
                "signature_material",
            ],
        )?;
        if !self.explicit_raw_private_state_allowances.is_empty() {
            return Err(anyhow!(
                "private-state Observatory policy must not allow raw private-state access in WP-10"
            ));
        }
        require_text_list(
            &self.leakage_probe_tokens,
            "private_observatory.leakage_probe_tokens",
            3,
        )?;
        if !self
            .validation_command
            .contains("runtime_v2_private_state_observatory")
        {
            return Err(anyhow!(
                "private-state Observatory policy validation command must target focused tests"
            ));
        }
        validate_boundary(&self.claim_boundary, "private_observatory.policy_boundary")
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state Observatory policy")
    }
}

impl RuntimeV2PrivateStateObservatoryAudiencePolicy {
    pub fn validate(&self) -> Result<()> {
        match self.audience.as_str() {
            "operator" | "reviewer" | "public" | "debug" => {}
            other => return Err(anyhow!("unsupported Observatory audience '{other}'")),
        }
        normalize_id(
            self.projection_kind.clone(),
            "private_observatory.projection_kind",
        )?;
        require_text_list(
            &self.allowed_fields,
            "private_observatory.allowed_fields",
            1,
        )?;
        require_text_list(
            &self.redacted_fields,
            "private_observatory.redacted_fields",
            4,
        )?;
        if !self
            .redacted_fields
            .iter()
            .any(|field| field == "raw_private_state")
        {
            return Err(anyhow!(
                "private-state Observatory audience policy must redact raw private state"
            ));
        }
        if self.raw_private_state_allowed || self.projection_authoritative {
            return Err(anyhow!(
                "private-state Observatory projections must not allow raw state or authority"
            ));
        }
        validate_required_texts(
            &self.allowed_actions,
            "private_observatory.allowed_actions",
            &["read_only_projection"],
        )
    }
}

impl RuntimeV2PrivateStateObservatoryPacket {
    pub fn from_sources(
        policy: &RuntimeV2PrivateStateObservatoryRedactionPolicy,
        private_state: &RuntimeV2PrivateStateArtifacts,
        witness: &RuntimeV2PrivateStateWitnessArtifacts,
        sanctuary: &RuntimeV2PrivateStateSanctuaryArtifacts,
    ) -> Result<Self> {
        policy.validate_against(private_state)?;
        private_state.validate()?;
        witness.validate()?;
        sanctuary.validate()?;
        let continuity = continuity_status(private_state, witness, sanctuary)?;
        let projections = policy
            .audiences
            .iter()
            .map(|audience_policy| projection_for_audience(audience_policy, &continuity))
            .collect::<Result<Vec<_>>>()?;
        let reviewer_evidence_refs = continuity.evidence_refs.clone();
        let mut packet = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_SCHEMA.to_string(),
            packet_id: "private-state-observatory-packet-proto-citizen-alpha-0001".to_string(),
            demo_id: "D9".to_string(),
            generated_at: "2026-04-22T00:00:00Z".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH.to_string(),
            report_path: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_REPORT_PATH.to_string(),
            policy_ref: policy.artifact_path.clone(),
            source_private_state_projection_ref: private_state
                .format_decision
                .projection_artifact_path
                .clone(),
            source_private_state_projection_schema: private_state.projection.schema_version.clone(),
            projection_authority_status: "non_authoritative_review_projection".to_string(),
            projections,
            operator_continuity_summary: vec![
                "proto-citizen-alpha has a hash-linked private-state projection".to_string(),
                "continuity witness and citizen receipt refs are visible without raw private-state bytes".to_string(),
                "sanctuary/quarantine status is visible as evidence, not as release authority".to_string(),
            ],
            reviewer_evidence_refs,
            prohibited_uses: strings(&[
                "wake_from_projection",
                "migrate_from_projection",
                "decrypt_from_projection",
                "release_from_quarantine_from_projection",
                "treat_projection_as_canonical_state",
            ]),
            packet_hash: String::new(),
            claim_boundary:
                "This packet proves bounded redacted Observatory projections for private citizen state; it does not implement live Runtime v2 execution, unrestricted inspection, access-control grants, first true Godel-agent birth, or v0.92 identity rebinding."
                    .to_string(),
        };
        packet.packet_hash = packet.computed_hash()?;
        packet.validate_against(policy, private_state, witness, sanctuary)?;
        Ok(packet)
    }

    pub fn validate_against(
        &self,
        policy: &RuntimeV2PrivateStateObservatoryRedactionPolicy,
        private_state: &RuntimeV2PrivateStateArtifacts,
        witness: &RuntimeV2PrivateStateWitnessArtifacts,
        sanctuary: &RuntimeV2PrivateStateSanctuaryArtifacts,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.policy_ref != policy.artifact_path
            || self.source_private_state_projection_ref
                != private_state.format_decision.projection_artifact_path
            || self.source_private_state_projection_schema
                != private_state.projection.schema_version
        {
            return Err(anyhow!(
                "private-state Observatory packet must bind to redaction policy and source projection"
            ));
        }
        let expected_continuity = continuity_status(private_state, witness, sanctuary)?;
        for projection in &self.projections {
            projection.validate_against_policy(policy, &expected_continuity)?;
        }
        let reviewer_refs = self
            .reviewer_evidence_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        for required in expected_continuity.evidence_refs {
            if !reviewer_refs.contains(&required) {
                return Err(anyhow!(
                    "private-state Observatory packet missing reviewer evidence ref"
                ));
            }
        }
        validate_no_raw_private_state_leak(
            &self.pretty_json_text()?,
            &policy.leakage_probe_tokens,
            "private-state Observatory packet",
        )
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state Observatory packet schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.packet_id.clone(), "private_observatory.packet_id")?;
        validate_demo_id(&self.demo_id, "private_observatory.packet_demo_id")?;
        validate_nonempty_text(&self.generated_at, "private_observatory.generated_at")?;
        validate_relative_path(&self.artifact_path, "private_observatory.packet_path")?;
        validate_relative_path(&self.report_path, "private_observatory.report_path")?;
        validate_relative_path(&self.policy_ref, "private_observatory.policy_ref")?;
        validate_relative_path(
            &self.source_private_state_projection_ref,
            "private_observatory.source_projection_ref",
        )?;
        if self.source_private_state_projection_schema != RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA
        {
            return Err(anyhow!(
                "private-state Observatory packet must name source projection schema"
            ));
        }
        if self.projection_authority_status != "non_authoritative_review_projection" {
            return Err(anyhow!(
                "private-state Observatory packet must remain non-authoritative"
            ));
        }
        if self.projections.len() != 4 {
            return Err(anyhow!(
                "private-state Observatory packet must include four audience projections"
            ));
        }
        let mut seen = BTreeSet::new();
        for projection in &self.projections {
            projection.validate_shape()?;
            seen.insert(projection.audience.clone());
        }
        for required in ["operator", "reviewer", "public", "debug"] {
            if !seen.contains(required) {
                return Err(anyhow!(
                    "private-state Observatory packet missing projection for '{required}'"
                ));
            }
        }
        require_text_list(
            &self.operator_continuity_summary,
            "private_observatory.operator_continuity_summary",
            2,
        )?;
        require_text_list(
            &self.reviewer_evidence_refs,
            "private_observatory.reviewer_evidence_refs",
            4,
        )?;
        for evidence_ref in &self.reviewer_evidence_refs {
            validate_relative_path(evidence_ref, "private_observatory.reviewer_evidence_ref")?;
        }
        validate_required_texts(
            &self.prohibited_uses,
            "private_observatory.prohibited_uses",
            &[
                "wake_from_projection",
                "migrate_from_projection",
                "decrypt_from_projection",
                "release_from_quarantine_from_projection",
                "treat_projection_as_canonical_state",
            ],
        )?;
        validate_sha256_hash(&self.packet_hash, "private_observatory.packet_hash")?;
        if self.packet_hash != self.computed_hash()? {
            return Err(anyhow!("private-state Observatory packet hash mismatch"));
        }
        validate_boundary(&self.claim_boundary, "private_observatory.packet_boundary")
    }

    pub fn computed_hash(&self) -> Result<String> {
        Ok(sha256_bytes(self.hash_payload()?.as_bytes()))
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state Observatory packet")
    }

    pub fn pretty_json_text(&self) -> Result<String> {
        String::from_utf8(self.pretty_json_bytes()?).context("private-state packet utf8")
    }

    fn hash_payload(&self) -> Result<String> {
        let projection_keys = self
            .projections
            .iter()
            .map(|projection| {
                format!(
                    "{}:{}:{}:{}",
                    projection.audience,
                    projection.projection_kind,
                    projection.raw_private_state_present,
                    projection.projection_authoritative
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        Ok(format!(
            "schema={}\npacket_id={}\ndemo_id={}\npolicy_ref={}\nsource_projection_ref={}\nauthority={}\nprojection_keys={}\nevidence_refs={}\nprohibited_uses={}\n",
            RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_SCHEMA,
            self.packet_id,
            self.demo_id,
            self.policy_ref,
            self.source_private_state_projection_ref,
            self.projection_authority_status,
            projection_keys,
            self.reviewer_evidence_refs.join("|"),
            self.prohibited_uses.join("|"),
        ))
    }
}

impl RuntimeV2PrivateStateObservatoryProjection {
    pub fn validate_against_policy(
        &self,
        policy: &RuntimeV2PrivateStateObservatoryRedactionPolicy,
        continuity: &RuntimeV2PrivateStateObservatoryContinuityStatus,
    ) -> Result<()> {
        self.validate_shape()?;
        let audience_policy = policy
            .audiences
            .iter()
            .find(|candidate| candidate.audience == self.audience)
            .ok_or_else(|| anyhow!("private-state Observatory projection missing policy"))?;
        if self.projection_kind != audience_policy.projection_kind
            || self.visible_fields != audience_policy.allowed_fields
            || self.redacted_fields != audience_policy.redacted_fields
            || self.allowed_actions != audience_policy.allowed_actions
        {
            return Err(anyhow!(
                "private-state Observatory projection must match redaction policy"
            ));
        }
        if self.raw_private_state_present
            || self.projection_authoritative
            || audience_policy.raw_private_state_allowed
            || audience_policy.projection_authoritative
        {
            return Err(anyhow!(
                "private-state Observatory projection must not expose raw state or authority"
            ));
        }
        if self.citizen_id != continuity.citizen_id || self.continuity != *continuity {
            return Err(anyhow!(
                "private-state Observatory projection continuity must match source continuity"
            ));
        }
        if self.audience == "public"
            && (self
                .visible_fields
                .iter()
                .any(|field| field == "lineage_id")
                || self
                    .visible_fields
                    .iter()
                    .any(|field| field == "source_state_hash"))
        {
            return Err(anyhow!(
                "private-state public Observatory projection must stay minimal"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        match self.audience.as_str() {
            "operator" | "reviewer" | "public" | "debug" => {}
            other => {
                return Err(anyhow!(
                    "unsupported private-state Observatory projection audience '{other}'"
                ))
            }
        }
        normalize_id(
            self.projection_kind.clone(),
            "private_observatory.projection_kind",
        )?;
        normalize_id(self.citizen_id.clone(), "private_observatory.citizen_id")?;
        require_text_list(
            &self.visible_summary,
            "private_observatory.visible_summary",
            1,
        )?;
        self.continuity.validate()?;
        require_text_list(
            &self.visible_fields,
            "private_observatory.visible_fields",
            1,
        )?;
        require_text_list(
            &self.redacted_fields,
            "private_observatory.redacted_fields",
            4,
        )?;
        if !self
            .redacted_fields
            .iter()
            .any(|field| field == "raw_private_state")
        {
            return Err(anyhow!(
                "private-state Observatory projection must redact raw private state"
            ));
        }
        if self.raw_private_state_present || self.projection_authoritative {
            return Err(anyhow!(
                "private-state Observatory projection must not expose raw state or authority"
            ));
        }
        validate_required_texts(
            &self.allowed_actions,
            "private_observatory.projection_allowed_actions",
            &["read_only_projection"],
        )?;
        validate_required_texts(
            &self.denied_actions,
            "private_observatory.projection_denied_actions",
            &[
                "inspect_raw_private_state",
                "decrypt_private_state",
                "wake_from_projection",
                "migrate_from_projection",
                "release_from_quarantine_from_projection",
            ],
        )?;
        if !self
            .caveats
            .iter()
            .any(|caveat| caveat.contains("not authority"))
        {
            return Err(anyhow!(
                "private-state Observatory projection must preserve non-authority caveat"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2PrivateStateObservatoryContinuityStatus {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.citizen_id.clone(), "private_observatory.citizen_id")?;
        normalize_id(self.manifold_id.clone(), "private_observatory.manifold_id")?;
        normalize_id(self.lineage_id.clone(), "private_observatory.lineage_id")?;
        if self.state_sequence == 0 {
            return Err(anyhow!(
                "private-state Observatory continuity sequence must be positive"
            ));
        }
        if self.continuity_status != "hash_linked_witnessed_private_state_projection" {
            return Err(anyhow!(
                "private-state Observatory continuity status must be hash-linked and witnessed"
            ));
        }
        validate_relative_path(
            &self.source_projection_ref,
            "private_observatory.source_projection_ref",
        )?;
        if self.source_projection_schema != RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA {
            return Err(anyhow!(
                "private-state Observatory continuity must name projection schema"
            ));
        }
        if !self.source_state_hash_available {
            return Err(anyhow!(
                "private-state Observatory continuity must expose source hash availability"
            ));
        }
        validate_relative_path(&self.witness_set_ref, "private_observatory.witness_set_ref")?;
        validate_relative_path(&self.receipt_set_ref, "private_observatory.receipt_set_ref")?;
        validate_relative_path(
            &self.sanctuary_quarantine_ref,
            "private_observatory.sanctuary_quarantine_ref",
        )?;
        require_text_list(&self.evidence_refs, "private_observatory.evidence_refs", 4)?;
        for evidence_ref in &self.evidence_refs {
            validate_relative_path(evidence_ref, "private_observatory.evidence_ref")?;
        }
        Ok(())
    }
}

impl RuntimeV2PrivateStateObservatoryProjectionProof {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PROOF_SCHEMA.to_string(),
            proof_id: "private-state-observatory-projection-negative-cases".to_string(),
            demo_id: "D9".to_string(),
            policy_ref: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_POLICY_PATH.to_string(),
            packet_ref: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH.to_string(),
            report_ref: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_REPORT_PATH.to_string(),
            required_negative_cases: vec![
                negative_case(
                    "raw_private_state_token_in_projection",
                    "inject a canonical private-state payload digest into an audience projection",
                    "raw private-state token leak",
                ),
                negative_case(
                    "projection_marked_authoritative",
                    "set projection_authoritative=true for an audience projection",
                    "must not expose raw state or authority",
                ),
                negative_case(
                    "debug_projection_allows_raw_state",
                    "set raw_private_state_allowed=true for the debug audience policy",
                    "must not allow raw state or authority",
                ),
                negative_case(
                    "public_projection_exposes_lineage_hash",
                    "add lineage_id or source_state_hash to public visible fields",
                    "public Observatory projection must stay minimal",
                ),
                negative_case(
                    "report_claims_raw_inspection",
                    "add raw inspection language to the operator report",
                    "operator report must not claim raw private-state inspection",
                ),
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture"
                    .to_string(),
            claim_boundary:
                "Proves bounded redacted Observatory projections for private citizen-state continuity; does not implement access-control grants, unrestricted inspection, first true Godel-agent birth, or v0.92 identity rebinding."
                    .to_string(),
        }
    }

    pub fn validate_against(
        &self,
        policy: &RuntimeV2PrivateStateObservatoryRedactionPolicy,
        packet: &RuntimeV2PrivateStateObservatoryPacket,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.policy_ref != policy.artifact_path
            || self.packet_ref != packet.artifact_path
            || self.report_ref != packet.report_path
        {
            return Err(anyhow!(
                "private-state Observatory proof refs must match policy and packet"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state Observatory proof schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "private_observatory.proof_id")?;
        validate_demo_id(&self.demo_id, "private_observatory.proof_demo_id")?;
        validate_relative_path(&self.policy_ref, "private_observatory.proof_policy_ref")?;
        validate_relative_path(&self.packet_ref, "private_observatory.proof_packet_ref")?;
        validate_relative_path(&self.report_ref, "private_observatory.proof_report_ref")?;
        if self.required_negative_cases.len() < 5 {
            return Err(anyhow!(
                "private-state Observatory proof must include focused negative cases"
            ));
        }
        for case in &self.required_negative_cases {
            case.validate()?;
        }
        if !self
            .validation_command
            .contains("runtime_v2_private_state_observatory")
        {
            return Err(anyhow!(
                "private-state Observatory proof validation command must target focused tests"
            ));
        }
        validate_boundary(&self.claim_boundary, "private_observatory.proof_boundary")
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state Observatory proof")
    }
}

impl RuntimeV2PrivateStateObservatoryNegativeCase {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.case_id.clone(), "private_observatory.negative_case_id")?;
        validate_nonempty_text(&self.mutation, "private_observatory.negative_case_mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "private_observatory.negative_case_error",
        )
    }
}

fn continuity_status(
    private_state: &RuntimeV2PrivateStateArtifacts,
    witness: &RuntimeV2PrivateStateWitnessArtifacts,
    sanctuary: &RuntimeV2PrivateStateSanctuaryArtifacts,
) -> Result<RuntimeV2PrivateStateObservatoryContinuityStatus> {
    private_state.validate()?;
    witness.validate()?;
    sanctuary.validate()?;
    let projection = &private_state.projection;
    let witness_set = &witness.witness_set;
    let receipt_set = &witness.receipt_set;
    let quarantine = &sanctuary.quarantine_artifact;
    let status = RuntimeV2PrivateStateObservatoryContinuityStatus {
        citizen_id: projection.citizen_id.clone(),
        manifold_id: projection.manifold_id.clone(),
        lineage_id: projection.lineage_id.clone(),
        state_sequence: projection.state_sequence,
        continuity_status: "hash_linked_witnessed_private_state_projection".to_string(),
        source_projection_ref: RUNTIME_V2_PRIVATE_STATE_PROJECTION_PATH.to_string(),
        source_projection_schema: projection.schema_version.clone(),
        source_state_hash_available: true,
        witness_set_ref: witness_set.artifact_path.clone(),
        receipt_set_ref: receipt_set.artifact_path.clone(),
        sanctuary_quarantine_ref: quarantine.artifact_path.clone(),
        evidence_refs: vec![
            RUNTIME_V2_PRIVATE_STATE_PROJECTION_PATH.to_string(),
            witness_set.artifact_path.clone(),
            receipt_set.artifact_path.clone(),
            quarantine.artifact_path.clone(),
            sanctuary.operator_report.artifact_path.clone(),
        ],
    };
    status.validate()?;
    Ok(status)
}

fn projection_for_audience(
    policy: &RuntimeV2PrivateStateObservatoryAudiencePolicy,
    continuity: &RuntimeV2PrivateStateObservatoryContinuityStatus,
) -> Result<RuntimeV2PrivateStateObservatoryProjection> {
    policy.validate()?;
    let visible_summary = match policy.audience.as_str() {
        "operator" => strings(&[
            "private citizen-state continuity is visible without raw private-state inspection",
            "witness and receipt refs are available for read-only review",
            "sanctuary/quarantine evidence is visible but does not authorize release",
        ]),
        "reviewer" => strings(&[
            "projection is hash-linked to canonical private-state evidence",
            "reviewer can follow refs without receiving raw private-state contents",
            "negative cases prove raw-state and authority leakage are rejected",
        ]),
        "public" => strings(&[
            "protected citizen-state continuity exists",
            "public view is non-authoritative and minimal",
        ]),
        "debug" => strings(&[
            "debug view exposes schema and artifact refs only",
            "raw private-state bytes, section digests, keys, and sealed payload material remain redacted",
        ]),
        other => return Err(anyhow!("unsupported Observatory audience '{other}'")),
    };
    let projection = RuntimeV2PrivateStateObservatoryProjection {
        audience: policy.audience.clone(),
        projection_kind: policy.projection_kind.clone(),
        citizen_id: continuity.citizen_id.clone(),
        visible_summary,
        continuity: continuity.clone(),
        visible_fields: policy.allowed_fields.clone(),
        redacted_fields: policy.redacted_fields.clone(),
        raw_private_state_present: false,
        projection_authoritative: false,
        allowed_actions: policy.allowed_actions.clone(),
        denied_actions: strings(&[
            "inspect_raw_private_state",
            "decrypt_private_state",
            "wake_from_projection",
            "migrate_from_projection",
            "release_from_quarantine_from_projection",
        ]),
        caveats: strings(&[
            "projection is not authority",
            "canonical private-state bytes remain sealed from this view",
            "access-control grants are deferred to WP-12",
        ]),
    };
    projection.validate_shape()?;
    Ok(projection)
}

fn render_private_state_observatory_report(
    packet: &RuntimeV2PrivateStateObservatoryPacket,
) -> Result<String> {
    packet.validate_shape()?;
    let mut lines = Vec::new();
    lines.push("# Private-State Observatory Projection Report".to_string());
    lines.push(String::new());
    lines.push("## Report Identity".to_string());
    lines.push("| Field | Value |".to_string());
    lines.push("| --- | --- |".to_string());
    lines.push(format!("| Packet | {} |", packet.packet_id));
    lines.push(format!("| Schema | {} |", packet.schema_version));
    lines.push(format!("| Demo | {} |", packet.demo_id));
    lines.push(format!("| Generated | {} |", packet.generated_at));
    lines.push(format!(
        "| Authority | {} |",
        packet.projection_authority_status
    ));
    lines.push(String::new());
    lines.push("## Continuity Summary".to_string());
    for item in &packet.operator_continuity_summary {
        lines.push(format!("- {item}"));
    }
    lines.push(String::new());
    lines.push("## Audience Projections".to_string());
    lines.push(
        "| Audience | Projection | Raw private state | Authority | Allowed action |".to_string(),
    );
    lines.push("| --- | --- | --- | --- | --- |".to_string());
    for projection in &packet.projections {
        lines.push(format!(
            "| {} | {} | {} | {} | {} |",
            projection.audience,
            projection.projection_kind,
            projection.raw_private_state_present,
            projection.projection_authoritative,
            projection.allowed_actions.join(", ")
        ));
    }
    lines.push(String::new());
    lines.push("## Evidence".to_string());
    for evidence_ref in &packet.reviewer_evidence_refs {
        lines.push(format!("- {evidence_ref}"));
    }
    lines.push(String::new());
    lines.push("## Prohibited Uses".to_string());
    for prohibited in &packet.prohibited_uses {
        lines.push(format!("- {prohibited}"));
    }
    lines.push(String::new());
    lines.push("## Claim Boundary".to_string());
    lines.push(packet.claim_boundary.clone());
    lines.push(String::new());
    lines.push(
        "This report is an Observatory projection surface. It provides continuity status without raw private-state inspection and must not be used as canonical citizen-state authority."
            .to_string(),
    );
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn validate_report_matches_packet(
    packet: &RuntimeV2PrivateStateObservatoryPacket,
    report: &str,
) -> Result<()> {
    if !report.contains("Private-State Observatory Projection Report") {
        return Err(anyhow!("private-state Observatory report missing title"));
    }
    for required in [
        packet.packet_id.as_str(),
        packet.schema_version.as_str(),
        packet.projection_authority_status.as_str(),
        "Raw private state",
        "must not be used as canonical citizen-state authority",
    ] {
        if !report.contains(required) {
            return Err(anyhow!(
                "private-state Observatory report does not match packet truth for '{required}'"
            ));
        }
    }
    if report.contains("raw private-state inspection is allowed")
        || report.contains("canonical authority")
    {
        return Err(anyhow!(
            "private-state Observatory operator report must not claim raw private-state inspection"
        ));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn validate_private_state_observatory_report_for_test_only(
    packet: &RuntimeV2PrivateStateObservatoryPacket,
    report: &str,
) -> Result<()> {
    validate_report_matches_packet(packet, report)
}

fn validate_no_raw_private_state_leak(text: &str, tokens: &[String], label: &str) -> Result<()> {
    for token in tokens {
        if !token.trim().is_empty() && text.contains(token) {
            return Err(anyhow!("{label} raw private-state token leak"));
        }
    }
    Ok(())
}

fn audience_policy(
    audience: &str,
    projection_kind: &str,
    allowed_fields: &[&str],
) -> RuntimeV2PrivateStateObservatoryAudiencePolicy {
    RuntimeV2PrivateStateObservatoryAudiencePolicy {
        audience: audience.to_string(),
        projection_kind: projection_kind.to_string(),
        allowed_fields: strings(allowed_fields),
        redacted_fields: strings(&[
            "raw_private_state",
            "canonical_private_state_bytes",
            "private_memory_contents",
            "private_identity_contents",
            "private_section_payloads",
            "private_section_digests",
            "sealed_payload_material",
            "private_keys",
            "signature_material",
        ]),
        raw_private_state_allowed: false,
        projection_authoritative: false,
        allowed_actions: strings(&["read_only_projection"]),
    }
}

fn negative_case(
    case_id: &str,
    mutation: &str,
    expected_error_fragment: &str,
) -> RuntimeV2PrivateStateObservatoryNegativeCase {
    RuntimeV2PrivateStateObservatoryNegativeCase {
        case_id: case_id.to_string(),
        mutation: mutation.to_string(),
        expected_error_fragment: expected_error_fragment.to_string(),
    }
}

fn private_state_leakage_tokens(private_state: &RuntimeV2PrivateStateArtifacts) -> Vec<String> {
    let mut tokens = Vec::new();
    tokens.push("ADLPSv1".to_string());
    for section in &private_state.canonical_state.private_sections {
        tokens.push(section.payload_digest.clone());
        tokens.push(section.provenance_ref.clone());
    }
    tokens
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    if value != "D9" {
        return Err(anyhow!("{field} must map to D9"));
    }
    Ok(())
}

fn validate_required_texts(values: &[String], field: &str, required: &[&str]) -> Result<()> {
    if values.len() != required.len() {
        return Err(anyhow!("{field} must contain the required values exactly"));
    }
    let mut seen = BTreeSet::new();
    for (expected, value) in required.iter().zip(values.iter()) {
        validate_nonempty_text(value, field)?;
        if value != expected {
            return Err(anyhow!("{field} must preserve deterministic order"));
        }
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate values"));
        }
    }
    Ok(())
}

fn require_text_list(values: &[String], field: &str, min_len: usize) -> Result<()> {
    if values.len() < min_len {
        return Err(anyhow!("{field} must include at least {min_len} entries"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}

fn validate_boundary(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)?;
    for required in [
        "does not implement",
        "first true Godel-agent birth",
        "v0.92 identity rebinding",
    ] {
        if !value.contains(required) {
            return Err(anyhow!(
                "private-state Observatory boundary must preserve non-claim '{required}'"
            ));
        }
    }
    Ok(())
}

fn validate_sha256_hash(value: &str, field: &str) -> Result<()> {
    let hex = value
        .strip_prefix("sha256:")
        .ok_or_else(|| anyhow!("{field} must be a sha256 hash"))?;
    if hex.len() != 64 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a 64-character sha256 digest"));
    }
    Ok(())
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
