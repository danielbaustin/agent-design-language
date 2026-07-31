use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use fs2::FileExt;
use markdown::{to_mdast, ParseOptions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cards::{
    apply, digest, initial_cards, render, terminal_validation_passed, validate_cross_card,
    validate_identity_version, validate_result, CardContent, CardKind, CardValues,
    InitialCardInput, SemanticOperation, StepStatus, ValidationResult,
};
use crate::error::{ErrorCode, Result, V2Error};
use crate::model::{
    AuditEvent, CardProjection, Claim, DesignReview, IssueRecord, LifecyclePhase,
    PublicationEvidence, ReadinessEvidence, ReconcileTerminalRequest, ReviewAssignment,
    ReviewEvidence, TerminalDesignRepairRequest, TerminalEvidence, TerminalPlanStepRepairRequest,
    TerminalReceipt, TerminalSorArtifactRepairRequest, TerminalSorValidationRepairRequest,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TerminalTransactionJournal {
    schema: String,
    issue: u64,
    stage: String,
    original_record_digest: String,
    target_record_digest: String,
    original_receipt: Option<Vec<u8>>,
    target_receipt: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RepairIdentityRequest {
    pub authority_issue: u64,
    pub target_issue: u64,
    pub expected_authority_generation: u64,
    pub expected_authority_digest: String,
    pub expected_target_generation: u64,
    pub expected_target_digest: String,
    pub claim_id: String,
    pub actor: String,
    pub operation: SemanticOperation,
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

    fn terminal_transaction_path(&self, issue: u64) -> Result<PathBuf> {
        let common = crate::git::run(
            &self.root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout;
        Ok(PathBuf::from(common)
            .join("csdlc-v2/terminal-transactions")
            .join(format!("{issue}.json")))
    }

    fn terminal_repair_lock(&self) -> Result<File> {
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
            .open(dir.join("terminal-repairs.lock"))?;
        file.lock_exclusive()?;
        Ok(file)
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

    pub fn repair_identity(&self, request: RepairIdentityRequest) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "identity repair requires a distinct authority issue",
            ));
        }
        let version = match &request.operation {
            SemanticOperation::UpdateIdentityVersion { version } => version,
            _ => {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    "identity repair requires update_identity_version",
                ))
            }
        };
        validate_identity_version(version)?;
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;
        let authority = self.load_record(request.authority_issue)?;
        let mut target = self.load_record(request.target_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "authority issue digest is stale",
            ));
        }
        if target.generation != request.expected_target_generation
            || target.digest != request.expected_target_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "target issue digest is stale",
            ));
        }
        authority
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "repair authority claim missing"))?
            .validate(&request.claim_id, now_seconds()?)?;
        let mut target_cards = self.load_cards(request.target_issue)?;
        let original_target = target.clone();
        let original_target_cards = target_cards.clone();
        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_receipt = self.read_terminal_receipt_snapshot(&receipt_path)?;
        for values in target_cards.values_mut() {
            apply(values, &request.operation)?;
        }
        if target_cards
            .values()
            .any(|values| values.identity.version != *version)
        {
            return Err(V2Error::new(
                ErrorCode::CardInvalid,
                "identity repair did not update all cards",
            ));
        }
        let target_design_digest = digest(&fs::read(self.root.join(&target.design_path))?);
        let target_diagram_digest = digest(&fs::read(self.root.join(&target.diagram_path))?);
        validate_cross_card(
            &target_cards,
            &target.design_path,
            &target_design_digest,
            &target.diagram_path,
            &target_diagram_digest,
        )?;
        target.generation += 1;
        for values in target_cards.values_mut() {
            values.identity.generation = target.generation;
        }
        if let Some(claim) = target.claim.as_mut() {
            claim.generation = target.generation;
        }
        target.audit.push(AuditEvent {
            sequence: target.audit.len() as u64 + 1,
            generation: target.generation,
            actor: request.actor.clone(),
            reason: format!(
                "typed identity repair authorized by issue {}",
                request.authority_issue
            ),
            operation: serde_json::to_string(&request.operation)?,
        });
        hydrate_projections(&mut target, &target_cards)?;
        target.digest = record_digest(&target)?;
        self.commit(request.target_issue, &target, &target_cards, false)?;
        if let Err(error) = self.refresh_terminal_receipt(&target, &target_cards) {
            self.commit(
                request.target_issue,
                &original_target,
                &original_target_cards,
                false,
            )?;
            if let Some(bytes) = original_receipt {
                self.restore_terminal_receipt(&receipt_path, &bytes)?;
            }
            return Err(error);
        }
        Ok(target)
    }

    pub fn repair_terminal_design(
        &self,
        request: TerminalDesignRepairRequest,
    ) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue
            || request.actor.trim().is_empty()
            || request.reviewer.trim().is_empty()
            || request.expected_authority_digest.trim().is_empty()
            || request.expected_target_digest.trim().is_empty()
            || request.expected_receipt_digest.trim().is_empty()
            || request.expected_design_digest.trim().is_empty()
            || request.expected_diagram_digest.trim().is_empty()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal design repair identity or authority is incomplete",
            ));
        }
        for path in [&request.source_design_path, &request.source_diagram_path] {
            if !crate::pvf::clean_relative(Path::new(path)) {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    "terminal design repair source path must be repository-relative",
                ));
            }
        }
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;
        let authority = self.load_record(request.authority_issue)?;
        let mut target = self.load_record(request.target_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair authority record is stale",
            ));
        }
        if target.generation != request.expected_target_generation
            || target.digest != request.expected_target_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair target record is stale",
            ));
        }
        if target.phase != LifecyclePhase::ClosedOut || target.claim.is_some() {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal design repair requires a closed-out target without a claim",
            ));
        }
        authority
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "repair authority claim missing"))?
            .validate(&request.authority_claim_id, now_seconds()?)?;
        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_receipt_bytes = fs::read(&receipt_path)?;
        let original_receipt: TerminalReceipt = serde_json::from_slice(&original_receipt_bytes)?;
        validate_terminal_receipt(&original_receipt)?;
        if original_receipt.digest != request.expected_receipt_digest
            || original_receipt.record.digest != target.digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal receipt is stale",
            ));
        }
        let design = fs::read(self.root.join(&request.source_design_path))?;
        let diagram = fs::read(self.root.join(&request.source_diagram_path))?;
        let design_digest = digest(&design);
        let diagram_digest = digest(&diagram);
        if design_digest != request.expected_design_digest
            || diagram_digest != request.expected_diagram_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair artifact hash does not match request",
            ));
        }
        let design_text = String::from_utf8(design).map_err(|_| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "repair design must be UTF-8 Markdown",
            )
        })?;
        let diagram_text = String::from_utf8(diagram).map_err(|_| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "repair diagram must be UTF-8 Mermaid",
            )
        })?;
        if design_text.trim().is_empty() || diagram_text.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "repair artifacts must not be empty",
            ));
        }
        if !valid_mermaid_diagram(&diagram_text) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "repair diagram is not recognized Mermaid source",
            ));
        }
        to_mdast(&design_text, &ParseOptions::gfm()).map_err(|error| {
            V2Error::new(
                ErrorCode::InvalidInput,
                format!("repair design Markdown failed AST validation: {error}"),
            )
        })?;
        let mut cards = self.load_cards(request.target_issue)?;
        verify_cards(self, &target, &cards)?;
        for kind in [CardKind::Spp, CardKind::Vpp] {
            match &mut cards.get_mut(&kind).expect("design-bearing card").content {
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
        target.design_review = DesignReview::Approved {
            reviewer: request.reviewer.clone(),
            revision: design_digest.clone(),
        };
        target.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = target.generation;
        }
        target.audit.push(AuditEvent {
            sequence: target.audit.len() as u64 + 1,
            generation: target.generation,
            actor: request.actor.clone(),
            reason: format!(
                "typed terminal design repair authorized by issue {}",
                request.authority_issue
            ),
            operation: "repair_terminal_design".into(),
        });
        hydrate_projections(&mut target, &cards)?;
        target.digest = record_digest(&target)?;
        let authored_artifacts = BTreeMap::from([
            (target.design_path.clone(), design_text),
            (target.diagram_path.clone(), diagram_text),
        ]);
        let mut repaired_receipt = original_receipt.clone();
        repaired_receipt.record = target.clone();
        repaired_receipt.cards = cards.clone();
        repaired_receipt.authored_artifacts = authored_artifacts.clone();
        repaired_receipt.digest.clear();
        repaired_receipt.digest = terminal_receipt_digest(&repaired_receipt)?;
        validate_terminal_receipt(&repaired_receipt)?;
        let target_receipt = serde_json::to_vec_pretty(&repaired_receipt)?;
        let mut journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            issue: request.target_issue,
            stage: "prepared_terminal_design_repair".into(),
            original_record_digest: original_receipt.record.digest.clone(),
            target_record_digest: target.digest.clone(),
            original_receipt: Some(original_receipt_bytes),
            target_receipt,
        };
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_journal") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        if let Err(error) = self.commit_with_authored(
            request.target_issue,
            &target,
            &cards,
            false,
            Some(&authored_artifacts),
        ) {
            let _ = self.recover_terminal_transaction(request.target_issue);
            return Err(error);
        }
        journal.stage = "projection_committed_terminal_design_repair".into();
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_projection") {
            let _ = self.recover_terminal_transaction(request.target_issue);
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))?;
        journal.stage = "receipt_committed_terminal_design_repair".into();
        self.write_terminal_transaction_journal(&journal)?;
        self.remove_terminal_transaction_journal(request.target_issue)?;
        Ok(target)
    }

    pub fn repair_terminal_plan_step(
        &self,
        request: TerminalPlanStepRepairRequest,
    ) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue
            || request.actor.trim().is_empty()
            || request.step_id.trim().is_empty()
            || request.expected_authority_digest.trim().is_empty()
            || request.expected_target_digest.trim().is_empty()
            || request.expected_receipt_digest.trim().is_empty()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal plan repair identity or authority is incomplete",
            ));
        }
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;
        let authority = self.load_record(request.authority_issue)?;
        let mut target = self.load_record(request.target_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair authority record is stale",
            ));
        }
        if target.generation != request.expected_target_generation
            || target.digest != request.expected_target_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair target record is stale",
            ));
        }
        if target.phase != LifecyclePhase::ClosedOut || target.claim.is_some() {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal plan repair requires a closed-out target without a claim",
            ));
        }
        authority
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "repair authority claim missing"))?
            .validate(&request.authority_claim_id, now_seconds()?)?;
        if !authority
            .claim
            .as_ref()
            .is_some_and(|claim| claim_covers_issue(claim, request.target_issue))
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "repair authority claim does not cover the target issue",
            ));
        }

        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_receipt_bytes = fs::read(&receipt_path)?;
        let original_receipt: TerminalReceipt = serde_json::from_slice(&original_receipt_bytes)?;
        validate_terminal_receipt(&original_receipt)?;
        if original_receipt.digest != request.expected_receipt_digest
            || original_receipt.record.digest != target.digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal receipt is stale",
            ));
        }
        let original_cards = self.load_cards(request.target_issue)?;
        verify_cards(self, &target, &original_cards)?;
        let mut cards = original_cards.clone();
        complete_terminal_plan_step(&mut cards, &request.step_id)?;

        let original_target = target.clone();
        target.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = target.generation;
        }
        target.audit.push(AuditEvent {
            sequence: target.audit.len() as u64 + 1,
            generation: target.generation,
            actor: request.actor.clone(),
            reason: format!(
                "typed terminal plan repair authorized by issue {}",
                request.authority_issue
            ),
            operation: format!("repair_terminal_plan_step:{}", request.step_id),
        });
        hydrate_projections(&mut target, &cards)?;
        target.digest = record_digest(&target)?;

        let mut repaired_receipt = original_receipt.clone();
        repaired_receipt.record = target.clone();
        repaired_receipt.cards = cards.clone();
        repaired_receipt.digest.clear();
        repaired_receipt.digest = terminal_receipt_digest(&repaired_receipt)?;
        validate_terminal_receipt(&repaired_receipt)?;
        let target_receipt = serde_json::to_vec_pretty(&repaired_receipt)?;
        let mut journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            issue: request.target_issue,
            stage: "prepared_terminal_plan_repair".into(),
            original_record_digest: original_receipt.record.digest.clone(),
            target_record_digest: target.digest.clone(),
            original_receipt: Some(original_receipt_bytes.clone()),
            target_receipt,
        };
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_journal") {
            self.remove_terminal_transaction_journal(request.target_issue)?;
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        if let Err(error) = self.commit(request.target_issue, &target, &cards, false) {
            let _ = self.remove_terminal_transaction_journal(request.target_issue);
            return Err(error);
        }
        journal.stage = "projection_committed_terminal_plan_repair".into();
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_projection") {
            self.rollback_terminal_repair(
                request.target_issue,
                &original_target,
                &original_cards,
                &receipt_path,
                &original_receipt_bytes,
            )?;
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        if let Err(error) = self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))
        {
            self.rollback_terminal_repair(
                request.target_issue,
                &original_target,
                &original_cards,
                &receipt_path,
                &original_receipt_bytes,
            )?;
            return Err(error);
        }
        journal.stage = "receipt_committed_terminal_plan_repair".into();
        self.write_terminal_transaction_journal(&journal)?;
        self.remove_terminal_transaction_journal(request.target_issue)?;
        Ok(target)
    }

    pub fn repair_terminal_sor_artifact(
        &self,
        request: TerminalSorArtifactRepairRequest,
    ) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue
            || request.actor.trim().is_empty()
            || request.expected_authority_digest.trim().is_empty()
            || request.expected_target_digest.trim().is_empty()
            || request.expected_receipt_digest.trim().is_empty()
            || request.expected_artifact_digest.trim().is_empty()
            || request.stale_ref == request.retained_ref
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal SOR artifact repair identity or authority is incomplete",
            ));
        }
        for path in [&request.stale_ref, &request.retained_ref] {
            if !crate::pvf::clean_relative(Path::new(path)) {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    "terminal SOR artifact repair paths must be repository-relative",
                ));
            }
        }

        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;
        let authority = self.load_record(request.authority_issue)?;
        let target = self.load_record(request.target_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair authority record is stale",
            ));
        }
        if target.generation != request.expected_target_generation
            || target.digest != request.expected_target_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair target record is stale",
            ));
        }
        if target.phase != LifecyclePhase::ClosedOut || target.claim.is_some() {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal SOR artifact repair requires a closed-out target without a claim",
            ));
        }
        authority
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "repair authority claim missing"))?
            .validate(&request.authority_claim_id, now_seconds()?)?;
        if !authority
            .claim
            .as_ref()
            .is_some_and(|claim| claim_covers_issue(claim, request.target_issue))
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "repair authority claim does not cover the target issue",
            ));
        }

        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_receipt_bytes = fs::read(&receipt_path)?;
        let original_receipt: TerminalReceipt = serde_json::from_slice(&original_receipt_bytes)?;
        validate_terminal_receipt(&original_receipt)?;
        if original_receipt.digest != request.expected_receipt_digest
            || original_receipt.record.digest != target.digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal receipt is stale",
            ));
        }
        if request.retained_ref != target.design_path && request.retained_ref != target.diagram_path
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "replacement is not a canonical retained authored artifact",
            ));
        }
        let retained_bytes = original_receipt
            .authored_artifacts
            .get(&request.retained_ref)
            .ok_or_else(|| {
                V2Error::new(
                    ErrorCode::InvalidInput,
                    "replacement artifact is absent from the terminal receipt",
                )
            })?;
        if digest(retained_bytes.as_bytes()) != request.expected_artifact_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "replacement artifact bytes differ from the request",
            ));
        }

        let original_cards = self.load_cards(request.target_issue)?;
        verify_cards(self, &target, &original_cards)?;
        let mut cards = original_cards.clone();
        replace_terminal_sor_artifact(&mut cards, &request.stale_ref, &request.retained_ref)?;

        self.commit_terminal_card_repair(
            target,
            cards,
            original_cards,
            original_receipt,
            original_receipt_bytes,
            receipt_path,
            &request.actor,
            format!(
                "typed terminal SOR artifact repair authorized by issue {}",
                request.authority_issue
            ),
            format!(
                "repair_terminal_sor_artifact:{}->{}",
                request.stale_ref, request.retained_ref
            ),
            "terminal_sor_artifact_repair",
            request.fail_after_stage.as_deref(),
        )
    }

    pub fn repair_terminal_sor_validation(
        &self,
        request: TerminalSorValidationRepairRequest,
    ) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue
            || request.actor.trim().is_empty()
            || request.expected_authority_digest.trim().is_empty()
            || request.expected_target_digest.trim().is_empty()
            || request.expected_receipt_digest.trim().is_empty()
            || request.expected_result == request.replacement_result
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal SOR validation repair identity or authority is incomplete",
            ));
        }
        validate_result(&request.replacement_result)?;
        validate_portable_validation_result(&request.replacement_result)?;

        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;
        let authority = self.load_record(request.authority_issue)?;
        let target = self.load_record(request.target_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair authority record is stale",
            ));
        }
        if target.generation != request.expected_target_generation
            || target.digest != request.expected_target_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair target record is stale",
            ));
        }
        if target.phase != LifecyclePhase::ClosedOut || target.claim.is_some() {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal SOR validation repair requires a closed-out target without a claim",
            ));
        }
        let authority_claim = authority.claim.as_ref().ok_or_else(|| {
            V2Error::new(ErrorCode::MissingClaim, "repair authority claim missing")
        })?;
        authority_claim.validate(&request.authority_claim_id, now_seconds()?)?;
        if !claim_covers_issue(authority_claim, request.target_issue) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "repair authority claim does not cover the target issue",
            ));
        }

        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_receipt_bytes = fs::read(&receipt_path)?;
        let original_receipt: TerminalReceipt = serde_json::from_slice(&original_receipt_bytes)?;
        validate_terminal_receipt(&original_receipt)?;
        if original_receipt.digest != request.expected_receipt_digest
            || original_receipt.record.digest != target.digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal receipt is stale",
            ));
        }

        let original_cards = self.load_cards(request.target_issue)?;
        verify_cards(self, &target, &original_cards)?;
        let mut cards = original_cards.clone();
        replace_terminal_sor_validation(
            &mut cards,
            &request.expected_result,
            &request.replacement_result,
        )?;

        self.commit_terminal_card_repair(
            target,
            cards,
            original_cards,
            original_receipt,
            original_receipt_bytes,
            receipt_path,
            &request.actor,
            format!(
                "typed terminal SOR validation repair authorized by issue {}",
                request.authority_issue
            ),
            "repair_terminal_sor_validation".into(),
            "terminal_sor_validation_repair",
            request.fail_after_stage.as_deref(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn commit_terminal_card_repair(
        &self,
        mut target: IssueRecord,
        mut cards: BTreeMap<CardKind, CardValues>,
        original_cards: BTreeMap<CardKind, CardValues>,
        original_receipt: TerminalReceipt,
        original_receipt_bytes: Vec<u8>,
        receipt_path: PathBuf,
        actor: &str,
        reason: String,
        operation: String,
        stage_suffix: &str,
        fail_after_stage: Option<&str>,
    ) -> Result<IssueRecord> {
        let original_target = target.clone();
        target.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = target.generation;
        }
        target.audit.push(AuditEvent {
            sequence: target.audit.len() as u64 + 1,
            generation: target.generation,
            actor: actor.into(),
            reason,
            operation,
        });
        hydrate_projections(&mut target, &cards)?;
        target.digest = record_digest(&target)?;

        let mut repaired_receipt = original_receipt.clone();
        repaired_receipt.record = target.clone();
        repaired_receipt.cards = cards.clone();
        repaired_receipt.digest.clear();
        repaired_receipt.digest = terminal_receipt_digest(&repaired_receipt)?;
        validate_terminal_receipt(&repaired_receipt)?;
        let mut journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            issue: target.issue,
            stage: format!("prepared_{stage_suffix}"),
            original_record_digest: original_receipt.record.digest,
            target_record_digest: target.digest.clone(),
            original_receipt: Some(original_receipt_bytes.clone()),
            target_receipt: serde_json::to_vec_pretty(&repaired_receipt)?,
        };
        self.write_terminal_transaction_journal(&journal)?;
        if fail_after_stage == Some("after_journal") {
            self.remove_terminal_transaction_journal(target.issue)?;
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        if let Err(error) = self.commit(target.issue, &target, &cards, false) {
            let _ = self.remove_terminal_transaction_journal(target.issue);
            return Err(error);
        }
        journal.stage = format!("projection_committed_{stage_suffix}");
        self.write_terminal_transaction_journal(&journal)?;
        if fail_after_stage == Some("after_projection") {
            self.rollback_terminal_repair(
                target.issue,
                &original_target,
                &original_cards,
                &receipt_path,
                &original_receipt_bytes,
            )?;
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        if let Err(error) = self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))
        {
            self.rollback_terminal_repair(
                target.issue,
                &original_target,
                &original_cards,
                &receipt_path,
                &original_receipt_bytes,
            )?;
            return Err(error);
        }
        journal.stage = format!("receipt_committed_{stage_suffix}");
        self.write_terminal_transaction_journal(&journal)?;
        self.remove_terminal_transaction_journal(target.issue)?;
        Ok(target)
    }

    fn rollback_terminal_repair(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
        receipt_path: &Path,
        receipt: &[u8],
    ) -> Result<()> {
        self.commit(issue, record, cards, false)?;
        self.replace_receipt_bytes(receipt_path, Some(receipt))?;
        self.remove_terminal_transaction_journal(issue)
    }

    fn refresh_terminal_receipt(
        &self,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
    ) -> Result<()> {
        let path = self.terminal_receipt_path(record.issue)?;
        if !path.is_file() {
            return Ok(());
        }
        let parent = path.parent().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidInput, "terminal receipt has no parent")
        })?;
        let receipt_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(parent.join("receipts.lock"))?;
        receipt_lock.lock_exclusive()?;
        let mut receipt: TerminalReceipt = read_json(&path)?;
        if receipt.issue != record.issue
            || receipt.repository != record.repository
            || receipt.initialization_digest != record.initialization_digest
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal receipt identity differs from repair target",
            ));
        }
        receipt.record = record.clone();
        receipt.cards = cards.clone();
        receipt.digest.clear();
        receipt.digest = terminal_receipt_digest(&receipt)?;
        validate_terminal_receipt(&receipt)?;
        let temporary = path.with_extension("json.repair-tmp");
        let mut file = File::create(&temporary)?;
        file.write_all(&serde_json::to_vec_pretty(&receipt)?)?;
        file.sync_all()?;
        fs::rename(temporary, &path)?;
        sync_dir(parent)?;
        Ok(())
    }

    fn write_terminal_transaction_journal(
        &self,
        journal: &TerminalTransactionJournal,
    ) -> Result<()> {
        let path = self.terminal_transaction_path(journal.issue)?;
        let parent = path
            .parent()
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "transaction has no parent"))?;
        fs::create_dir_all(parent)?;
        let temporary = path.with_extension("json.tmp");
        write_json(&temporary, journal)?;
        fs::rename(temporary, &path)?;
        sync_dir(parent)?;
        Ok(())
    }

    fn remove_terminal_transaction_journal(&self, issue: u64) -> Result<()> {
        let path = self.terminal_transaction_path(issue)?;
        if path.exists() {
            fs::remove_file(&path)?;
            sync_dir(path.parent().expect("transaction parent"))?;
        }
        Ok(())
    }

    fn replace_receipt_bytes(&self, path: &Path, bytes: Option<&[u8]>) -> Result<()> {
        let parent = path
            .parent()
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "receipt has no parent"))?;
        fs::create_dir_all(parent)?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(parent.join("receipts.lock"))?;
        lock.lock_exclusive()?;
        for suffix in [
            "json.reconcile-tmp",
            "json.recovery-tmp",
            "json.repair-tmp",
            "json.restore-tmp",
        ] {
            let temporary = path.with_extension(suffix);
            if temporary.exists() {
                fs::remove_file(temporary)?;
            }
        }
        match bytes {
            Some(bytes) => {
                let temporary = path.with_extension("json.recovery-tmp");
                let mut file = File::create(&temporary)?;
                file.write_all(bytes)?;
                file.sync_all()?;
                fs::rename(temporary, path)?;
                sync_dir(parent)?;
            }
            None if path.exists() => {
                fs::remove_file(path)?;
                sync_dir(parent)?;
            }
            None => {}
        }
        Ok(())
    }

    fn recover_terminal_transaction(&self, issue: u64) -> Result<()> {
        let path = self.terminal_transaction_path(issue)?;
        if !path.is_file() {
            return Ok(());
        }
        let journal: TerminalTransactionJournal = read_json(&path)?;
        if journal.schema != "csdlc.terminal_transaction.v1" || journal.issue != issue {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "terminal transaction journal identity is invalid",
            ));
        }
        let current = self.load_record(issue)?;
        let receipt_path = self.terminal_receipt_path(issue)?;
        if current.digest == journal.target_record_digest {
            self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))?;
        } else if current.digest == journal.original_record_digest {
            self.replace_receipt_bytes(&receipt_path, journal.original_receipt.as_deref())?;
        } else {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal transaction journal does not match current projection",
            ));
        }
        self.remove_terminal_transaction_journal(issue)
    }

    fn maybe_interrupt_terminal_transaction(issue: u64, stage: &str) -> Result<()> {
        let issue_matches = std::env::var("CSDLC_V2_TEST_INTERRUPT_ISSUE")
            .ok()
            .is_some_and(|value| value == issue.to_string());
        let stage_matches = std::env::var("CSDLC_V2_TEST_INTERRUPT_STAGE")
            .ok()
            .is_some_and(|value| value == stage);
        if issue_matches && stage_matches {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                format!("injected terminal interruption at {stage}"),
            ));
        }
        Ok(())
    }

    fn restore_terminal_receipt(&self, path: &Path, bytes: &[u8]) -> Result<()> {
        let parent = path.parent().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidInput, "terminal receipt has no parent")
        })?;
        let receipt_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(parent.join("receipts.lock"))?;
        receipt_lock.lock_exclusive()?;
        if path.is_file() && fs::read(path)? != bytes {
            return Ok(());
        }
        let temporary = path.with_extension("json.restore-tmp");
        let mut file = File::create(&temporary)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        fs::rename(temporary, path)?;
        sync_dir(parent)?;
        Ok(())
    }

    fn read_terminal_receipt_snapshot(&self, path: &Path) -> Result<Option<Vec<u8>>> {
        if !path.is_file() {
            return Ok(None);
        }
        let parent = path.parent().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidInput, "terminal receipt has no parent")
        })?;
        let receipt_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(parent.join("receipts.lock"))?;
        receipt_lock.lock_exclusive()?;
        Ok(Some(fs::read(path)?))
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
        if !path.is_file() {
            return Ok(None);
        }
        let receipt: TerminalReceipt = read_json(&path)?;
        validate_terminal_receipt(&receipt)?;
        Ok(Some(receipt))
    }

    pub fn retain_terminal_receipt(&self, issue: u64) -> Result<TerminalReceipt> {
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let _lock = self.lock(issue)?;
        self.recover_with_terminal_lock(issue)?;
        let mut record = self.load_record(issue)?;
        let mut cards = self.load_cards(issue)?;
        let receipt_ref = format!("csdlc-v2/closeout/{issue}.json");
        let path = self.terminal_receipt_path(issue)?;
        let parent = path.parent().expect("receipt parent");
        fs::create_dir_all(parent)?;
        let receipt_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(parent.join("receipts.lock"))?;
        receipt_lock.lock_exclusive()?;
        if path.is_file() {
            let existing: TerminalReceipt = read_json(&path)?;
            validate_terminal_receipt(&existing)?;
            let terminal_matches = record.terminal.as_ref().is_some_and(|local| {
                existing.record.terminal.as_ref().is_some_and(|retained| {
                    local.pull_request == retained.pull_request
                        && local.disposition == retained.disposition
                        && local.observed_sha == retained.observed_sha
                        && local.observed_state == retained.observed_state
                        && local.released_branch == retained.released_branch
                        && local.released_worktree == retained.released_worktree
                        && local.released_protected_paths == retained.released_protected_paths
                })
            });
            if existing.repository != record.repository
                || existing.initialization_digest != record.initialization_digest
                || existing.record.generation != record.generation
                || existing.record.digest != record.digest
                || existing.cards != cards
                || !terminal_matches
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "terminal receipt conflicts with retained authority",
                ));
            }
            return Ok(existing);
        }
        verify_cards(self, &record, &cards)?;
        let terminal = record.terminal.as_mut().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidTransition, "terminal evidence missing")
        })?;
        if terminal.receipt_path != receipt_ref {
            terminal.receipt_path = receipt_ref.clone();
            record.generation += 1;
            for values in cards.values_mut() {
                values.identity.generation = record.generation;
            }
            record.audit.push(AuditEvent {
                sequence: record.audit.len() as u64 + 1,
                generation: record.generation,
                actor: "csdlc-closeout".into(),
                reason: "normalize legacy terminal receipt path to portable reference".into(),
                operation: "normalize_terminal_receipt_ref".into(),
            });
            hydrate_projections(&mut record, &cards)?;
            record.digest = record_digest(&record)?;
            self.commit(issue, &record, &cards, false)?;
        }
        let authored_artifacts = [record.design_path.clone(), record.diagram_path.clone()]
            .into_iter()
            .map(|path| {
                let contents = fs::read_to_string(self.root.join(&path))?;
                Ok((path, contents))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        let mut receipt = TerminalReceipt {
            schema: "csdlc.terminal_receipt.v1".into(),
            issue,
            repository: record.repository.clone(),
            initialization_digest: record.initialization_digest.clone(),
            receipt_ref,
            authored_artifacts,
            record,
            cards,
            digest: String::new(),
        };
        receipt.digest = terminal_receipt_digest(&receipt)?;
        validate_terminal_receipt(&receipt)?;
        let temporary = parent.join(format!(".{issue}.tmp"));
        write_json(&temporary, &receipt)?;
        fs::rename(temporary, &path)?;
        sync_dir(parent)?;
        Ok(receipt)
    }

    pub fn reconcile_terminal(&self, request: ReconcileTerminalRequest) -> Result<IssueRecord> {
        if request.actor.trim().is_empty() || request.reason.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "reconciliation actor and reason are required",
            ));
        }
        if request.expected_branch.trim().is_empty()
            || request.expected_worktree.trim().is_empty()
            || request.expected_branch == "main"
            || crate::git::current_branch(&self.root)? != request.expected_branch
            || self.root.canonicalize()? != Path::new(&request.expected_worktree).canonicalize()?
        {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "terminal reconciliation requires the declared dedicated branch and worktree",
            ));
        }
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let _lock = self.lock(request.issue)?;
        self.recover_with_terminal_lock(request.issue)?;
        let mut receipt = self
            .load_terminal_receipt(request.issue)?
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "terminal receipt missing"))?;
        if receipt.issue != request.issue {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal receipt issue differs from reconciliation request",
            ));
        }
        if receipt.initialization_digest != request.expected_initialization_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal receipt initialization digest differs from reconciliation request",
            ));
        }
        let issue_dir = self.issue_dir(request.issue);
        let local = match self.load_record(request.issue) {
            Ok(local) => {
                if receipt.initialization_digest != local.initialization_digest
                    || receipt.repository != local.repository
                {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "terminal receipt identity differs from local issue",
                    ));
                }
                local
            }
            Err(error) if error.code == ErrorCode::Io && !issue_dir.exists() => {
                receipt.record.clone()
            }
            Err(error) => return Err(error),
        };
        let requested_follow_ups = request
            .follow_ups
            .iter()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();
        if requested_follow_ups.len() != request.follow_ups.len() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal follow-ups must be non-empty and unique",
            ));
        }
        let existing_follow_ups = match receipt
            .cards
            .get(&CardKind::Sor)
            .map(|values| &values.content)
        {
            Some(CardContent::Sor(values)) => {
                values.follow_ups.iter().cloned().collect::<BTreeSet<_>>()
            }
            _ => BTreeSet::new(),
        };
        let local_cards = match self.load_cards(request.issue) {
            Ok(cards) => Some(cards),
            Err(_) if !issue_dir.exists() => None,
            Err(error) => return Err(error),
        };
        let local_integrity = (|| -> Result<bool> {
            verify_record(&local)?;
            let Some(current_cards) = local_cards.as_ref() else {
                return Ok(false);
            };
            let mut checked = local.clone();
            hydrate_projections(&mut checked, current_cards)?;
            Ok(checked.digest == record_digest(&checked)? && checked.digest == local.digest)
        })()?;
        if local.phase == LifecyclePhase::ClosedOut
            && local.terminal == receipt.record.terminal
            && local.publication.as_ref().is_some_and(|publication| {
                !publication.draft && publication.observed_state == "merged"
            })
            && existing_follow_ups == requested_follow_ups
            && local_integrity
            && local.audit.last().is_some_and(|event| {
                event.operation == "reconcile_terminal"
                    && event.actor == request.actor
                    && event.reason == request.reason
            })
        {
            return Ok(local);
        }
        // A complete, valid tracked terminal projection may contain newer
        // append-only audit provenance than the machine-local receipt. Preserve
        // that history and refresh the receipt from it; use the receipt as the
        // recovery authority only when the tracked projection is absent,
        // incomplete, or invalid.
        let local_cards_match_receipt_semantics = local_cards.as_ref().is_some_and(|values| {
            let mut local_values = values.clone();
            let mut receipt_values = receipt.cards.clone();
            for card in local_values.values_mut() {
                card.identity.generation = 0;
            }
            for card in receipt_values.values_mut() {
                card.identity.generation = 0;
            }
            local_values == receipt_values
        });
        let prefer_local = local_integrity
            && local.phase == LifecyclePhase::ClosedOut
            && local.claim.is_none()
            && local.terminal == receipt.record.terminal
            && local_cards_match_receipt_semantics;
        let (mut projection, mut cards) = if prefer_local {
            (local.clone(), local_cards.expect("validated local cards"))
        } else {
            (receipt.record, receipt.cards)
        };
        if local_integrity
            && local.phase == LifecyclePhase::ClosedOut
            && local.claim.is_none()
            && local.terminal == projection.terminal
        {
            for (retained, tracked) in projection.audit.iter_mut().zip(local.audit.iter()) {
                if retained.sequence != tracked.sequence
                    || retained.generation != tracked.generation
                    || retained.operation != tracked.operation
                {
                    break;
                }
                // Sequence, generation, and operation identify the same durable
                // event. Preserve the tracked actor/reason provenance when an
                // older machine-local receipt retained different attribution.
                *retained = tracked.clone();
            }
        }
        if let (Some(publication), Some(terminal)) = (
            projection.publication.as_mut(),
            projection.terminal.as_ref(),
        ) {
            if terminal.disposition == crate::readiness::TerminalDisposition::Merged
                && terminal.pull_request == Some(publication.pull_request)
                && terminal.observed_state == "merged"
            {
                publication.draft = false;
                publication.observed_state = "merged".into();
            }
        }
        let current_review_passes = cards.get(&CardKind::Srp).is_some_and(|card| {
            matches!(&card.content, CardContent::Srp(srp)
            if srp.review_result == crate::cards::ReviewResult::Pass
                && srp.review_revision.as_deref().is_some_and(|value| !value.is_empty())
                && srp.reviewer.as_deref().is_some_and(|value| !value.is_empty())
                && !srp.findings.iter().any(|finding| {
                    finding.actionable
                        && finding.disposition == crate::cards::FindingDisposition::Open
                }))
        });
        if projection
            .review
            .as_ref()
            .is_some_and(|review| review.completed)
            && current_review_passes
        {
            cards.get_mut(&CardKind::Srp).expect("SRP card").status =
                crate::cards::CardStatus::Complete;
        }
        let routed = match cards.get(&CardKind::Srp).map(|values| &values.content) {
            Some(CardContent::Srp(values)) => values
                .residual_risk
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>(),
            _ => BTreeSet::new(),
        };
        if !requested_follow_ups.is_subset(&routed) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal follow-ups must be routed by SRP residual risk",
            ));
        }
        let design = receipt
            .authored_artifacts
            .get(&projection.design_path)
            .expect("validated receipt design")
            .clone();
        let diagram = receipt
            .authored_artifacts
            .get(&projection.diagram_path)
            .expect("validated receipt diagram")
            .clone();
        let design_path = format!(".csdlc/issues/{}/retained/design.md", request.issue);
        let diagram_path = format!(".csdlc/issues/{}/retained/diagram.mmd", request.issue);
        projection.design_path = design_path.clone();
        projection.diagram_path = diagram_path.clone();
        for kind in [CardKind::Spp, CardKind::Vpp] {
            match &mut cards.get_mut(&kind).expect("design card").content {
                CardContent::Spp(values) => {
                    values.design_ref = design_path.clone();
                    values.diagram_ref = diagram_path.clone();
                }
                CardContent::Vpp(values) => {
                    values.design_ref = design_path.clone();
                    values.diagram_ref = diagram_path.clone();
                }
                _ => unreachable!("design card"),
            }
        }
        if !requested_follow_ups.is_empty() {
            match &mut cards.get_mut(&CardKind::Sor).expect("SOR card").content {
                CardContent::Sor(values) => {
                    values.follow_ups = requested_follow_ups.into_iter().collect();
                }
                _ => unreachable!("SOR card"),
            }
        }
        projection.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = projection.generation;
        }
        projection.audit.push(AuditEvent {
            sequence: projection.audit.len() as u64 + 1,
            generation: projection.generation,
            actor: request.actor,
            reason: request.reason,
            operation: "reconcile_terminal".into(),
        });
        validate_cross_card(
            &cards,
            &design_path,
            &digest(design.as_bytes()),
            &diagram_path,
            &digest(diagram.as_bytes()),
        )?;
        hydrate_projections(&mut projection, &cards)?;
        projection.digest = record_digest(&projection)?;
        let retained_artifacts = BTreeMap::from([(design_path, design), (diagram_path, diagram)]);
        let receipt_path = self.terminal_receipt_path(request.issue)?;
        let receipt_parent = receipt_path.parent().expect("receipt parent");
        let receipt_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(receipt_parent.join("receipts.lock"))?;
        receipt_lock.lock_exclusive()?;
        let original_receipt = fs::read(&receipt_path)?;
        drop(receipt_lock);
        receipt.record = projection.clone();
        receipt.cards = cards.clone();
        receipt.authored_artifacts = retained_artifacts.clone();
        receipt.digest.clear();
        receipt.digest = terminal_receipt_digest(&receipt)?;
        validate_terminal_receipt(&receipt)?;
        let target_receipt = serde_json::to_vec_pretty(&receipt)?;
        let mut journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            issue: request.issue,
            stage: "prepared".into(),
            original_record_digest: local.digest.clone(),
            target_record_digest: projection.digest.clone(),
            original_receipt: Some(original_receipt),
            target_receipt,
        };
        Self::maybe_interrupt_terminal_transaction(request.issue, "before_journal")?;
        self.write_terminal_transaction_journal(&journal)?;
        Self::maybe_interrupt_terminal_transaction(request.issue, "after_journal")?;
        if let Err(error) = self.commit_with_authored(
            request.issue,
            &projection,
            &cards,
            false,
            Some(&retained_artifacts),
        ) {
            if !matches!(&error.code, ErrorCode::InterruptedTransaction) {
                let _ = self.recover_terminal_transaction(request.issue);
            }
            return Err(error);
        }
        Self::maybe_interrupt_terminal_transaction(request.issue, "after_projection")?;
        journal.stage = "projection_committed".into();
        self.write_terminal_transaction_journal(&journal)?;
        Self::maybe_interrupt_terminal_transaction(request.issue, "after_projection_journal")?;
        let refresh = (|| -> Result<()> {
            let parent = receipt_path.parent().expect("receipt parent");
            let lock = OpenOptions::new()
                .create(true)
                .truncate(false)
                .read(true)
                .write(true)
                .open(parent.join("receipts.lock"))?;
            lock.lock_exclusive()?;
            let temporary = receipt_path.with_extension("json.reconcile-tmp");
            let mut file = File::create(&temporary)?;
            file.write_all(&journal.target_receipt)?;
            file.sync_all()?;
            Self::maybe_interrupt_terminal_transaction(request.issue, "after_receipt_write")?;
            fs::rename(temporary, &receipt_path)?;
            sync_dir(parent)?;
            Self::maybe_interrupt_terminal_transaction(request.issue, "after_receipt_rename")?;
            Ok(())
        })();
        if let Err(error) = refresh {
            if !matches!(&error.code, ErrorCode::InterruptedTransaction) {
                let _ = self.recover_terminal_transaction(request.issue);
            }
            return Err(error);
        }
        journal.stage = "receipt_committed".into();
        self.write_terminal_transaction_journal(&journal)?;
        Self::maybe_interrupt_terminal_transaction(request.issue, "after_receipt_journal")?;
        self.remove_terminal_transaction_journal(request.issue)?;
        Ok(projection)
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
        self.recover_local_transaction(issue)?;
        if self.terminal_transaction_path(issue)?.is_file() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal transaction recovery requires the shared terminal lock",
            ));
        }
        Ok(())
    }

    fn recover_with_terminal_lock(&self, issue: u64) -> Result<()> {
        self.recover_local_transaction(issue)?;
        self.recover_terminal_transaction(issue)?;
        Ok(())
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
        merged: bool,
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
        sor.integration_state = if merged {
            crate::cards::IntegrationState::Merged
        } else {
            crate::cards::IntegrationState::PrOpen
        };
        sor.merge_state = if merged {
            crate::cards::MergeState::Merged
        } else {
            crate::cards::MergeState::NotMerged
        };
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
                if merged {
                    "observed exact merged PR after current review"
                } else {
                    "observed exact PR after current review"
                }
                .into(),
            )?;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason: if merged {
                "atomically record observed merged GitHub publication and SOR projection"
            } else {
                "atomically record observed GitHub publication and SOR projection"
            }
            .into(),
            operation: if merged {
                "record_merged_publication"
            } else {
                "record_publication"
            }
            .into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_ready_publication(
        &self,
        request: &crate::publication::ReadyPublicationRequest,
        evidence: PublicationEvidence,
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
                "ready publication record changed before commit",
            ));
        }
        if record.phase != LifecyclePhase::Published
            || record.publication.as_ref().is_none_or(|publication| {
                publication.repository != request.repository
                    || publication.pull_request != request.pull_request
                    || !publication.draft
            })
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "canonical publication is no longer the governed draft",
            ));
        }
        let publication = record.publication.as_ref().expect("publication checked");
        let observed_revision = crate::git::clean_commit_revision(&request.expected_head_sha);
        if publication.revision != observed_revision {
            let Some(from_commit) = publication
                .revision
                .strip_prefix("git-blake3:")
                .and_then(|value| value.split(':').next())
            else {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "canonical publication revision is invalid",
                ));
            };
            let changed = crate::git::metadata_only_changed_paths(
                &self.root,
                from_commit,
                &request.expected_head_sha,
            )
            .map_err(|_| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "ready head is not a forward metadata-only publication revision",
                )
            })?;
            if changed.is_empty() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "ready head changed without typed publication metadata",
                ));
            }
        }
        let mut cards = self.load_cards(request.issue)?;
        verify_cards(self, &record, &cards)?;
        let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            CardContent::Sor(values) => values,
            _ => unreachable!("SOR"),
        };
        sor.publication_state = crate::cards::PublicationState::Ready;
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
        }
        record.publication = Some(evidence);
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: request.actor.clone(),
            reason: "record exact existing PR ready-for-review after remote success".into(),
            operation: "record_ready_publication".into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(request.issue, &record, &cards, false)?;
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
        if publication.pull_request != request.pull_request {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "readiness observation does not match published PR revision",
            ));
        }
        let observed_revision = crate::git::clean_commit_revision(&request.head_sha);
        let publication_revision_reconciled = if publication.revision != observed_revision {
            let Some(from_commit) = publication
                .revision
                .strip_prefix("git-blake3:")
                .and_then(|value| value.split(':').next())
            else {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "published PR revision is not a clean commit identity",
                ));
            };
            let changed_paths =
                crate::git::metadata_only_changed_paths(&self.root, from_commit, &request.head_sha)
                    .map_err(|_| {
                        V2Error::new(
                            ErrorCode::ReconciliationRequired,
                            "readiness observation does not match published PR revision",
                        )
                    })?;
            if changed_paths.is_empty() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "published PR revision changed without typed metadata delta",
                ));
            }
            true
        } else {
            false
        };
        if record.readiness.as_ref() == Some(&evidence) {
            return Ok(record);
        }
        let mut cards = self.load_cards(request.issue)?;
        verify_cards(self, &record, &cards)?;
        if publication_revision_reconciled {
            record.publication.as_mut().expect("publication").revision = observed_revision;
        }
        let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            CardContent::Sor(value) => value,
            _ => unreachable!(),
        };
        if evidence.ready {
            let validation_ready = terminal_validation_passed(&sor.actual_validation);
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
            operation: if publication_revision_reconciled {
                "record_readiness_reconcile_metadata_only_published_revision"
            } else {
                "record_readiness"
            }
            .into(),
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
        let current_validation = match &cards[&CardKind::Sor].content {
            CardContent::Sor(value) => &value.actual_validation,
            _ => unreachable!(),
        };
        if !terminal_validation_passed(current_validation) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal closeout requires current passing validation evidence",
            ));
        }
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
    let validation_passed = terminal_validation_passed(&sor.actual_validation);
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
    if next == LifecyclePhase::MergeReady
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

