use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::Command;

use fs2::FileExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::{ErrorCode, Result, V2Error};
use crate::finish::{envelope_matches_record, load_cached_terminal, FinishDisposition};
use crate::git;
use crate::model::{IssueRecord, LifecyclePhase};
use crate::Store;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CleanupOperation {
    Classify,
    Remove,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CleanupStatus {
    CleanupReady,
    CleanupRemoved,
    CleanupAlreadyAbsent,
    CleanupSkippedDirty,
    CleanupSkippedMissing,
    CleanupSkippedDrift,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CleanupRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_branch: String,
    pub expected_worktree: String,
    pub operation: CleanupOperation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CleanupResult {
    pub schema: String,
    pub issue: u64,
    pub expected_branch: String,
    pub expected_worktree: String,
    pub operation: CleanupOperation,
    pub status: CleanupStatus,
    pub dirty_paths: Vec<String>,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LegacyTerminalIndexRequest {
    pub schema: String,
    pub issues: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TerminalMaterializeRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TerminalMaterializeResult {
    pub schema: String,
    pub issue: u64,
    pub phase: LifecyclePhase,
    pub generation: u64,
    pub digest: String,
    pub receipt_path: String,
    pub consumed_derived_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LegacyTerminalEntry {
    pub issue: u64,
    pub phase: LifecyclePhase,
    pub generation: u64,
    pub digest: String,
    pub receipt_present: bool,
    pub receipt_matches_projection: Option<bool>,
    pub derived_terminal_present: bool,
    pub derived_terminal_matches_projection: Option<bool>,
    pub derived_disposition: Option<FinishDisposition>,
    pub compatible: bool,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LegacyTerminalIndex {
    pub schema: String,
    pub issues: Vec<LegacyTerminalEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TerminalCensusReport {
    pub schema: String,
    pub expected_count: usize,
    pub observed_count: usize,
    pub compatible_count: usize,
    pub mismatches: Vec<String>,
    pub compatible: bool,
    pub index: LegacyTerminalIndex,
}

#[derive(Debug, Deserialize)]
struct AuditPacket {
    schema: String,
    repository: String,
    label: String,
    issues: Vec<AuditIssue>,
}

#[derive(Debug, Deserialize)]
struct UniversePacket {
    schema: String,
    repository: String,
    label: String,
    issues: Vec<UniverseIssue>,
}

#[derive(Debug, Deserialize)]
struct UniverseIssue {
    number: u64,
}

#[derive(Debug, Deserialize)]
struct AuditIssue {
    number: u64,
    terminal: AuditTerminal,
}

#[derive(Debug, Deserialize)]
struct AuditTerminal {
    phase: String,
    disposition: String,
    pull_request: Option<u64>,
    observed_sha: Option<String>,
    observed_state: String,
}

pub fn cleanup_schema_bundle() -> Value {
    json!({
        "schema": "csdlc.cleanup_schema_bundle.v1",
        "cleanup_request": schemars::schema_for!(CleanupRequest),
        "cleanup_result": schemars::schema_for!(CleanupResult),
        "legacy_terminal_index_request": schemars::schema_for!(LegacyTerminalIndexRequest),
        "legacy_terminal_index": schemars::schema_for!(LegacyTerminalIndex),
        "terminal_materialize_request": schemars::schema_for!(TerminalMaterializeRequest),
        "terminal_materialize_result": schemars::schema_for!(TerminalMaterializeResult),
        "terminal_census_report": schemars::schema_for!(TerminalCensusReport),
    })
}

pub fn execute_cleanup(root: &Path, request: &CleanupRequest) -> Result<CleanupResult> {
    validate_cleanup_request(request)?;
    let common = canonical_real_directory(&PathBuf::from(
        git::run(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    ))?;
    let _lock = cleanup_lock(&common, request.issue)?;
    let worktrees = git::worktrees(root)?;
    let expected_path = PathBuf::from(&request.expected_worktree);
    if !expected_path.is_absolute() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "expected_worktree must be an absolute path",
        ));
    }
    let expected = expected_path.to_string_lossy().into_owned();
    let exact = worktrees
        .iter()
        .find(|(branch, path)| branch == &request.expected_branch && path == &expected);
    if exact.is_none() {
        let drift = worktrees
            .iter()
            .any(|(branch, path)| branch == &request.expected_branch || path == &expected);
        return Ok(cleanup_result(
            request,
            if drift {
                CleanupStatus::CleanupSkippedDrift
            } else if request.operation == CleanupOperation::Remove {
                CleanupStatus::CleanupAlreadyAbsent
            } else {
                CleanupStatus::CleanupSkippedMissing
            },
            Vec::new(),
            vec![if drift {
                "registered worktree branch or path differs from the exact request".into()
            } else {
                "the exact worktree is not registered".into()
            }],
        ));
    }

    let metadata = match fs::symlink_metadata(&expected_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(cleanup_result(
                request,
                CleanupStatus::CleanupSkippedMissing,
                Vec::new(),
                vec!["the registered worktree directory is missing".into()],
            ));
        }
        Err(error) => return Err(error.into()),
    };
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Ok(cleanup_result(
            request,
            CleanupStatus::CleanupSkippedDrift,
            Vec::new(),
            vec!["the registered worktree path is not a real directory".into()],
        ));
    }
    let canonical_expected = fs::canonicalize(&expected_path)?;
    if canonical_expected != expected_path {
        return Ok(cleanup_result(
            request,
            CleanupStatus::CleanupSkippedDrift,
            Vec::new(),
            vec![
                "the registered worktree path contains a symlink or non-canonical component".into(),
            ],
        ));
    }
    let primary = fs::canonicalize(primary_worktree_root(&common)?)?;
    if canonical_expected == primary {
        return Ok(cleanup_result(
            request,
            CleanupStatus::CleanupSkippedDrift,
            Vec::new(),
            vec!["the primary checkout can never be removed".into()],
        ));
    }
    if let Err(error) = validate_issue_projection(&expected_path, request.issue) {
        return Ok(cleanup_result(
            request,
            CleanupStatus::CleanupSkippedDrift,
            Vec::new(),
            vec![format!(
                "issue projection identity is unsafe: {}",
                error.message
            )],
        ));
    }
    let dirty_paths = dirty_paths(&expected_path)?;
    if !dirty_paths.is_empty() {
        return Ok(cleanup_result(
            request,
            CleanupStatus::CleanupSkippedDirty,
            dirty_paths,
            vec!["tracked or untracked worktree content is present".into()],
        ));
    }
    if request.operation == CleanupOperation::Classify {
        return Ok(cleanup_result(
            request,
            CleanupStatus::CleanupReady,
            Vec::new(),
            Vec::new(),
        ));
    }

    let output = Command::new("git")
        .current_dir(&primary)
        .args(["worktree", "remove"])
        .arg(&expected_path)
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            format!(
                "non-forced worktree removal failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        ));
    }
    if git::worktrees(&primary)?
        .iter()
        .any(|(branch, path)| branch == &request.expected_branch || path == &expected)
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "worktree removal returned success but the registration remains",
        ));
    }
    Ok(cleanup_result(
        request,
        CleanupStatus::CleanupRemoved,
        Vec::new(),
        Vec::new(),
    ))
}

