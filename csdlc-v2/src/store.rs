use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use fs2::FileExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cards::{
    apply, digest, initial_cards, render, terminal_validation_passed, validate_cross_card,
    validate_result, CardContent, CardKind, CardValues, InitialCardInput, SemanticOperation,
    ValidationResult,
};
use crate::error::{ErrorCode, Result, V2Error};
use crate::model::{
    AuditEvent, CardProjection, Claim, DesignReview, IssueRecord, LifecyclePhase,
    PublicationEvidence, ReviewAssignment, ReviewEvidence, TerminalEvidence, TerminalReceipt,
    TransitionEvent,
};
use crate::review::evaluate_publication_review_in_repo;

#[derive(Debug, Clone)]
pub struct Store {
    root: PathBuf,
}

#[derive(Debug, Clone)]
pub(crate) struct ImplementationCommit {
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub claim_id: String,
    pub actor: String,
    pub summary: String,
    pub changes: Vec<String>,
    pub artifacts: Vec<String>,
    pub validation: Vec<ValidationResult>,
}

#[derive(Debug, Clone)]
pub(crate) struct ReviewCommit {
    pub issue: u64,
    pub expected_digest: String,
    pub actor: String,
    pub claim_id: String,
    pub evidence: ReviewEvidence,
    pub result: crate::cards::ReviewResult,
    pub advance_reviewed: bool,
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
        let relative = PathBuf::from(format!(".csdlc/locks/{issue}.lock"));
        require_canonical_parent_beneath(&self.root, &relative)?;
        let dir = self.root.join(".csdlc/locks");
        fs::create_dir_all(&dir)?;
        require_canonical_parent_beneath(&self.root, &relative)?;
        require_regular_or_absent_beneath(&self.root, &relative)?;
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(self.root.join(&relative))?;
        file.lock_exclusive()?;
        require_canonical_parent_beneath(&self.root, &relative)?;
        require_regular_or_absent_beneath(&self.root, &relative)?;
        Ok(file)
    }

    pub fn authority_projection_lock(&self, issue: u64) -> Result<File> {
        self.lock(issue)
    }

    pub(crate) fn binding_lock(&self) -> Result<File> {
        let common = crate::git::run(
            &self.root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout;
        let dir = PathBuf::from(common).join("csdlc-v2");
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

    pub(crate) fn remove_unstarted_binding_projection(
        &self,
        issue: u64,
        claim_id: &str,
        expected_digest: &str,
    ) -> Result<()> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let record = self.load_record(issue)?;
        if record.digest != expected_digest
            || record.claim.as_ref().map(|claim| claim.id.as_str()) != Some(claim_id)
            || !matches!(
                record.phase,
                LifecyclePhase::Initialized | LifecyclePhase::Ready | LifecyclePhase::Bound
            )
            || record.publication.is_some()
            || record.review.is_some()
            || record.terminal.is_some()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "binding projection has execution or mismatched ownership and cannot be released",
            ));
        }
        fs::remove_dir_all(self.issue_dir(issue))?;
        sync_dir(&self.root.join(".csdlc/issues"))?;
        Ok(())
    }

    pub fn load_record(&self, issue: u64) -> Result<IssueRecord> {
        let record: IssueRecord = read_json(&self.issue_dir(issue).join("index.json"))?;
        if record.issue != issue {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!(
                    "issue projection namespace mismatch: requested {issue}, embedded {}",
                    record.issue
                ),
            ));
        }
        Ok(record)
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

    fn git_common_relative(&self, path: &Path) -> Result<(PathBuf, PathBuf)> {
        let common = PathBuf::from(
            crate::git::run(
                &self.root,
                &["rev-parse", "--path-format=absolute", "--git-common-dir"],
            )?
            .stdout,
        );
        let relative = path.strip_prefix(&common).map_err(|_| {
            V2Error::new(
                ErrorCode::UnsafeCheckout,
                "terminal receipt path escapes its Git-common root",
            )
        })?;
        if !crate::pvf::clean_relative(relative) {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "terminal receipt path is not clean beneath its Git-common root",
            ));
        }
        Ok((common, relative.to_path_buf()))
    }

    pub fn terminal_receipt_path(&self, issue: u64) -> Result<PathBuf> {
        let common = crate::git::run(
            &self.root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout;
        Ok(PathBuf::from(common)
            .join("csdlc-v2/closeout")
            .join(format!("{issue}.json")))
    }

    pub fn load_terminal_receipt(&self, issue: u64) -> Result<Option<TerminalReceipt>> {
        let path = self.terminal_receipt_path(issue)?;
        let (common, relative) = self.git_common_relative(&path)?;
        let Some(metadata) = canonical_path_metadata_beneath(&common, &relative)? else {
            return Ok(None);
        };
        if !metadata.is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "terminal receipt is not a canonical regular file: {}",
                    path.display()
                ),
            ));
        }
        let receipt: TerminalReceipt = read_json(&path)?;
        validate_terminal_receipt(&receipt)?;
        if receipt.issue != issue {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!(
                    "terminal receipt namespace mismatch: requested {issue}, embedded {}",
                    receipt.issue
                ),
            ));
        }
        Ok(Some(receipt))
    }

    pub(crate) fn has_claim_free_terminal_authority(
        &self,
        issue: u64,
        repository: &str,
        initialization_digest: &str,
    ) -> Result<bool> {
        let local = self.load_record(issue)?;
        if local.phase != LifecyclePhase::ClosedOut
            || local.claim.is_some()
            || local.repository != repository
            || local.initialization_digest != initialization_digest
        {
            return Ok(false);
        }
        let Some(receipt) = self.load_terminal_receipt(issue)? else {
            return Ok(false);
        };
        self.legacy_receipt_matches_projection(&receipt)
    }

    pub(crate) fn has_claim_free_retained_terminal_authority(
        &self,
        observed: &IssueRecord,
    ) -> Result<bool> {
        let Some(observed_claim) = observed.claim.as_ref() else {
            return Ok(false);
        };
        let local = self.load_record(observed.issue)?;
        if local.claim.is_some()
            || local.repository != observed.repository
            || local.initialization_digest != observed.initialization_digest
        {
            return Ok(false);
        }
        let Some(receipt) = self.load_terminal_receipt(observed.issue)? else {
            return Ok(false);
        };
        if !self.legacy_receipt_matches_projection(&receipt)? {
            return Ok(false);
        }
        let Some(terminal) = receipt.record.terminal.as_ref() else {
            return Ok(false);
        };
        let mut released_paths = terminal.released_protected_paths.clone();
        let mut observed_paths = observed_claim.protected_paths.clone();
        released_paths.sort();
        observed_paths.sort();
        Ok(receipt.record.phase == LifecyclePhase::ClosedOut
            && receipt.record.claim.is_none()
            && receipt.record.generation > observed.generation
            && terminal.released_branch == observed_claim.branch
            && terminal.released_worktree == observed_claim.worktree
            && released_paths == observed_paths)
    }

    fn legacy_receipt_matches_projection(&self, receipt: &TerminalReceipt) -> Result<bool> {
        let local = self.load_record(receipt.issue)?;
        let cards = self.load_cards(receipt.issue)?;
        if receipt.record != local
            || receipt.cards != cards
            || verify_cards(self, &local, &cards).is_err()
        {
            return Ok(false);
        }
        for (path, expected) in &receipt.authored_artifacts {
            let Some(actual) = read_regular_authored_artifact(&self.root, Path::new(path))? else {
                return Ok(false);
            };
            if actual != expected.as_bytes() {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn recover_local_transaction(&self, issue: u64) -> Result<()> {
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

    fn recover_if_needed(&self, issue: u64) -> Result<()> {
        self.recover_local_transaction(issue)
    }

    fn commit(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
        fail_after_backup: bool,
    ) -> Result<()> {
        self.commit_with_authored(issue, record, cards, fail_after_backup, None)
    }

    fn commit_with_authored(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
        fail_after_backup: bool,
        authored_overrides: Option<&BTreeMap<String, String>>,
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
                let destination = staging.join(relative);
                if let Some(parent) = destination.parent() {
                    fs::create_dir_all(parent)?;
                }
                if let Some(contents) =
                    authored_overrides.and_then(|overrides| overrides.get(authored_path))
                {
                    let mut file = File::create(destination)?;
                    file.write_all(contents.as_bytes())?;
                    file.sync_all()?;
                } else if source.is_file() {
                    fs::copy(source, destination)?;
                }
            }
        }
        if let Some(overrides) = authored_overrides {
            for (authored_path, contents) in overrides {
                if !crate::pvf::clean_relative(Path::new(authored_path)) {
                    return Err(V2Error::new(
                        ErrorCode::InvalidInput,
                        "authored override path must be repository-relative",
                    ));
                }
                let destination = self.root.join(authored_path);
                if let Ok(relative) = destination.strip_prefix(&current) {
                    let staged = staging.join(relative);
                    if let Some(parent) = staged.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    let mut file = File::create(staged)?;
                    file.write_all(contents.as_bytes())?;
                    file.sync_all()?;
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
        if let Some(overrides) = authored_overrides {
            for (authored_path, contents) in overrides {
                let destination = self.root.join(authored_path);
                if destination.strip_prefix(&current).is_ok() {
                    continue;
                }
                destination.parent().ok_or_else(|| {
                    V2Error::new(ErrorCode::InvalidInput, "authored override has no parent")
                })?;
                replace_regular_authored_artifact(
                    &self.root,
                    Path::new(authored_path),
                    contents.as_bytes(),
                    "authored-commit-tmp",
                )?;
            }
        }
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

    pub(crate) fn replace_authority_record(
        &self,
        issue: u64,
        expected_digest: &str,
        record: &IssueRecord,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let current = self.load_record(issue)?;
        if current.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "record changed before compare-and-swap authority commit",
            ));
        }
        let cards = self.load_cards(issue)?;
        // Authority recovery accepts projection drift only when the typed card
        // values, identities, generations, and rendered Markdown agree.
        verify_authority_card_inputs(self, &current, &cards)?;
        let mut repaired = record.clone();
        hydrate_projections(&mut repaired, &cards)?;
        repaired.digest = record_digest(&repaired)?;
        self.commit(issue, &repaired, &cards, false)?;
        Ok(repaired)
    }

    pub(crate) fn replace_authority_projection_locked(
        &self,
        issue: u64,
        expected_digest: &str,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
    ) -> Result<IssueRecord> {
        self.recover_if_needed(issue)?;
        let current = self.load_record(issue)?;
        if current.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "projection changed before compare-and-swap authority materialization",
            ));
        }
        let current_cards = self.load_cards(issue)?;
        verify_cards(self, &current, &current_cards)?;
        verify_canonical_projection_bytes(self, &current, &current_cards)?;
        let mut materialized = record.clone();
        hydrate_projections(&mut materialized, cards)?;
        materialized.digest = record_digest(&materialized)?;
        self.commit(issue, &materialized, cards, false)?;
        if let Err(error) = verify_cards(self, &materialized, cards) {
            self.commit(issue, &current, &current_cards, false)?;
            verify_cards(self, &current, &current_cards)?;
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!(
                    "authority projection failed post-commit verification and was rolled back: {}",
                    error.message
                ),
            ));
        }
        Ok(materialized)
    }

    pub(crate) fn materialize_terminal_from_derived(
        &self,
        issue: u64,
        expected_generation: u64,
        expected_digest: &str,
        actor: &str,
        reason: &str,
        envelope: &crate::finish::DerivedTerminalEnvelope,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.generation != expected_generation {
            return Err(V2Error::new(
                ErrorCode::StaleGeneration,
                "terminal materialization generation is stale",
            ));
        }
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal materialization digest is stale",
            ));
        }
        if actor.trim().is_empty() || reason.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal materialization actor and reason are required",
            ));
        }
        let source_projection_match = envelope.issue == record.issue
            && envelope.repository == record.repository
            && envelope.initialization_digest == record.initialization_digest
            && envelope.canonical_generation == record.generation
            && envelope.canonical_digest == record.digest;
        let already_materialized_match = terminal_matches_derived(&record, envelope);
        if !source_projection_match && !already_materialized_match {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "derived terminal envelope does not match the expected source projection",
            ));
        }
        crate::finish::validate_envelope(envelope)?;
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        verify_canonical_projection_bytes(self, &record, &cards)?;
        if already_materialized_match {
            if self
                .load_terminal_receipt(issue)?
                .as_ref()
                .is_some_and(|receipt| {
                    self.legacy_receipt_matches_projection(receipt)
                        .unwrap_or(false)
                })
            {
                return Ok(record);
            }
            let receipt = self.build_terminal_receipt(issue, &record, &cards)?;
            self.write_terminal_receipt(issue, &receipt)?;
            return Ok(record);
        }
        let rollback_record = record.clone();
        let rollback_cards = cards.clone();

        let released = match (record.claim.clone(), record.terminal.clone()) {
            (Some(claim), _) => (claim.branch, claim.worktree, claim.protected_paths),
            (None, Some(terminal)) => (
                terminal.released_branch,
                terminal.released_worktree,
                terminal.released_protected_paths,
            ),
            (None, None) => {
                return Err(V2Error::new(
                    ErrorCode::MissingClaim,
                    "terminal materialization requires a source claim or terminal release evidence",
                ));
            }
        };
        let publication = record.publication.as_mut().ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal materialization requires publication evidence",
            )
        })?;
        if publication.pull_request != envelope.pull_request.unwrap_or_default() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal materialization PR does not match publication evidence",
            ));
        }
        let design_bytes = fs::read(self.root.join(&record.design_path))?;
        let diagram_bytes = fs::read(self.root.join(&record.diagram_path))?;
        let design_digest = digest(&design_bytes);
        let diagram_digest = digest(&diagram_bytes);
        if let DesignReview::Approved { revision, .. } = &mut record.design_review {
            *revision = design_digest.clone();
        }
        match &mut cards.get_mut(&CardKind::Spp).expect("SPP").content {
            CardContent::Spp(values) => {
                values.design_ref = record.design_path.clone();
                values.design_digest = design_digest.clone();
                values.diagram_ref = record.diagram_path.clone();
                values.diagram_digest = diagram_digest.clone();
            }
            _ => unreachable!("SPP"),
        }
        match &mut cards.get_mut(&CardKind::Vpp).expect("VPP").content {
            CardContent::Vpp(values) => {
                values.design_ref = record.design_path.clone();
                values.design_digest = design_digest;
                values.diagram_ref = record.diagram_path.clone();
                values.diagram_digest = diagram_digest;
            }
            _ => unreachable!("VPP"),
        }

        let (terminal_disposition, observed_state, integration_state, merge_state) =
            match envelope.disposition {
                crate::finish::FinishDisposition::Merged => (
                    crate::readiness::TerminalDisposition::Merged,
                    "merged",
                    crate::cards::IntegrationState::Merged,
                    crate::cards::MergeState::Merged,
                ),
                crate::finish::FinishDisposition::ClosedUnmerged => (
                    crate::readiness::TerminalDisposition::ClosedUnmerged,
                    "closed_unmerged",
                    crate::cards::IntegrationState::ClosedNoPr,
                    crate::cards::MergeState::ClosedUnmerged,
                ),
                crate::finish::FinishDisposition::ClosedNoPr => (
                    crate::readiness::TerminalDisposition::ClosedNoPr,
                    "closed_no_pr",
                    crate::cards::IntegrationState::ClosedNoPr,
                    crate::cards::MergeState::ClosedUnmerged,
                ),
            };
        publication.observed_state = observed_state.into();

        let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            CardContent::Sor(values) => values,
            _ => unreachable!("SOR"),
        };
        sor.integration_state = integration_state;
        sor.publication_state = crate::cards::PublicationState::Closed;
        sor.merge_state = merge_state;
        sor.closeout_state = crate::cards::CloseoutState::Complete;

        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        record.claim = None;
        record.terminal = Some(TerminalEvidence {
            pull_request: envelope.pull_request,
            disposition: terminal_disposition,
            observed_sha: envelope.head_sha.clone(),
            observed_state: observed_state.into(),
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
            released_branch: released.0,
            released_worktree: released.1,
            released_protected_paths: released.2,
        });
        match record.phase {
            LifecyclePhase::Published => {
                push_legacy_terminal_transition(
                    &mut record,
                    LifecyclePhase::MergeReady,
                    actor,
                    "observed required checks, review, and conflict readiness",
                );
                push_legacy_terminal_transition(
                    &mut record,
                    LifecyclePhase::Merged,
                    actor,
                    "observed exact PR merged",
                );
            }
            LifecyclePhase::MergeReady => {
                push_legacy_terminal_transition(
                    &mut record,
                    LifecyclePhase::Merged,
                    actor,
                    "observed exact PR merged",
                );
            }
            LifecyclePhase::Merged => {}
            LifecyclePhase::ClosedOut => {}
            _ => {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "terminal materialization requires published, merge_ready, or merged phase",
                ));
            }
        }
        if record.phase != LifecyclePhase::ClosedOut {
            push_legacy_terminal_transition(&mut record, LifecyclePhase::ClosedOut, actor, reason);
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: actor.into(),
            reason: reason.into(),
            operation: "materialize_derived_terminal".into(),
        });
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        let receipt = self.build_terminal_receipt(issue, &record, &cards)?;
        self.commit(issue, &record, &cards, false)?;
        if let Err(error) = self.write_terminal_receipt(issue, &receipt) {
            self.commit(issue, &rollback_record, &rollback_cards, false)?;
            verify_cards(self, &rollback_record, &rollback_cards)?;
            return Err(error);
        }
        Ok(record)
    }

    fn build_terminal_receipt(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
    ) -> Result<TerminalReceipt> {
        let mut receipt = TerminalReceipt {
            schema: "csdlc.terminal_receipt.v1".into(),
            issue,
            repository: record.repository.clone(),
            initialization_digest: record.initialization_digest.clone(),
            receipt_ref: format!("csdlc-v2/closeout/{issue}.json"),
            authored_artifacts: BTreeMap::new(),
            record: record.clone(),
            cards: cards.clone(),
            digest: String::new(),
        };
        for authored_path in [&record.design_path, &record.diagram_path] {
            let bytes = read_regular_projection(&self.root, Path::new(authored_path))?;
            receipt.authored_artifacts.insert(
                authored_path.clone(),
                String::from_utf8(bytes).map_err(|error| {
                    V2Error::new(
                        ErrorCode::CorruptRecord,
                        format!("terminal authored artifact is not UTF-8: {error}"),
                    )
                })?,
            );
        }
        receipt.digest = terminal_receipt_digest(&receipt)?;
        validate_terminal_receipt(&receipt)?;
        Ok(receipt)
    }

    fn write_terminal_receipt(&self, issue: u64, receipt: &TerminalReceipt) -> Result<()> {
        let receipt_path = self.terminal_receipt_path(issue)?;
        let (common, relative) = self.git_common_relative(&receipt_path)?;
        replace_regular_authored_artifact(
            &common,
            &relative,
            &serde_json::to_vec_pretty(&receipt)?,
            "tmp",
        )
    }

    pub(crate) fn verify_canonical_authority_projection(
        &self,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
    ) -> Result<()> {
        verify_canonical_projection_bytes(self, record, cards)
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
        sor.merge_state = crate::cards::MergeState::NotMerged;
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
                "observed exact PR after current review".into(),
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

    pub(crate) fn commit_implementation(
        &self,
        commit: ImplementationCommit,
        staged_evidence: &Path,
        evidence_dir: &Path,
    ) -> Result<IssueRecord> {
        let issue = commit.issue;
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.generation != commit.expected_generation
            || record.digest != commit.expected_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "implementation finalization changed before commit",
            ));
        }
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(&commit.claim_id, now_seconds()?)?;
        if record.phase != LifecyclePhase::Bound || commit.validation.is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "implementation finalization requires bound phase and validation evidence",
            ));
        }
        for result in &commit.validation {
            validate_result(result)?;
        }
        if !terminal_validation_passed(&commit.validation) {
            return Err(V2Error::new(
                ErrorCode::ValidationFailed,
                "implementation finalization requires passing validation evidence",
            ));
        }
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            CardContent::Sor(values) => values,
            _ => unreachable!("SOR"),
        };
        sor.summary = commit.summary;
        sor.actual_changes.extend(commit.changes);
        sor.artifacts.extend(commit.artifacts);
        sor.actual_validation.extend(commit.validation);
        record.advance(
            LifecyclePhase::Implemented,
            commit.actor.clone(),
            "execution and passing validation finalized atomically".into(),
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
            actor: commit.actor,
            reason: "atomically record execution, validation, and implemented phase".into(),
            operation: "finalize_implementation".into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        let staged_metadata = fs::symlink_metadata(staged_evidence).map_err(|_| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "staged finalize evidence is missing",
            )
        })?;
        if staged_metadata.file_type().is_symlink() || !staged_metadata.is_dir() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "staged finalize evidence must be a real directory",
            ));
        }
        let evidence_parent = evidence_dir.parent().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidInput, "evidence directory has no parent")
        })?;
        let backup = evidence_parent.join(format!(
            ".csdlc-finalize-backup-{issue}-{}",
            std::process::id()
        ));
        if backup.exists() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "stale finalize evidence backup requires reconciliation",
            ));
        }
        let had_evidence = evidence_dir.exists();
        if had_evidence {
            fs::rename(evidence_dir, &backup)?;
        }
        if let Err(error) = fs::rename(staged_evidence, evidence_dir) {
            if had_evidence && fs::rename(&backup, evidence_dir).is_err() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "failed to restore evidence after finalize publication error",
                ));
            }
            return Err(error.into());
        }
        if fs::symlink_metadata(evidence_dir)
            .map(|metadata| metadata.file_type().is_symlink() || !metadata.is_dir())
            .unwrap_or(true)
        {
            let _ = fs::remove_file(evidence_dir);
            if had_evidence && fs::rename(&backup, evidence_dir).is_err() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "failed to restore evidence after unsafe finalize publication",
                ));
            }
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "published finalize evidence must be a real directory",
            ));
        }
        if let Err(error) = self.commit(issue, &record, &cards, false) {
            let remove_result = fs::remove_dir_all(evidence_dir);
            let restore_result = if had_evidence {
                fs::rename(&backup, evidence_dir)
            } else {
                Ok(())
            };
            if remove_result.is_err() || restore_result.is_err() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    format!(
                        "state commit failed and evidence rollback requires reconciliation: {}",
                        error.message
                    ),
                ));
            }
            return Err(error);
        }
        if had_evidence {
            fs::remove_dir_all(&backup)?;
        }
        Ok(record)
    }

    pub(crate) fn commit_review(&self, commit: ReviewCommit) -> Result<IssueRecord> {
        let issue = commit.issue;
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(&commit.claim_id, now_seconds()?)?;
        if record.digest != commit.expected_digest {
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
        srp.reviewer = Some(commit.evidence.reviewer.clone());
        srp.review_scope = commit.evidence.scope.join("\n");
        srp.review_revision = Some(commit.evidence.reviewed_revision.clone());
        srp.review_result = commit.result;
        srp.residual_risk = commit.evidence.residual_risks.clone();
        srp.findings = commit
            .evidence
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
        record.review = Some(commit.evidence);
        if commit.advance_reviewed && commit.result == crate::cards::ReviewResult::Pass {
            record.advance(
                LifecyclePhase::Reviewed,
                commit.actor.clone(),
                "exact scoped review passed".into(),
            )?;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: commit.actor,
            reason: if commit.advance_reviewed {
                "atomically record exact review evidence and reviewed phase"
            } else {
                "atomically record assigned review evidence and SRP projection"
            }
            .into(),
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
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        let actor = assignment.assigned_by.clone();
        let srp = match &mut cards.get_mut(&CardKind::Srp).expect("SRP").content {
            CardContent::Srp(values) => values,
            _ => unreachable!("SRP"),
        };
        srp.review_scope = assignment.scope.join("\n");
        record.review_assignment = Some(assignment);
        record.review = None;
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason: "assign bounded exact-revision review".into(),
            operation: "assign_review".into(),
        });
        hydrate_projections(&mut record, &cards)?;
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
    #[serde(default)]
    pub prepared_cards: Option<BTreeMap<CardKind, CardValues>>,
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
    record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
        .validate(&request.claim_id, now_seconds()?)?;
    let mut cards = store.load_cards(request.issue)?;
    verify_card_projections(store, &record, &cards)?;
    let design_digest = digest(&fs::read(store.root.join(&record.design_path))?);
    let diagram_digest = digest(&fs::read(store.root.join(&record.diagram_path))?);
    let initial_approval = record.phase == LifecyclePhase::Initialized
        && matches!(
            record.design_review,
            DesignReview::Pending | DesignReview::ChangesRequired { .. }
        );
    let initialized_reapproval = record.phase == LifecyclePhase::Initialized
        && matches!(record.design_review, DesignReview::Approved { .. })
        && [CardKind::Spp, CardKind::Vpp]
            .iter()
            .any(|kind| match &cards[kind].content {
                CardContent::Spp(values) => {
                    values.design_digest != design_digest || values.diagram_digest != diagram_digest
                }
                CardContent::Vpp(values) => {
                    values.design_digest != design_digest || values.diagram_digest != diagram_digest
                }
                _ => unreachable!("design-bearing card"),
            });
    let lifecycle_reapproval = matches!(
        record.phase,
        LifecyclePhase::Bound | LifecyclePhase::Implemented
    );
    if !initial_approval && !initialized_reapproval && !lifecycle_reapproval {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "design approval requires pending initialized review, stale initialized approved inputs, or bound/implemented reapproval",
        ));
    }
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
        reason: if initialized_reapproval {
            "reapprove stale initialized issue design"
        } else if lifecycle_reapproval {
            "reapprove changed issue design"
        } else {
            "approve completed issue design"
        }
        .into(),
        operation: "approve_design".into(),
    });
    hydrate_projections(&mut record, &cards)?;
    record.digest = record_digest(&record)?;
    store.commit(request.issue, &record, &cards, false)?;
    Ok(record)
}

