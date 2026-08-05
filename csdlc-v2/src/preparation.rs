use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use fs2::FileExt;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::cards::{
    digest, initial_cards, render, validate_cross_card, CardContent, CardKind, CardValues,
    InitialCardInput, PlanningProfile,
};
use crate::error::{ErrorCode, Result, V2Error};
use crate::lifecycle::{
    bind_issue, initialize_prepared_issue_under_binding_lock, validate_validation_lanes,
    BindRequest, BindResult,
};
use crate::model::{Claim, DesignReview, LifecyclePhase};
use crate::store::{now_seconds, BootstrapRequest, Store};

const PREPARATION_SCHEMA: &str = "csdlc.preparation_manifest.v1";
const GENERATION_SCHEMA: &str = "csdlc.prepared_generation.v1";
const RECEIPT_SCHEMA: &str = "csdlc.execution_readiness_receipt.v1";
const INTENT_SCHEMA: &str = "csdlc.binding_intent.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PreparationState {
    Draft,
    Prepared,
    ExecutionReady,
    Binding,
    Bound,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IssueCreateRequest {
    pub issue: u64,
    pub repository: String,
    pub title: String,
    pub slug: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IssueDraft {
    pub schema: String,
    pub issue: u64,
    pub repository: String,
    pub title: String,
    pub slug: String,
    pub version: String,
    pub state: PreparationState,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DependencyRevision {
    pub issue: u64,
    pub revision: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PrepareSyncRequest {
    pub issue: u64,
    pub repository: String,
    pub design_path: String,
    pub diagram_path: String,
    pub design_reviewer: String,
    #[serde(default)]
    pub design_approved: bool,
    pub owned_paths: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<DependencyRevision>,
    pub base_revision: String,
    pub initial: InitialCardInput,
    #[serde(default)]
    pub expected_manifest_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PreparedGeneration {
    pub schema: String,
    pub issue: u64,
    pub repository: String,
    pub sequence: u64,
    pub generation_id: String,
    pub semantic_digest: String,
    pub design_path: String,
    pub design_digest: String,
    pub diagram_path: String,
    pub diagram_digest: String,
    pub design_reviewer: String,
    pub design_approved: bool,
    pub owned_paths: Vec<String>,
    pub dependencies: Vec<DependencyRevision>,
    pub base_revision: String,
    pub initial: InitialCardInput,
    pub cards: BTreeMap<CardKind, CardValues>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PreparationManifest {
    pub schema: String,
    pub issue: u64,
    pub state: PreparationState,
    pub current_generation: Option<String>,
    pub current_sequence: u64,
    pub semantic_digest: Option<String>,
    pub receipt_digest: Option<String>,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PrepareSealRequest {
    pub issue: u64,
    pub expected_generation: String,
    pub expected_semantic_digest: String,
    pub expected_manifest_digest: String,
    pub dependencies: Vec<DependencyRevision>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionReadinessReceipt {
    pub schema: String,
    pub issue: u64,
    pub generation_id: String,
    pub semantic_digest: String,
    pub manifest_digest: String,
    pub base_revision: String,
    pub dependencies: Vec<DependencyRevision>,
    pub owned_paths: Vec<String>,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PrepareRunRequest {
    pub sync: PrepareSyncRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PrepareRunResult {
    pub generation: PreparedGeneration,
    pub receipt: Option<ExecutionReadinessReceipt>,
    pub next_operation: String,
    pub error: Option<String>,
    pub error_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PrepareBatchRequest {
    pub batch_id: String,
    pub children: Vec<PrepareRunRequest>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BatchChildOutcome {
    ExecutionReady,
    Prepared,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BatchChildResult {
    pub issue: u64,
    pub outcome: BatchChildOutcome,
    pub generation_id: Option<String>,
    pub receipt_digest: Option<String>,
    pub warnings: Vec<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PrepareBatchResult {
    pub schema: String,
    pub batch_id: String,
    pub ready: bool,
    pub cycle_issues: Vec<u64>,
    pub overlap_issues: Vec<u64>,
    pub children: Vec<BatchChildResult>,
    pub next_operation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegacyPreparationMigrationRequest {
    pub issue: u64,
    pub expected_legacy_digest: String,
    pub actor: String,
    pub reason: String,
    pub base_revision: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LegacyPreparationDisposition {
    ImportedPrepared,
    RetainedActive,
    RetainedTerminal,
    Quarantined,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegacyPreparationMigrationResult {
    pub schema: String,
    pub issue: u64,
    pub disposition: LegacyPreparationDisposition,
    pub original_digest: String,
    pub resulting_digest: Option<String>,
    pub snapshot_path: Option<String>,
    pub next_operation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LegacyPreparationRepairDisposition {
    RetainLegacyAuthority,
    TombstoneStalePreparation,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegacyPreparationRepairRequest {
    pub issue: u64,
    pub expected_legacy_digest: String,
    pub expected_quarantine_digest: String,
    #[serde(default)]
    pub expected_preparation_digest: Option<String>,
    pub quarantine_path: String,
    pub disposition: LegacyPreparationRepairDisposition,
    pub actor: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegacyPreparationRepairResult {
    pub schema: String,
    pub issue: u64,
    pub disposition: LegacyPreparationRepairDisposition,
    pub repaired: bool,
    pub audit_path: String,
    pub next_operation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LegacyPreparationQuarantine {
    schema: String,
    reason: String,
    record: crate::IssueRecord,
    digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DerivedBindRequest {
    pub issue: u64,
    pub session_id: String,
    pub base_branch: String,
    pub expected_base_revision: String,
    pub lease_seconds: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BindingIntentState {
    Reserved,
    Initialized,
    Bound,
    Releasing,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BindingIntent {
    pub schema: String,
    pub issue: u64,
    pub session_id: String,
    pub owner: String,
    pub claim_id: String,
    pub acquired_unix_seconds: u64,
    pub expires_unix_seconds: u64,
    pub receipt_digest: String,
    pub generation_id: String,
    pub base_revision: String,
    pub branch: String,
    pub worktree: String,
    pub branch_preexisting: bool,
    pub worktree_preexisting: bool,
    pub protected_paths: Vec<String>,
    pub state: BindingIntentState,
    pub created_artifacts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub materialized_lifecycle_digest: Option<String>,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DerivedBindResult {
    pub schema: String,
    pub issue: u64,
    pub state: PreparationState,
    pub branch: String,
    pub worktree: String,
    pub owner: String,
    pub bind: BindResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BindReleaseRequest {
    pub issue: u64,
    pub session_id: String,
    #[serde(default)]
    pub expected_intent_digest: Option<String>,
}

struct GovernedSessionAuthority {
    owner: String,
    expires_unix_seconds: u64,
}

#[derive(Debug, Deserialize)]
struct SessionLedger {
    schema: String,
    #[serde(default)]
    global_freeze: Option<SessionLedgerFreeze>,
    #[serde(default)]
    claims: Vec<SessionLedgerClaim>,
}

#[derive(Debug, Deserialize)]
struct SessionLedgerFreeze {
    active: bool,
}

#[derive(Debug, Deserialize)]
struct SessionLedgerClaim {
    session_id: String,
    owner: String,
    mode: String,
    expires_at: String,
    #[serde(default)]
    released_at: Option<String>,
    #[serde(default)]
    github: SessionLedgerGithub,
}

#[derive(Debug, Default, Deserialize)]
struct SessionLedgerGithub {
    issue: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BindReleaseResult {
    pub schema: String,
    pub issue: u64,
    pub released: bool,
    pub removed_artifacts: Vec<String>,
    pub state: PreparationState,
}

pub fn create_issue_draft(store: &Store, request: IssueCreateRequest) -> Result<IssueDraft> {
    validate_identity(request.issue, &request.repository, &request.slug)?;
    let _lock = preparation_lock(store, request.issue)?;
    create_issue_draft_locked(store, request)
}

fn create_issue_draft_locked(store: &Store, request: IssueCreateRequest) -> Result<IssueDraft> {
    let path = issue_preparation_dir(store, request.issue).join("draft.json");
    if path.exists() {
        let existing: IssueDraft = read_json(&path)?;
        verify_draft(&existing)?;
        let candidate = draft_from_request(request)?;
        if existing == candidate {
            return Ok(existing);
        }
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "draft exists with different source intent",
        ));
    }
    let draft = draft_from_request(request)?;
    create_dir_all_safe(path.parent().expect("draft parent"))?;
    atomic_write_json(&path, &draft)?;
    let manifest = manifest(draft.issue, PreparationState::Draft, None, 0, None, None)?;
    atomic_write_json(
        &issue_preparation_dir(store, draft.issue).join("manifest.json"),
        &manifest,
    )?;
    Ok(draft)
}

pub fn sync_preparation(store: &Store, request: PrepareSyncRequest) -> Result<PreparedGeneration> {
    validate_identity(request.issue, &request.repository, &request.initial.slug)?;
    validate_relative(&request.design_path, "design_path")?;
    validate_relative(&request.diagram_path, "diagram_path")?;
    if request.design_path == request.diagram_path || request.owned_paths.is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design, diagram, and owned paths are incomplete",
        ));
    }
    let owned_paths = normalize_paths(&request.owned_paths)?;
    let dependencies = normalize_dependencies(&request.dependencies)?;
    let _lock = preparation_lock(store, request.issue)?;
    let draft: IssueDraft =
        read_json(&issue_preparation_dir(store, request.issue).join("draft.json"))?;
    verify_draft(&draft)?;
    if draft.repository != request.repository || draft.slug != request.initial.slug {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "sync identity does not match draft",
        ));
    }
    let current = load_manifest_optional(store, request.issue)?;
    if current.as_ref().is_some_and(|manifest| {
        matches!(
            manifest.state,
            PreparationState::Binding | PreparationState::Bound
        )
    }) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "bound preparation must be edited through the canonical issue lifecycle",
        ));
    }
    if store.issue_dir(request.issue).exists() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "canonical issue authority must be migrated before preparation can be synchronized",
        ));
    }
    if let Some(expected) = &request.expected_manifest_digest {
        if current.as_ref().map(|value| &value.digest) != Some(expected) {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "preparation manifest changed before sync",
            ));
        }
    } else if current
        .as_ref()
        .is_some_and(|manifest| manifest.current_generation.is_some())
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "successor sync requires the current manifest digest",
        ));
    }
    let design =
        crate::store::read_regular_projection(store.root(), Path::new(&request.design_path))?;
    let diagram =
        crate::store::read_regular_projection(store.root(), Path::new(&request.diagram_path))?;
    let design_digest = digest(&design);
    let diagram_digest = digest(&diagram);
    let sequence = current
        .as_ref()
        .map_or(1, |value| value.current_sequence + 1);
    let cards = initial_cards(
        request.issue,
        &request.repository,
        &request.design_path,
        &design_digest,
        &request.diagram_path,
        &diagram_digest,
        request.initial.clone(),
    )?;
    validate_cross_card(
        &cards,
        &request.design_path,
        &design_digest,
        &request.diagram_path,
        &diagram_digest,
    )?;
    let semantic_digest = semantic_digest(&SemanticPayload {
        issue: request.issue,
        repository: &request.repository,
        design_digest: &design_digest,
        diagram_digest: &diagram_digest,
        owned_paths: &owned_paths,
        dependencies: &dependencies,
        design_approved: request.design_approved,
        design_reviewer: &request.design_reviewer,
        initial: &request.initial,
        base_revision: &request.base_revision,
        cards: &cards,
    })?;
    let generation_id = format!("{sequence}-{}", &semantic_digest[..16]);
    let generation = PreparedGeneration {
        schema: GENERATION_SCHEMA.into(),
        issue: request.issue,
        repository: request.repository,
        sequence,
        generation_id: generation_id.clone(),
        semantic_digest: semantic_digest.clone(),
        design_path: request.design_path,
        design_digest,
        diagram_path: request.diagram_path,
        diagram_digest,
        design_reviewer: request.design_reviewer,
        design_approved: request.design_approved,
        owned_paths,
        dependencies,
        base_revision: request.base_revision,
        initial: request.initial,
        cards,
    };
    write_generation(store, &generation, &design, &diagram)?;
    let next = manifest(
        generation.issue,
        PreparationState::Prepared,
        Some(generation_id),
        sequence,
        Some(semantic_digest),
        None,
    )?;
    atomic_write_json(
        &issue_preparation_dir(store, generation.issue).join("manifest.json"),
        &next,
    )?;
    Ok(generation)
}

pub fn seal_preparation(
    store: &Store,
    request: PrepareSealRequest,
) -> Result<ExecutionReadinessReceipt> {
    let _lock = preparation_lock(store, request.issue)?;
    let manifest_path = issue_preparation_dir(store, request.issue).join("manifest.json");
    let current = load_manifest(store, request.issue)?;
    if current.digest != request.expected_manifest_digest
        || current.current_generation.as_deref() != Some(&request.expected_generation)
        || current.semantic_digest.as_deref() != Some(&request.expected_semantic_digest)
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "prepared generation changed before seal",
        ));
    }
    let generation = load_generation(store, request.issue, &request.expected_generation)?;
    let dependencies = normalize_dependencies(&request.dependencies)?;
    if generation.dependencies != dependencies {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "dependency vector changed before seal",
        ));
    }
    validate_generation_for_seal(store, &generation)?;
    let actual_revision = crate::git::run(store.root(), &["rev-parse", "HEAD"])?.stdout;
    if actual_revision != generation.base_revision {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "prepared base revision is not the current Git revision",
        ));
    }
    validate_live_dependencies(store, &generation.dependencies)?;
    let mut receipt = ExecutionReadinessReceipt {
        schema: RECEIPT_SCHEMA.into(),
        issue: generation.issue,
        generation_id: generation.generation_id.clone(),
        semantic_digest: generation.semantic_digest.clone(),
        manifest_digest: current.digest.clone(),
        base_revision: generation.base_revision.clone(),
        dependencies: generation.dependencies.clone(),
        owned_paths: generation.owned_paths.clone(),
        digest: String::new(),
    };
    receipt.digest = object_digest(&receipt)?;
    write_immutable_json(
        &issue_preparation_dir(store, request.issue)
            .join("receipts")
            .join(format!("{}.json", receipt.digest)),
        &receipt,
    )?;
    atomic_write_json(
        &issue_preparation_dir(store, request.issue).join("receipt.json"),
        &receipt,
    )?;
    let sealed = manifest(
        request.issue,
        PreparationState::ExecutionReady,
        Some(generation.generation_id),
        generation.sequence,
        Some(generation.semantic_digest),
        Some(receipt.digest.clone()),
    )?;
    atomic_write_json(&manifest_path, &sealed)?;
    Ok(receipt)
}

pub fn run_preparation(store: &Store, request: PrepareRunRequest) -> Result<PrepareRunResult> {
    let generation = sync_preparation(store, request.sync)?;
    let manifest = load_manifest(store, generation.issue)?;
    let seal = PrepareSealRequest {
        issue: generation.issue,
        expected_generation: generation.generation_id.clone(),
        expected_semantic_digest: generation.semantic_digest.clone(),
        expected_manifest_digest: manifest.digest,
        dependencies: generation.dependencies.clone(),
    };
    match seal_preparation(store, seal) {
        Ok(receipt) => Ok(PrepareRunResult {
            generation,
            receipt: Some(receipt),
            next_operation: "csdlc-bind run".into(),
            error: None,
            error_code: None,
        }),
        Err(error) => {
            let next_operation = match error.code {
                ErrorCode::StaleDigest | ErrorCode::StaleGeneration => "csdlc-prepare sync",
                ErrorCode::CorruptRecord | ErrorCode::InterruptedTransaction => {
                    "csdlc-migrate repair"
                }
                _ => "csdlc-prepare seal",
            };
            Ok(PrepareRunResult {
                generation,
                receipt: None,
                next_operation: next_operation.into(),
                error: Some(error.to_string()),
                error_code: Some(error.code.to_string()),
            })
        }
    }
}

pub fn run_preparation_batch(
    store: &Store,
    request: PrepareBatchRequest,
) -> Result<PrepareBatchResult> {
    if request.batch_id.trim().is_empty() || request.children.is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "batch_id and at least one child are required",
        ));
    }
    let mut issues = BTreeSet::new();
    for child in &request.children {
        if !issues.insert(child.sync.issue) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "batch contains duplicate issue",
            ));
        }
    }
    let cycle_issues = dependency_cycle_issues(&request.children);
    let overlap_issues = intra_batch_overlap_issues(&request.children);
    let mut children = Vec::with_capacity(request.children.len());
    for child in request.children {
        let issue = child.sync.issue;
        let mut warnings = Vec::new();
        if cycle_issues.contains(&issue) {
            warnings.push("dependency cycle participates in this child".into());
        }
        if overlap_issues.contains(&issue) {
            warnings.push("owned paths overlap another child in this batch".into());
        }
        match run_preparation(store, child) {
            Ok(result) => children.push(BatchChildResult {
                issue,
                outcome: if result.receipt.is_some() {
                    BatchChildOutcome::ExecutionReady
                } else {
                    BatchChildOutcome::Prepared
                },
                generation_id: Some(result.generation.generation_id),
                receipt_digest: result.receipt.map(|receipt| receipt.digest),
                warnings,
                error: result.error,
            }),
            Err(error) => children.push(BatchChildResult {
                issue,
                outcome: BatchChildOutcome::Failed,
                generation_id: None,
                receipt_digest: None,
                warnings,
                error: Some(error.to_string()),
            }),
        }
    }
    let ready = cycle_issues.is_empty()
        && overlap_issues.is_empty()
        && children
            .iter()
            .all(|child| child.outcome == BatchChildOutcome::ExecutionReady);
    let next_operation = if ready {
        "csdlc-bind run"
    } else if !cycle_issues.is_empty() || !overlap_issues.is_empty() {
        "resolve_batch_conflicts"
    } else {
        "retry_non_ready_children"
    };
    let result = PrepareBatchResult {
        schema: "csdlc.prepare_batch_result.v1".into(),
        batch_id: request.batch_id,
        ready,
        cycle_issues: cycle_issues.into_iter().collect(),
        overlap_issues: overlap_issues.into_iter().collect(),
        children,
        next_operation: next_operation.into(),
    };
    validate_component(&result.batch_id, "batch_id")?;
    write_immutable_json(
        &store
            .root()
            .join(".csdlc/preparation/batches")
            .join(format!("{}.json", result.batch_id)),
        &result,
    )?;
    Ok(result)
}