pub fn materialize_terminal(
    root: &Path,
    request: &TerminalMaterializeRequest,
) -> Result<TerminalMaterializeResult> {
    if request.schema != "csdlc.terminal_materialize_request.v1"
        || request.issue == 0
        || request.expected_generation == 0
        || request.expected_digest.trim().is_empty()
        || request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "terminal materialize request identity, expected projection, actor, and reason are required",
        ));
    }
    let envelope = load_cached_terminal(root, request.issue)?.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "terminal materialization requires a retained derived terminal envelope",
        )
    })?;
    let store = Store::new(root);
    let record = store.materialize_terminal_from_derived(
        request.issue,
        request.expected_generation,
        &request.expected_digest,
        &request.actor,
        &request.reason,
        &envelope,
    )?;
    let receipt_path = record
        .terminal
        .as_ref()
        .map(|terminal| terminal.receipt_path.clone())
        .unwrap_or_default();
    Ok(TerminalMaterializeResult {
        schema: "csdlc.terminal_materialize_result.v1".into(),
        issue: request.issue,
        phase: record.phase,
        generation: record.generation,
        digest: record.digest,
        receipt_path,
        consumed_derived_digest: envelope.digest,
    })
}

pub fn build_legacy_terminal_index(
    root: &Path,
    request: &LegacyTerminalIndexRequest,
) -> Result<LegacyTerminalIndex> {
    if request.schema != "csdlc.legacy_terminal_index_request.v1"
        || request.issues.is_empty()
        || request.issues.contains(&0)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "legacy terminal index request is invalid",
        ));
    }
    let unique = request.issues.iter().copied().collect::<BTreeSet<_>>();
    if unique.len() != request.issues.len() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "legacy terminal index request contains duplicate issues",
        ));
    }
    let store = Store::new(root);
    let mut issues = Vec::with_capacity(unique.len());
    for issue in unique {
        validate_projection_layout(root, issue)?;
        let record = store.load_record(issue)?;
        let cards = store.load_cards(issue)?;
        let receipt = store.load_terminal_receipt(issue)?;
        let receipt_matches_projection = receipt
            .as_ref()
            .map(|value| value.record == record && value.cards == cards);
        let derived = load_cached_terminal(root, issue)?;
        let derived_terminal_matches_projection = match derived.as_ref() {
            Some(value) => {
                let direct = envelope_matches_record(value, &record)?;
                Some(
                    direct
                        || derived_consumed_by_materialized_projection(
                            value,
                            &record,
                            receipt_matches_projection == Some(true),
                        ),
                )
            }
            None => None,
        };
        let mut diagnostics = Vec::new();
        if receipt_matches_projection == Some(false) {
            diagnostics.push("retained receipt does not match the tracked projection".into());
        }
        if derived_terminal_matches_projection == Some(false) {
            diagnostics
                .push("derived terminal envelope does not match the tracked projection".into());
        }
        let compatible = record.phase == LifecyclePhase::ClosedOut
            && receipt_matches_projection.unwrap_or(true)
            && derived_terminal_matches_projection.unwrap_or(true);
        if record.phase != LifecyclePhase::ClosedOut {
            diagnostics.push("tracked projection is not closed_out".into());
        }
        issues.push(LegacyTerminalEntry {
            issue,
            phase: record.phase,
            generation: record.generation,
            digest: record.digest,
            receipt_present: receipt.is_some(),
            receipt_matches_projection,
            derived_terminal_present: derived.is_some(),
            derived_terminal_matches_projection,
            derived_disposition: derived.as_ref().map(|value| value.disposition),
            compatible,
            diagnostics,
        });
    }
    Ok(LegacyTerminalIndex {
        schema: "csdlc.legacy_terminal_index.v1".into(),
        issues,
    })
}

