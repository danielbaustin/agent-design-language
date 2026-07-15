use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use fs2::FileExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cards::{
    apply, digest, initial_cards, render, validate_cross_card, CardContent, CardKind, CardValues,
    InitialCardInput, SemanticOperation,
};
use crate::error::{ErrorCode, Result, V2Error};
use crate::model::{
    AuditEvent, CardProjection, Claim, DesignReview, IssueRecord, LifecyclePhase,
    PublicationEvidence, ReadinessEvidence, ReviewAssignment, ReviewEvidence, TerminalEvidence,
    TransitionEvent,
};
use crate::review::evaluate_publication_review_in_repo;

#[derive(Debug, Clone)]
pub struct Store {
    root: PathBuf,
}

impl Store {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn issue_dir(&self, issue: u64) -> PathBuf {
        self.root.join(".csdlc/issues").join(issue.to_string())
    }

    pub fn interrupted_backup(&self, issue: u64) -> PathBuf {
        self.root
            .join(".csdlc/issues")
            .join(format!(".{issue}.backup"))
    }

    fn staging_dir(&self, issue: u64) -> PathBuf {
        self.root
            .join(".csdlc/issues")
            .join(format!(".{issue}.staging"))
    }

    fn lock(&self, issue: u64) -> Result<File> {
        let dir = self.root.join(".csdlc/locks");
        fs::create_dir_all(&dir)?;
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(dir.join(format!("{issue}.lock")))?;
        file.lock_exclusive()?;
        Ok(file)
    }

    pub(crate) fn binding_lock(&self) -> Result<File> {
        let dir = self.root.join(".csdlc/locks");
        fs::create_dir_all(&dir)?;
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(dir.join("bindings.lock"))?;
        file.lock_exclusive()?;
        Ok(file)
    }

    pub fn load_record(&self, issue: u64) -> Result<IssueRecord> {
        read_json(&self.issue_dir(issue).join("index.json"))
    }

    pub fn load_cards(&self, issue: u64) -> Result<BTreeMap<CardKind, CardValues>> {
        let mut cards = BTreeMap::new();
        for kind in enum_iterator() {
            let path = self
                .issue_dir(issue)
                .join("cards")
                .join(format!("{kind}.values.json"));
            cards.insert(kind, read_json(&path)?);
        }
        Ok(cards)
    }

    fn recover_if_needed(&self, issue: u64) -> Result<()> {
        let current = self.issue_dir(issue);
        let backup = self.interrupted_backup(issue);
        let staging = self.staging_dir(issue);
        if !current.exists() && backup.exists() {
            fs::rename(&backup, &current)?;
        }
        if staging.exists() {
            fs::remove_dir_all(staging)?;
        }
        Ok(())
    }