pub fn migrate_legacy_preparation(
    store: &Store,
    request: LegacyPreparationMigrationRequest,
) -> Result<LegacyPreparationMigrationResult> {
    if request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
        || request.base_revision.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "migration actor, reason, and base revision are required",
        ));
    }
    let _preparation = preparation_lock(store, request.issue)?;
    let record = store.load_record(request.issue)?;
    if record.digest != request.expected_legacy_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "legacy issue changed before migration",
        ));
    }
    let preparation_dir = issue_preparation_dir(store, request.issue);
    if preparation_dir.exists() {
        let snapshot = preparation_dir
            .join("migration")
            .join(format!("legacy-{}.json", record.digest));
        if !snapshot.exists() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "canonical legacy authority coexists with unrelated preparation state",
            ));
        }
        let retained: crate::IssueRecord = read_json(&snapshot)?;
        if retained.digest != record.digest {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "interrupted migration snapshot does not match canonical authority",
            ));
        }
        fs::remove_dir_all(&preparation_dir)?;
    }
    if matches!(
        record.phase,
        LifecyclePhase::Merged | LifecyclePhase::ClosedOut
    ) {
        return Ok(LegacyPreparationMigrationResult {
            schema: "csdlc.legacy_preparation_migration_result.v1".into(),
            issue: request.issue,
            disposition: LegacyPreparationDisposition::RetainedTerminal,
            original_digest: record.digest,
            resulting_digest: None,
            snapshot_path: None,
            next_operation: "none".into(),
        });
    }
    if !matches!(
        record.phase,
        LifecyclePhase::Initialized | LifecyclePhase::Ready
    ) {
        return Ok(LegacyPreparationMigrationResult {
            schema: "csdlc.legacy_preparation_migration_result.v1".into(),
            issue: request.issue,
            disposition: LegacyPreparationDisposition::RetainedActive,
            original_digest: record.digest,
            resulting_digest: None,
            snapshot_path: None,
            next_operation: "continue_existing_lifecycle".into(),
        });
    }
    let Some(claim) = record.claim.clone() else {
        return quarantine_migration(store, &record, "legacy preparation has no provable owner");
    };
    if record.review.is_some() || record.publication.is_some() || record.terminal.is_some() {
        return quarantine_migration(
            store,
            &record,
            "legacy preparation contains execution evidence",
        );
    }
    if legacy_claim_has_active_topology_or_session(store, record.issue, &claim)? {
        return Ok(LegacyPreparationMigrationResult {
            schema: "csdlc.legacy_preparation_migration_result.v1".into(),
            issue: request.issue,
            disposition: LegacyPreparationDisposition::RetainedActive,
            original_digest: record.digest,
            resulting_digest: None,
            snapshot_path: None,
            next_operation: "continue_existing_lifecycle".into(),
        });
    }
    let cards = store.load_cards(request.issue)?;
    let initial = initial_from_cards(&cards)?;
    let design =
        crate::store::read_regular_projection(store.root(), Path::new(&record.design_path))?;
    let diagram =
        crate::store::read_regular_projection(store.root(), Path::new(&record.diagram_path))?;
    let design_digest = digest(&design);
    let diagram_digest = digest(&diagram);
    if validate_cross_card(
        &cards,
        &record.design_path,
        &design_digest,
        &record.diagram_path,
        &diagram_digest,
    )
    .is_err()
    {
        return quarantine_migration(
            store,
            &record,
            "legacy card projections do not match current design inputs",
        );
    }
    let (design_reviewer, design_approved) = match &record.design_review {
        DesignReview::Approved { reviewer, revision } if revision == &design_digest => {
            (reviewer.clone(), true)
        }
        DesignReview::Pending | DesignReview::ChangesRequired { .. } => (String::new(), false),
        DesignReview::Approved { .. } => {
            return quarantine_migration(store, &record, "legacy design approval is stale")
        }
    };
    create_issue_draft_locked(
        store,
        IssueCreateRequest {
            issue: record.issue,
            repository: record.repository.clone(),
            title: initial.title.clone(),
            slug: initial.slug.clone(),
            version: initial.version.clone(),
        },
    )?;
    let current = load_manifest(store, request.issue)?;
    let sequence = current.current_sequence + 1;
    let owned_paths = normalize_paths(&claim.protected_paths)?;
    let semantic_digest = semantic_digest(&SemanticPayload {
        issue: record.issue,
        repository: &record.repository,
        design_digest: &design_digest,
        diagram_digest: &diagram_digest,
        owned_paths: &owned_paths,
        dependencies: &[],
        design_approved,
        design_reviewer: &design_reviewer,
        initial: &initial,
        base_revision: &request.base_revision,
        cards: &cards,
    })?;
    let generation_id = format!("{sequence}-{}", &semantic_digest[..16]);
    let generation = PreparedGeneration {
        schema: GENERATION_SCHEMA.into(),
        issue: record.issue,
        repository: record.repository.clone(),
        sequence,
        generation_id: generation_id.clone(),
        semantic_digest: semantic_digest.clone(),
        design_path: record.design_path.clone(),
        design_digest,
        diagram_path: record.diagram_path.clone(),
        diagram_digest,
        design_reviewer,
        design_approved,
        owned_paths,
        dependencies: Vec::new(),
        base_revision: request.base_revision,
        initial,
        cards,
    };
    let snapshot_relative = format!(
        ".csdlc/preparation/issues/{}/migration/legacy-{}.json",
        request.issue, record.digest
    );
    write_immutable_json(&store.root().join(&snapshot_relative), &record)?;
    write_generation(store, &generation, &design, &diagram)?;
    let next = manifest(
        record.issue,
        PreparationState::Prepared,
        Some(generation_id),
        sequence,
        Some(semantic_digest),
        None,
    )?;
    atomic_write_json(
        &issue_preparation_dir(store, record.issue).join("manifest.json"),
        &next,
    )?;
    if let Err(error) =
        store.remove_unstarted_binding_projection(record.issue, &claim.id, &record.digest)
    {
        if issue_preparation_dir(store, request.issue).exists() {
            fs::remove_dir_all(issue_preparation_dir(store, request.issue))?;
        }
        return Err(error);
    }
    Ok(LegacyPreparationMigrationResult {
        schema: "csdlc.legacy_preparation_migration_result.v1".into(),
        issue: record.issue,
        disposition: LegacyPreparationDisposition::ImportedPrepared,
        original_digest: record.digest,
        resulting_digest: Some(next.digest),
        snapshot_path: Some(snapshot_relative),
        next_operation: "csdlc-prepare seal".into(),
    })
}