fn derived_consumed_by_materialized_projection(
    envelope: &crate::finish::DerivedTerminalEnvelope,
    record: &IssueRecord,
    receipt_matches_projection: bool,
) -> bool {
    if !receipt_matches_projection
        || record.phase != LifecyclePhase::ClosedOut
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
        FinishDisposition::Merged => crate::readiness::TerminalDisposition::Merged,
        FinishDisposition::ClosedUnmerged => crate::readiness::TerminalDisposition::ClosedUnmerged,
        FinishDisposition::ClosedNoPr => crate::readiness::TerminalDisposition::ClosedNoPr,
    };
    terminal.disposition == disposition
        && terminal.pull_request == envelope.pull_request
        && terminal.observed_sha == envelope.head_sha
}

pub fn validate_terminal_census(root: &Path, audit_path: &Path) -> Result<TerminalCensusReport> {
    require_real_file(audit_path, "terminal audit packet")?;
    let audit: AuditPacket = serde_json::from_slice(&fs::read(audit_path)?)?;
    validate_census_identity(
        &audit.schema,
        &audit.repository,
        &audit.label,
        "adl.v0918.remote_terminal_audit.v1",
    )?;
    let universe_path =
        fs::canonicalize(root)?.join(".csdlc/evidence/5748/v0918-closed-issue-universe.json");
    require_real_file(&universe_path, "closed issue universe packet")?;
    let universe: UniversePacket = serde_json::from_slice(&fs::read(universe_path)?)?;
    validate_census_identity(
        &universe.schema,
        &universe.repository,
        &universe.label,
        "adl.v0918.closed_issue_universe.v1",
    )?;
    let audit_set = audit
        .issues
        .iter()
        .map(|entry| entry.number)
        .collect::<BTreeSet<_>>();
    let universe_set = universe
        .issues
        .iter()
        .map(|entry| entry.number)
        .collect::<BTreeSet<_>>();
    if audit.issues.len() != 114
        || universe.issues.len() != 114
        || audit_set.len() != audit.issues.len()
        || universe_set.len() != universe.issues.len()
        || audit_set != universe_set
    {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "v0.91.8 audit and closed-issue universe must contain the same 114 unique issues",
        ));
    }
    let request = LegacyTerminalIndexRequest {
        schema: "csdlc.legacy_terminal_index_request.v1".into(),
        issues: audit.issues.iter().map(|entry| entry.number).collect(),
    };
    let index = build_legacy_terminal_index(root, &request)?;
    let indexed = index
        .issues
        .iter()
        .map(|entry| (entry.issue, entry))
        .collect::<BTreeMap<_, _>>();
    let store = Store::new(root);
    let mut mismatches = Vec::new();
    for expected in &audit.issues {
        let Some(actual) = indexed.get(&expected.number) else {
            mismatches.push(format!(
                "issue {} is absent from the compatibility index",
                expected.number
            ));
            continue;
        };
        let record = store.load_record(expected.number)?;
        let terminal = record.terminal.as_ref();
        let disposition = terminal
            .map(|value| value.disposition.to_string())
            .unwrap_or_default();
        let matches = actual.compatible
            && expected.terminal.phase == "closed_out"
            && actual.phase == LifecyclePhase::ClosedOut
            && terminal.is_some_and(|value| {
                disposition == expected.terminal.disposition
                    && value.pull_request == expected.terminal.pull_request
                    && value.observed_sha == expected.terminal.observed_sha
                    && value.observed_state == expected.terminal.observed_state
            });
        if !matches {
            mismatches.push(format!(
                "issue {} differs from the v0.91.8 terminal census",
                expected.number
            ));
        }
    }
    let compatible_count = index.issues.iter().filter(|entry| entry.compatible).count();
    Ok(TerminalCensusReport {
        schema: "csdlc.terminal_census_report.v1".into(),
        expected_count: universe.issues.len(),
        observed_count: index.issues.len(),
        compatible_count,
        compatible: mismatches.is_empty()
            && compatible_count == audit.issues.len()
            && index.issues.len() == audit.issues.len(),
        mismatches,
        index,
    })
}