    fn commit(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
        fail_after_backup: bool,
    ) -> Result<()> {
        let current = self.issue_dir(issue);
        let staging = self.staging_dir(issue);
        let backup = self.interrupted_backup(issue);
        if staging.exists() {
            fs::remove_dir_all(&staging)?;
        }
        if backup.exists() {
            fs::remove_dir_all(&backup)?;
        }
        write_complete(&staging, record, cards)?;
        // Preserve authored design artifacts when they live inside the issue
        // directory. The atomic directory swap must not discard them.
        for authored_path in [&record.design_path, &record.diagram_path] {
            let source = self.root.join(authored_path);
            if let Ok(relative) = source.strip_prefix(&current) {
                if source.is_file() {
                    let destination = staging.join(relative);
                    if let Some(parent) = destination.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::copy(source, destination)?;
                }
            }
        }
        if current.exists() {
            fs::rename(&current, &backup)?;
            sync_dir(current.parent().expect("issue parent"))?;
        }
        if fail_after_backup {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected interruption after preserving complete prior generation",
            ));
        }
        fs::rename(&staging, &current)?;
        sync_dir(current.parent().expect("issue parent"))?;
        if backup.exists() {
            fs::remove_dir_all(&backup)?;
            sync_dir(current.parent().expect("issue parent"))?;
        }
        Ok(())
    }

    pub(crate) fn replace_record(
        &self,
        issue: u64,
        expected_digest: &str,
        record: &IssueRecord,
    ) -> Result<()> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let current = self.load_record(issue)?;
        if current.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "record changed before compare-and-swap commit",
            ));
        }
        let cards = self.load_cards(issue)?;
        verify_cards(self, &current, &cards)?;
        self.commit(issue, record, &cards, false)
    }

    pub(crate) fn commit_migration(
        &self,
        issue: u64,
        expected_digest: &str,
        evidence: crate::model::MigrationEvidence,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "migration record changed before commit",
            ));
        }
        if let Some(existing) = &record.migration {
            if existing == &evidence {
                return Ok(record);
            }
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "existing migration evidence differs from the source digest or retained authored truth",
            ));
        }
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
        }
        record.migration = Some(evidence);
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: "csdlc-import".into(),
            reason: "attach one-way legacy authored-content evidence and sunset metadata".into(),
            operation: "record_migration".into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_publication(
        &self,
        issue: u64,
        expected_digest: &str,
        claim_id: &str,
        actor: String,
        evidence: PublicationEvidence,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(claim_id, now_seconds()?)?;
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "publication record changed before commit",
            ));
        }
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            CardContent::Sor(values) => values,
            _ => unreachable!("SOR"),
        };
        sor.integration_state = crate::cards::IntegrationState::PrOpen;
        sor.publication_state = if evidence.draft {
            crate::cards::PublicationState::Draft
        } else {
            crate::cards::PublicationState::Ready
        };
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
        }
        record.publication = Some(evidence);
        if record.phase == LifecyclePhase::Reviewed {
            record.advance(
                LifecyclePhase::Published,
                actor.clone(),
                "observed exact draft PR after current review".into(),
            )?;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason: "atomically record observed GitHub publication and SOR projection".into(),
            operation: "record_publication".into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_readiness(
        &self,
        request: crate::readiness::ReadinessRequest,
        evidence: ReadinessEvidence,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(request.issue)?;
        self.recover_if_needed(request.issue)?;
        let mut record = self.load_record(request.issue)?;
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(&request.claim_id, now_seconds()?)?;
        if record.generation != request.expected_generation
            || record.digest != request.expected_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "readiness request does not match canonical record",
            ));
        }
        if !matches!(
            record.phase,
            LifecyclePhase::Published | LifecyclePhase::MergeReady
        ) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "readiness requires published state",
            ));
        }
        let publication = record.publication.as_ref().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidTransition, "publication evidence missing")
        })?;
        if publication.pull_request != request.pull_request
            || publication.revision != crate::git::clean_commit_revision(&request.head_sha)
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "readiness observation does not match published PR revision",
            ));
        }
        if record.readiness.as_ref() == Some(&evidence) {
            return Ok(record);
        }
        let mut cards = self.load_cards(request.issue)?;
        verify_cards(self, &record, &cards)?;
        let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            CardContent::Sor(value) => value,
            _ => unreachable!(),
        };
        if evidence.ready {
            let validation_ready = !sor.actual_validation.is_empty()
                && sor.actual_validation.iter().all(|item| {
                    matches!(
                        item.outcome,
                        crate::cards::EvidenceOutcome::Passed
                            | crate::cards::EvidenceOutcome::SkippedNonGoal
                    )
                });
            if !validation_ready {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "merge readiness requires passing local PVF evidence",
                ));
            }
            sor.publication_state = crate::cards::PublicationState::Ready;
            if record.phase == LifecyclePhase::Published {
                record.advance(
                    LifecyclePhase::MergeReady,
                    request.actor.clone(),
                    "observed required checks, review, and conflict readiness".into(),
                )?;
            }
        } else {
            sor.publication_state = crate::cards::PublicationState::Draft;
            if record.phase == LifecyclePhase::MergeReady {
                record.advance(
                    LifecyclePhase::Published,
                    request.actor.clone(),
                    "latest remote observation revoked merge readiness".into(),
                )?;
            }
        }
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
        }
        record.readiness = Some(evidence);
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: request.actor,
            reason: "record normalized remote readiness without replacing pre-publication review"
                .into(),
            operation: "record_readiness".into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(request.issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_terminal(
        &self,
        observation: crate::readiness::TerminalObservation,
        mut evidence: TerminalEvidence,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(observation.issue)?;
        self.recover_if_needed(observation.issue)?;
        let mut record = self.load_record(observation.issue)?;
        if let Some(current) = &record.terminal {
            if record.phase == LifecyclePhase::ClosedOut
                && current.pull_request == evidence.pull_request
                && current.disposition == evidence.disposition
                && current.observed_sha == evidence.observed_sha
                && current.observed_state == evidence.observed_state
                && current.receipt_path == evidence.receipt_path
            {
                return Ok(record);
            }
        }
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(&observation.claim_id, now_seconds()?)?;
        if record.generation != observation.expected_generation
            || record.digest != observation.expected_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal observation does not match canonical record",
            ));
        }
        if !crate::readiness::terminal_phase_allowed(record.phase, observation.disposition) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal disposition is not valid from the current lifecycle phase",
            ));
        }
        match (
            &record.publication,
            observation.pull_request,
            observation.observed_sha.as_deref(),
        ) {
            (Some(publication), Some(pr), Some(sha)) => {
                if publication.pull_request != pr
                    || publication.revision != crate::git::clean_commit_revision(sha)
                {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "terminal PR or SHA differs from exact publication evidence",
                    ));
                }
            }
            (Some(_), None, _) => {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "published issue cannot use no-PR closeout",
                ));
            }
            (Some(_), Some(_), None) => {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "published terminal observation is missing the exact head SHA",
                ));
            }
            (None, Some(_), _) => {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "terminal PR has no canonical publication evidence",
                ));
            }
            _ => {}
        }
        let mut cards = self.load_cards(observation.issue)?;
        verify_cards(self, &record, &cards)?;
        let sor_values = cards.get_mut(&CardKind::Sor).expect("SOR");
        sor_values.status = crate::cards::CardStatus::Complete;
        let sor = match &mut sor_values.content {
            CardContent::Sor(value) => value,
            _ => unreachable!(),
        };
        sor.publication_state = crate::cards::PublicationState::Closed;
        sor.closeout_state = crate::cards::CloseoutState::Complete;
        match observation.disposition {
            crate::readiness::TerminalDisposition::Merged => {
                sor.integration_state = crate::cards::IntegrationState::Merged;
                sor.merge_state = crate::cards::MergeState::Merged;
                record.advance(
                    LifecyclePhase::Merged,
                    observation.actor.clone(),
                    "observed exact PR merged".into(),
                )?;
                record.advance(
                    LifecyclePhase::ClosedOut,
                    observation.actor.clone(),
                    "terminal truth recorded and claim released".into(),
                )?;
            }
            crate::readiness::TerminalDisposition::ClosedUnmerged
            | crate::readiness::TerminalDisposition::ClosedNoPr => {
                sor.integration_state = crate::cards::IntegrationState::ClosedNoPr;
                sor.merge_state = crate::cards::MergeState::ClosedUnmerged;
                let from = record.phase;
                record.phase = LifecyclePhase::ClosedOut;
                record.transitions.push(TransitionEvent {
                    sequence: record.transitions.len() as u64 + 1,
                    from,
                    to: LifecyclePhase::ClosedOut,
                    actor: observation.actor.clone(),
                    reason: "observed approved non-merged terminal disposition".into(),
                });
            }
        }
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        let released = record.claim.take().expect("validated claim");
        evidence.released_branch = released.branch;
        evidence.released_worktree = released.worktree;
        evidence.released_protected_paths = released.protected_paths;
        record.terminal = Some(evidence);
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: observation.actor,
            reason: "atomically finalize SOR/index and release claim/protected paths".into(),
            operation: "record_terminal".into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(observation.issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_review(
        &self,
        issue: u64,
        expected_digest: &str,
        actor: String,
        claim_id: &str,
        evidence: ReviewEvidence,
        result: crate::cards::ReviewResult,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(claim_id, now_seconds()?)?;
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "review record changed before commit",
            ));
        }
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        let srp = match &mut cards.get_mut(&CardKind::Srp).expect("SRP").content {
            CardContent::Srp(values) => values,
            _ => unreachable!("SRP"),
        };
        srp.reviewer = Some(evidence.reviewer.clone());
        srp.review_scope = evidence.scope.join("\n");
        srp.review_revision = Some(evidence.reviewed_revision.clone());
        srp.review_result = result;
        srp.residual_risk = evidence.residual_risks.clone();
        srp.findings = evidence
            .findings
            .iter()
            .map(|finding| crate::cards::ReviewFinding {
                id: finding.id.clone(),
                severity: finding.severity,
                summary: finding.summary.clone(),
                actionable: finding.actionable,
                in_scope: finding.in_scope,
                disposition: finding.disposition,
                fix_revision: finding.fix_revision.clone(),
                route: finding.route.clone(),
            })
            .collect();
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
        }
        record.review = Some(evidence);
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason: "atomically record review evidence and SRP projection".into(),
            operation: "record_review".into(),
        });
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_review_assignment(
        &self,
        issue: u64,
        expected_digest: &str,
        claim_id: &str,
        assignment: ReviewAssignment,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "review assignment changed before commit",
            ));
        }
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(claim_id, now_seconds()?)?;
        if record.phase != LifecyclePhase::Implemented {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "review assignment requires implemented phase",
            ));
        }
        let cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        let actor = assignment.assigned_by.clone();
        record.review_assignment = Some(assignment);
        record.review = None;
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason: "assign bounded exact-revision review".into(),
            operation: "assign_review".into(),
        });
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_review_recovery(
        &self,
        issue: u64,
        expected_generation: u64,
        expected_digest: &str,
        claim_id: &str,
        actor: String,
        reason: String,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.generation != expected_generation {
            return Err(V2Error::new(
                ErrorCode::StaleGeneration,
                "review recovery generation changed before commit",
            ));
        }
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "review recovery record changed before commit",
            ));
        }
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(claim_id, now_seconds()?)?;
        if !matches!(
            record.phase,
            LifecyclePhase::Reviewed | LifecyclePhase::Published | LifecyclePhase::MergeReady
        ) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "review recovery requires reviewed phase",
            ));
        }
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        cards.get_mut(&CardKind::Srp).expect("SRP").status = crate::cards::CardStatus::Draft;
        let srp = match &mut cards.get_mut(&CardKind::Srp).expect("SRP").content {
            CardContent::Srp(values) => values,
            _ => unreachable!("SRP"),
        };
        srp.review_scope.clear();
        srp.review_revision = None;
        srp.reviewer = None;
        srp.findings.clear();
        srp.residual_risk.clear();
        srp.review_result = crate::cards::ReviewResult::PreReview;
        if let CardContent::Sor(sor) = &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            sor.publication_state = crate::cards::PublicationState::NotPublished;
            sor.integration_state = crate::cards::IntegrationState::WorktreeOnly;
            sor.merge_state = crate::cards::MergeState::NotMerged;
            sor.closeout_state = crate::cards::CloseoutState::NotStarted;
        }
        record.advance(LifecyclePhase::Implemented, actor.clone(), reason.clone())?;
        record.review_assignment = None;
        record.review = None;
        record.publication = None;
        record.readiness = None;
        record.terminal = None;
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason,
            operation: "recover_review".into(),
        });
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BootstrapRequest {
    pub issue: u64,
    pub repository: String,
    pub design_path: String,
    pub diagram_path: String,
    pub design_reviewer: String,
    #[serde(default)]
    pub design_approved: bool,
    pub claim: Claim,
    pub initial: InitialCardInput,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EditRequest {
    pub issue: u64,
    pub card: CardKind,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub claim_id: String,
    pub actor: String,
    pub reason: String,
    pub operation: SemanticOperation,
    #[serde(default)]
    pub fail_after_backup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ApproveDesignRequest {
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub claim_id: String,
    pub reviewer: String,
}

pub fn approve_design(store: &Store, request: ApproveDesignRequest) -> Result<IssueRecord> {
    let _lock = store.lock(request.issue)?;
    store.recover_if_needed(request.issue)?;
    let mut record = store.load_record(request.issue)?;
    if record.generation != request.expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "design approval generation is stale",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "design approval digest is stale",
        ));
    }
    if request.reviewer.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design reviewer is required",
        ));
    }
    if record.phase != LifecyclePhase::Initialized
        || !matches!(
            record.design_review,
            DesignReview::Pending | DesignReview::ChangesRequired { .. }
        )
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "design approval is allowed only before readiness",
        ));
    }
    record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
        .validate(&request.claim_id, now_seconds()?)?;
    let mut cards = store.load_cards(request.issue)?;
    verify_record(&record)?;
    let design_digest = digest(&fs::read(store.root.join(&record.design_path))?);
    let diagram_digest = digest(&fs::read(store.root.join(&record.diagram_path))?);
    for kind in [CardKind::Spp, CardKind::Vpp] {
        match &mut cards.get_mut(&kind).expect("card").content {
            CardContent::Spp(values) => {
                values.design_digest = design_digest.clone();
                values.diagram_digest = diagram_digest.clone();
            }
            CardContent::Vpp(values) => {
                values.design_digest = design_digest.clone();
                values.diagram_digest = diagram_digest.clone();
            }
            _ => unreachable!("design-bearing card"),
        }
    }
    record.design_review = DesignReview::Approved {
        reviewer: request.reviewer.clone(),
        revision: design_digest,
    };
    record.generation += 1;
    for values in cards.values_mut() {
        values.identity.generation = record.generation;
    }
    if let Some(claim) = record.claim.as_mut() {
        claim.generation = record.generation;
    }
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.reviewer,
        reason: "approve completed issue design".into(),
        operation: "approve_design".into(),
    });
    hydrate_projections(&mut record, &cards)?;
    record.digest = record_digest(&record)?;
    store.commit(request.issue, &record, &cards, false)?;
    Ok(record)
}