pub(crate) fn bootstrap_issue(store: &Store, request: BootstrapRequest) -> Result<IssueRecord> {
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
    let cards = if let Some(mut cards) = request.prepared_cards {
        for values in cards.values_mut() {
            values.identity.generation = 0;
        }
        validate_cross_card(
            &cards,
            &request.design_path,
            &design_digest,
            &request.diagram_path,
            &diagram_digest,
        )?;
        cards
    } else {
        initial_cards(
            request.issue,
            &request.repository,
            &request.design_path,
            &design_digest,
            &request.diagram_path,
            &diagram_digest,
            request.initial,
        )?
    };
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
    let identity_update = matches!(
        request.operation,
        SemanticOperation::UpdateIdentityVersion { .. }
    );
    if identity_update {
        if !matches!(
            record.phase,
            LifecyclePhase::Initialized
                | LifecyclePhase::Ready
                | LifecyclePhase::Bound
                | LifecyclePhase::Implemented
        ) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "identity version repair requires an active pre-review issue",
            ));
        }
    } else {
        authorize_card_operation(record.phase, request.card, &request.operation)?;
    }
    if matches!(
        request.operation,
        SemanticOperation::CorrectReviewPromptsAfterRecovery { .. }
    ) {
        let recovered = record.transitions.last().is_some_and(|transition| {
            transition.to == LifecyclePhase::Implemented
                && matches!(
                    transition.from,
                    LifecyclePhase::Reviewed
                        | LifecyclePhase::Published
                        | LifecyclePhase::MergeReady
                )
        });
        if !recovered
            || record.review_assignment.is_some()
            || record.review.is_some()
            || record.publication.is_some()
            || record.readiness.is_some()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "post-recovery review prompt correction requires cleared review and publication truth",
            ));
        }
    }
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
    if identity_update {
        for values in cards.values_mut() {
            apply(values, &request.operation)?;
        }
    } else if let SemanticOperation::ReplaceAcceptancePlan {
        acceptance_criteria,
        steps,
        validation_lanes,
    } = &request.operation
    {
        crate::cards::replace_acceptance_plan(
            &mut cards,
            acceptance_criteria,
            steps,
            validation_lanes,
        )?;
    } else {
        let values = cards
            .get_mut(&request.card)
            .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "card projection missing"))?;
        if let Some(next) = apply(values, &request.operation)? {
            validate_phase_guard(store, &record, &cards, next)?;
            record.advance(next, request.actor.clone(), request.reason.clone())?;
        }
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
        (CardContent::Srp(value), crate::cards::TextField::ReviewScope) => {
            Ok(value.review_scope.clone())
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
    verify_card_projections(store, record, cards)?;
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

fn verify_card_projections(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    verify_authority_card_inputs(store, record, cards)?;
    for (kind, values) in cards {
        let rendered = render(values)?;
        let projection = record.cards.get(kind).ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("missing {kind} projection"),
            )
        })?;
        if projection.values_digest != rendered.values_digest
            || projection.rendered_digest != rendered.rendered_digest
            || projection.ast_digest != rendered.ast_digest
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{kind} digest drift"),
            ));
        }
    }
    Ok(())
}