fn complete_terminal_plan_step(
    cards: &mut BTreeMap<CardKind, CardValues>,
    step_id: &str,
) -> Result<()> {
    let spp = match &mut cards.get_mut(&CardKind::Spp).expect("SPP").content {
        CardContent::Spp(values) => values,
        _ => unreachable!("SPP card content"),
    };
    let step = spp
        .steps
        .iter_mut()
        .find(|step| step.id == step_id)
        .ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidInput, "terminal plan step does not exist")
        })?;
    complete_step_status(&mut step.status)
}

fn replace_terminal_sor_artifact(
    cards: &mut BTreeMap<CardKind, CardValues>,
    stale_ref: &str,
    retained_ref: &str,
) -> Result<()> {
    let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
        CardContent::Sor(values) => values,
        _ => unreachable!("SOR card content"),
    };
    let stale_count = sor
        .artifacts
        .iter()
        .filter(|artifact| artifact.as_str() == stale_ref)
        .count();
    if stale_count != 1 {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "terminal SOR artifact repair requires exactly one stale reference",
        ));
    }
    if sor
        .artifacts
        .iter()
        .any(|artifact| artifact == retained_ref)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "terminal SOR artifact replacement is already present",
        ));
    }
    *sor.artifacts
        .iter_mut()
        .find(|artifact| artifact.as_str() == stale_ref)
        .expect("count checked") = retained_ref.to_owned();
    Ok(())
}