pub fn repair_legacy_preparation(
    store: &Store,
    request: LegacyPreparationRepairRequest,
) -> Result<LegacyPreparationRepairResult> {
    if request.issue == 0 || request.actor.trim().is_empty() || request.reason.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "repair issue, actor, and reason are required",
        ));
    }
    if request.expected_legacy_digest.len() != 64 || request.expected_quarantine_digest.len() != 64
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "repair requires full legacy and quarantine digests",
        ));
    }
    validate_relative(&request.quarantine_path, "quarantine_path")?;
    let expected_prefix = format!(".csdlc/preparation/issues/{}/migration/", request.issue);
    if !request.quarantine_path.starts_with(&expected_prefix) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "quarantine packet is outside the issue migration namespace",
        ));
    }
    let packet: LegacyPreparationQuarantine = serde_json::from_slice(
        &crate::store::read_regular_projection(store.root(), Path::new(&request.quarantine_path))?,
    )?;
    let mut payload = packet.clone();
    let claimed = std::mem::take(&mut payload.digest);
    if claimed != object_digest(&payload)? || claimed != request.expected_quarantine_digest {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "quarantine packet digest mismatch",
        ));
    }
    if packet.record.issue != request.issue
        || packet.record.digest != request.expected_legacy_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "quarantined legacy authority does not match the repair request",
        ));
    }

    let _lock = preparation_lock(store, request.issue)?;
    match request.disposition {
        LegacyPreparationRepairDisposition::RetainLegacyAuthority => {
            let current = store.load_record(request.issue)?;
            if current.digest != request.expected_legacy_digest {
                return Err(V2Error::new(
                    ErrorCode::StaleDigest,
                    "canonical legacy authority changed before repair",
                ));
            }
        }
        LegacyPreparationRepairDisposition::TombstoneStalePreparation => {
            if store.issue_dir(request.issue).exists() {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "cannot tombstone preparation while canonical legacy authority exists",
                ));
            }
            let current = load_manifest(store, request.issue)?;
            if request.expected_preparation_digest.as_deref() != Some(&current.digest) {
                return Err(V2Error::new(
                    ErrorCode::StaleDigest,
                    "tombstone requires the current preparation manifest digest",
                ));
            }
            let issue_dir = issue_preparation_dir(store, request.issue);
            for name in ["draft.json", "manifest.json", "receipt.json"] {
                let path = issue_dir.join(name);
                if path.exists() {
                    fs::remove_file(path)?;
                }
            }
            let generations = issue_dir.join("generations");
            if generations.exists() {
                fs::remove_dir_all(generations)?;
            }
        }
    }

    let audit = serde_json::json!({
        "schema": "csdlc.legacy_preparation_repair_audit.v1",
        "issue": request.issue,
        "legacy_digest": request.expected_legacy_digest,
        "quarantine_digest": request.expected_quarantine_digest,
        "quarantine_path": request.quarantine_path,
        "disposition": request.disposition,
        "actor": request.actor,
        "reason": request.reason,
    });
    let audit_digest = digest(&serde_json::to_vec(&audit)?);
    let audit_path = format!(
        ".csdlc/preparation/issues/{}/migration/repair-{}.json",
        request.issue,
        &audit_digest[..16]
    );
    write_immutable_json(&store.root().join(&audit_path), &audit)?;
    Ok(LegacyPreparationRepairResult {
        schema: "csdlc.legacy_preparation_repair_result.v1".into(),
        issue: request.issue,
        disposition: request.disposition,
        repaired: true,
        audit_path,
        next_operation: match request.disposition {
            LegacyPreparationRepairDisposition::RetainLegacyAuthority => {
                "continue_existing_lifecycle".into()
            }
            LegacyPreparationRepairDisposition::TombstoneStalePreparation => {
                "csdlc-issue create".into()
            }
        },
    })
}