fn verify_authority_card_inputs(
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
        let tracked = fs::read(
            store
                .issue_dir(record.issue)
                .join("cards")
                .join(format!("{kind}.md")),
        )?;
        if digest(&tracked) != rendered.rendered_digest {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{kind} rendered Markdown drift"),
            ));
        }
    }
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
    } else if let Some(claim) = record.claim.as_ref() {
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
    let recordless_recovery = record.generation == 1
        && record.audit.len() == 1
        && record.transitions.len() == 1
        && record.audit[0].operation == "recover_recordless_terminal"
        && record.audit[0].generation == 1
        && record.audit[0].actor == record.transitions[0].actor
        && record.audit[0].reason == record.transitions[0].reason;
    for (index, event) in record.transitions.iter().enumerate() {
        let direct_recordless_closeout = recordless_recovery
            && record.transitions.len() == 1
            && event.from == LifecyclePhase::Initialized
            && event.to == LifecyclePhase::ClosedOut;
        let legacy_terminal_transition = matches!(
            (event.from, event.to),
            (LifecyclePhase::Published, LifecyclePhase::MergeReady)
                | (LifecyclePhase::MergeReady, LifecyclePhase::Published)
                | (LifecyclePhase::MergeReady, LifecyclePhase::Implemented)
                | (LifecyclePhase::MergeReady, LifecyclePhase::Merged)
                | (LifecyclePhase::Merged, LifecyclePhase::ClosedOut)
                | (LifecyclePhase::Reviewed, LifecyclePhase::ClosedOut)
        );
        if event.sequence != index as u64 + 1
            || event.from != phase
            || (!event.from.allows(event.to)
                && !direct_recordless_closeout
                && !legacy_terminal_transition)
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
    if next == LifecyclePhase::Published
        && (!review_current
            || record.review.as_ref().is_none_or(|review| {
                crate::git::substantive_revision(store.root(), &review.scope).map_or(
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
            CardKind::Sip,
            SemanticOperation::ReplaceOperatorConstraints { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Stp,
            SemanticOperation::ReplaceAcceptanceCriteria { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Sip | CardKind::Stp | CardKind::Spp | CardKind::Srp,
            SemanticOperation::ReplacePlanningCollection { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Spp,
            SemanticOperation::ReplacePlanSteps { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Spp,
            SemanticOperation::ReplaceAcceptancePlan { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Sip | CardKind::Stp | CardKind::Spp,
            SemanticOperation::Replan { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Srp,
            SemanticOperation::Replan {
                field: crate::cards::TextField::ReviewScope,
                ..
            },
        ) | (
            LifecyclePhase::Bound | LifecyclePhase::Implemented,
            CardKind::Spp,
            SemanticOperation::UpdatePlanStep { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Spp,
            SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::AffectedAreas,
                ..
            } | SemanticOperation::ReplacePlanSteps { .. }
                | SemanticOperation::ReplacePlanningCollection {
                    field: crate::cards::PlanningCollectionField::Invariants
                        | crate::cards::PlanningCollectionField::StopConditions,
                    ..
                },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Sip,
            SemanticOperation::ReplaceOperatorConstraints { .. }
                | SemanticOperation::ReplacePlanningCollection {
                    field: crate::cards::PlanningCollectionField::AuthorityBoundary,
                    ..
                },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Stp,
            SemanticOperation::ReplaceAcceptanceCriteria { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Srp,
            SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::ReviewPrompts,
                ..
            },
        ) | (
            LifecyclePhase::Bound | LifecyclePhase::Implemented,
            CardKind::Vpp,
            SemanticOperation::ReplaceValidationLanes { .. },
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
            SemanticOperation::CorrectReviewPromptsAfterRecovery { .. }
                | SemanticOperation::RecordReview { .. }
                | SemanticOperation::RecordFinding { .. }
                | SemanticOperation::DisposeFinding { .. }
                | SemanticOperation::AppendReference { .. }
                | SemanticOperation::AdvanceStatus { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Sor,
            SemanticOperation::RecordExecution { .. }
                | SemanticOperation::ReplaceExecution { .. }
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

fn terminal_receipt_digest(receipt: &TerminalReceipt) -> Result<String> {
    let mut value = receipt.clone();
    value.digest.clear();
    Ok(digest(&serde_json::to_vec(&value)?))
}

fn validate_terminal_receipt(receipt: &TerminalReceipt) -> Result<()> {
    if receipt.schema != "csdlc.terminal_receipt.v1"
        || receipt.issue == 0
        || receipt.issue != receipt.record.issue
        || receipt.repository != receipt.record.repository
        || receipt.initialization_digest != receipt.record.initialization_digest
        || receipt.receipt_ref != format!("csdlc-v2/closeout/{}.json", receipt.issue)
        || receipt.record.phase != LifecyclePhase::ClosedOut
        || receipt.record.claim.is_some()
        || receipt.record.terminal.is_none()
        || receipt.cards.len() != 6
        || receipt.authored_artifacts.len() != 2
        || receipt.digest != terminal_receipt_digest(receipt)?
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal receipt identity, phase, or digest is invalid",
        ));
    }
    if !crate::pvf::clean_relative(Path::new(&receipt.record.design_path))
        || !crate::pvf::clean_relative(Path::new(&receipt.record.diagram_path))
        || receipt
            .authored_artifacts
            .keys()
            .any(|path| !crate::pvf::clean_relative(Path::new(path)))
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal receipt authored paths must be clean repository-relative paths",
        ));
    }
    verify_record(&receipt.record)?;
    let design = receipt
        .authored_artifacts
        .get(&receipt.record.design_path)
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "receipt design missing"))?;
    let diagram = receipt
        .authored_artifacts
        .get(&receipt.record.diagram_path)
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "receipt diagram missing"))?;
    for (kind, values) in &receipt.cards {
        if values.kind() != *kind
            || values.identity.issue != receipt.issue
            || values.identity.repository != receipt.repository
            || values.identity.generation != receipt.record.generation
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "terminal receipt card identity is invalid",
            ));
        }
        let rendered = render(values)?;
        let projection = receipt.record.cards.get(kind).ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "terminal receipt projection missing",
            )
        })?;
        if projection.values_digest != rendered.values_digest
            || projection.rendered_digest != rendered.rendered_digest
            || projection.ast_digest != rendered.ast_digest
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "terminal receipt card digest drift",
            ));
        }
    }
    validate_cross_card(
        &receipt.cards,
        &receipt.record.design_path,
        &digest(design.as_bytes()),
        &receipt.record.diagram_path,
        &digest(diagram.as_bytes()),
    )?;
    if !matches!(
        &receipt.record.design_review,
        DesignReview::Approved { revision, .. } if revision == &digest(design.as_bytes())
    ) {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal receipt design review is stale",
        ));
    }
    Ok(())
}