fn replace_terminal_sor_validation(
    cards: &mut BTreeMap<CardKind, CardValues>,
    expected: &ValidationResult,
    replacement: &ValidationResult,
) -> Result<()> {
    let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
        CardContent::Sor(values) => values,
        _ => unreachable!("SOR card content"),
    };
    let matches: Vec<_> = sor
        .actual_validation
        .iter_mut()
        .filter(|result| *result == expected)
        .collect();
    if matches.len() != 1 {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "terminal SOR validation repair requires exactly one expected result",
        ));
    }
    *matches.into_iter().next().expect("one match") = replacement.clone();
    Ok(())
}

fn validate_portable_validation_result(result: &ValidationResult) -> Result<()> {
    if result
        .command
        .iter()
        .any(|part| contains_machine_local_path(part, true))
        || contains_machine_local_path(&result.evidence_ref, false)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "terminal SOR validation replacement contains a machine-local path",
        ));
    }
    Ok(())
}

fn contains_machine_local_path(value: &str, shell_context: bool) -> bool {
    if value.to_ascii_lowercase().contains("file://")
        || contains_shell_expansion(value)
        || (shell_context && value.contains('`'))
        || contains_backtick_path_expansion(value)
        || contains_windows_environment_expansion(value)
    {
        return true;
    }
    value.split_whitespace().any(|word| {
        if word.starts_with("http://") || word.starts_with("https://") {
            return false;
        }
        word.split(['=', '[', '(', '{', ',', ';', '>', '<', '|', '&'])
            .any(|segment| {
                let candidate = segment.trim_matches(|character: char| {
                    matches!(character, '\'' | '"' | ')' | ']' | '}')
                });
                candidate.starts_with('/')
                    || candidate.starts_with("~/")
                    || candidate.starts_with("~\\")
                    || candidate.starts_with("\\\\")
                    || candidate.starts_with("//")
                    || is_windows_absolute_path(candidate)
            })
    })
}