pub fn bootstrap_issue(store: &Store, request: BootstrapRequest) -> Result<IssueRecord> {
    validate_bootstrap_request(&request)?;
    let initialization_digest = digest(&serde_json::to_vec(&request)?);
    let _lock = store.lock(request.issue)?;
    store.recover_if_needed(request.issue)?;
    let index_path = store.issue_dir(request.issue).join("index.json");
    if index_path.exists() {
        let existing = store.load_record(request.issue)?;
        verify_cards(store, &existing, &store.load_cards(request.issue)?)?;
        if existing.initialization_digest == initialization_digest {
            return Ok(existing);
        }
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue exists with different initialization truth",
        ));
    }
    let bootstrap_actor = request.claim.owner.clone();
    let design_digest = digest(&fs::read(store.root.join(&request.design_path))?);
    let diagram_digest = digest(&fs::read(store.root.join(&request.diagram_path))?);
    let cards = initial_cards(
        request.issue,
        &request.repository,
        &request.design_path,
        &design_digest,
        &request.diagram_path,
        &diagram_digest,
        request.initial,
    )?;
    let mut record = IssueRecord {
        schema: "csdlc.issue.index.v1".into(),
        issue: request.issue,
        repository: request.repository,
        initialization_digest,
        phase: LifecyclePhase::Initialized,
        generation: 0,
        digest: String::new(),
        claim: Some(request.claim),
        review_assignment: None,
        review: None,
        publication: None,
        readiness: None,
        terminal: None,
        migration: None,
        design_path: request.design_path,
        diagram_path: request.diagram_path,
        design_review: if request.design_approved {
            DesignReview::Approved {
                reviewer: request.design_reviewer,
                revision: design_digest,
            }
        } else {
            DesignReview::Pending
        },
        cards: BTreeMap::new(),
        transitions: Vec::new(),
        audit: vec![AuditEvent {
            sequence: 1,
            generation: 0,
            actor: bootstrap_actor,
            reason: "initialize issue record and all six cards".into(),
            operation: "bootstrap".into(),
        }],
    };
    hydrate_projections(&mut record, &cards)?;
    record.digest = record_digest(&record)?;
    store.commit(request.issue, &record, &cards, false)?;
    Ok(record)
}