fn governed_session_owner(
    store: &Store,
    issue: u64,
    session_id: &str,
    now_unix_seconds: u64,
) -> Result<GovernedSessionAuthority> {
    let common_dir = PathBuf::from(
        crate::git::run(
            store.root(),
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    );
    let primary_root = common_dir.parent().ok_or_else(|| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            "Git common directory has no primary checkout parent",
        )
    })?;
    let ledger_path = primary_root.join(".adl/session-ledger/ledger.json");
    if !ledger_path.exists() {
        return Err(V2Error::new(
            ErrorCode::MissingClaim,
            format!(
                "governed session ledger is unavailable at {}",
                ledger_path.display()
            ),
        ));
    }
    let ledger: SessionLedger =
        read_regular_json(primary_root, Path::new(".adl/session-ledger/ledger.json")).map_err(
            |error| {
                let code = if error.code == ErrorCode::Io {
                    ErrorCode::MissingClaim
                } else {
                    error.code
                };
                V2Error::new(
                    code,
                    format!(
                        "governed session ledger is unavailable at {}: {}",
                        ledger_path.display(),
                        error.message
                    ),
                )
            },
        )?;
    if ledger.schema != "adl.session_ledger.v1" {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "governed session ledger schema is unsupported",
        ));
    }
    if ledger
        .global_freeze
        .as_ref()
        .is_some_and(|freeze| freeze.active)
    {
        return Err(V2Error::new(
            ErrorCode::ClaimCollision,
            "governed session ledger is globally frozen",
        ));
    }
    let mut matching = Vec::new();
    let mut competing = Vec::new();
    for claim in ledger.claims.iter().filter(|claim| {
        claim.github.issue == Some(issue) && claim.mode == "active" && claim.released_at.is_none()
    }) {
        let expires = time::OffsetDateTime::parse(
            &claim.expires_at,
            &time::format_description::well_known::Rfc3339,
        )
        .map_err(|error| V2Error::new(ErrorCode::CorruptRecord, error.to_string()))?;
        if expires.unix_timestamp() <= now_unix_seconds as i64 {
            continue;
        }
        if claim.session_id == session_id {
            matching.push(claim);
        } else {
            competing.push(claim);
        }
    }
    if !competing.is_empty() {
        return Err(V2Error::new(
            ErrorCode::ClaimCollision,
            "another active governed session-ledger claim already owns this issue",
        ));
    }
    let claim = matching.first().copied().ok_or_else(|| {
        V2Error::new(
            ErrorCode::MissingClaim,
            "no active governed session-ledger claim matches this issue and session",
        )
    })?;
    if matching.len() > 1 {
        return Err(V2Error::new(
            ErrorCode::ClaimCollision,
            "multiple active governed session-ledger claims match this issue and session",
        ));
    }
    if claim.owner.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "matching governed session-ledger claim has no owner",
        ));
    }
    let expires = time::OffsetDateTime::parse(
        &claim.expires_at,
        &time::format_description::well_known::Rfc3339,
    )
    .map_err(|error| V2Error::new(ErrorCode::CorruptRecord, error.to_string()))?;
    Ok(GovernedSessionAuthority {
        owner: claim.owner.clone(),
        expires_unix_seconds: u64::try_from(expires.unix_timestamp()).map_err(|_| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "governed session expiry predates the Unix epoch",
            )
        })?,
    })
}

fn legacy_claim_has_active_topology_or_session(
    store: &Store,
    issue: u64,
    claim: &Claim,
) -> Result<bool> {
    if store.root().join(&claim.worktree).exists() {
        return Ok(true);
    }
    let inside_git = std::process::Command::new("git")
        .current_dir(store.root())
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .is_ok_and(|output| output.status.success());
    if inside_git && git_branch_exists(store.root(), &claim.branch)? {
        return Ok(true);
    }
    if !inside_git {
        return Ok(false);
    }
    let common_dir = PathBuf::from(
        crate::git::run(
            store.root(),
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    );
    let Some(primary_root) = common_dir.parent() else {
        return Ok(false);
    };
    let ledger_path = primary_root.join(".adl/session-ledger/ledger.json");
    if !ledger_path.exists() {
        return Ok(false);
    }
    let ledger: SessionLedger =
        read_regular_json(primary_root, Path::new(".adl/session-ledger/ledger.json"))?;
    if ledger.schema != "adl.session_ledger.v1" {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "legacy migration found an unsupported session ledger",
        ));
    }
    let now = now_seconds()? as i64;
    for entry in ledger.claims.iter().filter(|entry| {
        entry.github.issue == Some(issue)
            && entry.owner == claim.owner
            && entry.mode == "active"
            && entry.released_at.is_none()
    }) {
        let expires = time::OffsetDateTime::parse(
            &entry.expires_at,
            &time::format_description::well_known::Rfc3339,
        )
        .map_err(|error| V2Error::new(ErrorCode::CorruptRecord, error.to_string()))?;
        if expires.unix_timestamp() > now {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn run_derived_bind(store: &Store, request: DerivedBindRequest) -> Result<DerivedBindResult> {
    if request.issue == 0 || request.session_id.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue and session_id are required",
        ));
    }
    if request.lease_seconds == 0 {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "lease_seconds must be positive",
        ));
    }
    let _operation = binding_operation_lock(store, request.issue)?;
    let _preparation = preparation_lock(store, request.issue)?;
    let _registry = store.binding_lock()?;
    let manifest = load_manifest(store, request.issue)?;
    if !matches!(
        manifest.state,
        PreparationState::ExecutionReady | PreparationState::Binding | PreparationState::Bound
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "issue is not execution_ready",
        ));
    }
    let receipt: ExecutionReadinessReceipt =
        read_json(&issue_preparation_dir(store, request.issue).join("receipt.json"))?;
    verify_receipt(&receipt)?;
    if manifest.receipt_digest.as_deref() != Some(&receipt.digest) {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "readiness receipt is not current",
        ));
    }
    let generation = load_generation(store, request.issue, &receipt.generation_id)?;
    validate_generation_for_seal(store, &generation)?;
    validate_live_dependencies(store, &generation.dependencies)?;
    if receipt.issue != request.issue
        || receipt.generation_id != generation.generation_id
        || receipt.semantic_digest != generation.semantic_digest
        || receipt.owned_paths != generation.owned_paths
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "readiness receipt does not match its prepared generation",
        ));
    }
    if request.expected_base_revision != receipt.base_revision {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "base revision changed after seal",
        ));
    }
    let draft: IssueDraft =
        read_json(&issue_preparation_dir(store, request.issue).join("draft.json"))?;
    verify_draft(&draft)?;
    if draft.issue != request.issue || draft.repository != generation.repository {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "issue draft does not match its prepared generation",
        ));
    }
    let branch = format!("codex/{}-{}", request.issue, draft.slug);
    let current_branch = crate::git::current_branch(store.root())?;
    if current_branch != request.base_branch && current_branch != branch {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "bind must run from the declared base branch or the derived issue branch",
        ));
    }
    let worktree = if current_branch == branch {
        ".".into()
    } else {
        format!(".worktrees/adl-wp-{}-{}", request.issue, draft.slug)
    };
    let actual_revision = crate::git::run(store.root(), &["rev-parse", "HEAD"])?.stdout;
    if actual_revision != request.expected_base_revision {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "current Git revision does not match the sealed base revision",
        ));
    }
    validate_bind_dirty_paths(store.root(), request.issue)?;
    let trusted_now = now_seconds()?;
    let authority = governed_session_owner(store, request.issue, &request.session_id, trusted_now)?;
    let derived_owner = authority.owner;
    let derived_claim_id = digest(
        format!(
            "{}:{}:{}:{}",
            request.issue, request.session_id, receipt.digest, derived_owner
        )
        .as_bytes(),
    );
    let existing_intent = load_binding_intent(store, request.issue)?;
    let branch_preexisting = git_branch_exists(store.root(), &branch)?;
    let worktree_preexisting = worktree == "." || store.root().join(&worktree).exists();
    if existing_intent.is_none() && worktree != "." && (branch_preexisting || worktree_preexisting)
    {
        return Err(V2Error::new(
            ErrorCode::ClaimCollision,
            "derived branch or worktree already exists without a matching binding intent",
        ));
    }
    let (owner, claim_id, acquired_unix_seconds, expires_unix_seconds) =
        if let Some(existing) = &existing_intent {
            if existing.session_id != request.session_id
                || existing.owner != derived_owner
                || existing.receipt_digest != receipt.digest
                || existing.branch != branch
                || existing.worktree != worktree
                || existing.base_revision != request.expected_base_revision
                || existing.state == BindingIntentState::Releasing
            {
                return Err(V2Error::new(
                    ErrorCode::ClaimCollision,
                    "existing binding intent is owned by different session truth",
                ));
            }
            (
                existing.owner.clone(),
                existing.claim_id.clone(),
                existing.acquired_unix_seconds,
                existing.expires_unix_seconds,
            )
        } else {
            (
                derived_owner,
                derived_claim_id,
                trusted_now,
                trusted_now
                    .saturating_add(request.lease_seconds)
                    .min(authority.expires_unix_seconds),
            )
        };
    let claim = Claim {
        id: claim_id.clone(),
        owner: owner.clone(),
        generation: 0,
        acquired_unix_seconds,
        expires_unix_seconds,
        heartbeat_unix_seconds: acquired_unix_seconds,
        branch: branch.clone(),
        worktree: worktree.clone(),
        protected_paths: receipt.owned_paths.clone(),
        purpose: format!(
            "execute issue {} sealed generation {}",
            request.issue, receipt.generation_id
        ),
    };
    let mut intent = if let Some(existing) = existing_intent {
        existing
    } else {
        let mut intent = BindingIntent {
            schema: INTENT_SCHEMA.into(),
            issue: request.issue,
            session_id: request.session_id,
            owner: owner.clone(),
            claim_id: claim_id.clone(),
            acquired_unix_seconds,
            expires_unix_seconds,
            receipt_digest: receipt.digest,
            generation_id: generation.generation_id.clone(),
            base_revision: request.expected_base_revision,
            branch: branch.clone(),
            worktree: worktree.clone(),
            branch_preexisting,
            worktree_preexisting,
            protected_paths: generation.owned_paths.clone(),
            state: BindingIntentState::Reserved,
            created_artifacts: Vec::new(),
            materialized_lifecycle_digest: None,
            digest: String::new(),
        };
        intent.digest = object_digest(&intent)?;
        write_binding_intent(store, &intent)?;
        intent
    };
    if intent.state != BindingIntentState::Bound {
        update_manifest_state(store, request.issue, PreparationState::Binding)?;
    }
    let bootstrap = BootstrapRequest {
        issue: generation.issue,
        repository: generation.repository.clone(),
        design_path: generation.design_path.clone(),
        diagram_path: generation.diagram_path.clone(),
        design_reviewer: generation.design_reviewer.clone(),
        design_approved: generation.design_approved,
        claim: claim.clone(),
        initial: generation.initial.clone(),
        prepared_cards: Some(generation.cards.clone()),
    };
    if let Err(error) = initialize_prepared_issue_under_binding_lock(store, bootstrap) {
        if error.code == ErrorCode::ClaimCollision {
            let path = binding_intent_path(store, request.issue)?;
            if path.exists() {
                fs::remove_file(path)?;
            }
            update_manifest_state(store, request.issue, PreparationState::ExecutionReady)?;
        }
        return Err(error);
    }
    intent.state = BindingIntentState::Initialized;
    intent.digest.clear();
    intent.digest = object_digest(&intent)?;
    write_binding_intent(store, &intent)?;
    drop(_registry);
    let bind = bind_issue(
        store,
        BindRequest {
            issue: request.issue,
            base_branch: request.base_branch,
            branch: branch.clone(),
            worktree: worktree.clone(),
            claim,
        },
    )?;
    intent.state = BindingIntentState::Bound;
    if !intent.branch_preexisting {
        intent.created_artifacts.push(format!("branch:{branch}"));
    }
    if !intent.worktree_preexisting {
        intent
            .created_artifacts
            .push(format!("worktree:{worktree}"));
    }
    intent.created_artifacts.sort();
    intent.created_artifacts.dedup();
    update_manifest_state(store, request.issue, PreparationState::Bound)?;
    if bind.created {
        update_manifest_state(
            &Store::new(store.root().join(&worktree)),
            request.issue,
            PreparationState::Bound,
        )?;
    }
    let materialized_root = if intent.worktree == "." {
        store.root().to_path_buf()
    } else {
        store.root().join(&intent.worktree)
    };
    intent.materialized_lifecycle_digest = Some(materialized_lifecycle_digest(
        &materialized_root,
        request.issue,
    )?);
    intent.digest.clear();
    intent.digest = object_digest(&intent)?;
    write_binding_intent(store, &intent)?;
    Ok(DerivedBindResult {
        schema: "csdlc.derived_bind_result.v1".into(),
        issue: request.issue,
        state: PreparationState::Bound,
        branch,
        worktree,
        owner,
        bind,
    })
}