fn push_legacy_terminal_transition(
    record: &mut IssueRecord,
    next: LifecyclePhase,
    actor: &str,
    reason: &str,
) {
    let from = record.phase;
    record.phase = next;
    record.transitions.push(TransitionEvent {
        sequence: record.transitions.len() as u64 + 1,
        from,
        to: next,
        actor: actor.into(),
        reason: reason.into(),
    });
}

fn terminal_matches_derived(
    record: &IssueRecord,
    envelope: &crate::finish::DerivedTerminalEnvelope,
) -> bool {
    if record.phase != LifecyclePhase::ClosedOut
        || record.claim.is_some()
        || envelope.issue != record.issue
        || envelope.repository != record.repository
        || envelope.initialization_digest != record.initialization_digest
    {
        return false;
    }
    let Some(terminal) = record.terminal.as_ref() else {
        return false;
    };
    let disposition = match envelope.disposition {
        crate::finish::FinishDisposition::Merged => crate::readiness::TerminalDisposition::Merged,
        crate::finish::FinishDisposition::ClosedUnmerged => {
            crate::readiness::TerminalDisposition::ClosedUnmerged
        }
        crate::finish::FinishDisposition::ClosedNoPr => {
            crate::readiness::TerminalDisposition::ClosedNoPr
        }
    };
    terminal.disposition == disposition
        && terminal.pull_request == envelope.pull_request
        && terminal.observed_sha == envelope.head_sha
}