fn contains_shell_expansion(value: &str) -> bool {
    value.char_indices().any(|(index, character)| {
        if character != '$' {
            return false;
        }
        let suffix = &value[index + character.len_utf8()..];
        if suffix.starts_with(['(', '{']) {
            return true;
        }
        let boundary = value[..index].chars().next_back().is_none_or(|previous| {
            previous.is_whitespace() || matches!(previous, '=' | '\'' | '"')
        });
        boundary
            && suffix
                .chars()
                .next()
                .is_some_and(|next| next.is_ascii_alphabetic() || next == '_')
    })
}

fn contains_backtick_path_expansion(value: &str) -> bool {
    let Some(start) = value.find('`') else {
        return false;
    };
    let Some(end) = value[start + 1..].find('`') else {
        return false;
    };
    value[start + end + 2..].starts_with(['/', '\\'])
}

fn contains_windows_environment_expansion(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.iter().enumerate().any(|(start, byte)| {
        if *byte != b'%' {
            return false;
        }
        let name = &bytes[start + 1..];
        let Some(end) = name.iter().position(|candidate| *candidate == b'%') else {
            return false;
        };
        end > 0
            && name[..end]
                .iter()
                .all(|candidate| candidate.is_ascii_alphanumeric() || *candidate == b'_')
    })
}