pub fn release_derived_bind(
    store: &Store,
    request: BindReleaseRequest,
) -> Result<BindReleaseResult> {
    let _operation = binding_operation_lock(store, request.issue)?;
    let _preparation = preparation_lock(store, request.issue)?;
    let _registry = store.binding_lock()?;
    let path = binding_intent_path(store, request.issue)?;
    if !path.exists() {
        let state = load_manifest(store, request.issue)?.state;
        if matches!(state, PreparationState::Binding | PreparationState::Bound) {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "binding state has no durable intent; use csdlc-migrate repair",
            ));
        }
        return Ok(BindReleaseResult {
            schema: "csdlc.bind_release_result.v1".into(),
            issue: request.issue,
            released: false,
            removed_artifacts: Vec::new(),
            state,
        });
    }
    let mut intent = load_binding_intent(store, request.issue)?.ok_or_else(|| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "binding intent disappeared while release was serialized",
        )
    })?;
    let trusted_now = now_seconds()?;
    let current = governed_session_owner(store, request.issue, &request.session_id, trusted_now)?;
    let same_owner = intent.session_id == request.session_id && intent.owner == current.owner;
    let governed_takeover = !same_owner
        && intent.expires_unix_seconds <= trusted_now
        && request.expected_intent_digest.as_deref() == Some(&intent.digest);
    if !same_owner && !governed_takeover {
        return Err(V2Error::new(
            ErrorCode::MissingClaim,
            "current session does not own binding intent; expired recovery requires its exact digest",
        ));
    }
    if governed_takeover {
        recover_interrupted_artifact_evidence(store, &mut intent)?;
    }
    intent.state = BindingIntentState::Releasing;
    intent.digest.clear();
    intent.digest = object_digest(&intent)?;
    atomic_write_json(&path, &intent)?;
    let mut removed = Vec::new();
    let owns_worktree = intent
        .created_artifacts
        .iter()
        .any(|artifact| artifact == &format!("worktree:{}", intent.worktree));
    let owns_branch = intent
        .created_artifacts
        .iter()
        .any(|artifact| artifact == &format!("branch:{}", intent.branch));
    let worktrees = crate::git::worktrees(store.root())?;
    if intent.worktree == "." {
        if crate::git::current_branch(store.root())? != intent.branch {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "issue-local binding branch no longer matches durable intent",
            ));
        }
        validate_release_target(store.root(), request.issue, &intent)?;
    }
    if intent.worktree != "."
        && !intent.worktree_preexisting
        && worktrees.iter().any(|(branch, _)| branch == &intent.branch)
        && !owns_worktree
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "derived worktree exists without durable creation evidence",
        ));
    }
    if let Some((branch, registered)) = worktrees
        .iter()
        .find(|(branch, _)| branch == &intent.branch && owns_worktree)
    {
        let expected = store.root().join(&intent.worktree);
        let expected = if expected.exists() {
            expected.canonicalize()?.to_string_lossy().to_string()
        } else {
            expected.to_string_lossy().to_string()
        };
        if branch != &intent.branch || registered != &expected {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "binding worktree topology does not match durable intent",
            ));
        }
        let target = PathBuf::from(registered);
        validate_release_target(&target, request.issue, &intent)?;
        crate::git::run(
            store.root(),
            &["worktree", "remove", "--force", registered.as_str()],
        )?;
        removed.push(format!("worktree:{}", intent.worktree));
    }
    let branch_exists = std::process::Command::new("git")
        .current_dir(store.root())
        .args(["show-ref", "--verify", "--quiet"])
        .arg(format!("refs/heads/{}", intent.branch))
        .status()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?
        .success();
    if branch_exists && !intent.branch_preexisting && !owns_branch {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "derived branch exists without durable creation evidence",
        ));
    }
    if branch_exists && owns_branch {
        let branch_revision =
            crate::git::run(store.root(), &["rev-parse", intent.branch.as_str()])?.stdout;
        if branch_revision != intent.base_revision {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "intent-owned branch advanced beyond its sealed base revision",
            ));
        }
        crate::git::run(store.root(), &["branch", "-D", intent.branch.as_str()])?;
        removed.push(format!("branch:{}", intent.branch));
    }
    if store.issue_dir(request.issue).exists() {
        let record = store.load_record(request.issue)?;
        store.remove_unstarted_binding_projection(
            request.issue,
            &intent.claim_id,
            &record.digest,
        )?;
        removed.push(format!("projection:{}", request.issue));
    }
    update_manifest_state(store, request.issue, PreparationState::ExecutionReady)?;
    fs::remove_file(&path)?;
    removed.push(format!("intent:{}", request.issue));
    Ok(BindReleaseResult {
        schema: "csdlc.bind_release_result.v1".into(),
        issue: request.issue,
        released: true,
        removed_artifacts: removed,
        state: PreparationState::ExecutionReady,
    })
}

fn recover_interrupted_artifact_evidence(store: &Store, intent: &mut BindingIntent) -> Result<()> {
    if intent.worktree == "." || intent.worktree_preexisting || intent.branch_preexisting {
        return Ok(());
    }
    let worktrees = crate::git::worktrees(store.root())?;
    if let Some((branch, registered)) = worktrees
        .iter()
        .find(|(branch, _)| branch == &intent.branch)
    {
        let expected = store.root().join(&intent.worktree);
        let expected = expected.canonicalize()?.to_string_lossy().to_string();
        if branch != &intent.branch || registered != &expected {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "interrupted binding topology does not match durable intent",
            ));
        }
        let target = PathBuf::from(registered);
        let target_store = Store::new(&target);
        let record = target_store.load_record(intent.issue)?;
        let cards = target_store.load_cards(intent.issue)?;
        target_store.verify_canonical_authority_projection(&record, &cards)?;
        if record.claim.as_ref().map(|claim| claim.id.as_str()) != Some(&intent.claim_id)
            || record.phase != crate::LifecyclePhase::Bound
            || record.review.is_some()
            || record.publication.is_some()
            || record.terminal.is_some()
            || crate::git::run(&target, &["rev-parse", "HEAD"])?.stdout != intent.base_revision
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "interrupted binding worktree does not prove unstarted intent ownership",
            ));
        }
        intent
            .created_artifacts
            .push(format!("worktree:{}", intent.worktree));
        intent
            .created_artifacts
            .push(format!("branch:{}", intent.branch));
        intent.materialized_lifecycle_digest =
            Some(materialized_lifecycle_digest(&target, intent.issue)?);
    } else if git_branch_exists(store.root(), &intent.branch)? {
        let revision =
            crate::git::run(store.root(), &["rev-parse", intent.branch.as_str()])?.stdout;
        if revision != intent.base_revision {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "interrupted binding branch does not match the sealed base revision",
            ));
        }
        intent
            .created_artifacts
            .push(format!("branch:{}", intent.branch));
    }
    intent.created_artifacts.sort();
    intent.created_artifacts.dedup();
    Ok(())
}