pub(crate) fn validate_bootstrap_request(request: &BootstrapRequest) -> Result<()> {
    if request.issue == 0 || request.repository.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue and repository are required",
        ));
    }
    let now = now_seconds()?;
    if (request.design_approved && request.design_reviewer.trim().is_empty())
        || request.claim.id.trim().is_empty()
        || request.claim.owner.trim().is_empty()
        || request.claim.purpose.trim().is_empty()
        || request.claim.branch.trim().is_empty()
        || request.claim.worktree.trim().is_empty()
        || request.claim.generation != 0
        || request.claim.protected_paths.is_empty()
        || request.claim.heartbeat_unix_seconds < request.claim.acquired_unix_seconds
        || request.claim.expires_unix_seconds <= request.claim.heartbeat_unix_seconds
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "bootstrap claim/reviewer invariants are incomplete",
        ));
    }
    request.claim.validate(&request.claim.id, now)?;
    Ok(())
}

pub fn edit_issue(store: &Store, request: EditRequest) -> Result<IssueRecord> {
    let _lock = store.lock(request.issue)?;
    store.recover_if_needed(request.issue)?;
    let mut record = store.load_record(request.issue)?;
    if record.generation != request.expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "expected generation is stale",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "expected issue digest is stale",
        ));
    }
    let now = now_seconds()?;
    let claim = record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "mutation requires a claim"))?;
    claim.validate(&request.claim_id, now)?;
    if claim.generation != record.generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "claim generation is stale",
        ));
    }
    let mut cards = store.load_cards(request.issue)?;
    verify_cards(store, &record, &cards)?;
    if matches!(
        record.phase,
        LifecyclePhase::Reviewed | LifecyclePhase::Published | LifecyclePhase::MergeReady
    ) && matches!(
        request.operation,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Implemented
        }
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "reviewed work must use typed csdlc-review recover",
        ));
    }
    authorize_card_operation(record.phase, request.card, &request.operation)?;
    let replan_before = match &request.operation {
        SemanticOperation::Replan { field, .. } => Some(current_text_value(
            cards
                .get(&request.card)
                .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "card projection missing"))?,
            *field,
        )?),
        _ => None,
    };
    let audit_operation = match (&request.operation, replan_before) {
        (SemanticOperation::Replan { field, value }, Some(previous)) => serde_json::json!({
            "operation": "replan",
            "field": field.as_ref(),
            "previous_value": previous,
            "new_value": value,
        })
        .to_string(),
        _ => serde_json::to_string(&request.operation)?,
    };
    let values = cards
        .get_mut(&request.card)
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "card projection missing"))?;
    if let Some(next) = apply(values, &request.operation)? {
        validate_phase_guard(store, &record, &cards, next)?;
        record.advance(next, request.actor.clone(), request.reason.clone())?;
    }
    let design_digest = digest(&fs::read(store.root.join(&record.design_path))?);
    let diagram_digest = digest(&fs::read(store.root.join(&record.diagram_path))?);
    validate_cross_card(
        &cards,
        &record.design_path,
        &design_digest,
        &record.diagram_path,
        &diagram_digest,
    )?;
    record.generation += 1;
    for values in cards.values_mut() {
        values.identity.generation = record.generation;
    }
    if let Some(claim) = record.claim.as_mut() {
        claim.generation = record.generation;
    }
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor,
        reason: request.reason,
        operation: audit_operation,
    });
    hydrate_projections(&mut record, &cards)?;
    record.digest = record_digest(&record)?;
    store.commit(request.issue, &record, &cards, request.fail_after_backup)?;
    Ok(record)
}