fn verify_canonical_projection_bytes(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    let issue_dir = PathBuf::from(".csdlc/issues").join(record.issue.to_string());
    let mut index = serde_json::to_vec_pretty(record)?;
    index.push(b'\n');
    if read_regular_projection(&store.root, &issue_dir.join("index.json"))? != index {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "authority target index projection is not canonical",
        ));
    }
    let mut audit = Vec::new();
    for event in &record.audit {
        serde_json::to_writer(&mut audit, event)?;
        audit.push(b'\n');
    }
    if read_regular_projection(&store.root, &issue_dir.join("audit.jsonl"))? != audit {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "authority target audit projection is not canonical",
        ));
    }
    for (kind, values) in cards {
        let mut encoded = serde_json::to_vec_pretty(values)?;
        encoded.push(b'\n');
        let rendered = render(values)?;
        if read_regular_projection(
            &store.root,
            &issue_dir.join(format!("cards/{kind}.values.json")),
        )? != encoded
            || read_regular_projection(&store.root, &issue_dir.join(format!("cards/{kind}.md")))?
                != rendered.markdown.as_bytes()
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "authority target card projection is not canonical",
            ));
        }
    }
    Ok(())
}

pub(crate) fn read_regular_projection(root: &Path, relative: &Path) -> Result<Vec<u8>> {
    let path = root.join(relative);
    let metadata = canonical_path_metadata_beneath(root, relative)?.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!("authority projection is absent: {}", path.display()),
        )
    })?;
    if !metadata.is_file() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "authority projection is not a regular file: {}",
                path.display()
            ),
        ));
    }
    Ok(fs::read(path)?)
}

