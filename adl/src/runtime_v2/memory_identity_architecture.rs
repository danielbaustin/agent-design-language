//! Runtime-v2 memory and identity architecture packet for v0.91.1.
//!
//! This packet documents the bounded memory and identity evidence surfaces that
//! v0.91.1 exposes for downstream ToM, learning, and birthday-readiness work.

use super::*;
use crate::obsmem_contract::{
    MemoryCitation, MemoryQuery, MemoryTraceRef, MemoryWriteRequest, OBSMEM_CONTRACT_VERSION,
};
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_MEMORY_IDENTITY_ARCHITECTURE_SCHEMA: &str =
    "runtime_v2.memory_identity_architecture_packet.v1";
pub const RUNTIME_V2_MEMORY_IDENTITY_ARCHITECTURE_PATH: &str =
    "runtime_v2/memory_identity/memory_identity_architecture.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2MemoryIdentitySurface {
    pub surface_kind: String,
    pub artifact_ref: String,
    pub authority_status: String,
    pub evidence_role: String,
    pub allows_private_state_inspection: bool,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2MemoryIdentityFixture {
    pub fixture_kind: String,
    pub artifact_ref: String,
    pub proving_surface: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2MemoryIdentityArchitecturePacket {
    pub schema_version: String,
    pub architecture_id: String,
    pub demo_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub state_dependency_ref: String,
    pub identity_evidence_refs: Vec<String>,
    pub memory_surfaces: Vec<RuntimeV2MemoryIdentitySurface>,
    pub memory_write_example: MemoryWriteRequest,
    pub memory_query_example: MemoryQuery,
    pub fixture_matrix: Vec<RuntimeV2MemoryIdentityFixture>,
    pub handoff_targets: Vec<String>,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

impl RuntimeV2MemoryIdentityArchitecturePacket {
    pub fn prototype() -> Result<Self> {
        let citizen_state = runtime_v2_citizen_state_substrate_contract()?;
        let boot = runtime_v2_csm_boot_admission_contract()?;
        let lineage = runtime_v2_private_state_lineage_contract()?;
        let witness = runtime_v2_private_state_witness_contract()?;
        let observatory = runtime_v2_private_state_observatory_contract()?;

        let packet = Self {
            schema_version: RUNTIME_V2_MEMORY_IDENTITY_ARCHITECTURE_SCHEMA.to_string(),
            architecture_id: "memory-identity-architecture-v0-91-1-wp-07".to_string(),
            demo_id: "memory_tom_evidence_demo".to_string(),
            milestone: "v0.91.1".to_string(),
            wp: "WP-07".to_string(),
            artifact_path: RUNTIME_V2_MEMORY_IDENTITY_ARCHITECTURE_PATH.to_string(),
            source_feature_doc:
                "docs/milestones/v0.91.1/features/MEMORY_IDENTITY_ARCHITECTURE.md".to_string(),
            state_dependency_ref: citizen_state.artifact_path.clone(),
            identity_evidence_refs: identity_evidence_refs(&boot, &lineage, &witness, &observatory),
            memory_surfaces: memory_surfaces(&boot, &lineage, &witness, &observatory),
            memory_write_example: memory_write_example(&boot, &witness, &observatory),
            memory_query_example: memory_query_example(),
            fixture_matrix: fixture_matrix(),
            handoff_targets: vec![
                "WP-08".to_string(),
                "WP-11".to_string(),
                "WP-23".to_string(),
            ],
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_memory_identity_architecture -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml obsmem_contract -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_witness -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture".to_string(),
                "git diff --check".to_string(),
            ],
            claim_boundary:
                "WP-07 proves one bounded v0.91.1 memory/identity architecture packet over existing boot admission, state, continuity witness, citizen receipt, observatory, and ObsMem citation surfaces. It does not prove full identity continuity, first true birthday semantics, metaphysical personhood, or hidden memory authority."
                    .to_string(),
            non_claims: vec![
                "does not prove full identity continuity".to_string(),
                "does not prove the first true birthday or rebinding event".to_string(),
                "does not permit raw private-state or hidden memory inspection".to_string(),
                "does not turn memory citations or receipts into final metaphysical identity claims".to_string(),
            ],
        };
        packet.validate_against(&citizen_state, &boot, &lineage, &witness, &observatory)?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_MEMORY_IDENTITY_ARCHITECTURE_SCHEMA {
            return Err(anyhow!(
                "unsupported memory/identity architecture schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.architecture_id.clone(),
            "memory_identity.architecture_id",
        )?;
        if self.demo_id != "memory_tom_evidence_demo" {
            return Err(anyhow!(
                "memory/identity architecture must target the shared memory/ToM evidence demo route"
            ));
        }
        if self.milestone != "v0.91.1" {
            return Err(anyhow!(
                "memory/identity architecture must target milestone v0.91.1"
            ));
        }
        if self.wp != "WP-07" {
            return Err(anyhow!(
                "memory/identity architecture must remain bound to WP-07"
            ));
        }
        validate_relative_path(&self.artifact_path, "memory_identity.artifact_path")?;
        if self.source_feature_doc
            != "docs/milestones/v0.91.1/features/MEMORY_IDENTITY_ARCHITECTURE.md"
        {
            return Err(anyhow!(
                "memory/identity architecture must point at the v0.91.1 feature doc"
            ));
        }
        validate_relative_path(
            &self.source_feature_doc,
            "memory_identity.source_feature_doc",
        )?;
        if self.state_dependency_ref != RUNTIME_V2_CITIZEN_STATE_SUBSTRATE_PATH {
            return Err(anyhow!(
                "memory/identity architecture must depend on the landed citizen-state substrate packet"
            ));
        }
        for reference in &self.identity_evidence_refs {
            validate_relative_path(reference, "memory_identity.identity_evidence_ref")?;
        }
        validate_memory_surfaces(&self.memory_surfaces)?;
        self.memory_write_example
            .validate()
            .map_err(|err| anyhow!(err))?;
        self.memory_query_example
            .validate()
            .map_err(|err| anyhow!(err))?;
        ensure_memory_write_is_normalized(&self.memory_write_example)?;
        ensure_memory_query_is_normalized(&self.memory_query_example)?;
        validate_fixture_matrix(&self.fixture_matrix)?;
        if self.handoff_targets != ["WP-08", "WP-11", "WP-23"] {
            return Err(anyhow!(
                "memory/identity architecture handoff targets must remain WP-08, WP-11, and WP-23"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_memory_identity_architecture"))
        {
            return Err(anyhow!(
                "memory/identity architecture must include its focused validation command"
            ));
        }
        if !self
            .claim_boundary
            .contains("does not prove full identity continuity")
        {
            return Err(anyhow!(
                "memory/identity architecture must preserve the bounded identity claim boundary"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("first true birthday"))
        {
            return Err(anyhow!(
                "memory/identity architecture must preserve the birthday non-claim"
            ));
        }
        Ok(())
    }

    pub fn validate_against(
        &self,
        citizen_state: &RuntimeV2CitizenStateSubstratePacket,
        boot: &RuntimeV2CsmBootAdmissionArtifacts,
        lineage: &RuntimeV2PrivateStateLineageArtifacts,
        witness: &RuntimeV2PrivateStateWitnessArtifacts,
        observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
    ) -> Result<()> {
        self.validate()?;
        citizen_state.validate()?;
        boot.validate()?;
        lineage.validate()?;
        witness.validate()?;
        observatory.validate()?;

        if self.state_dependency_ref != citizen_state.artifact_path {
            return Err(anyhow!(
                "memory/identity architecture state dependency drifted from citizen-state substrate"
            ));
        }
        if self.identity_evidence_refs
            != identity_evidence_refs(boot, lineage, witness, observatory)
        {
            return Err(anyhow!(
                "memory/identity architecture identity_evidence_refs must stay aligned with canonical boot, lineage, witness, receipt, and observatory artifacts"
            ));
        }

        let roster_surface = surface_by_kind(&self.memory_surfaces, "citizen_roster_memory_roots")?;
        if roster_surface.artifact_ref != boot.citizen_roster.artifact_path {
            return Err(anyhow!(
                "memory/identity architecture citizen-roster surface must bind the boot roster artifact"
            ));
        }
        if !boot
            .citizen_roster
            .entries
            .iter()
            .all(|entry| entry.identity_handle.starts_with("runtime-v2://"))
        {
            return Err(anyhow!(
                "boot roster identity handles must remain runtime-v2 scoped"
            ));
        }
        if !boot
            .citizen_roster
            .entries
            .iter()
            .all(|entry| entry.memory_root_ref.starts_with("runtime_v2/citizens/"))
        {
            return Err(anyhow!(
                "boot roster memory_root_ref paths must remain repo-relative citizen memory roots"
            ));
        }

        let lineage_surface =
            surface_by_kind(&self.memory_surfaces, "private_state_lineage_ledger")?;
        if lineage_surface.artifact_ref != lineage.ledger.artifact_path {
            return Err(anyhow!(
                "memory/identity architecture lineage surface must bind the lineage ledger"
            ));
        }
        if !lineage
            .ledger
            .authority_rule
            .contains("authoritative continuity record")
        {
            return Err(anyhow!(
                "lineage ledger must preserve authoritative continuity wording"
            ));
        }

        let witness_surface = surface_by_kind(&self.memory_surfaces, "continuity_witness_set")?;
        if witness_surface.artifact_ref != witness.witness_set.artifact_path {
            return Err(anyhow!(
                "memory/identity architecture witness surface must bind the witness set"
            ));
        }
        if !witness
            .witness_set
            .claim_boundary
            .contains("do not expose private state")
        {
            return Err(anyhow!(
                "witness set must preserve non-private-state claim boundary"
            ));
        }

        let receipt_surface = surface_by_kind(&self.memory_surfaces, "citizen_receipt_set")?;
        if receipt_surface.artifact_ref != witness.receipt_set.artifact_path {
            return Err(anyhow!(
                "memory/identity architecture receipt surface must bind the citizen receipt set"
            ));
        }
        if !witness
            .receipt_set
            .receipts
            .iter()
            .all(|receipt| !receipt.privacy_boundary.is_empty())
        {
            return Err(anyhow!(
                "citizen receipts must preserve explicit privacy boundaries"
            ));
        }

        let observatory_surface =
            surface_by_kind(&self.memory_surfaces, "observatory_projection_packet")?;
        if observatory_surface.artifact_ref != observatory.projection_packet.artifact_path {
            return Err(anyhow!(
                "memory/identity architecture observatory surface must bind the projection packet"
            ));
        }
        if observatory_surface.authority_status != "read_only_projection_not_authority" {
            return Err(anyhow!(
                "memory/identity architecture observatory projection surface must remain read-only and non-authoritative"
            ));
        }
        if observatory
            .projection_packet
            .projections
            .iter()
            .any(|projection| projection.raw_private_state_present)
        {
            return Err(anyhow!(
                "observatory projections must not expose raw private state"
            ));
        }

        let obsmem_models_surface =
            surface_by_kind(&self.memory_surfaces, "obsmem_write_contract")?;
        if obsmem_models_surface.artifact_ref != "adl/src/obsmem_contract/models.rs" {
            return Err(anyhow!(
                "memory/identity architecture must point the ObsMem write-contract surface at the canonical models module"
            ));
        }

        let obsmem_index_surface = surface_by_kind(&self.memory_surfaces, "indexed_memory_entry")?;
        if obsmem_index_surface.artifact_ref != "adl/src/obsmem_indexing.rs" {
            return Err(anyhow!(
                "memory/identity architecture must point the indexed-memory surface at the canonical indexing module"
            ));
        }

        let citation_paths = self
            .memory_write_example
            .citations
            .iter()
            .map(|citation| citation.path.as_str())
            .collect::<BTreeSet<_>>();
        for expected in [
            boot.citizen_roster.artifact_path.as_str(),
            witness.witness_set.artifact_path.as_str(),
            observatory.projection_packet.artifact_path.as_str(),
        ] {
            if !citation_paths.contains(expected) {
                return Err(anyhow!(
                    "memory write example must cite {expected} as explicit evidence"
                ));
            }
        }
        if self.fixture_matrix != fixture_matrix() {
            return Err(anyhow!(
                "memory/identity architecture fixture_matrix must stay aligned with the canonical memory, witness, receipt, and observatory proof fixtures"
            ));
        }

        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize memory/identity architecture packet")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        write_relative(
            root.as_ref(),
            RUNTIME_V2_MEMORY_IDENTITY_ARCHITECTURE_PATH,
            self.pretty_json_bytes()?,
        )
    }

    pub fn write_to_path(&self, output_path: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let output_path = output_path.as_ref();
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create memory/identity architecture parent {}",
                    parent.display()
                )
            })?;
        }
        std::fs::write(output_path, self.pretty_json_bytes()?).with_context(|| {
            format!(
                "failed to write memory/identity architecture packet to {}",
                output_path.display()
            )
        })
    }
}

fn memory_surfaces(
    boot: &RuntimeV2CsmBootAdmissionArtifacts,
    lineage: &RuntimeV2PrivateStateLineageArtifacts,
    witness: &RuntimeV2PrivateStateWitnessArtifacts,
    observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
) -> Vec<RuntimeV2MemoryIdentitySurface> {
    vec![
        RuntimeV2MemoryIdentitySurface {
            surface_kind: "citizen_roster_memory_roots".to_string(),
            artifact_ref: boot.citizen_roster.artifact_path.clone(),
            authority_status: "provisional_boot_identity".to_string(),
            evidence_role:
                "declares runtime-v2 identity handles and citizen memory-root references for admitted workers"
                    .to_string(),
            allows_private_state_inspection: false,
            notes:
                "Boot identity is provisional and bounded; it does not constitute birthday or final continuity proof."
                    .to_string(),
        },
        RuntimeV2MemoryIdentitySurface {
            surface_kind: "private_state_lineage_ledger".to_string(),
            artifact_ref: lineage.ledger.artifact_path.clone(),
            authority_status: "authoritative_continuity_record".to_string(),
            evidence_role:
                "append-only continuity ledger that later memory and identity surfaces must cite rather than replace"
                    .to_string(),
            allows_private_state_inspection: false,
            notes:
                "Lineage proves bounded state succession only; it does not prove metaphysical identity or the first true birthday."
                    .to_string(),
        },
        RuntimeV2MemoryIdentitySurface {
            surface_kind: "continuity_witness_set".to_string(),
            artifact_ref: witness.witness_set.artifact_path.clone(),
            authority_status: "reviewable_transition_evidence".to_string(),
            evidence_role:
                "witnesses tie cited continuity transitions back to ledger, envelope, and checkpoint evidence"
                    .to_string(),
            allows_private_state_inspection: false,
            notes:
                "Witnesses support review and replay; they do not reveal canonical private state."
                    .to_string(),
        },
        RuntimeV2MemoryIdentitySurface {
            surface_kind: "citizen_receipt_set".to_string(),
            artifact_ref: witness.receipt_set.artifact_path.clone(),
            authority_status: "citizen_visible_continuity_explanation".to_string(),
            evidence_role:
                "citizen-facing continuity explanations and privacy-bound evidence summaries"
                    .to_string(),
            allows_private_state_inspection: false,
            notes:
                "Receipts explain continuity without converting privacy-bound evidence into final identity claims."
                    .to_string(),
        },
        RuntimeV2MemoryIdentitySurface {
            surface_kind: "observatory_projection_packet".to_string(),
            artifact_ref: observatory.projection_packet.artifact_path.clone(),
            authority_status: "read_only_projection_not_authority".to_string(),
            evidence_role:
                "operator/reviewer/public observatory packet that keeps continuity visible without raw private-state inspection"
                    .to_string(),
            allows_private_state_inspection: false,
            notes:
                "Projection packets are visibility surfaces only and cannot rebind identity or mutate state."
                    .to_string(),
        },
        RuntimeV2MemoryIdentitySurface {
            surface_kind: "obsmem_write_contract".to_string(),
            artifact_ref: "adl/src/obsmem_contract/models.rs".to_string(),
            authority_status: "memory_citation_contract".to_string(),
            evidence_role:
                "normalized memory write/query contract requiring explicit citations and trace-event references"
                    .to_string(),
            allows_private_state_inspection: false,
            notes:
                "ObsMem memory writes are evidence-bound summaries, not hidden identity or continuity authority."
                    .to_string(),
        },
        RuntimeV2MemoryIdentitySurface {
            surface_kind: "indexed_memory_entry".to_string(),
            artifact_ref: "adl/src/obsmem_indexing.rs".to_string(),
            authority_status: "derived_trace_memory_projection".to_string(),
            evidence_role:
                "trace-derived memory indexing surface that preserves step and event references for replayable review"
                    .to_string(),
            allows_private_state_inspection: false,
            notes:
                "Indexed memory entries summarize traces with explicit refs; they do not create hidden continuity claims."
                    .to_string(),
        },
    ]
}

fn identity_evidence_refs(
    boot: &RuntimeV2CsmBootAdmissionArtifacts,
    lineage: &RuntimeV2PrivateStateLineageArtifacts,
    witness: &RuntimeV2PrivateStateWitnessArtifacts,
    observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
) -> Vec<String> {
    vec![
        boot.citizen_roster.artifact_path.clone(),
        lineage.ledger.artifact_path.clone(),
        witness.witness_set.artifact_path.clone(),
        witness.receipt_set.artifact_path.clone(),
        observatory.projection_packet.artifact_path.clone(),
    ]
}

fn memory_write_example(
    boot: &RuntimeV2CsmBootAdmissionArtifacts,
    witness: &RuntimeV2PrivateStateWitnessArtifacts,
    observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
) -> MemoryWriteRequest {
    let mut request = MemoryWriteRequest {
        contract_version: OBSMEM_CONTRACT_VERSION,
        run_id: "memory-identity-architecture-0001".to_string(),
        workflow_id: "runtime_v2_memory_identity_architecture".to_string(),
        trace_bundle_rel_path: boot.admission_trace_path.clone(),
        activation_log_rel_path: boot.admission_trace_path.clone(),
        failure_code: None,
        summary:
            "bounded memory/identity evidence cites roster, witness, and observatory artifacts without claiming full identity continuity"
                .to_string(),
        tags: vec![
            "continuity".to_string(),
            "memory-identity".to_string(),
            "reviewable-evidence".to_string(),
        ],
        citations: vec![
            MemoryCitation {
                path: boot.citizen_roster.artifact_path.clone(),
                hash: "boot-roster-hash-0001".to_string(),
            },
            MemoryCitation {
                path: witness.witness_set.artifact_path.clone(),
                hash: "continuity-witness-hash-0001".to_string(),
            },
            MemoryCitation {
                path: observatory.projection_packet.artifact_path.clone(),
                hash: "observatory-projection-hash-0001".to_string(),
            },
        ],
        trace_event_refs: vec![
            MemoryTraceRef {
                event_sequence: 2,
                event_kind: "admit_worker_citizen".to_string(),
                step_id: None,
                delegation_id: None,
            },
            MemoryTraceRef {
                event_sequence: 4,
                event_kind: "record_boot_manifest".to_string(),
                step_id: None,
                delegation_id: None,
            },
        ],
    };
    request.normalize();
    request
}

fn memory_query_example() -> MemoryQuery {
    let mut query = MemoryQuery {
        contract_version: OBSMEM_CONTRACT_VERSION,
        workflow_id: Some("runtime_v2_memory_identity_architecture".to_string()),
        failure_code: None,
        tags: vec![
            "reviewable-evidence".to_string(),
            "memory-identity".to_string(),
            "continuity".to_string(),
        ],
        limit: 5,
    };
    query.normalize();
    query
}

fn fixture_matrix() -> Vec<RuntimeV2MemoryIdentityFixture> {
    vec![
        RuntimeV2MemoryIdentityFixture {
            fixture_kind: "memory_root_binding".to_string(),
            artifact_ref: "adl/tests/fixtures/runtime_v2/csm_run/citizen_roster.json".to_string(),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_boot_admission -- --nocapture"
                    .to_string(),
            summary:
                "Boot admission exposes reviewable identity handles and memory-root refs without claiming final identity."
                    .to_string(),
        },
        RuntimeV2MemoryIdentityFixture {
            fixture_kind: "continuity_witness_evidence".to_string(),
            artifact_ref: "adl/tests/fixtures/runtime_v2/private_state/continuity_witnesses.json"
                .to_string(),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_witness -- --nocapture"
                    .to_string(),
            summary:
                "Continuity witnesses cite ledger and checkpoint evidence instead of hiding continuity claims."
                    .to_string(),
        },
        RuntimeV2MemoryIdentityFixture {
            fixture_kind: "citizen_receipt_evidence".to_string(),
            artifact_ref: "adl/tests/fixtures/runtime_v2/private_state/citizen_receipts.json"
                .to_string(),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_witness -- --nocapture"
                    .to_string(),
            summary:
                "Citizen receipts remain privacy-bound and explain continuity without escalating into final identity proof."
                    .to_string(),
        },
        RuntimeV2MemoryIdentityFixture {
            fixture_kind: "observatory_projection_evidence".to_string(),
            artifact_ref:
                "adl/tests/fixtures/runtime_v2/observatory/private_state_projection_packet.json"
                    .to_string(),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture"
                    .to_string(),
            summary:
                "Observatory projections keep memory/continuity evidence visible without raw private-state exposure."
                    .to_string(),
        },
    ]
}

fn validate_memory_surfaces(surfaces: &[RuntimeV2MemoryIdentitySurface]) -> Result<()> {
    let expected = BTreeSet::from([
        "citizen_roster_memory_roots".to_string(),
        "private_state_lineage_ledger".to_string(),
        "continuity_witness_set".to_string(),
        "citizen_receipt_set".to_string(),
        "observatory_projection_packet".to_string(),
        "obsmem_write_contract".to_string(),
        "indexed_memory_entry".to_string(),
    ]);
    let actual = surfaces
        .iter()
        .map(|surface| surface.surface_kind.clone())
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(anyhow!(
            "memory/identity architecture must cover roster, lineage, witness, receipt, observatory, and ObsMem surfaces"
        ));
    }
    for surface in surfaces {
        validate_relative_path(&surface.artifact_ref, "memory_identity.artifact_ref")?;
        validate_nonempty_text(
            &surface.authority_status,
            "memory_identity.authority_status",
        )?;
        validate_nonempty_text(&surface.evidence_role, "memory_identity.evidence_role")?;
        validate_nonempty_text(&surface.notes, "memory_identity.notes")?;
        if surface.allows_private_state_inspection {
            return Err(anyhow!(
                "memory/identity architecture surfaces must never allow raw private-state inspection"
            ));
        }
    }
    Ok(())
}

fn validate_fixture_matrix(fixtures: &[RuntimeV2MemoryIdentityFixture]) -> Result<()> {
    let expected = BTreeSet::from([
        "memory_root_binding".to_string(),
        "continuity_witness_evidence".to_string(),
        "citizen_receipt_evidence".to_string(),
        "observatory_projection_evidence".to_string(),
    ]);
    let actual = fixtures
        .iter()
        .map(|fixture| fixture.fixture_kind.clone())
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(anyhow!(
            "memory/identity architecture fixtures must cover roster, witness, receipt, and observatory evidence"
        ));
    }
    for fixture in fixtures {
        validate_relative_path(&fixture.artifact_ref, "memory_identity.fixture_ref")?;
        validate_nonempty_text(&fixture.proving_surface, "memory_identity.proving_surface")?;
        validate_nonempty_text(&fixture.summary, "memory_identity.fixture_summary")?;
    }
    Ok(())
}

fn ensure_memory_write_is_normalized(request: &MemoryWriteRequest) -> Result<()> {
    let mut normalized = request.clone();
    normalized.normalize();
    if &normalized != request {
        return Err(anyhow!(
            "memory/identity architecture memory_write_example must be pre-normalized"
        ));
    }
    Ok(())
}

fn ensure_memory_query_is_normalized(query: &MemoryQuery) -> Result<()> {
    let mut normalized = query.clone();
    normalized.normalize();
    if &normalized != query {
        return Err(anyhow!(
            "memory/identity architecture memory_query_example must be pre-normalized"
        ));
    }
    Ok(())
}

fn surface_by_kind<'a>(
    surfaces: &'a [RuntimeV2MemoryIdentitySurface],
    kind: &str,
) -> Result<&'a RuntimeV2MemoryIdentitySurface> {
    surfaces
        .iter()
        .find(|surface| surface.surface_kind == kind)
        .ok_or_else(|| anyhow!("missing {kind} surface"))
}