fn current_text_value(values: &CardValues, field: crate::cards::TextField) -> Result<String> {
    match (&values.content, field) {
        (CardContent::Sip(value), crate::cards::TextField::Goal) => Ok(value.goal.clone()),
        (CardContent::Sip(value), crate::cards::TextField::RequiredOutcome) => {
            Ok(value.required_outcome.clone())
        }
        (CardContent::Stp(value), crate::cards::TextField::TaskBoundary) => {
            Ok(value.task_boundary.clone())
        }
        (CardContent::Spp(value), crate::cards::TextField::PlanSummary) => {
            Ok(value.summary.clone())
        }
        _ => Err(V2Error::new(
            ErrorCode::FieldOwnership,
            "replan field is not owned by the selected planning card",
        )),
    }
}

pub(crate) fn verify_cards(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    verify_record(record)?;
    for (kind, values) in cards {
        if values.kind() != *kind
            || values.identity.issue != record.issue
            || values.identity.repository != record.repository
            || values.identity.generation != record.generation
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{kind} identity/generation mismatch"),
            ));
        }
        let rendered = render(values)?;
        let projection = record.cards.get(kind).ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("missing {kind} projection"),
            )
        })?;
        let tracked = fs::read(
            store
                .issue_dir(record.issue)
                .join("cards")
                .join(format!("{kind}.md")),
        )?;
        if projection.values_digest != rendered.values_digest
            || projection.rendered_digest != rendered.rendered_digest
            || projection.ast_digest != rendered.ast_digest
            || digest(&tracked) != rendered.rendered_digest
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{kind} digest drift"),
            ));
        }
    }
    let design_digest = digest(&fs::read(store.root.join(&record.design_path))?);
    let diagram_digest = digest(&fs::read(store.root.join(&record.diagram_path))?);
    validate_cross_card(
        cards,
        &record.design_path,
        &design_digest,
        &record.diagram_path,
        &diagram_digest,
    )?;
    Ok(())
}