fn validate_release_target(target: &Path, issue: u64, intent: &BindingIntent) -> Result<()> {
    let target_store = Store::new(target);
    let target_revision = crate::git::run(target, &["rev-parse", "HEAD"])?.stdout;
    if target_revision != intent.base_revision {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "binding worktree advanced beyond its sealed base revision",
        ));
    }
    if target_store.issue_dir(issue).exists() {
        let record = target_store.load_record(issue)?;
        if record.claim.as_ref().map(|claim| claim.id.as_str()) != Some(&intent.claim_id)
            || record.phase != crate::LifecyclePhase::Bound
            || record.review.is_some()
            || record.publication.is_some()
            || record.terminal.is_some()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "bound worktree has started execution or no longer matches intent",
            ));
        }
    }
    validate_release_dirty_paths(
        target,
        issue,
        intent.materialized_lifecycle_digest.as_deref(),
    )
}

fn validate_release_dirty_paths(
    target_root: &Path,
    issue: u64,
    expected_materialized_digest: Option<&str>,
) -> Result<()> {
    let status = crate::git::run(
        target_root,
        &["status", "--porcelain", "--untracked-files=all"],
    )?;
    if status.stdout.trim().is_empty() {
        return Ok(());
    }
    let lifecycle = [
        format!(".csdlc/issues/{issue}/"),
        format!(".csdlc/prepared/issues/{issue}/"),
        format!(".csdlc/evidence/{issue}/"),
        format!(".csdlc/preparation/issues/{issue}/"),
    ];
    let actual = materialized_lifecycle_digest(target_root, issue)?;
    if expected_materialized_digest != Some(actual.as_str()) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "bound worktree lifecycle files differ from intent-materialized bytes",
        ));
    }
    let allowed_lock = format!(".csdlc/locks/{issue}.lock");
    let allowed_preparation_lock = format!(".csdlc/preparation/locks/{issue}.lock");
    let unexpected = status
        .stdout
        .lines()
        .filter_map(|line| {
            let path = line.get(3..)?.split(" -> ").last()?.trim_matches('"');
            (path != allowed_lock
                && path != allowed_preparation_lock
                && path != ".adl/session-ledger/ledger.json"
                && !lifecycle.iter().any(|prefix| path.starts_with(prefix)))
            .then(|| path.to_string())
        })
        .collect::<Vec<_>>();
    if !unexpected.is_empty() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "binding worktree contains changes outside exact intent materialization for issue {issue}: {}",
                unexpected.join(", ")
            ),
        ));
    }
    Ok(())
}

fn materialized_lifecycle_digest(root: &Path, issue: u64) -> Result<String> {
    let mut entries = Vec::new();
    for relative in [
        format!(".csdlc/issues/{issue}"),
        format!(".csdlc/prepared/issues/{issue}"),
        format!(".csdlc/evidence/{issue}"),
        format!(".csdlc/preparation/issues/{issue}"),
    ] {
        collect_lifecycle_bytes(root, Path::new(&relative), &mut entries)?;
    }
    Ok(digest(&serde_json::to_vec(&entries)?))
}

fn collect_lifecycle_bytes(
    root: &Path,
    relative: &Path,
    entries: &mut Vec<(String, String)>,
) -> Result<()> {
    let path = root.join(relative);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    if metadata.file_type().is_symlink() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "intent-materialized lifecycle tree contains a symlink",
        ));
    }
    if metadata.is_file() {
        entries.push((
            relative.to_string_lossy().to_string(),
            digest(&fs::read(path)?),
        ));
        return Ok(());
    }
    let mut children = fs::read_dir(path)?.collect::<std::io::Result<Vec<_>>>()?;
    children.sort_by_key(std::fs::DirEntry::file_name);
    for child in children {
        collect_lifecycle_bytes(root, &relative.join(child.file_name()), entries)?;
    }
    Ok(())
}

fn git_branch_exists(root: &Path, branch: &str) -> Result<bool> {
    Ok(std::process::Command::new("git")
        .current_dir(root)
        .args(["show-ref", "--verify", "--quiet"])
        .arg(format!("refs/heads/{branch}"))
        .status()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?
        .success())
}

fn validate_bind_dirty_paths(root: &Path, _issue: u64) -> Result<()> {
    let status = crate::git::run(root, &["status", "--porcelain", "--untracked-files=all"])?;
    let mut allowed = vec![
        ".csdlc/issues/".into(),
        ".csdlc/prepared/issues/".into(),
        ".csdlc/evidence/".into(),
        ".csdlc/preparation/issues/".into(),
        ".csdlc/locks/".into(),
        ".csdlc/preparation/locks/".into(),
        ".adl/session-ledger/".into(),
    ];
    let canonical_root = root.canonicalize()?;
    for (_, registered) in crate::git::worktrees(root)? {
        let registered = PathBuf::from(registered);
        if let Ok(relative) = registered.strip_prefix(&canonical_root) {
            let prefix = format!("{}/", relative.to_string_lossy().trim_end_matches('/'));
            if prefix != "./" {
                allowed.push(prefix);
            }
        }
    }
    let unexpected = status
        .stdout
        .lines()
        .filter_map(|line| {
            let path = line.get(3..)?.split(" -> ").last()?.trim_matches('"');
            (!allowed.iter().any(|prefix| path.starts_with(prefix))).then(|| path.to_string())
        })
        .collect::<Vec<_>>();
    if !unexpected.is_empty() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "bind checkout contains non-lifecycle changes: {}",
                unexpected.join(", ")
            ),
        ));
    }
    Ok(())
}

pub fn load_manifest(store: &Store, issue: u64) -> Result<PreparationManifest> {
    let value: PreparationManifest =
        read_json(&issue_preparation_dir(store, issue).join("manifest.json"))?;
    let mut payload = value.clone();
    let claimed = std::mem::take(&mut payload.digest);
    if claimed != object_digest(&payload)? {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "preparation manifest digest mismatch",
        ));
    }
    if value.issue != issue {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "preparation manifest issue namespace mismatch",
        ));
    }
    Ok(value)
}

pub fn load_binding_intent(store: &Store, issue: u64) -> Result<Option<BindingIntent>> {
    let path = binding_intent_path(store, issue)?;
    if path.exists() {
        let value: BindingIntent = read_json(&path)?;
        let mut payload = value.clone();
        let claimed = std::mem::take(&mut payload.digest);
        if claimed != object_digest(&payload)? {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "binding intent digest mismatch",
            ));
        }
        if value.issue != issue {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "binding intent issue namespace mismatch",
            ));
        }
        if value.schema != INTENT_SCHEMA
            || value.session_id.trim().is_empty()
            || value.owner.trim().is_empty()
            || value.claim_id.trim().is_empty()
            || value.receipt_digest.trim().is_empty()
            || value.generation_id.trim().is_empty()
            || value.base_revision.trim().is_empty()
            || value.branch.trim().is_empty()
            || value.worktree.trim().is_empty()
            || value.expires_unix_seconds < value.acquired_unix_seconds
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "binding intent has invalid identity or lease fields",
            ));
        }
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

fn draft_from_request(request: IssueCreateRequest) -> Result<IssueDraft> {
    let mut draft = IssueDraft {
        schema: "csdlc.issue_draft.v1".into(),
        issue: request.issue,
        repository: request.repository,
        title: request.title,
        slug: request.slug,
        version: request.version,
        state: PreparationState::Draft,
        digest: String::new(),
    };
    draft.digest = object_digest(&draft)?;
    Ok(draft)
}

fn verify_draft(draft: &IssueDraft) -> Result<()> {
    let mut payload = draft.clone();
    let claimed = std::mem::take(&mut payload.digest);
    if claimed != object_digest(&payload)? {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "issue draft digest mismatch",
        ));
    }
    Ok(())
}

fn verify_receipt(receipt: &ExecutionReadinessReceipt) -> Result<()> {
    let mut payload = receipt.clone();
    let claimed = std::mem::take(&mut payload.digest);
    if claimed != object_digest(&payload)? {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "execution readiness receipt digest mismatch",
        ));
    }
    Ok(())
}

#[derive(Serialize)]
struct SemanticPayload<'a> {
    base_revision: &'a str,
    cards: &'a BTreeMap<CardKind, CardValues>,
    dependencies: &'a [DependencyRevision],
    design_approved: bool,
    design_digest: &'a str,
    design_reviewer: &'a str,
    diagram_digest: &'a str,
    initial: &'a InitialCardInput,
    issue: u64,
    owned_paths: &'a [String],
    repository: &'a str,
}

fn semantic_digest(payload: &SemanticPayload<'_>) -> Result<String> {
    Ok(digest(&serde_json::to_vec(payload)?))
}