fn validate_cleanup_request(request: &CleanupRequest) -> Result<()> {
    if request.schema != "csdlc.cleanup_request.v1"
        || request.issue == 0
        || request.expected_branch.trim().is_empty()
        || request.expected_worktree.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "cleanup request identity is incomplete",
        ));
    }
    Ok(())
}

fn cleanup_result(
    request: &CleanupRequest,
    status: CleanupStatus,
    dirty_paths: Vec<String>,
    diagnostics: Vec<String>,
) -> CleanupResult {
    CleanupResult {
        schema: "csdlc.cleanup_result.v1".into(),
        issue: request.issue,
        expected_branch: request.expected_branch.clone(),
        expected_worktree: request.expected_worktree.clone(),
        operation: request.operation,
        status,
        dirty_paths,
        diagnostics,
    }
}

fn cleanup_lock(common: &Path, issue: u64) -> Result<File> {
    let csdlc = ensure_real_directory(common, "csdlc-v2")?;
    let directory = ensure_real_directory(&csdlc, "cleanup")?;
    let path = directory.join(format!("{issue}.lock"));
    if fs::symlink_metadata(&path)
        .is_ok_and(|metadata| metadata.file_type().is_symlink() || !metadata.file_type().is_file())
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "cleanup lock is not a regular file",
        ));
    }
    let mut options = OpenOptions::new();
    options.create(true).truncate(false).read(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    let lock = options.open(path)?;
    lock.lock_exclusive()?;
    Ok(lock)
}