pub(crate) fn verify_record(record: &IssueRecord) -> Result<()> {
    if record.schema != "csdlc.issue.index.v1"
        || record.issue == 0
        || record.repository.is_empty()
        || record.initialization_digest.is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "invalid index identity/schema",
        ));
    }
    if record.digest != record_digest(record)? {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "index digest mismatch",
        ));
    }
    if record.phase == LifecyclePhase::ClosedOut {
        if record.claim.is_some() || record.terminal.is_none() {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "closed-out record must have terminal evidence and no active claim",
            ));
        }
    } else {
        let claim = record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "claim missing"))?;
        if claim.generation != record.generation
            || claim.id.is_empty()
            || claim.owner.is_empty()
            || claim.protected_paths.is_empty()
            || claim.branch.is_empty()
            || claim.worktree.is_empty()
            || claim.heartbeat_unix_seconds < claim.acquired_unix_seconds
            || claim.expires_unix_seconds <= claim.heartbeat_unix_seconds
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "claim invariant failed",
            ));
        }
    }
    if let DesignReview::Approved { reviewer, revision } = &record.design_review {
        if reviewer.trim().is_empty() || revision.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "design review evidence is empty",
            ));
        }
    }
    if record.audit.is_empty()
        || record.audit.iter().enumerate().any(|(index, event)| {
            event.sequence != index as u64 + 1
                || event.generation > record.generation
                || event.actor.is_empty()
                || event.reason.is_empty()
        })
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "audit sequence invariant failed",
        ));
    }
    let mut phase = LifecyclePhase::Initialized;
    for (index, event) in record.transitions.iter().enumerate() {
        if event.sequence != index as u64 + 1
            || event.from != phase
            || !event.from.allows(event.to)
            || event.actor.is_empty()
            || event.reason.is_empty()
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "transition log invariant failed",
            ));
        }
        phase = event.to;
    }
    if phase != record.phase {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "phase does not match transition log",
        ));
    }
    Ok(())
}

