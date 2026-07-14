use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use strum::{AsRefStr, Display, EnumIter, EnumString};
use time::OffsetDateTime;

use crate::cutover::CutoverEvidence;
use crate::error::{ErrorCode, Result, V2Error};
use crate::proof::{require_clean_revision, PreSwitchEvidence};
use crate::{Generation, GenerationSelector};

const BASELINE_REVISION: &str = "020bba17deb9f172e91a2ec5c0599cf42e4defe9";
const BASELINE_LINES: u64 = 49_979;
const BASELINE_FILES: usize = 95;
const RUST_LIST_SHA256: &str = "c3118c1f3766b5f4a3e549c9073b33fb83164b3006175785b4c08f84c898558f";
const SHELL_LIST_SHA256: &str = "7160399a788e467ebd934309a99f9777fb0f14d4270989e5114c73b63fa3d8cc";

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EntryDisposition {
    Remove,
    Retain,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DeletionReason {
    MissingApproval,
    ApprovalEvidenceMismatch,
    PhaseEvidenceNotGreen,
    SelectorNotV2,
    RollbackWindowActive,
    ImporterWindowActive,
    ProtectedWindowActive,
    BelowMinimumRemoval,
    QualificationNotApproved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeletionEntry {
    pub path: PathBuf,
    pub disposition: EntryDisposition,
    pub owner: Option<String>,
    pub justification: Option<String>,
    pub protected_until: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeletionManifest {
    pub schema: String,
    pub baseline_revision: String,
    pub target_percent: u16,
    pub minimum_percent: u16,
    pub default_disposition: EntryDisposition,
    pub default_owner: Option<String>,
    pub default_justification: Option<String>,
    pub entries: Vec<DeletionEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeletionApproval {
    pub schema: String,
    pub approved_by: String,
    pub approved_at: String,
    pub phase_b_blake3: String,
    pub phase_c_blake3: String,
    pub selector_blake3: String,
    pub manifest_blake3: String,
    pub code_revision: String,
    pub allow_qualified_80_to_89: bool,
    pub waive_protection_windows: bool,
    pub operator_instruction: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeletionEligibilityRequest {
    pub schema: String,
    pub issue: u64,
    pub phase_b_evidence: PathBuf,
    pub phase_c_evidence: PathBuf,
    pub selector: PathBuf,
    pub manifest: DeletionManifest,
    pub approval: Option<DeletionApproval>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DeletionDecision {
    pub schema: String,
    pub issue: u64,
    pub code_revision: String,
    pub evaluated_at: String,
    pub baseline_revision: String,
    pub baseline_files: usize,
    pub baseline_lines: u64,
    pub rust_list_sha256: String,
    pub shell_list_sha256: String,
    pub phase_b_blake3: String,
    pub phase_c_blake3: String,
    pub selector_blake3: String,
    pub manifest_blake3: String,
    pub removed_lines: u64,
    pub retained_lines: u64,
    pub removal_basis_points: u16,
    pub target_met: bool,
    pub eligible: bool,
    pub reasons: Vec<DeletionReason>,
    pub deletion_executed: bool,
}

#[derive(Debug, Clone)]
struct Baseline {
    lines: BTreeMap<PathBuf, u64>,
    rust_hash: String,
    shell_hash: String,
}

pub fn eligibility_schema_bundle() -> Value {
    json!({
        "schema": "csdlc.deletion_eligibility_schemas.v1",
        "request": schemars::schema_for!(DeletionEligibilityRequest),
        "manifest": schemars::schema_for!(DeletionManifest),
        "entry": schemars::schema_for!(DeletionEntry),
        "approval": schemars::schema_for!(DeletionApproval),
        "decision": schemars::schema_for!(DeletionDecision),
    })
}

pub fn evaluate_deletion_eligibility(
    repo: &Path,
    request: &DeletionEligibilityRequest,
) -> Result<DeletionDecision> {
    require_clean_revision(repo)?;
    let baseline = load_baseline(repo)?;
    evaluate_with_time_and_baseline(repo, request, OffsetDateTime::now_utc(), &baseline)
}

fn evaluate_with_time_and_baseline(
    repo: &Path,
    request: &DeletionEligibilityRequest,
    now: OffsetDateTime,
    baseline: &Baseline,
) -> Result<DeletionDecision> {
    validate_request(request, baseline)?;
    let phase_b_bytes = read_regular_repo_file(repo, &request.phase_b_evidence)?;
    let phase_c_bytes = read_regular_repo_file(repo, &request.phase_c_evidence)?;
    let selector_bytes = read_regular_repo_file(repo, &request.selector)?;
    let phase_b: PreSwitchEvidence = serde_json::from_slice(&phase_b_bytes)?;
    let phase_c: CutoverEvidence = serde_json::from_slice(&phase_c_bytes)?;
    let selector: GenerationSelector = serde_json::from_slice(&selector_bytes)?;
    let manifest_bytes = serde_json::to_vec(&request.manifest)?;
    let phase_b_blake3 = digest(&phase_b_bytes);
    let phase_c_blake3 = digest(&phase_c_bytes);
    let selector_blake3 = digest(&selector_bytes);
    let manifest_blake3 = digest(&manifest_bytes);
    let code_revision = git_text(repo, &["rev-parse", "HEAD"])?;
    let mut reasons = BTreeSet::new();

    if phase_b.schema != "csdlc.pre_switch_evidence.v1"
        || phase_c.schema != "csdlc.cutover_evidence.v1"
        || !phase_b.passed
        || phase_b.default_before != Generation::V1
        || phase_b.default_after != Generation::V1
        || !phase_b.v1_paths_before
        || !phase_b.v1_paths_after
        || !phase_c.passed
        || phase_c.final_generation != Generation::V2
        || !phase_c.explicit_v1_override
        || !phase_c.v1_paths_before
        || !phase_c.v1_paths_after
        || phase_c.deletion_authorized
        || phase_c.pre_switch_evidence_blake3 != phase_b_blake3
    {
        reasons.insert(DeletionReason::PhaseEvidenceNotGreen);
    }
    if selector.schema != "csdlc.generation_selector.v1"
        || selector.default_generation != Generation::V2
    {
        reasons.insert(DeletionReason::SelectorNotV2);
    }
    let rollback_expires = parse_time(&phase_c.rollback_expires_at)?;
    let importer_expires = parse_time(&phase_c.importer_expires_at)?;
    let waive_protection_windows = request
        .approval
        .as_ref()
        .is_some_and(|approval| approval.waive_protection_windows);
    if now < rollback_expires && !waive_protection_windows {
        reasons.insert(DeletionReason::RollbackWindowActive);
    }
    if now < importer_expires && !waive_protection_windows {
        reasons.insert(DeletionReason::ImporterWindowActive);
    }

    let overrides = request
        .manifest
        .entries
        .iter()
        .map(|entry| (&entry.path, entry))
        .collect::<BTreeMap<_, _>>();
    let removed_lines = baseline
        .lines
        .iter()
        .filter(|(path, _)| {
            if path.as_path() == Path::new("adl/src/session_ledger.rs") {
                return false;
            }
            overrides
                .get(path)
                .map(|entry| entry.disposition)
                .unwrap_or(request.manifest.default_disposition)
                == EntryDisposition::Remove
        })
        .map(|(_, lines)| lines)
        .sum::<u64>();
    let retained_lines = BASELINE_LINES - removed_lines;
    let basis_points = (removed_lines.saturating_mul(10_000) / BASELINE_LINES)
        .try_into()
        .unwrap_or(u16::MAX);
    if basis_points < request.manifest.minimum_percent * 100 {
        reasons.insert(DeletionReason::BelowMinimumRemoval);
    }
    for entry in request
        .manifest
        .entries
        .iter()
        .filter(|e| e.disposition == EntryDisposition::Remove)
    {
        if let Some(value) = &entry.protected_until {
            if now < parse_time(value)? && !waive_protection_windows {
                reasons.insert(DeletionReason::ProtectedWindowActive);
            }
        }
    }

    match &request.approval {
        None => {
            reasons.insert(DeletionReason::MissingApproval);
        }
        Some(approval) => {
            validate_approval(approval)?;
            let approved_at = parse_time(&approval.approved_at)?;
            let cutover_at = parse_time(&phase_c.cutover_at)?;
            if approval.phase_b_blake3 != phase_b_blake3
                || approval.phase_c_blake3 != phase_c_blake3
                || approval.selector_blake3 != selector_blake3
                || approval.manifest_blake3 != manifest_blake3
                || (!approval.code_revision.eq(&code_revision)
                    && git_text(
                        repo,
                        &[
                            "merge-base",
                            "--is-ancestor",
                            &approval.code_revision,
                            &code_revision,
                        ],
                    )
                    .is_err())
                || approved_at < cutover_at
                || approved_at > now
            {
                reasons.insert(DeletionReason::ApprovalEvidenceMismatch);
            }
            if basis_points < request.manifest.target_percent * 100
                && !approval.allow_qualified_80_to_89
            {
                reasons.insert(DeletionReason::QualificationNotApproved);
            }
        }
    }
    let reasons = reasons.into_iter().collect::<Vec<_>>();
    Ok(DeletionDecision {
        schema: "csdlc.deletion_eligibility.v1".into(),
        issue: request.issue,
        code_revision,
        evaluated_at: now
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(time_error)?,
        baseline_revision: BASELINE_REVISION.into(),
        baseline_files: baseline.lines.len(),
        baseline_lines: BASELINE_LINES,
        rust_list_sha256: baseline.rust_hash.clone(),
        shell_list_sha256: baseline.shell_hash.clone(),
        phase_b_blake3,
        phase_c_blake3,
        selector_blake3,
        manifest_blake3,
        removed_lines,
        retained_lines,
        removal_basis_points: basis_points,
        target_met: basis_points >= request.manifest.target_percent * 100,
        eligible: reasons.is_empty(),
        reasons,
        deletion_executed: false,
    })
}

fn load_baseline(repo: &Path) -> Result<Baseline> {
    let paths = git_text(repo, &["ls-tree", "-r", "--name-only", BASELINE_REVISION])?;
    let mut rust = Vec::new();
    let mut shell = Vec::new();
    for path in paths.lines() {
        if is_baseline_rust(path) {
            rust.push(path.to_owned());
        }
        if is_baseline_shell(path) {
            shell.push(path.to_owned());
        }
    }
    rust.sort();
    rust.dedup();
    shell.sort();
    shell.dedup();
    let rust_hash = sha256(format!("{}\n", rust.join("\n")).as_bytes());
    let shell_hash = sha256(format!("{}\n", shell.join("\n")).as_bytes());
    if rust_hash != RUST_LIST_SHA256
        || shell_hash != SHELL_LIST_SHA256
        || rust.len() + shell.len() != BASELINE_FILES
    {
        return Err(invalid(
            "pinned baseline path inventory does not match Gate 1 hashes",
        ));
    }
    let mut lines = BTreeMap::new();
    for path in rust.iter().chain(shell.iter()) {
        let bytes = git_bytes(repo, &["show", &format!("{BASELINE_REVISION}:{path}")])?;
        let count = bytes.iter().filter(|byte| **byte == b'\n').count() as u64;
        lines.insert(PathBuf::from(path), count);
        // The eligibility evaluator runs both before deletion and after the
        // approved sunset. Historical bytes come from the pinned revision;
        // removed paths are expected to be absent in the final tree.
    }
    if lines.values().sum::<u64>() != BASELINE_LINES {
        return Err(invalid("pinned baseline line total does not match Gate 1"));
    }
    Ok(Baseline {
        lines,
        rust_hash,
        shell_hash,
    })
}

fn is_baseline_rust(path: &str) -> bool {
    if path.starts_with("adl/src/cli/pr_cmd/") && path.ends_with(".rs") {
        return true;
    }
    matches!(
        path,
        "adl/src/cli/pr_cmd.rs"
            | "adl/src/cli/pr_cmd_args.rs"
            | "adl/src/cli/pr_cmd_cards.rs"
            | "adl/src/cli/pr_cmd_prompt.rs"
            | "adl/src/cli/pr_cmd_validate.rs"
            | "adl/src/csdlc_prompt_editor.rs"
            | "adl/src/session_ledger.rs"
            | "adl/src/pr_dispatch_support.rs"
            | "adl/src/cli/tooling_cmd/prompt_template.rs"
            | "adl/src/cli/tooling_cmd/structured_prompt.rs"
    ) || (path.starts_with("adl/src/bin/")
        && !path[12..].contains('/')
        && path.ends_with(".rs")
        && {
            let name = &path[12..];
            name.starts_with("adl_pr_")
                || matches!(
                    name,
                    "adl_csdlc.rs" | "csdlc.rs" | "adl_issue.rs" | "adl_session.rs"
                )
        })
}

fn is_baseline_shell(path: &str) -> bool {
    if !path.starts_with("adl/tools/") || path[10..].contains('/') || !path.ends_with(".sh") {
        return false;
    }
    let name = &path[10..];
    matches!(
        name,
        "pr.sh"
            | "card_paths.sh"
            | "pr_cards.sh"
            | "pr_delegate.sh"
            | "pr_usage.sh"
            | "validate_structured_prompt.sh"
            | "prompt_template.sh"
    ) || name.starts_with("check_pr_")
        || name.starts_with("test_pr_")
        || name.starts_with("test_prompt_template")
}

fn validate_request(request: &DeletionEligibilityRequest, baseline: &Baseline) -> Result<()> {
    if request.schema != "csdlc.deletion_eligibility_request.v1" || request.issue != 5306 {
        return Err(invalid("request schema and issue must identify Gate 10D2"));
    }
    for path in [
        &request.phase_b_evidence,
        &request.phase_c_evidence,
        &request.selector,
    ] {
        validate_relative_path(path)?;
    }
    let m = &request.manifest;
    if m.schema != "csdlc.proposed_deletion_manifest.v1"
        || m.baseline_revision != BASELINE_REVISION
        || m.target_percent != 90
        || m.minimum_percent != 80
    {
        return Err(invalid(
            "manifest must use the reviewed Gate 1 revision and thresholds",
        ));
    }
    if m.default_disposition == EntryDisposition::Retain
        && (empty(&m.default_owner) || empty(&m.default_justification))
    {
        return Err(invalid(
            "default retained surfaces need an owner and justification",
        ));
    }
    let mut paths = BTreeSet::new();
    for entry in &m.entries {
        validate_relative_path(&entry.path)?;
        if !baseline.lines.contains_key(&entry.path) || !paths.insert(&entry.path) {
            return Err(invalid(
                "manifest overrides must be unique exact pinned inventory paths",
            ));
        }
        if entry.disposition == EntryDisposition::Retain
            && (empty(&entry.owner) || empty(&entry.justification))
        {
            return Err(invalid(
                "every retained override needs an owner and justification",
            ));
        }
        if let Some(value) = &entry.protected_until {
            parse_time(value)?;
        }
    }
    Ok(())
}

fn empty(value: &Option<String>) -> bool {
    value.as_deref().is_none_or(|v| v.trim().is_empty())
}
fn validate_approval(a: &DeletionApproval) -> Result<()> {
    if a.schema != "csdlc.deletion_approval.v2"
        || a.approved_by.trim().is_empty()
        || a.operator_instruction.trim().is_empty()
        || !is_lower_hex(&a.phase_b_blake3, 64)
        || !is_lower_hex(&a.phase_c_blake3, 64)
        || !is_lower_hex(&a.selector_blake3, 64)
        || !is_lower_hex(&a.manifest_blake3, 64)
        || !is_lower_hex(&a.code_revision, 40)
    {
        return Err(invalid("approval is malformed"));
    }
    parse_time(&a.approved_at)?;
    Ok(())
}
fn is_lower_hex(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn validate_relative_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path.components().any(|c| {
            matches!(
                c,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(invalid("paths must be safe repository-relative paths"));
    }
    Ok(())
}
fn read_regular_repo_file(repo: &Path, relative: &Path) -> Result<Vec<u8>> {
    validate_relative_path(relative)?;
    let path = repo.join(relative);
    let metadata = fs::symlink_metadata(&path)?;
    if !metadata.file_type().is_file() {
        return Err(invalid(format!(
            "input must be a regular file: {}",
            relative.display()
        )));
    }
    fs::read(path).map_err(Into::into)
}
fn parse_time(value: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339)
        .map_err(|e| invalid(e.to_string()))
}
fn git_text(repo: &Path, args: &[&str]) -> Result<String> {
    Ok(String::from_utf8_lossy(&git_bytes(repo, args)?)
        .trim()
        .into())
}
fn git_bytes(repo: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git").args(args).current_dir(repo).output()?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            format!("git {} failed", args.first().unwrap_or(&"command")),
        ));
    }
    Ok(output.stdout)
}
fn digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}
fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn invalid(message: impl Into<String>) -> V2Error {
    V2Error::new(ErrorCode::InvalidManifest, message)
}
fn time_error(error: time::error::Format) -> V2Error {
    V2Error::new(ErrorCode::InvalidInput, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn baseline() -> Baseline {
        Baseline {
            lines: [(PathBuf::from("a"), 45_000), (PathBuf::from("b"), 4_979)].into(),
            rust_hash: RUST_LIST_SHA256.into(),
            shell_hash: SHELL_LIST_SHA256.into(),
        }
    }
    fn fixture() -> tempfile::TempDir {
        let repo = tempfile::tempdir().unwrap();
        fs::write(
            repo.path().join("b.json"),
            include_bytes!("../../docs/architecture/csdlc-v2/gate10b/PRE_SWITCH_EVIDENCE.json"),
        )
        .unwrap();
        fs::write(
            repo.path().join("c.json"),
            include_bytes!("../../docs/architecture/csdlc-v2/gate10c/CUTOVER_EVIDENCE.json"),
        )
        .unwrap();
        fs::write(repo.path().join("s.json"), br#"{"schema":"csdlc.generation_selector.v1","default_generation":"v2","opted_in_issues":[]}"#).unwrap();
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["config", "user.email", "e@invalid"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "E"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "f"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        repo
    }
    fn request(repo: &Path) -> DeletionEligibilityRequest {
        let b = fs::read(repo.join("b.json")).unwrap();
        let c = fs::read(repo.join("c.json")).unwrap();
        let s = fs::read(repo.join("s.json")).unwrap();
        let manifest = DeletionManifest {
            schema: "csdlc.proposed_deletion_manifest.v1".into(),
            baseline_revision: BASELINE_REVISION.into(),
            target_percent: 90,
            minimum_percent: 80,
            default_disposition: EntryDisposition::Remove,
            default_owner: None,
            default_justification: None,
            entries: vec![],
        };
        let mb = serde_json::to_vec(&manifest).unwrap();
        let rev = git_text(repo, &["rev-parse", "HEAD"]).unwrap();
        DeletionEligibilityRequest {
            schema: "csdlc.deletion_eligibility_request.v1".into(),
            issue: 5306,
            phase_b_evidence: "b.json".into(),
            phase_c_evidence: "c.json".into(),
            selector: "s.json".into(),
            manifest,
            approval: Some(DeletionApproval {
                schema: "csdlc.deletion_approval.v2".into(),
                approved_by: "operator".into(),
                approved_at: "2026-08-13T00:00:00Z".into(),
                phase_b_blake3: digest(&b),
                phase_c_blake3: digest(&c),
                selector_blake3: digest(&s),
                manifest_blake3: digest(&mb),
                code_revision: rev,
                allow_qualified_80_to_89: true,
                waive_protection_windows: false,
                operator_instruction: "Complete the reviewed deletion wave.".into(),
            }),
        }
    }
    fn at(v: &str) -> OffsetDateTime {
        parse_time(v).unwrap()
    }
    #[test]
    fn missing_approval_fails_closed() {
        let r = fixture();
        let mut q = request(r.path());
        q.approval = None;
        let d =
            evaluate_with_time_and_baseline(r.path(), &q, at("2026-08-13T00:00:00Z"), &baseline())
                .unwrap();
        assert_eq!(d.reasons, vec![DeletionReason::MissingApproval]);
        assert!(!d.deletion_executed);
    }
    #[test]
    fn mandatory_phase_c_windows_fail_closed() {
        let r = fixture();
        let q = request(r.path());
        let d =
            evaluate_with_time_and_baseline(r.path(), &q, at("2026-07-20T00:00:00Z"), &baseline())
                .unwrap();
        assert!(d.reasons.contains(&DeletionReason::RollbackWindowActive));
        assert!(d.reasons.contains(&DeletionReason::ImporterWindowActive));
    }
    #[test]
    fn exact_operator_approval_can_accelerate_protection_windows() {
        let r = fixture();
        let mut q = request(r.path());
        let approval = q.approval.as_mut().unwrap();
        approval.approved_at = "2026-07-14T00:00:00Z".into();
        approval.waive_protection_windows = true;
        approval.operator_instruction =
            "Get C-SDLC v2 parity, deletion, rollback sunset, and importer sunset done tonight."
                .into();
        let d =
            evaluate_with_time_and_baseline(r.path(), &q, at("2026-07-14T01:00:00Z"), &baseline())
                .unwrap();
        assert!(d.eligible);
        assert!(!d.reasons.contains(&DeletionReason::RollbackWindowActive));
        assert!(!d.reasons.contains(&DeletionReason::ImporterWindowActive));
        assert!(!d.deletion_executed);
    }
    #[test]
    fn all_authoritative_inputs_are_approval_bound() {
        let r = fixture();
        let mut q = request(r.path());
        q.approval.as_mut().unwrap().selector_blake3 = "0".repeat(64);
        let d =
            evaluate_with_time_and_baseline(r.path(), &q, at("2026-08-13T00:00:00Z"), &baseline())
                .unwrap();
        assert!(d
            .reasons
            .contains(&DeletionReason::ApprovalEvidenceMismatch));
    }
    #[test]
    fn malformed_digest_is_rejected() {
        let r = fixture();
        let mut q = request(r.path());
        q.approval.as_mut().unwrap().phase_b_blake3 = "G".repeat(64);
        assert!(evaluate_with_time_and_baseline(
            r.path(),
            &q,
            at("2026-08-13T00:00:00Z"),
            &baseline()
        )
        .is_err());
    }
    #[test]
    fn default_partition_and_exact_overrides_derive_lines() {
        let r = fixture();
        let mut q = request(r.path());
        q.manifest.default_disposition = EntryDisposition::Retain;
        q.manifest.default_owner = Some("owner".into());
        q.manifest.default_justification = Some("useful".into());
        q.manifest.entries.push(DeletionEntry {
            path: "a".into(),
            disposition: EntryDisposition::Remove,
            owner: None,
            justification: None,
            protected_until: None,
        });
        q.approval = None;
        let d =
            evaluate_with_time_and_baseline(r.path(), &q, at("2026-08-13T00:00:00Z"), &baseline())
                .unwrap();
        assert_eq!(d.removed_lines, 45_000);
        assert_eq!(d.retained_lines, 4_979);
    }
    #[test]
    fn unknown_or_duplicate_overrides_are_invalid() {
        let r = fixture();
        let mut q = request(r.path());
        q.manifest.entries.push(DeletionEntry {
            path: "unknown".into(),
            disposition: EntryDisposition::Remove,
            owner: None,
            justification: None,
            protected_until: None,
        });
        assert!(evaluate_with_time_and_baseline(
            r.path(),
            &q,
            at("2026-08-13T00:00:00Z"),
            &baseline()
        )
        .is_err());
    }
    #[test]
    fn schema_bundle_is_versioned() {
        assert_eq!(
            eligibility_schema_bundle()["schema"],
            "csdlc.deletion_eligibility_schemas.v1"
        );
    }
}