fn primary_worktree_root(common: &Path) -> Result<PathBuf> {
    common.parent().map(Path::to_path_buf).ok_or_else(|| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            "Git-common directory has no primary checkout parent",
        )
    })
}

fn validate_issue_projection(worktree: &Path, issue: u64) -> Result<IssueRecord> {
    let issue_dir = projection_directory(worktree, issue)?;
    let path = issue_dir.join("index.json");
    require_real_file(&path, "issue projection")?;
    let record: IssueRecord = serde_json::from_slice(&fs::read(path)?)?;
    if record.issue != issue {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "issue projection does not match the cleanup request",
        ));
    }
    Ok(record)
}

fn validate_projection_layout(root: &Path, issue: u64) -> Result<()> {
    let issue_dir = projection_directory(root, issue)?;
    require_real_file(&issue_dir.join("index.json"), "issue projection")?;
    let cards = require_real_directory(&issue_dir.join("cards"), "cards directory")?;
    for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        require_real_file(
            &cards.join(format!("{kind}.values.json")),
            "card values projection",
        )?;
    }
    Ok(())
}

fn projection_directory(root: &Path, issue: u64) -> Result<PathBuf> {
    let root = fs::canonicalize(root)?;
    require_real_directory(&root, "repository root")?;
    let csdlc = require_real_directory(&root.join(".csdlc"), "C-SDLC directory")?;
    let issues = require_real_directory(&csdlc.join("issues"), "issues directory")?;
    require_real_directory(&issues.join(issue.to_string()), "issue directory")
}

fn canonical_real_directory(path: &Path) -> Result<PathBuf> {
    let canonical = fs::canonicalize(path)?;
    if canonical != path {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "directory path is not canonical and symlink-free: {}",
                path.display()
            ),
        ));
    }
    require_real_directory(&canonical, "canonical directory")
}

fn require_real_directory(path: &Path, label: &str) -> Result<PathBuf> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!("{label} is unavailable at {}: {error}", path.display()),
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!("{label} is not a real directory: {}", path.display()),
        ));
    }
    Ok(path.to_path_buf())
}

fn ensure_real_directory(parent: &Path, name: &str) -> Result<PathBuf> {
    let path = parent.join(name);
    match fs::symlink_metadata(&path) {
        Ok(_) => require_real_directory(&path, "cleanup lock directory"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if let Err(error) = fs::create_dir(&path) {
                if error.kind() != std::io::ErrorKind::AlreadyExists {
                    return Err(error.into());
                }
            }
            require_real_directory(&path, "cleanup lock directory")
        }
        Err(error) => Err(error.into()),
    }
}

fn require_real_file(path: &Path, label: &str) -> Result<()> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!("{label} is unavailable at {}: {error}", path.display()),
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!("{label} is not a real file: {}", path.display()),
        ));
    }
    Ok(())
}

fn validate_census_identity(
    schema: &str,
    repository: &str,
    label: &str,
    expected_schema: &str,
) -> Result<()> {
    if schema != expected_schema
        || repository != "danielbaustin/agent-design-language"
        || label != "version:v0.91.8"
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "v0.91.8 census packet identity is invalid",
        ));
    }
    Ok(())
}

fn dirty_paths(worktree: &Path) -> Result<Vec<String>> {
    let output = Command::new("git")
        .current_dir(worktree)
        .args(["status", "--porcelain=v1", "-z", "--untracked-files=all"])
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            String::from_utf8_lossy(&output.stderr).trim(),
        ));
    }
    let mut paths = Vec::new();
    let mut rename_source = false;
    for raw in output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|raw| !raw.is_empty())
    {
        let value = std::str::from_utf8(raw).map_err(|_| {
            V2Error::new(
                ErrorCode::UnsafeCheckout,
                "worktree contains a non-UTF-8 dirty path",
            )
        })?;
        if rename_source {
            paths.push(value.to_owned());
            rename_source = false;
            continue;
        }
        if value.len() < 4 || value.as_bytes()[2] != b' ' {
            return Err(V2Error::new(
                ErrorCode::GitFailure,
                "unexpected porcelain status record",
            ));
        }
        let status = &value.as_bytes()[..2];
        paths.push(value[3..].to_owned());
        rename_source = status.contains(&b'R') || status.contains(&b'C');
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
}