fn read_regular_authored_artifact(root: &Path, relative: &Path) -> Result<Option<Vec<u8>>> {
    if !crate::pvf::clean_relative(relative) {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal authored artifact path must be clean and repository-relative",
        ));
    }
    let path = root.join(relative);
    let Some(metadata) = canonical_path_metadata_beneath(root, relative)? else {
        return Ok(None);
    };
    if !metadata.is_file() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!(
                "transport target authored path is not a regular file: {}",
                path.display()
            ),
        ));
    }
    Ok(Some(fs::read(path)?))
}

fn canonical_path_metadata_beneath(root: &Path, relative: &Path) -> Result<Option<fs::Metadata>> {
    if !crate::pvf::clean_relative(relative) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "canonical path is not clean and root-relative: {}",
                relative.display()
            ),
        ));
    }
    let root_metadata = fs::symlink_metadata(root)?;
    if root_metadata.file_type().is_symlink() || !root_metadata.is_dir() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "canonical root is not a regular directory: {}",
                root.display()
            ),
        ));
    }
    let mut current = root.to_path_buf();
    let components: Vec<_> = relative.components().collect();
    for (index, component) in components.iter().enumerate() {
        match component {
            std::path::Component::Normal(part) => current.push(part),
            _ => unreachable!("clean_relative accepted a non-normal component"),
        }
        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        if metadata.file_type().is_symlink() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!("canonical path contains a symlink: {}", current.display()),
            ));
        }
        if index + 1 < components.len() && !metadata.is_dir() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "canonical path ancestor is not a directory: {}",
                    current.display()
                ),
            ));
        }
        if index + 1 == components.len() {
            return Ok(Some(metadata));
        }
    }
    Ok(None)
}