fn validate_generation_for_seal(store: &Store, generation: &PreparedGeneration) -> Result<()> {
    let computed = semantic_digest(&SemanticPayload {
        issue: generation.issue,
        repository: &generation.repository,
        design_digest: &generation.design_digest,
        diagram_digest: &generation.diagram_digest,
        owned_paths: &generation.owned_paths,
        dependencies: &generation.dependencies,
        design_approved: generation.design_approved,
        design_reviewer: &generation.design_reviewer,
        initial: &generation.initial,
        base_revision: &generation.base_revision,
        cards: &generation.cards,
    })?;
    let expected_id = format!("{}-{}", generation.sequence, &computed[..16]);
    if generation.semantic_digest != computed || generation.generation_id != expected_id {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "prepared generation semantic digest mismatch",
        ));
    }
    let design =
        crate::store::read_regular_projection(store.root(), Path::new(&generation.design_path))?;
    let diagram =
        crate::store::read_regular_projection(store.root(), Path::new(&generation.diagram_path))?;
    if digest(&design) != generation.design_digest || digest(&diagram) != generation.diagram_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "design or diagram changed after sync",
        ));
    }
    validate_cross_card(
        &generation.cards,
        &generation.design_path,
        &generation.design_digest,
        &generation.diagram_path,
        &generation.diagram_digest,
    )?;
    let serialized = serde_json::to_string(&generation.cards)?.to_ascii_lowercase();
    for marker in ["[todo]", "placeholder", "design required", "tbd"] {
        if serialized.contains(marker) {
            return Err(V2Error::new(
                ErrorCode::ValidationFailed,
                format!("prepared generation contains unresolved marker '{marker}'"),
            ));
        }
    }
    if !generation.design_approved || generation.design_reviewer.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "design approval is required before seal",
        ));
    }
    validate_validation_lanes(store.root(), &generation.initial.validation_lanes)?;
    Ok(())
}

fn write_generation(
    store: &Store,
    generation: &PreparedGeneration,
    design: &[u8],
    diagram: &[u8],
) -> Result<()> {
    let base = issue_preparation_dir(store, generation.issue).join("generations");
    create_dir_all_safe(&base)?;
    let final_path = base.join(&generation.generation_id);
    if final_path.exists() {
        let existing: PreparedGeneration = read_json(&final_path.join("generation.json"))?;
        if existing == *generation {
            return Ok(());
        }
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "generation id collision",
        ));
    }
    let staging = base.join(format!(".{}.staging", generation.generation_id));
    match fs::symlink_metadata(&staging) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "generation staging path is a symlink",
            ));
        }
        Ok(metadata) if metadata.is_dir() => fs::remove_dir_all(&staging)?,
        Ok(_) => {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "generation staging path is not a directory",
            ));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    create_dir_all_safe(&staging.join("cards"))?;
    atomic_write_json(&staging.join("generation.json"), generation)?;
    write_bytes(&staging.join("design.snapshot"), design)?;
    write_bytes(&staging.join("diagram.snapshot"), diagram)?;
    for (kind, values) in &generation.cards {
        atomic_write_json(
            &staging.join("cards").join(format!("{kind}.values.json")),
            values,
        )?;
        write_bytes(
            &staging.join("cards").join(format!("{kind}.md")),
            render(values)?.markdown.as_bytes(),
        )?;
    }
    File::open(&staging)?.sync_all()?;
    fs::rename(&staging, &final_path)?;
    File::open(&base)?.sync_all()?;
    Ok(())
}

fn load_generation(store: &Store, issue: u64, id: &str) -> Result<PreparedGeneration> {
    validate_component(id, "generation_id")?;
    read_json(
        &issue_preparation_dir(store, issue)
            .join("generations")
            .join(id)
            .join("generation.json"),
    )
}

fn manifest(
    issue: u64,
    state: PreparationState,
    generation: Option<String>,
    sequence: u64,
    semantic_digest: Option<String>,
    receipt_digest: Option<String>,
) -> Result<PreparationManifest> {
    let mut value = PreparationManifest {
        schema: PREPARATION_SCHEMA.into(),
        issue,
        state,
        current_generation: generation,
        current_sequence: sequence,
        semantic_digest,
        receipt_digest,
        digest: String::new(),
    };
    value.digest = object_digest(&value)?;
    Ok(value)
}

fn update_manifest_state(store: &Store, issue: u64, state: PreparationState) -> Result<()> {
    let path = issue_preparation_dir(store, issue).join("manifest.json");
    let mut value = load_manifest(store, issue)?;
    value.state = state;
    value.digest.clear();
    value.digest = object_digest(&value)?;
    atomic_write_json(&path, &value)
}

fn load_manifest_optional(store: &Store, issue: u64) -> Result<Option<PreparationManifest>> {
    let path = issue_preparation_dir(store, issue).join("manifest.json");
    if path.exists() {
        Ok(Some(load_manifest(store, issue)?))
    } else {
        Ok(None)
    }
}

fn issue_preparation_dir(store: &Store, issue: u64) -> PathBuf {
    store
        .root()
        .join(".csdlc/preparation/issues")
        .join(issue.to_string())
}

fn preparation_lock(store: &Store, issue: u64) -> Result<File> {
    let dir = store.root().join(".csdlc/preparation/locks");
    create_dir_all_safe(&dir)?;
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(dir.join(format!("{issue}.lock")))?;
    file.lock_exclusive()?;
    Ok(file)
}

fn binding_intent_path(store: &Store, issue: u64) -> Result<PathBuf> {
    let common = crate::git::run(
        store.root(),
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?
    .stdout;
    let common = PathBuf::from(common);
    if !common.is_absolute()
        || common
            .components()
            .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "Git common directory is not an absolute normalized path",
        ));
    }
    let common = common.canonicalize()?;
    if !common.is_dir() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "Git common directory is not a directory",
        ));
    }
    Ok(common
        .join("csdlc-v2/binding-intents")
        .join(format!("{issue}.json")))
}

fn binding_operation_lock(store: &Store, issue: u64) -> Result<File> {
    let path = binding_intent_path(store, issue)?.with_extension("lock");
    create_dir_all_safe(path.parent().expect("binding operation lock parent"))?;
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)?;
    file.lock_exclusive()?;
    Ok(file)
}

fn write_binding_intent(store: &Store, intent: &BindingIntent) -> Result<()> {
    let path = binding_intent_path(store, intent.issue)?;
    create_dir_all_safe(path.parent().expect("intent parent"))?;
    if path.exists() {
        let existing = load_binding_intent(store, intent.issue)?.ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "binding intent disappeared during serialized update",
            )
        })?;
        if existing.session_id != intent.session_id
            || existing.receipt_digest != intent.receipt_digest
        {
            return Err(V2Error::new(
                ErrorCode::ClaimCollision,
                "another session owns binding intent",
            ));
        }
    }
    atomic_write_json(&path, intent)
}

fn normalize_paths(paths: &[String]) -> Result<Vec<String>> {
    let mut normalized = BTreeSet::new();
    for path in paths {
        validate_relative(path, "owned_path")?;
        normalized.insert(path.trim_end_matches('/').to_string());
    }
    Ok(normalized.into_iter().collect())
}

fn quarantine_migration(
    store: &Store,
    record: &crate::IssueRecord,
    reason: &str,
) -> Result<LegacyPreparationMigrationResult> {
    let relative = format!(
        ".csdlc/preparation/issues/{}/migration/quarantine-{}.json",
        record.issue, record.digest
    );
    let mut packet = LegacyPreparationQuarantine {
        schema: "csdlc.legacy_preparation_quarantine.v1".into(),
        reason: reason.into(),
        record: record.clone(),
        digest: String::new(),
    };
    packet.digest = object_digest(&packet)?;
    write_immutable_json(&store.root().join(&relative), &packet)?;
    Ok(LegacyPreparationMigrationResult {
        schema: "csdlc.legacy_preparation_migration_result.v1".into(),
        issue: record.issue,
        disposition: LegacyPreparationDisposition::Quarantined,
        original_digest: record.digest.clone(),
        resulting_digest: Some(packet.digest),
        snapshot_path: Some(relative),
        next_operation: "csdlc-migrate repair".into(),
    })
}

fn initial_from_cards(cards: &BTreeMap<CardKind, CardValues>) -> Result<InitialCardInput> {
    let identity = &cards
        .get(&CardKind::Sip)
        .ok_or_else(|| V2Error::new(ErrorCode::CardInvalid, "SIP card is missing"))?
        .identity;
    let sip = match &cards[&CardKind::Sip].content {
        CardContent::Sip(value) => value,
        _ => {
            return Err(V2Error::new(
                ErrorCode::CardInvalid,
                "SIP card kind mismatch",
            ))
        }
    };
    let stp = match &cards[&CardKind::Stp].content {
        CardContent::Stp(value) => value,
        _ => {
            return Err(V2Error::new(
                ErrorCode::CardInvalid,
                "STP card kind mismatch",
            ))
        }
    };
    let spp = match &cards[&CardKind::Spp].content {
        CardContent::Spp(value) => value,
        _ => {
            return Err(V2Error::new(
                ErrorCode::CardInvalid,
                "SPP card kind mismatch",
            ))
        }
    };
    let vpp = match &cards[&CardKind::Vpp].content {
        CardContent::Vpp(value) => value,
        _ => {
            return Err(V2Error::new(
                ErrorCode::CardInvalid,
                "VPP card kind mismatch",
            ))
        }
    };
    let srp = match &cards[&CardKind::Srp].content {
        CardContent::Srp(value) => value,
        _ => {
            return Err(V2Error::new(
                ErrorCode::CardInvalid,
                "SRP card kind mismatch",
            ))
        }
    };
    let planning_profile = [
        PlanningProfile::Small,
        PlanningProfile::Medium,
        PlanningProfile::Large,
        PlanningProfile::Migration,
    ]
    .into_iter()
    .find(|profile| {
        let (estimates, tokens) = profile.estimates();
        estimates == spp.execution_estimates
            && estimates.validation_seconds == vpp.planned_validation_seconds
            && tokens == vpp.planned_validation_tokens
    })
    .ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "legacy planning profile cannot be reconstructed exactly",
        )
    })?;
    Ok(InitialCardInput {
        title: identity.title.clone(),
        slug: identity.slug.clone(),
        version: identity.version.clone(),
        goal: sip.goal.clone(),
        required_outcome: sip.required_outcome.clone(),
        declared_scope: sip.declared_scope.clone(),
        authority_boundary: sip.authority_boundary.clone(),
        operator_constraints: sip.operator_constraints.clone(),
        task_boundary: stp.task_boundary.clone(),
        deliverables: stp.deliverables.clone(),
        acceptance_criteria: stp.acceptance_criteria.clone(),
        dependencies: stp.dependencies.clone(),
        repo_inputs: stp.repo_inputs.clone(),
        non_goals: stp.non_goals.clone(),
        plan_summary: spp.summary.clone(),
        steps: spp.steps.clone(),
        invariants: spp.invariants.clone(),
        risks: spp.risks.clone(),
        planning_profile,
        stop_conditions: spp.stop_conditions.clone(),
        validation_lanes: vpp.lanes.clone(),
        failure_policy: vpp.failure_policy.clone(),
        review_prompts: srp.review_prompts.clone(),
        review_scope: srp.review_scope.clone(),
    })
}

