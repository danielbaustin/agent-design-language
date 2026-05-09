//! Runtime-v2 citizen-state substrate packet for v0.91.1.
//!
//! This packet does not replace the inherited private-state machinery. It
//! records how v0.91.1 WP-06 consumes the v0.90.3 canonical private-state
//! baseline, lineage/recovery evidence, and observatory-safe projections as one
//! bounded citizen-state substrate.

use super::*;
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_CITIZEN_STATE_SUBSTRATE_SCHEMA: &str =
    "runtime_v2.citizen_state_substrate_packet.v1";
pub const RUNTIME_V2_CITIZEN_STATE_SUBSTRATE_PATH: &str =
    "runtime_v2/citizen_state/citizen_state_substrate.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CitizenStateSubstrateAudienceView {
    pub audience: String,
    pub surface_kind: String,
    pub artifact_ref: String,
    pub projection_selector: String,
    pub authority_status: String,
    pub raw_private_state_allowed: bool,
    pub intended_use: String,
    pub denied_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CitizenStateSubstrateFixture {
    pub fixture_kind: String,
    pub artifact_ref: String,
    pub proving_surface: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CitizenStateSubstratePacket {
    pub schema_version: String,
    pub substrate_id: String,
    pub demo_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub inherited_private_state_decision_id: String,
    pub inherited_private_state_milestone: String,
    pub inherited_private_state_refs: Vec<String>,
    pub audience_views: Vec<RuntimeV2CitizenStateSubstrateAudienceView>,
    pub fixture_matrix: Vec<RuntimeV2CitizenStateSubstrateFixture>,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

impl RuntimeV2CitizenStateSubstratePacket {
    pub fn prototype() -> Result<Self> {
        let private_state = runtime_v2_private_state_contract()?;
        let lineage = runtime_v2_private_state_lineage_contract()?;
        let observatory = runtime_v2_private_state_observatory_contract()?;

        let packet = Self {
            schema_version: RUNTIME_V2_CITIZEN_STATE_SUBSTRATE_SCHEMA.to_string(),
            substrate_id: "citizen-state-substrate-v0-91-1-wp-06".to_string(),
            demo_id: "D10".to_string(),
            milestone: "v0.91.1".to_string(),
            wp: "WP-06".to_string(),
            artifact_path: RUNTIME_V2_CITIZEN_STATE_SUBSTRATE_PATH.to_string(),
            source_feature_doc: "docs/milestones/v0.91.1/features/CITIZEN_STATE_SUBSTRATE.md"
                .to_string(),
            inherited_private_state_decision_id: private_state.format_decision.decision_id.clone(),
            inherited_private_state_milestone: private_state.format_decision.milestone.clone(),
            inherited_private_state_refs: private_state.format_decision.source_audit_refs.clone(),
            audience_views: audience_views(&private_state, &observatory),
            fixture_matrix: fixture_matrix(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_citizen_state_substrate -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_envelope -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_lineage -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture".to_string(),
                "git diff --check".to_string(),
            ],
            claim_boundary:
                "WP-06 proves one v0.91.1 citizen-state substrate by truthfully consuming the inherited v0.90.3 canonical private-state baseline, fail-closed lineage/recovery evidence, and runtime/operator/reviewer/public projection boundaries. It does not claim birthday, full identity continuity, public release of private diagnostics, or new external transport guarantees."
                    .to_string(),
            non_claims: vec![
                "does not replace the inherited v0.90.3 canonical private-state format with a newly authored v0.91.1 authority surface".to_string(),
                "does not permit raw private-state inspection from runtime, operator, reviewer, or public projections".to_string(),
                "does not prove full v0.92 identity continuity or first true birthday semantics".to_string(),
                "does not prove external communication readiness without later TLS or mutual-TLS-equivalent work".to_string(),
            ],
        };
        packet.validate_against(&private_state, &lineage, &observatory)?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CITIZEN_STATE_SUBSTRATE_SCHEMA {
            return Err(anyhow!(
                "unsupported citizen-state substrate schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.substrate_id.clone(),
            "citizen_state_substrate.substrate_id",
        )?;
        if self.demo_id != "D10" {
            return Err(anyhow!(
                "citizen-state substrate must attach to shared standing/state demo row D10"
            ));
        }
        if self.milestone != "v0.91.1" {
            return Err(anyhow!(
                "citizen-state substrate must target milestone v0.91.1"
            ));
        }
        if self.wp != "WP-06" {
            return Err(anyhow!(
                "citizen-state substrate must remain bound to WP-06"
            ));
        }
        validate_relative_path(&self.artifact_path, "citizen_state_substrate.artifact_path")?;
        if self.source_feature_doc != "docs/milestones/v0.91.1/features/CITIZEN_STATE_SUBSTRATE.md"
        {
            return Err(anyhow!(
                "citizen-state substrate must point at the v0.91.1 feature doc"
            ));
        }
        validate_relative_path(
            &self.source_feature_doc,
            "citizen_state_substrate.source_feature_doc",
        )?;
        if self.inherited_private_state_milestone != "v0.90.3" {
            return Err(anyhow!(
                "citizen-state substrate must truthfully preserve inherited private-state milestone v0.90.3"
            ));
        }
        if self.inherited_private_state_decision_id != "v0-90-3-wp-03-private-state-format" {
            return Err(anyhow!(
                "citizen-state substrate must preserve the inherited v0.90.3 private-state decision id"
            ));
        }
        if self.inherited_private_state_refs.is_empty() {
            return Err(anyhow!(
                "citizen-state substrate must preserve inherited private-state references"
            ));
        }
        for reference in &self.inherited_private_state_refs {
            validate_relative_path(reference, "citizen_state_substrate.inherited_ref")?;
        }
        validate_audience_views(&self.audience_views)?;
        validate_fixture_matrix(&self.fixture_matrix)?;
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_citizen_state_substrate"))
        {
            return Err(anyhow!(
                "citizen-state substrate must include its focused validation command"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_private_state_observatory"))
        {
            return Err(anyhow!(
                "citizen-state substrate must preserve private-state observatory validation"
            ));
        }
        if !self
            .claim_boundary
            .contains("v0.90.3 canonical private-state baseline")
        {
            return Err(anyhow!(
                "citizen-state substrate must preserve the inherited-baseline claim boundary"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("raw private-state inspection"))
        {
            return Err(anyhow!(
                "citizen-state substrate must preserve the raw private-state non-claim"
            ));
        }
        Ok(())
    }

    pub fn validate_against(
        &self,
        private_state: &RuntimeV2PrivateStateArtifacts,
        lineage: &RuntimeV2PrivateStateLineageArtifacts,
        observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
    ) -> Result<()> {
        self.validate()?;
        private_state.validate()?;
        lineage.validate()?;
        observatory.validate()?;

        if self.inherited_private_state_decision_id != private_state.format_decision.decision_id {
            return Err(anyhow!(
                "citizen-state substrate inherited decision id drifted from private-state artifacts"
            ));
        }
        if self.inherited_private_state_refs != private_state.format_decision.source_audit_refs {
            return Err(anyhow!(
                "citizen-state substrate inherited references drifted from private-state artifacts"
            ));
        }

        let runtime_view = self
            .audience_views
            .iter()
            .find(|view| view.audience == "runtime")
            .ok_or_else(|| anyhow!("missing runtime audience view"))?;
        if runtime_view.artifact_ref != RUNTIME_V2_PRIVATE_STATE_PROJECTION_PATH {
            return Err(anyhow!(
                "runtime audience must point at the canonical runtime projection artifact"
            ));
        }
        if runtime_view.authority_status != private_state.projection.authority_status {
            return Err(anyhow!(
                "runtime audience authority status must match private-state projection"
            ));
        }

        for audience in ["operator", "reviewer", "public"] {
            let packet_view = self
                .audience_views
                .iter()
                .find(|view| view.audience == audience)
                .ok_or_else(|| anyhow!("missing {} audience view", audience))?;
            let observatory_projection = observatory
                .projection_packet
                .projections
                .iter()
                .find(|projection| projection.audience == audience)
                .ok_or_else(|| anyhow!("missing {} observatory projection", audience))?;
            if packet_view.artifact_ref != RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH {
                return Err(anyhow!(
                    "{} audience must point at the observatory packet",
                    audience
                ));
            }
            if packet_view.projection_selector != observatory_projection.audience {
                return Err(anyhow!(
                    "{} audience selector drifted from observatory packet",
                    audience
                ));
            }
            if packet_view.authority_status
                != observatory.projection_packet.projection_authority_status
            {
                return Err(anyhow!(
                    "{} audience authority status must match observatory packet",
                    audience
                ));
            }
            if observatory_projection.raw_private_state_present {
                return Err(anyhow!(
                    "{} observatory projection must not expose raw private state",
                    audience
                ));
            }
        }

        let stale_fixture = self
            .fixture_matrix
            .iter()
            .find(|fixture| fixture.fixture_kind == "stale_state")
            .ok_or_else(|| anyhow!("missing stale_state fixture"))?;
        if stale_fixture.artifact_ref
            != "adl/tests/fixtures/runtime_v2/private_state/lineage_negative_cases.json"
        {
            return Err(anyhow!(
                "stale_state fixture must point at the lineage negative-case surface"
            ));
        }
        if !lineage
            .negative_cases
            .required_negative_cases
            .iter()
            .any(|case| {
                case.case_id == "head_disagreement"
                    && case
                        .expected_error_fragment
                        .contains("recovery_or_quarantine")
            })
        {
            return Err(anyhow!(
                "citizen-state substrate requires stale-state lineage evidence"
            ));
        }

        let overexposed_fixture = self
            .fixture_matrix
            .iter()
            .find(|fixture| fixture.fixture_kind == "overexposed_projection")
            .ok_or_else(|| anyhow!("missing overexposed_projection fixture"))?;
        if overexposed_fixture.artifact_ref
            != "adl/tests/fixtures/runtime_v2/observatory/private_state_projection_negative_cases.json"
        {
            return Err(anyhow!(
                "overexposed_projection fixture must point at the observatory negative-case surface"
            ));
        }
        if !observatory
            .negative_cases
            .required_negative_cases
            .iter()
            .any(|case| case.case_id == "public_projection_exposes_lineage_hash")
        {
            return Err(anyhow!(
                "citizen-state substrate requires public overexposure evidence"
            ));
        }

        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize citizen-state substrate packet")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_CITIZEN_STATE_SUBSTRATE_PATH,
            self.pretty_json_bytes()?,
        )
    }

    pub fn write_to_path(&self, output_path: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let output_path = output_path.as_ref();
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create citizen-state substrate parent {}",
                    parent.display()
                )
            })?;
        }
        std::fs::write(output_path, self.pretty_json_bytes()?).with_context(|| {
            format!(
                "failed to write citizen-state substrate packet to {}",
                output_path.display()
            )
        })
    }
}

fn audience_views(
    private_state: &RuntimeV2PrivateStateArtifacts,
    observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
) -> Vec<RuntimeV2CitizenStateSubstrateAudienceView> {
    vec![
        RuntimeV2CitizenStateSubstrateAudienceView {
            audience: "runtime".to_string(),
            surface_kind: "derived_runtime_projection".to_string(),
            artifact_ref: RUNTIME_V2_PRIVATE_STATE_PROJECTION_PATH.to_string(),
            projection_selector: "root".to_string(),
            authority_status: private_state.projection.authority_status.clone(),
            raw_private_state_allowed: false,
            intended_use:
                "runtime may consume only the derived projection while canonical binary private state remains the authority surface"
                    .to_string(),
            denied_actions: vec![
                "promote_projection_to_authority".to_string(),
                "inspect_raw_private_state".to_string(),
            ],
        },
        RuntimeV2CitizenStateSubstrateAudienceView {
            audience: "operator".to_string(),
            surface_kind: "observatory_projection".to_string(),
            artifact_ref: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH.to_string(),
            projection_selector: "operator".to_string(),
            authority_status: observatory
                .projection_packet
                .projection_authority_status
                .clone(),
            raw_private_state_allowed: false,
            intended_use:
                "operators review continuity and quarantine-safe state summaries without raw private-state inspection"
                    .to_string(),
            denied_actions: vec![
                "inspect_raw_private_state".to_string(),
                "mutate_canonical_state".to_string(),
            ],
        },
        RuntimeV2CitizenStateSubstrateAudienceView {
            audience: "reviewer".to_string(),
            surface_kind: "observatory_projection".to_string(),
            artifact_ref: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH.to_string(),
            projection_selector: "reviewer".to_string(),
            authority_status: observatory
                .projection_packet
                .projection_authority_status
                .clone(),
            raw_private_state_allowed: false,
            intended_use:
                "reviewers inspect evidence-linked continuity summaries and claim boundaries without raw private-state release"
                    .to_string(),
            denied_actions: vec![
                "inspect_raw_private_state".to_string(),
                "treat_projection_as_authority".to_string(),
            ],
        },
        RuntimeV2CitizenStateSubstrateAudienceView {
            audience: "public".to_string(),
            surface_kind: "observatory_projection".to_string(),
            artifact_ref: RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH.to_string(),
            projection_selector: "public".to_string(),
            authority_status: observatory
                .projection_packet
                .projection_authority_status
                .clone(),
            raw_private_state_allowed: false,
            intended_use:
                "public visibility stays minimal and privacy-safe; it cannot reveal lineage internals or private-state hashes"
                    .to_string(),
            denied_actions: vec![
                "inspect_raw_private_state".to_string(),
                "expose_lineage_internals".to_string(),
            ],
        },
    ]
}

fn fixture_matrix() -> Vec<RuntimeV2CitizenStateSubstrateFixture> {
    vec![
        RuntimeV2CitizenStateSubstrateFixture {
            fixture_kind: "valid_state".to_string(),
            artifact_ref:
                "adl/tests/fixtures/runtime_v2/private_state/proto-citizen-alpha.projection.json"
                    .to_string(),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture"
                    .to_string(),
            summary:
                "Valid state projection is hash-linked to canonical private-state authority and remains non-authoritative."
                    .to_string(),
        },
        RuntimeV2CitizenStateSubstrateFixture {
            fixture_kind: "malformed_state".to_string(),
            artifact_ref:
                "adl/tests/fixtures/runtime_v2/private_state/envelope_negative_cases.json"
                    .to_string(),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_envelope -- --nocapture"
                    .to_string(),
            summary:
                "Malformed or unsafe state mutations fail closed rather than being accepted as citizen-state authority."
                    .to_string(),
        },
        RuntimeV2CitizenStateSubstrateFixture {
            fixture_kind: "stale_state".to_string(),
            artifact_ref:
                "adl/tests/fixtures/runtime_v2/private_state/lineage_negative_cases.json"
                    .to_string(),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_lineage -- --nocapture"
                    .to_string(),
            summary:
                "Stale or disagreeing materialized state enters recovery_or_quarantine instead of silently winning."
                    .to_string(),
        },
        RuntimeV2CitizenStateSubstrateFixture {
            fixture_kind: "overexposed_projection".to_string(),
            artifact_ref:
                "adl/tests/fixtures/runtime_v2/observatory/private_state_projection_negative_cases.json"
                    .to_string(),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture"
                    .to_string(),
            summary:
                "Overexposed public or operator projections are rejected before raw private-state leakage can occur."
                    .to_string(),
        },
    ]
}

fn validate_audience_views(
    audience_views: &[RuntimeV2CitizenStateSubstrateAudienceView],
) -> Result<()> {
    let expected = BTreeSet::from([
        "operator".to_string(),
        "public".to_string(),
        "reviewer".to_string(),
        "runtime".to_string(),
    ]);
    let actual = audience_views
        .iter()
        .map(|view| view.audience.clone())
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(anyhow!(
            "citizen-state substrate must include runtime/operator/reviewer/public audience views"
        ));
    }
    for view in audience_views {
        validate_nonempty_text(&view.surface_kind, "citizen_state_substrate.surface_kind")?;
        validate_relative_path(&view.artifact_ref, "citizen_state_substrate.artifact_ref")?;
        validate_nonempty_text(
            &view.projection_selector,
            "citizen_state_substrate.projection_selector",
        )?;
        validate_nonempty_text(
            &view.authority_status,
            "citizen_state_substrate.authority_status",
        )?;
        validate_nonempty_text(&view.intended_use, "citizen_state_substrate.intended_use")?;
        if view.raw_private_state_allowed {
            return Err(anyhow!(
                "citizen-state substrate audience views must never allow raw private state"
            ));
        }
        if !view
            .denied_actions
            .iter()
            .any(|action| action == "inspect_raw_private_state")
        {
            return Err(anyhow!(
                "citizen-state substrate audience views must deny raw private-state inspection"
            ));
        }
    }
    Ok(())
}

fn validate_fixture_matrix(fixtures: &[RuntimeV2CitizenStateSubstrateFixture]) -> Result<()> {
    let expected = BTreeSet::from([
        "malformed_state".to_string(),
        "overexposed_projection".to_string(),
        "stale_state".to_string(),
        "valid_state".to_string(),
    ]);
    let actual = fixtures
        .iter()
        .map(|fixture| fixture.fixture_kind.clone())
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(anyhow!(
            "citizen-state substrate fixture matrix must cover valid/malformed/stale/overexposed states"
        ));
    }
    for fixture in fixtures {
        validate_relative_path(
            &fixture.artifact_ref,
            "citizen_state_substrate.fixture_artifact_ref",
        )?;
        validate_nonempty_text(
            &fixture.proving_surface,
            "citizen_state_substrate.proving_surface",
        )?;
        validate_nonempty_text(&fixture.summary, "citizen_state_substrate.summary")?;
    }
    Ok(())
}