fn validate_phase_guard(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
    next: LifecyclePhase,
) -> Result<()> {
    if next == LifecyclePhase::Ready {
        verify_cards(store, record, cards)?;
        if !matches!(record.design_review, DesignReview::Approved { .. })
            || !matches!(
                cards[&CardKind::Sip].status,
                crate::cards::CardStatus::Ready | crate::cards::CardStatus::Approved
            )
            || !matches!(
                cards[&CardKind::Stp].status,
                crate::cards::CardStatus::Ready | crate::cards::CardStatus::Approved
            )
            || !matches!(
                cards[&CardKind::Spp].status,
                crate::cards::CardStatus::Ready | crate::cards::CardStatus::Approved
            )
            || !matches!(
                cards[&CardKind::Vpp].status,
                crate::cards::CardStatus::Ready | crate::cards::CardStatus::Approved
            )
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "ready phase guard failed",
            ));
        }
    }
    if next == LifecyclePhase::Implemented {
        if let CardContent::Sor(sor) = &cards[&CardKind::Sor].content {
            if sor.actual_changes.is_empty() {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "implementation evidence missing",
                ));
            }
        }
    }
    if next == LifecyclePhase::Reviewed {
        if let CardContent::Srp(srp) = &cards[&CardKind::Srp].content {
            if srp
                .review_revision
                .as_deref()
                .unwrap_or_default()
                .is_empty()
                || srp.reviewer.as_deref().unwrap_or_default().is_empty()
                || srp.review_result != crate::cards::ReviewResult::Pass
                || srp.findings.iter().any(|finding| {
                    finding.actionable
                        && finding.disposition == crate::cards::FindingDisposition::Open
                })
            {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "review evidence is incomplete",
                ));
            }
        }
    }
    let srp = match &cards[&CardKind::Srp].content {
        CardContent::Srp(values) => values,
        _ => unreachable!("SRP"),
    };
    let sor = match &cards[&CardKind::Sor].content {
        CardContent::Sor(values) => values,
        _ => unreachable!("SOR"),
    };
    let review_current = srp.review_result == crate::cards::ReviewResult::Pass
        && srp.review_revision.as_deref().unwrap_or_default() != ""
        && srp.reviewer.as_deref().unwrap_or_default() != ""
        && !srp.findings.iter().any(|finding| {
            finding.actionable && finding.disposition == crate::cards::FindingDisposition::Open
        });
    let validation_passed = !sor.actual_validation.is_empty()
        && sor.actual_validation.iter().all(|result| {
            !result.command.is_empty()
                && !result.purpose.is_empty()
                && !result.evidence_ref.is_empty()
                && matches!(
                    result.outcome,
                    crate::cards::EvidenceOutcome::Passed
                        | crate::cards::EvidenceOutcome::SkippedNonGoal
                )
        });
    if next == LifecyclePhase::Published
        && (!review_current
            || record.review_assignment.as_ref().is_none_or(|assignment| {
                crate::git::substantive_revision(store.root(), &assignment.scope).map_or(
                    true,
                    |current| {
                        !evaluate_publication_review_in_repo(
                            store.root(),
                            record.review.as_ref(),
                            &current,
                        )
                        .ready
                    },
                )
            })
            || !matches!(
                sor.publication_state,
                crate::cards::PublicationState::Draft | crate::cards::PublicationState::Ready
            ))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "publication observation or current review evidence is missing",
        ));
    }
    if next == LifecyclePhase::MergeReady
        && (!review_current
            || record.review_assignment.as_ref().is_none_or(|assignment| {
                crate::git::substantive_revision(store.root(), &assignment.scope).map_or(
                    true,
                    |current| {
                        !evaluate_publication_review_in_repo(
                            store.root(),
                            record.review.as_ref(),
                            &current,
                        )
                        .ready
                    },
                )
            })
            || !validation_passed
            || sor.publication_state != crate::cards::PublicationState::Ready)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "merge readiness requires current review, passing evidence, and ready publication",
        ));
    }
    if next == LifecyclePhase::Merged && sor.merge_state != crate::cards::MergeState::Merged {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "merged phase requires observed merged state",
        ));
    }
    if next == LifecyclePhase::ClosedOut
        && (sor.closeout_state != crate::cards::CloseoutState::Complete
            || !matches!(
                sor.integration_state,
                crate::cards::IntegrationState::Merged | crate::cards::IntegrationState::ClosedNoPr
            )
            || !matches!(
                sor.merge_state,
                crate::cards::MergeState::Merged | crate::cards::MergeState::ClosedUnmerged
            )
            || cards[&CardKind::Sor].status != crate::cards::CardStatus::Complete)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "closeout phase requires terminal SOR truth",
        ));
    }
    Ok(())
}