pub(crate) fn require_canonical_parent_beneath(root: &Path, relative: &Path) -> Result<()> {
    if !crate::pvf::clean_relative(relative) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "terminal write path is not clean and root-relative: {}",
                relative.display()
            ),
        ));
    }
    let parent = relative.parent().ok_or_else(|| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            "terminal write path has no parent",
        )
    })?;
    if parent.as_os_str().is_empty() {
        let metadata = fs::symlink_metadata(root)?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!("terminal write root is unsafe: {}", root.display()),
            ));
        }
        return Ok(());
    }
    if let Some(metadata) = canonical_path_metadata_beneath(root, parent)? {
        if !metadata.is_dir() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "terminal write parent is not a directory: {}",
                    root.join(parent).display()
                ),
            ));
        }
    }
    Ok(())
}

pub(crate) fn require_regular_or_absent_beneath(root: &Path, relative: &Path) -> Result<()> {
    if let Some(metadata) = canonical_path_metadata_beneath(root, relative)? {
        if !metadata.is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "terminal write target is not a regular file: {}",
                    root.join(relative).display()
                ),
            ));
        }
    }
    Ok(())
}

fn replace_regular_authored_artifact(
    root: &Path,
    relative: &Path,
    bytes: &[u8],
    temporary_extension: &str,
) -> Result<()> {
    require_canonical_parent_beneath(root, relative)?;
    let destination = root.join(relative);
    let parent = destination.parent().ok_or_else(|| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            "terminal write target has no parent",
        )
    })?;
    fs::create_dir_all(parent)?;
    require_canonical_parent_beneath(root, relative)?;
    require_regular_or_absent_beneath(root, relative)?;

    let temporary_relative = relative.with_extension(temporary_extension);
    require_canonical_parent_beneath(root, &temporary_relative)?;
    if canonical_path_metadata_beneath(root, &temporary_relative)?.is_some() {
        require_regular_or_absent_beneath(root, &temporary_relative)?;
        fs::remove_file(root.join(&temporary_relative))?;
    }
    let temporary = root.join(&temporary_relative);
    let mut file = File::create(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;

    require_canonical_parent_beneath(root, relative)?;
    require_regular_or_absent_beneath(root, relative)?;
    require_regular_or_absent_beneath(root, &temporary_relative)?;
    fs::rename(&temporary, &destination)?;
    sync_dir(parent)?;
    Ok(())
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

#[cfg(test)]
mod edit_authorization_tests {
    use super::*;

    fn replacement_steps() -> Vec<crate::cards::PlanStep> {
        vec![crate::cards::PlanStep {
            id: "review-fix".into(),
            action: "correct bounded review finding".into(),
            acceptance_ids: vec!["AC-1".into()],
            status: crate::cards::StepStatus::Pending,
        }]
    }

    #[test]
    fn implemented_spp_review_remediation_authorizes_only_bounded_replacements() {
        for operation in [
            SemanticOperation::ReplacePlanSteps {
                steps: replacement_steps(),
            },
            SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::Invariants,
                values: vec!["invariant".into()],
            },
            SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::StopConditions,
                values: vec!["stop".into()],
            },
        ] {
            authorize_card_operation(LifecyclePhase::Implemented, CardKind::Spp, &operation)
                .expect("implemented bounded SPP remediation");
        }

        let error = authorize_card_operation(
            LifecyclePhase::Implemented,
            CardKind::Spp,
            &SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::Risks,
                values: vec!["risk".into()],
            },
        )
        .expect_err("unbounded SPP collection remains rejected");
        assert_eq!(error.code, ErrorCode::InvalidTransition);
    }

    #[test]
    fn post_review_spp_replacements_remain_rejected() {
        for phase in [
            LifecyclePhase::Reviewed,
            LifecyclePhase::Published,
            LifecyclePhase::MergeReady,
        ] {
            for operation in [
                SemanticOperation::ReplacePlanSteps {
                    steps: replacement_steps(),
                },
                SemanticOperation::ReplacePlanningCollection {
                    field: crate::cards::PlanningCollectionField::Invariants,
                    values: vec!["late invariant".into()],
                },
                SemanticOperation::ReplacePlanningCollection {
                    field: crate::cards::PlanningCollectionField::StopConditions,
                    values: vec!["late stop".into()],
                },
            ] {
                let error = authorize_card_operation(phase, CardKind::Spp, &operation)
                    .expect_err("late SPP replacement remains rejected");
                assert_eq!(error.code, ErrorCode::InvalidTransition);
            }
        }
    }
}