fn dependency_cycle_issues(children: &[PrepareRunRequest]) -> BTreeSet<u64> {
    let child_issues = children
        .iter()
        .map(|child| child.sync.issue)
        .collect::<BTreeSet<_>>();
    let graph = children
        .iter()
        .map(|child| {
            (
                child.sync.issue,
                child
                    .sync
                    .dependencies
                    .iter()
                    .map(|dependency| dependency.issue)
                    .filter(|issue| child_issues.contains(issue))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut cyclic = BTreeSet::new();
    for start in graph.keys().copied() {
        let mut stack = vec![(start, vec![start])];
        while let Some((node, path)) = stack.pop() {
            for next in graph.get(&node).into_iter().flatten().copied() {
                if next == start {
                    cyclic.extend(path.iter().copied());
                    cyclic.insert(next);
                } else if !path.contains(&next) {
                    let mut successor = path.clone();
                    successor.push(next);
                    stack.push((next, successor));
                }
            }
        }
    }
    cyclic
}

fn intra_batch_overlap_issues(children: &[PrepareRunRequest]) -> BTreeSet<u64> {
    let mut overlaps = BTreeSet::new();
    for (index, left) in children.iter().enumerate() {
        for right in &children[index + 1..] {
            if left.sync.owned_paths.iter().any(|left_path| {
                right
                    .sync
                    .owned_paths
                    .iter()
                    .any(|right_path| paths_overlap(left_path, right_path))
            }) {
                overlaps.insert(left.sync.issue);
                overlaps.insert(right.sync.issue);
            }
        }
    }
    overlaps
}

fn paths_overlap(left: &str, right: &str) -> bool {
    let left = left.trim_end_matches('/');
    let right = right.trim_end_matches('/');
    left == right
        || left
            .strip_prefix(right)
            .is_some_and(|suffix| suffix.starts_with('/'))
        || right
            .strip_prefix(left)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

fn normalize_dependencies(values: &[DependencyRevision]) -> Result<Vec<DependencyRevision>> {
    let mut map = BTreeMap::new();
    for value in values {
        if value.issue == 0 || value.revision.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "dependency issue and revision are required",
            ));
        }
        if map.insert(value.issue, value.revision.clone()).is_some() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "duplicate dependency issue",
            ));
        }
    }
    Ok(map
        .into_iter()
        .map(|(issue, revision)| DependencyRevision { issue, revision })
        .collect())
}

fn validate_live_dependencies(store: &Store, dependencies: &[DependencyRevision]) -> Result<()> {
    for dependency in dependencies {
        let preparation_receipt =
            issue_preparation_dir(store, dependency.issue).join("receipt.json");
        let live_revision = if preparation_receipt.exists() {
            let manifest = load_manifest(store, dependency.issue)?;
            let receipt: ExecutionReadinessReceipt = read_json(&preparation_receipt)?;
            verify_receipt(&receipt)?;
            if manifest.state != PreparationState::ExecutionReady
                || manifest.receipt_digest.as_deref() != Some(&receipt.digest)
            {
                return Err(V2Error::new(
                    ErrorCode::StaleDigest,
                    format!("dependency {} is not execution ready", dependency.issue),
                ));
            }
            receipt.digest
        } else if store.issue_dir(dependency.issue).exists() {
            let record = store.load_record(dependency.issue)?;
            crate::store::verify_record(&record)?;
            record.digest
        } else {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                format!("dependency {} has no local authority", dependency.issue),
            ));
        };
        if dependency.revision != live_revision {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                format!("dependency {} revision changed", dependency.issue),
            ));
        }
    }
    Ok(())
}

fn validate_identity(issue: u64, repository: &str, slug: &str) -> Result<()> {
    if issue == 0
        || repository.trim().is_empty()
        || slug.trim().is_empty()
        || !slug
            .chars()
            .all(|value| value.is_ascii_alphanumeric() || value == '-')
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue, repository, and safe slug are required",
        ));
    }
    Ok(())
}

fn validate_relative(value: &str, field: &str) -> Result<()> {
    let path = Path::new(value);
    if value.trim().is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            format!("{field} must be repository-relative"),
        ));
    }
    Ok(())
}

fn validate_component(value: &str, field: &str) -> Result<()> {
    if value.is_empty() || value.contains('/') || value.contains("..") {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            format!("unsafe {field}"),
        ));
    }
    Ok(())
}

fn object_digest<T: Serialize>(value: &T) -> Result<String> {
    Ok(digest(&serde_json::to_vec(value)?))
}

fn atomic_write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value)?;
    let mut newline = bytes;
    newline.push(b'\n');
    atomic_write(path, &newline)
}

fn write_immutable_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let expected = serde_json::to_vec_pretty(value)?;
    let mut bytes = expected;
    bytes.push(b'\n');
    let parent = path
        .parent()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "path has no parent"))?;
    create_dir_all_safe(parent)?;
    let (root, relative) = preparation_root_and_relative(path)?;
    crate::store::require_canonical_parent_beneath(root, &relative)?;
    match OpenOptions::new().create_new(true).write(true).open(path) {
        Ok(mut file) => {
            file.write_all(&bytes)?;
            file.sync_all()?;
            File::open(parent)?.sync_all()?;
            Ok(())
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let existing = crate::store::read_regular_projection(root, &relative)?;
            if existing == bytes {
                Ok(())
            } else {
                Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    "immutable preparation artifact already exists with different content",
                ))
            }
        }
        Err(error) => Err(error.into()),
    }
}

fn preparation_root_and_relative(path: &Path) -> Result<(&Path, PathBuf)> {
    let namespace = path
        .ancestors()
        .find(|ancestor| ancestor.file_name().is_some_and(|name| name == ".csdlc"))
        .or_else(|| {
            path.ancestors().find(|ancestor| {
                ancestor.file_name().is_some_and(|name| name == "csdlc-v2")
                    && ancestor
                        .parent()
                        .and_then(Path::file_name)
                        .is_some_and(|name| name == ".git")
            })
        })
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::UnsafeCheckout,
                "preparation path is outside governed lifecycle namespaces",
            )
        })?;
    let root = if namespace.file_name().is_some_and(|name| name == ".csdlc") {
        namespace.parent()
    } else {
        namespace.parent().and_then(Path::parent)
    }
    .ok_or_else(|| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            "preparation namespace has no repository root",
        )
    })?;
    let relative = path
        .strip_prefix(root)
        .map_err(|_| V2Error::new(ErrorCode::UnsafeCheckout, "preparation path escaped root"))?;
    Ok((root, relative.to_path_buf()))
}

fn create_dir_all_safe(path: &Path) -> Result<()> {
    let probe = path.join(".csdlc-directory-probe");
    let (root, relative) = preparation_root_and_relative(&probe)?;
    crate::store::require_canonical_parent_beneath(root, &relative)?;
    fs::create_dir_all(path)?;
    crate::store::require_canonical_parent_beneath(root, &relative)?;
    Ok(())
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "path has no parent"))?;
    create_dir_all_safe(parent)?;
    let (root, relative) = preparation_root_and_relative(path)?;
    crate::store::require_canonical_parent_beneath(root, &relative)?;
    crate::store::require_regular_or_absent_beneath(root, &relative)?;
    let tmp = parent.join(format!(
        ".{}.{}.{}.tmp",
        path.file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("write"),
        std::process::id(),
        time::OffsetDateTime::now_utc().unix_timestamp_nanos()
    ));
    let (_, tmp_relative) = preparation_root_and_relative(&tmp)?;
    crate::store::require_canonical_parent_beneath(root, &tmp_relative)?;
    let mut file = OpenOptions::new().create_new(true).write(true).open(&tmp)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(&tmp, path)?;
    crate::store::require_regular_or_absent_beneath(root, &relative)?;
    File::open(parent)?.sync_all()?;
    Ok(())
}

fn write_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    let (root, relative) = preparation_root_and_relative(path)?;
    crate::store::require_canonical_parent_beneath(root, &relative)?;
    crate::store::require_regular_or_absent_beneath(root, &relative)?;
    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let bytes = if let Ok((root, relative)) = preparation_root_and_relative(path) {
        crate::store::read_regular_projection(root, &relative)?
    } else {
        let metadata = fs::symlink_metadata(path)?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "governed JSON input is not a regular file: {}",
                    path.display()
                ),
            ));
        }
        fs::read(path)?
    };
    Ok(serde_json::from_slice(&bytes)?)
}

fn read_regular_json<T: DeserializeOwned>(root: &Path, relative: &Path) -> Result<T> {
    Ok(serde_json::from_slice(
        &crate::store::read_regular_projection(root, relative)?,
    )?)
}