fn is_windows_absolute_path(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && matches!(bytes[2], b'/' | b'\\')
}

fn claim_covers_issue(claim: &Claim, issue: u64) -> bool {
    let target = format!(".csdlc/issues/{issue}");
    claim
        .protected_paths
        .iter()
        .any(|path| path.trim_end_matches('/') == target)
}

fn complete_step_status(status: &mut StepStatus) -> Result<()> {
    if !matches!(*status, StepStatus::Pending | StepStatus::InProgress) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "terminal plan repair only allows forward completion",
        ));
    }
    *status = StepStatus::Completed;
    Ok(())
}

fn valid_mermaid_diagram(diagram: &str) -> bool {
    let first = diagram
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or_default()
        .trim();
    (first.starts_with("flowchart ")
        || first == "stateDiagram-v2"
        || first.starts_with("sequenceDiagram"))
        && diagram.lines().count() >= 2
}

#[cfg(test)]
mod terminal_design_repair_tests {
    use super::*;

    fn copy_tree(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).expect("create fixture directory");
        for entry in fs::read_dir(source).expect("read fixture directory") {
            let entry = entry.expect("fixture entry");
            let target = destination.join(entry.file_name());
            if entry.file_type().expect("fixture type").is_dir() {
                copy_tree(&entry.path(), &target);
            } else {
                fs::copy(entry.path(), target).expect("copy fixture file");
            }
        }
    }

    fn terminal_validation_fixture() -> (
        tempfile::TempDir,
        Store,
        IssueRecord,
        IssueRecord,
        TerminalReceipt,
        ValidationResult,
    ) {
        let source_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let source_store = Store::new(&source_root);
        let temp = tempfile::tempdir().expect("temp root");
        let status = std::process::Command::new("git")
            .args(["init", "-q"])
            .current_dir(temp.path())
            .status()
            .expect("git init");
        assert!(status.success());
        for issue in [5358, 5613] {
            copy_tree(
                &source_store.issue_dir(issue),
                &temp.path().join(".csdlc/issues").join(issue.to_string()),
            );
            let record = source_store.load_record(issue).expect("source record");
            for path in [&record.design_path, &record.diagram_path] {
                let destination = temp.path().join(path);
                fs::create_dir_all(destination.parent().expect("authored parent"))
                    .expect("create authored parent");
                fs::copy(source_root.join(path), destination).expect("copy authored file");
            }
        }

        let store = Store::new(temp.path());
        let mut authority = store.load_record(5613).expect("authority");
        authority
            .claim
            .as_mut()
            .expect("authority claim")
            .expires_unix_seconds = u64::MAX;
        let authority_cards = store.load_cards(5613).expect("authority cards");
        hydrate_projections(&mut authority, &authority_cards).expect("authority projections");
        authority.digest = record_digest(&authority).expect("authority digest");
        store
            .commit(5613, &authority, &authority_cards, false)
            .expect("authority commit");

        let target = store.load_record(5358).expect("target");
        let cards = store.load_cards(5358).expect("target cards");
        let mut authored_artifacts = BTreeMap::new();
        for path in [&target.design_path, &target.diagram_path] {
            authored_artifacts.insert(
                path.clone(),
                fs::read_to_string(temp.path().join(path)).expect("authored contents"),
            );
        }
        let mut receipt = TerminalReceipt {
            schema: "csdlc.terminal_receipt.v1".into(),
            issue: target.issue,
            repository: target.repository.clone(),
            initialization_digest: target.initialization_digest.clone(),
            receipt_ref: format!("csdlc-v2/closeout/{}.json", target.issue),
            authored_artifacts,
            record: target.clone(),
            cards: cards.clone(),
            digest: String::new(),
        };
        receipt.digest = terminal_receipt_digest(&receipt).expect("receipt digest");
        validate_terminal_receipt(&receipt).expect("valid receipt");
        let receipt_path = store
            .terminal_receipt_path(target.issue)
            .expect("receipt path");
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("create receipt parent");
        write_json(&receipt_path, &receipt).expect("write receipt");
        let CardContent::Sor(sor) = &cards[&CardKind::Sor].content else {
            panic!("SOR");
        };
        let expected = sor.actual_validation.first().expect("validation").clone();
        (temp, store, authority, target, receipt, expected)
    }

    fn validation_repair_request(
        authority: &IssueRecord,
        target: &IssueRecord,
        receipt: &TerminalReceipt,
        expected: ValidationResult,
        fail_after_stage: Option<&str>,
    ) -> TerminalSorValidationRepairRequest {
        let mut replacement = expected.clone();
        replacement.evidence_ref = "issue-5358:portable-terminal-proof".into();
        TerminalSorValidationRepairRequest {
            authority_issue: authority.issue,
            target_issue: target.issue,
            expected_authority_generation: authority.generation,
            expected_authority_digest: authority.digest.clone(),
            expected_target_generation: target.generation,
            expected_target_digest: target.digest.clone(),
            expected_receipt_digest: receipt.digest.clone(),
            authority_claim_id: authority.claim.as_ref().expect("claim").id.clone(),
            actor: "codex:test".into(),
            expected_result: expected,
            replacement_result: replacement,
            fail_after_stage: fail_after_stage.map(str::to_owned),
        }
    }

    #[test]
    fn terminal_design_repair_rejects_incomplete_authority_before_io() {
        let root = tempfile::tempdir().expect("temp root");
        let error = Store::new(root.path())
            .repair_terminal_design(TerminalDesignRepairRequest {
                authority_issue: 5487,
                target_issue: 5467,
                expected_authority_generation: 1,
                expected_authority_digest: String::new(),
                expected_target_generation: 18,
                expected_target_digest: "target".into(),
                expected_receipt_digest: "receipt".into(),
                authority_claim_id: "claim".into(),
                actor: "codex".into(),
                reviewer: "reviewer".into(),
                source_design_path: "design.md".into(),
                source_diagram_path: "diagram.mmd".into(),
                expected_design_digest: "design".into(),
                expected_diagram_digest: "diagram".into(),
                fail_after_stage: None,
            })
            .expect_err("missing authority digest must fail closed");
        assert_eq!(error.code.to_string(), "invalid_input");
    }

    #[test]
    fn terminal_design_repair_mermaid_guard_is_fail_closed() {
        assert!(valid_mermaid_diagram("flowchart LR\n  A-->B\n"));
        assert!(!valid_mermaid_diagram("not mermaid\n  A-->B\n"));
        assert!(!valid_mermaid_diagram("flowchart LR\n"));
    }

    #[test]
    fn terminal_plan_repair_rejects_incomplete_authority_before_io() {
        let root = tempfile::tempdir().expect("temp root");
        let error = Store::new(root.path())
            .repair_terminal_plan_step(TerminalPlanStepRepairRequest {
                authority_issue: 5518,
                target_issue: 5516,
                expected_authority_generation: 0,
                expected_authority_digest: String::new(),
                expected_target_generation: 18,
                expected_target_digest: "target".into(),
                expected_receipt_digest: "receipt".into(),
                authority_claim_id: "claim".into(),
                actor: "codex".into(),
                step_id: "S3".into(),
                fail_after_stage: None,
            })
            .expect_err("missing authority digest must fail closed");
        assert_eq!(error.code.to_string(), "invalid_input");
    }

    #[test]
    fn terminal_plan_repair_status_is_forward_only() {
        for initial in [StepStatus::Pending, StepStatus::InProgress] {
            let mut status = initial;
            complete_step_status(&mut status).expect("forward completion");
            assert_eq!(status, StepStatus::Completed);
        }
        let mut completed = StepStatus::Completed;
        let error = complete_step_status(&mut completed).expect_err("no rewrite");
        assert_eq!(error.code.to_string(), "invalid_transition");
    }

    #[test]
    fn terminal_plan_repair_requires_exact_target_scope() {
        let mut claim = Claim {
            id: "claim".into(),
            owner: "agent".into(),
            generation: 0,
            acquired_unix_seconds: 1,
            expires_unix_seconds: u64::MAX,
            heartbeat_unix_seconds: 1,
            branch: "issue".into(),
            worktree: ".".into(),
            protected_paths: vec![".csdlc/issues/5517".into()],
            purpose: "repair".into(),
        };
        assert!(!claim_covers_issue(&claim, 5516));
        claim.protected_paths.push(".csdlc/issues/5516/".into());
        assert!(claim_covers_issue(&claim, 5516));
    }

    #[test]
    fn terminal_sor_artifact_repair_rejects_incomplete_authority_before_io() {
        let root = tempfile::tempdir().expect("temp root");
        let error = Store::new(root.path())
            .repair_terminal_sor_artifact(TerminalSorArtifactRepairRequest {
                authority_issue: 5527,
                target_issue: 5390,
                expected_authority_generation: 0,
                expected_authority_digest: String::new(),
                expected_target_generation: 39,
                expected_target_digest: "target".into(),
                expected_receipt_digest: "receipt".into(),
                authority_claim_id: "claim".into(),
                actor: "codex".into(),
                stale_ref: ".csdlc/issues/5390/diagram.mmd".into(),
                retained_ref: ".csdlc/issues/5390/retained/diagram.mmd".into(),
                expected_artifact_digest: "diagram".into(),
                fail_after_stage: None,
            })
            .expect_err("missing authority digest must fail closed");
        assert_eq!(error.code.to_string(), "invalid_input");
    }

    #[test]
    fn terminal_sor_artifact_replacement_is_exact_and_nonduplicating() {
        let mut cards = initial_cards(
            1,
            "example/repo",
            "docs/design.md",
            "design",
            "docs/diagram.mmd",
            "diagram",
            InitialCardInput {
                title: "test".into(),
                slug: "test".into(),
                version: "v0.91.7".into(),
                goal: "test".into(),
                required_outcome: "test".into(),
                declared_scope: vec!["test".into()],
                authority_boundary: vec!["test".into()],
                operator_constraints: vec!["none".into()],
                task_boundary: "test".into(),
                deliverables: vec!["test".into()],
                acceptance_criteria: vec!["test".into()],
                dependencies: vec!["test".into()],
                repo_inputs: vec!["test".into()],
                non_goals: vec!["test".into()],
                plan_summary: "test".into(),
                steps: vec![crate::cards::PlanStep {
                    id: "S1".into(),
                    action: "test".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: StepStatus::Pending,
                }],
                invariants: vec!["test".into()],
                risks: vec!["test".into()],
                planning_profile: crate::cards::PlanningProfile::Small,
                stop_conditions: vec!["test".into()],
                validation_lanes: vec![crate::cards::ValidationLane {
                    lane: "test".into(),
                    proof_role: "test".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    deterministic: true,
                    resource_profile: crate::cards::ResourceProfile::Small,
                    budget_seconds: 1,
                    budget_tokens: 1,
                    argv: vec!["test".into()],
                    parallel_group: "test".into(),
                    defer_reason: None,
                }],
                failure_policy: "test".into(),
                review_prompts: vec!["test".into()],
                review_scope: "test".into(),
            },
        )
        .expect("cards");
        let CardContent::Sor(sor) = &mut cards.get_mut(&CardKind::Sor).unwrap().content else {
            panic!("SOR");
        };
        sor.artifacts = vec!["old".into()];
        replace_terminal_sor_artifact(&mut cards, "old", "retained").expect("replacement");
        let CardContent::Sor(sor) = &cards[&CardKind::Sor].content else {
            panic!("SOR");
        };
        assert_eq!(sor.artifacts, vec!["retained"]);
        assert!(replace_terminal_sor_artifact(&mut cards, "old", "retained").is_err());
    }

    #[test]
    fn terminal_sor_validation_repair_updates_projection_and_receipt_atomically() {
        let (_temp, store, authority, target, receipt, expected) = terminal_validation_fixture();
        let request =
            validation_repair_request(&authority, &target, &receipt, expected.clone(), None);
        let replacement = request.replacement_result.clone();
        let repaired = store
            .repair_terminal_sor_validation(request)
            .expect("terminal validation repair");
        assert_eq!(repaired.phase, LifecyclePhase::ClosedOut);
        assert!(repaired.claim.is_none());
        assert_eq!(repaired.generation, target.generation + 1);
        let cards = store.load_cards(target.issue).expect("repaired cards");
        let CardContent::Sor(sor) = &cards[&CardKind::Sor].content else {
            panic!("SOR");
        };
        assert!(!sor.actual_validation.contains(&expected));
        assert_eq!(
            sor.actual_validation
                .iter()
                .filter(|result| *result == &replacement)
                .count(),
            1
        );
        let repaired_receipt = store
            .load_terminal_receipt(target.issue)
            .expect("receipt load")
            .expect("receipt");
        assert_eq!(repaired_receipt.record.digest, repaired.digest);
        assert_eq!(repaired_receipt.cards, cards);
    }

    #[test]
    fn terminal_sor_validation_repair_rejects_stale_receipt_without_mutation() {
        let (_temp, store, authority, target, receipt, expected) = terminal_validation_fixture();
        let index_path = store.issue_dir(target.issue).join("index.json");
        let receipt_path = store
            .terminal_receipt_path(target.issue)
            .expect("receipt path");
        let original_index = fs::read(&index_path).expect("index bytes");
        let original_receipt = fs::read(&receipt_path).expect("receipt bytes");
        let mut request = validation_repair_request(&authority, &target, &receipt, expected, None);
        request.expected_receipt_digest = "stale".into();
        let error = store
            .repair_terminal_sor_validation(request)
            .expect_err("stale receipt must fail");
        assert_eq!(error.code.to_string(), "stale_digest");
        assert_eq!(fs::read(index_path).expect("index bytes"), original_index);
        assert_eq!(
            fs::read(receipt_path).expect("receipt bytes"),
            original_receipt
        );
    }

    #[test]
    fn terminal_sor_validation_repair_rolls_back_projection_and_receipt() {
        let (_temp, store, authority, target, receipt, expected) = terminal_validation_fixture();
        let index_path = store.issue_dir(target.issue).join("index.json");
        let receipt_path = store
            .terminal_receipt_path(target.issue)
            .expect("receipt path");
        let original_index = fs::read(&index_path).expect("index bytes");
        let original_receipt = fs::read(&receipt_path).expect("receipt bytes");
        let request = validation_repair_request(
            &authority,
            &target,
            &receipt,
            expected,
            Some("after_projection"),
        );
        let error = store
            .repair_terminal_sor_validation(request)
            .expect_err("injected failure must roll back");
        assert_eq!(error.code.to_string(), "interrupted_transaction");
        assert_eq!(fs::read(index_path).expect("index bytes"), original_index);
        assert_eq!(
            fs::read(receipt_path).expect("receipt bytes"),
            original_receipt
        );
    }

    #[test]
    fn terminal_sor_validation_repair_enforces_portable_replacements() {
        for machine_local in [
            "/tmp/build",
            "--target-dir=/home/alice/build",
            "cd /mnt/worker/checkout",
            r"C:\Users\alice\checkout",
            r"--out=Z:\build\target",
            r"\\server\share\checkout",
            "~/checkout",
            "CARGO_TARGET_DIR=$HOME/build",
            "CARGO_TARGET_DIR=${HOME}/build",
            "sh -c 'cd ${HOME}/checkout'",
            "$(pwd)/target",
            "`pwd`/target",
            "file:///home/alice/proof.json",
            r"%USERPROFILE%\checkout",
        ] {
            let result = ValidationResult {
                command: vec!["proof".into(), machine_local.into()],
                purpose: "proof".into(),
                outcome: crate::cards::EvidenceOutcome::Passed,
                evidence_ref: "evidence/portable.json".into(),
            };
            let error = validate_portable_validation_result(&result).expect_err(machine_local);
            assert_eq!(error.code.to_string(), "invalid_input");
        }

        for machine_local in [
            "proof\u{a0}/home/alice/out",
            "proof=[/home/alice/out]",
            "echo proof >/tmp/result",
            "tool 2>/home/alice/log",
            "cmd|/opt/local/tool",
            r"type NUL >C:\Users\alice\proof",
            r"proof&\\server\share\result",
        ] {
            let result = ValidationResult {
                command: vec!["proof".into()],
                purpose: "proof".into(),
                outcome: crate::cards::EvidenceOutcome::Passed,
                evidence_ref: machine_local.into(),
            };
            validate_portable_validation_result(&result).expect_err(machine_local);
        }

        for portable in [
            "evidence/portable.json",
            "evidence/report$final.json",
            "--target-dir=target/coverage",
            "https://example.invalid/proof",
            "https://example.invalid/proof?$select=id",
            "retained terminal receipt",
        ] {
            let result = ValidationResult {
                command: vec!["proof".into(), portable.into()],
                purpose: "proof".into(),
                outcome: crate::cards::EvidenceOutcome::Passed,
                evidence_ref: portable.into(),
            };
            validate_portable_validation_result(&result).expect(portable);
        }

        let symbolic_result = ValidationResult {
            command: vec!["proof".into()],
            purpose: "proof".into(),
            outcome: crate::cards::EvidenceOutcome::Passed,
            evidence_ref: "reviewed `proof command`".into(),
        };
        validate_portable_validation_result(&symbolic_result).expect("stable symbolic evidence");

        let result = ValidationResult {
            command: vec!["proof".into()],
            purpose: "proof".into(),
            outcome: crate::cards::EvidenceOutcome::Passed,
            evidence_ref: "/home/runner/evidence.json".into(),
        };
        validate_portable_validation_result(&result).expect_err("machine-local evidence reference");
    }
}