fn authorize_card_operation(
    phase: LifecyclePhase,
    card: CardKind,
    operation: &SemanticOperation,
) -> Result<()> {
    if matches!(operation, SemanticOperation::AdvancePhase { .. }) {
        return Ok(());
    }
    let allowed = matches!(
        (phase, card, operation),
        (
            LifecyclePhase::Initialized | LifecyclePhase::Ready,
            CardKind::Sip | CardKind::Stp | CardKind::Spp | CardKind::Vpp,
            SemanticOperation::SetField { .. }
                | SemanticOperation::AppendReference { .. }
                | SemanticOperation::AdvanceStatus { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Sip | CardKind::Stp | CardKind::Spp,
            SemanticOperation::Replan { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Sor,
            SemanticOperation::RecordExecution { .. }
                | SemanticOperation::RecordValidation { .. }
                | SemanticOperation::AppendReference { .. }
                | SemanticOperation::AdvanceStatus { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Srp,
            SemanticOperation::RecordReview { .. }
                | SemanticOperation::RecordFinding { .. }
                | SemanticOperation::DisposeFinding { .. }
                | SemanticOperation::AppendReference { .. }
                | SemanticOperation::AdvanceStatus { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Sor,
            SemanticOperation::RecordExecution { .. }
                | SemanticOperation::RecordValidation { .. }
                | SemanticOperation::AppendReference { .. },
        ) | (
            LifecyclePhase::Reviewed | LifecyclePhase::Published,
            CardKind::Srp,
            SemanticOperation::RecordFinding { .. }
                | SemanticOperation::DisposeFinding { .. }
                | SemanticOperation::AppendReference { .. },
        ) | (
            LifecyclePhase::Reviewed | LifecyclePhase::Published,
            CardKind::Sor,
            SemanticOperation::RecordPublication { .. }
                | SemanticOperation::RecordValidation { .. }
                | SemanticOperation::AppendReference { .. },
        ) | (
            LifecyclePhase::MergeReady,
            CardKind::Sor,
            SemanticOperation::RecordMerge { .. }
                | SemanticOperation::RecordValidation { .. }
                | SemanticOperation::AppendReference { .. },
        ) | (
            LifecyclePhase::MergeReady,
            CardKind::Srp,
            SemanticOperation::RecordFinding { .. } | SemanticOperation::DisposeFinding { .. },
        ) | (
            LifecyclePhase::Merged,
            CardKind::Sor,
            SemanticOperation::RecordCloseout { .. }
                | SemanticOperation::RecordValidation { .. }
                | SemanticOperation::AppendReference { .. }
                | SemanticOperation::AdvanceStatus { .. },
        )
    );
    if allowed {
        Ok(())
    } else {
        Err(V2Error::new(
            ErrorCode::InvalidTransition,
            format!("{card} mutation is not allowed during {phase}"),
        ))
    }
}

fn hydrate_projections(
    record: &mut IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    record.cards.clear();
    for (kind, values) in cards {
        let rendered = render(values)?;
        record.cards.insert(
            *kind,
            CardProjection {
                values_digest: rendered.values_digest,
                rendered_digest: rendered.rendered_digest,
                ast_digest: rendered.ast_digest,
            },
        );
    }
    Ok(())
}

fn validate_updated_cards(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    let design_digest = digest(&fs::read(store.root.join(&record.design_path))?);
    let diagram_digest = digest(&fs::read(store.root.join(&record.diagram_path))?);
    validate_cross_card(
        cards,
        &record.design_path,
        &design_digest,
        &record.diagram_path,
        &diagram_digest,
    )
}

pub(crate) fn record_digest(record: &IssueRecord) -> Result<String> {
    let mut value = record.clone();
    value.digest.clear();
    Ok(digest(&serde_json::to_vec(&value)?))
}

fn write_complete(
    path: &Path,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    let cards_dir = path.join("cards");
    fs::create_dir_all(&cards_dir)?;
    write_json(&path.join("index.json"), record)?;
    let mut audit = File::create(path.join("audit.jsonl"))?;
    for event in &record.audit {
        serde_json::to_writer(&mut audit, event)?;
        audit.write_all(b"\n")?;
    }
    audit.sync_all()?;
    for (kind, values) in cards {
        let rendered = render(values)?;
        write_json(&cards_dir.join(format!("{kind}.values.json")), values)?;
        let mut file = File::create(cards_dir.join(format!("{kind}.md")))?;
        file.write_all(rendered.markdown.as_bytes())?;
        file.sync_all()?;
    }
    sync_dir(&cards_dir)?;
    sync_dir(path)?;
    Ok(())
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let mut file = File::create(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn sync_dir(path: &Path) -> Result<()> {
    File::open(path)?.sync_all()?;
    Ok(())
}

pub(crate) fn now_seconds() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| V2Error::new(ErrorCode::InvalidInput, error.to_string()))
}

fn enum_iterator() -> impl Iterator<Item = CardKind> {
    use strum::IntoEnumIterator;
    CardKind::iter()
}
