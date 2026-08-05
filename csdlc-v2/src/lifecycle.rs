use std::fs;
use std::path::{Component, Path, PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{ErrorCode, Result, V2Error};
use crate::git;
use crate::model::{AuditEvent, Claim, ClaimRecovery};
use crate::store::{bootstrap_issue, validate_bootstrap_request, BootstrapRequest, Store};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BindRequest {
    pub issue: u64,
    pub base_branch: String,
    pub branch: String,
    pub worktree: String,
    pub claim: Claim,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BindResult {
    pub created: bool,
    pub branch: String,
    pub worktree: String,
    pub claim_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RecoverClaimRequest {
    pub issue: u64,
    pub expected_claim_id: String,
    pub expected_generation: u64,
    pub now_unix_seconds: u64,
    pub replacement: Claim,
    pub recovery_actor: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReacquireClaimRequest {
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub now_unix_seconds: u64,
    pub actor: String,
    pub reason: String,
    pub replacement: Claim,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReacquireClaimResult {
    pub schema: String,
    pub issue: u64,
    pub claim: Claim,
    pub previous_claim_id: Option<String>,
    pub previous_owner: Option<String>,
    pub phase: crate::LifecyclePhase,
    pub generation: u64,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RehomeClaimAuthorityRequest {
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub expected_initialization_digest: String,
    pub source_worktree: String,
    pub source_branch: String,
    pub source_commit: String,
    pub expected_source_generation: u64,
    pub expected_source_digest: String,
    pub now_unix_seconds: u64,
    pub current_session_id: String,
    pub session_ledger_path: String,
    pub actor: String,
    pub operator_authority: String,
    pub reason: String,
    pub replacement: Claim,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RehomeClaimAuthorityResult {
    pub schema: String,
    pub issue: u64,
    pub source_commit: String,
    pub initialization_digest: String,
    pub preserved_bindings: Vec<String>,
    pub claim: Claim,
    pub generation: u64,
    pub digest: String,
}

#[derive(Debug, Deserialize)]
struct SessionLedgerView {
    schema: String,
    #[serde(default)]
    claims: Vec<SessionClaimView>,
}

#[derive(Debug, Deserialize)]
struct SessionClaimView {
    session_id: String,
    mode: String,
    expires_at: String,
    #[serde(default)]
    released_at: Option<String>,
    #[serde(default)]
    github: SessionGithubView,
}

#[derive(Debug, Default, Deserialize)]
struct SessionGithubView {
    issue: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReleaseClosedClaimRequest {
    pub issue: u64,
    pub repository: String,
    pub expected_claim_id: String,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub reason: String,
    pub observed_issue_state: String,
    pub observed_issue: u64,
    pub observation_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RevokeActiveClaimRequest {
    pub issue: u64,
    pub repository: String,
    pub expected_claim_id: String,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub now_unix_seconds: u64,
    pub actor: String,
    pub operator_authority: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RevokeActiveClaimResult {
    pub schema: String,
    pub issue: u64,
    pub claim_id: String,
    pub previous_owner: String,
    pub actor: String,
    pub operator_authority: String,
    pub reason: String,
    pub generation: u64,
    pub digest: String,
    pub released: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HeartbeatRequest {
    pub issue: u64,
    pub claim_id: String,
    pub expected_generation: u64,
    pub now_unix_seconds: u64,
    pub extend_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AmendClaimScopeRequest {
    pub issue: u64,
    pub claim_id: String,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub now_unix_seconds: u64,
    pub actor: String,
    pub reason: String,
    pub add_protected_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TransitionActiveClaimRequest {
    pub issue: u64,
    pub claim_id: String,
    pub expected_owner: String,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub now_unix_seconds: u64,
    pub actor: String,
    pub reason: String,
    pub expected_purpose: String,
    pub purpose: String,
    pub add_protected_paths: Vec<String>,
}

fn clean_relative(value: &str) -> bool {
    !value.is_empty()
        && Path::new(value)
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}

fn overlaps(left: &str, right: &str) -> bool {
    let left = Path::new(left);
    let right = Path::new(right);
    left == right || left.starts_with(right) || right.starts_with(left)
}

fn terminal_projection_overlap_is_released(
    store: &Store,
    observed_store: &Store,
    local: &crate::IssueRecord,
    reserved: &str,
    candidate: &str,
    now_unix_seconds: u64,
) -> Result<bool> {
    let cached_terminal = crate::finish::load_cached_terminal(store.root(), local.issue)?;
    let immutable_terminal = if let Some(terminal) = &cached_terminal {
        terminal.disposition == crate::finish::FinishDisposition::Merged
            && crate::finish::envelope_releases_claim(
                observed_store.root(),
                terminal,
                local,
                now_unix_seconds,
            )?
    } else {
        false
    };
    if immutable_terminal {
        return Ok(true);
    }
    let issue_path = format!(".csdlc/issues/{}", local.issue);
    let exact_issue_projection = reserved.trim_end_matches('/') == issue_path
        && candidate.trim_end_matches('/') == issue_path;
    let Some(claim) = local.claim.as_ref() else {
        return Ok(false);
    };
    if claim.expires_unix_seconds > now_unix_seconds
        && claim_matches_active_checkout(observed_store, claim)?
    {
        return Ok(false);
    }
    if let Some(terminal) = &cached_terminal {
        if crate::finish::envelope_releases_claim(
            observed_store.root(),
            terminal,
            local,
            now_unix_seconds,
        )? {
            return Ok(true);
        }
    }
    if exact_issue_projection && store.has_claim_free_retained_terminal_authority(local)? {
        return Ok(true);
    }
    if !exact_expired_terminal_projection_overlap(
        local.issue,
        claim.expires_unix_seconds,
        reserved,
        candidate,
        now_unix_seconds,
    ) {
        return Ok(false);
    }
    store.has_claim_free_terminal_authority(
        local.issue,
        &local.repository,
        &local.initialization_digest,
    )
}

fn exact_expired_terminal_projection_overlap(
    issue: u64,
    expires_unix_seconds: u64,
    reserved: &str,
    candidate: &str,
    now_unix_seconds: u64,
) -> bool {
    let issue_path = format!(".csdlc/issues/{issue}");
    expires_unix_seconds <= now_unix_seconds
        && reserved.trim_end_matches('/') == issue_path
        && candidate.trim_end_matches('/') == issue_path
}

fn active_issue_records_across_worktrees(
    store: &Store,
) -> Result<Vec<(Store, crate::IssueRecord)>> {
    let mut roots = std::collections::BTreeMap::new();
    for (branch, root) in git::worktrees(store.root())? {
        roots.insert(PathBuf::from(root).canonicalize()?, branch);
    }
    let current_root = store.root().canonicalize()?;
    if let std::collections::btree_map::Entry::Vacant(entry) = roots.entry(current_root) {
        entry.insert(git::current_branch(store.root())?);
    }
    let mut records = Vec::new();
    for (root, branch) in roots {
        let scoped = Store::new(root);
        let issues = scoped.root().join(".csdlc/issues");
        if !issues.exists() {
            continue;
        }
        for entry in fs::read_dir(issues)? {
            let entry = entry?;
            let Some(issue) = entry
                .file_name()
                .to_str()
                .and_then(|name| name.parse().ok())
            else {
                continue;
            };
            let record = scoped.load_record(issue)?;
            let Some(claim) = record.claim.as_ref() else {
                continue;
            };
            if claim.branch != branch || !claim_worktree_matches_root(&scoped, claim)? {
                continue;
            }
            records.push((scoped.clone(), record));
        }
    }
    Ok(records)
}

fn claim_worktree_matches_root(store: &Store, claim: &Claim) -> Result<bool> {
    if claim.worktree == "." {
        return Ok(true);
    }
    let common_dir = PathBuf::from(
        git::run(
            store.root(),
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    );
    Ok(common_dir
        .parent()
        .map(|primary| primary.join(&claim.worktree))
        .and_then(|expected| expected.canonicalize().ok())
        .zip(store.root().canonicalize().ok())
        .is_some_and(|(expected, current)| expected == current))
}

fn claim_matches_active_checkout(store: &Store, claim: &Claim) -> Result<bool> {
    Ok(git::current_branch(store.root())? == claim.branch
        && claim_worktree_matches_root(store, claim)?)
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    let Ok(source_metadata) = fs::symlink_metadata(source) else {
        return Ok(());
    };
    if source_metadata.file_type().is_symlink() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "bound lifecycle materialization refuses symlinked source state",
        ));
    }
    if !source_metadata.is_dir() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "bound lifecycle materialization source must be a directory",
        ));
    }
    if fs::symlink_metadata(destination).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "bound lifecycle materialization refuses symlinked target state",
        ));
    }
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let metadata = fs::symlink_metadata(&source_path)?;
        if metadata.file_type().is_symlink() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "bound lifecycle materialization refuses symlinked source entries",
            ));
        }
        if fs::symlink_metadata(&destination_path)
            .is_ok_and(|target| target.file_type().is_symlink())
        {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "bound lifecycle materialization refuses symlinked target entries",
            ));
        }
        if metadata.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else {
            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source_path, &destination_path)?;
        }
    }
    Ok(())
}

fn directory_matches_recursive(source: &Path, destination: &Path) -> Result<bool> {
    let source_metadata = match fs::symlink_metadata(source) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return match fs::symlink_metadata(destination) {
                Ok(metadata) if metadata.file_type().is_symlink() => Err(V2Error::new(
                    ErrorCode::UnsafeCheckout,
                    "bound lifecycle materialization refuses symlinked state",
                )),
                Ok(metadata) if metadata.is_dir() => {
                    Ok(fs::read_dir(destination)?.next().is_none())
                }
                Ok(_) => Ok(false),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(true),
                Err(error) => Err(error.into()),
            };
        }
        Err(error) => return Err(error.into()),
    };
    let destination_metadata = match fs::symlink_metadata(destination) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.into()),
    };
    if source_metadata.file_type().is_symlink() || destination_metadata.file_type().is_symlink() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "bound lifecycle materialization refuses symlinked state",
        ));
    }
    if source_metadata.is_dir() != destination_metadata.is_dir() {
        return Ok(false);
    }
    if source_metadata.is_dir() {
        let mut source_entries = std::collections::BTreeSet::new();
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            source_entries.insert(entry.file_name());
        }
        let mut destination_entries = std::collections::BTreeSet::new();
        for entry in fs::read_dir(destination)? {
            let entry = entry?;
            destination_entries.insert(entry.file_name());
        }
        if source_entries != destination_entries {
            return Ok(false);
        }
        for entry in source_entries {
            if !directory_matches_recursive(&source.join(&entry), &destination.join(&entry))? {
                return Ok(false);
            }
        }
        return Ok(true);
    }
    Ok(fs::read(source)? == fs::read(destination)?)
}

fn require_matching_tree(source: &Path, destination: &Path, message: &str) -> Result<()> {
    if destination.exists() && !directory_matches_recursive(source, destination)? {
        return Err(V2Error::new(ErrorCode::ReconciliationRequired, message));
    }
    Ok(())
}

fn materialize_bound_issue_state(source: &Store, target_root: &Path, issue: u64) -> Result<Store> {
    let target = Store::new(target_root.to_path_buf());
    if source.root().canonicalize()? == target.root().canonicalize()? {
        return Ok(target);
    }

    let source_record = source.load_record(issue)?;
    let target_issue_dir = target.issue_dir(issue);
    if target_issue_dir.exists() {
        let target_record = target.load_record(issue)?;
        if target_record.issue != issue
            || target_record.repository != source_record.repository
            || target_record.initialization_digest != source_record.initialization_digest
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "bound worktree already contains different issue lifecycle state",
            ));
        }
        require_matching_tree(
            &source.issue_dir(issue),
            &target_issue_dir,
            "bound worktree already contains stale issue lifecycle state",
        )?;
    } else {
        copy_dir_recursive(&source.issue_dir(issue), &target_issue_dir)?;
    }

    let source_prepared = source
        .root()
        .join(".csdlc/prepared/issues")
        .join(issue.to_string());
    let target_prepared = target
        .root()
        .join(".csdlc/prepared/issues")
        .join(issue.to_string());
    require_matching_tree(
        &source_prepared,
        &target_prepared,
        "bound worktree already contains different prepared lifecycle state",
    )?;
    copy_dir_recursive(&source_prepared, &target_prepared)?;
    let source_preparation = source
        .root()
        .join(".csdlc/preparation/issues")
        .join(issue.to_string());
    let target_preparation = target
        .root()
        .join(".csdlc/preparation/issues")
        .join(issue.to_string());
    require_matching_tree(
        &source_preparation,
        &target_preparation,
        "bound worktree already contains different claim-free preparation state",
    )?;
    copy_dir_recursive(&source_preparation, &target_preparation)?;
    let source_evidence = source
        .root()
        .join(".csdlc/evidence")
        .join(issue.to_string());
    let target_evidence = target
        .root()
        .join(".csdlc/evidence")
        .join(issue.to_string());
    require_matching_tree(
        &source_evidence,
        &target_evidence,
        "bound worktree already contains different evidence lifecycle state",
    )?;
    copy_dir_recursive(&source_evidence, &target_evidence)?;
    fs::create_dir_all(&target_evidence)?;
    fs::create_dir_all(target.root().join(".csdlc/locks"))?;
    Ok(target)
}

fn initialize_issue_under_binding_lock(
    store: &Store,
    mut request: BootstrapRequest,
) -> Result<crate::IssueRecord> {
    if !clean_relative(&request.design_path)
        || !clean_relative(&request.diagram_path)
        || request.design_path == request.diagram_path
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design and diagram paths must be distinct and repository-relative",
        ));
    }
    validate_bootstrap_request(&request)?;
    validate_validation_lanes(store.root(), &request.initial.validation_lanes)?;
    let issue_dir = store.issue_dir(request.issue);
    for authored_path in [&request.design_path, &request.diagram_path] {
        let path = store.root().join(authored_path);
        if path == issue_dir.join("index.json")
            || path == issue_dir.join("audit.jsonl")
            || path.starts_with(issue_dir.join("cards"))
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "design and diagram paths cannot target issue control files",
            ));
        }
    }
    let now_unix_seconds = unix_now()?;
    for (other_store, other) in active_issue_records_across_worktrees(store)? {
        if other.issue != request.issue {
            if let Some(claim) = other.claim.as_ref() {
                if let Some((reserved, requested)) = claim.protected_paths.iter().find_map(|a| {
                    request
                        .claim
                        .protected_paths
                        .iter()
                        .find(|b| overlaps(a, b))
                        .map(|b| (a, b))
                }) {
                    if terminal_projection_overlap_is_released(
                        store,
                        &other_store,
                        &other,
                        reserved,
                        requested,
                        now_unix_seconds,
                    )? {
                        continue;
                    }
                    return Err(V2Error::new(
                        ErrorCode::ClaimCollision,
                        format!(
                            "protected path '{}' overlaps requested '{}' from issue {} in phase {:?}",
                            reserved, requested, other.issue, other.phase
                        ),
                    ));
                }
            }
        }
    }
    let design = store.root().join(&request.design_path);
    let diagram = store.root().join(&request.diagram_path);
    if !design.exists() {
        if let Some(parent) = design.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(
            &design,
            format!(
                "# Issue {} design\n\nStatus: design required before Ready.\n",
                request.issue
            ),
        )?;
        request.design_approved = false;
    }
    if !diagram.exists() {
        if let Some(parent) = diagram.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(
            &diagram,
            format!(
                "flowchart LR\n  I[\"Issue {}\"] --> D[\"Design required\"]\n",
                request.issue
            ),
        )?;
    }
    bootstrap_issue(store, request)
}

pub(crate) fn initialize_issue(
    store: &Store,
    request: BootstrapRequest,
) -> Result<crate::IssueRecord> {
    let _binding_lock = store.binding_lock()?;
    initialize_issue_under_binding_lock(store, request)
}

pub(crate) fn initialize_prepared_issue_under_binding_lock(
    store: &Store,
    request: BootstrapRequest,
) -> Result<crate::IssueRecord> {
    crate::registry::validate_native_registry(store.root())?;
    initialize_issue_under_binding_lock(store, request)
}

fn initialize_native_issue(store: &Store, request: BootstrapRequest) -> Result<crate::IssueRecord> {
    crate::registry::validate_native_registry(store.root())?;
    initialize_issue(store, request)
}

/// The sole public native initialization entrypoint validates raw field presence.
///
/// Bypassing that proof through a typed request is intentionally not public:
///
/// ```compile_fail
/// let _ = csdlc_v2::initialize_issue;
/// let _ = csdlc_v2::initialize_native_issue;
/// let _ = csdlc_v2::bootstrap_issue;
/// ```
pub fn initialize_native_json(store: &Store, bytes: &[u8]) -> Result<crate::IssueRecord> {
    let value: serde_json::Value = serde_json::from_slice(bytes)?;
    let initial = value
        .get("initial")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "native initial input is missing"))?;
    if !initial.contains_key("operator_constraints") || !initial.contains_key("review_scope") {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "native bootstrap requires explicit operator_constraints and review_scope",
        ));
    }
    let request: BootstrapRequest = serde_json::from_value(value)?;
    initialize_native_issue(store, request)
}

pub(crate) fn validate_validation_lanes(
    root: &std::path::Path,
    lanes: &[crate::cards::ValidationLane],
) -> Result<()> {
    for lane in lanes {
        let executable = lane.argv.first().ok_or_else(|| {
            V2Error::new(
                ErrorCode::InvalidInput,
                format!("validation lane {} has no executable", lane.lane),
            )
        })?;
        let executable_exists = if executable.contains('/') {
            let path = if Path::new(executable).is_absolute() {
                Path::new(executable).to_path_buf()
            } else {
                root.join(executable)
            };
            path.is_file()
        } else {
            std::env::var_os("PATH").is_some_and(|path| {
                std::env::split_paths(&path).any(|directory| directory.join(executable).is_file())
            })
        };
        if !executable_exists {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                format!(
                    "validation lane {} names unavailable executable {}",
                    lane.lane, executable
                ),
            ));
        }
        for command in lane.argv.iter().skip(1) {
            if !command.contains('/') {
                continue;
            }
            let path = if Path::new(command).is_absolute() {
                Path::new(command).to_path_buf()
            } else {
                root.join(command)
            };
            if !path.is_file() {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    format!(
                        "validation lane {} names missing command {}",
                        lane.lane, command
                    ),
                ));
            }
        }
    }
    Ok(())
}

pub fn bind_issue(store: &Store, request: BindRequest) -> Result<BindResult> {
    let reserved_worktree = Path::new(&request.worktree);
    let current_path_matches = request.worktree == "."
        || (|| {
            let current_root = store.root().canonicalize().ok()?;
            let common_dir = PathBuf::from(
                git::run(
                    store.root(),
                    &["rev-parse", "--path-format=absolute", "--git-common-dir"],
                )
                .ok()?
                .stdout,
            )
            .canonicalize()
            .ok()?;
            let primary_root = common_dir.parent()?;
            primary_root
                .join(reserved_worktree)
                .canonicalize()
                .ok()
                .is_some_and(|reserved| reserved == current_root)
                .then_some(())
        })()
        .is_some();
    let issue_local = current_path_matches
        && git::current_branch(store.root()).is_ok_and(|branch| branch == request.branch);
    if request.branch == "main"
        || request.branch == request.base_branch
        || request.claim.branch != request.branch
        || request.claim.worktree != request.worktree
        || (request.worktree != "." && !clean_relative(&request.worktree))
        || request
            .claim
            .protected_paths
            .iter()
            .any(|path| !clean_relative(path))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "unsafe branch, worktree, or protected path",
        ));
    }
    let _binding_lock = store.binding_lock()?;
    if !issue_local && git::current_branch(store.root())? != request.base_branch {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "primary checkout is not on the declared base branch",
        ));
    }
    let wanted = if issue_local {
        store.root().to_path_buf()
    } else {
        store.root().join(&request.worktree)
    };
    let wanted_compare = if wanted.exists() {
        fs::canonicalize(&wanted)?
    } else {
        wanted.clone()
    };
    let wanted_text = wanted_compare.to_string_lossy();
    let listed = git::worktrees(store.root())?;
    let listed_for_wanted = listed.iter().find(|(_, path)| path == &wanted_text);
    if let Some((branch, _)) = listed_for_wanted {
        if branch != &request.branch {
            return Err(V2Error::new(
                ErrorCode::ClaimCollision,
                "worktree is bound to a different branch",
            ));
        }
    } else if !issue_local && wanted.exists() {
        return Err(V2Error::new(
            ErrorCode::ClaimCollision,
            "requested worktree path exists but is not a registered Git worktree",
        ));
    }
    if let Some((_, path)) = listed.iter().find(|(branch, _)| branch == &request.branch) {
        if path != &wanted_text {
            return Err(V2Error::new(
                ErrorCode::ClaimCollision,
                "branch is bound to a different worktree",
            ));
        }
    }
    let now_unix_seconds = unix_now()?;
    for (other_store, other) in active_issue_records_across_worktrees(store)? {
        if !issue_local && other_store.root() == wanted_compare {
            // Existing-target identity and side-state reconciliation below owns
            // this worktree. Preserve its more specific fail-closed result
            // before applying repository-wide claim collision checks.
            continue;
        }
        if other.issue != request.issue {
            if let Some(claim) = other.claim.as_ref() {
                if let Some((reserved, requested)) = claim.protected_paths.iter().find_map(|a| {
                    request
                        .claim
                        .protected_paths
                        .iter()
                        .find(|b| overlaps(a, b))
                        .map(|b| (a, b))
                }) {
                    if terminal_projection_overlap_is_released(
                        store,
                        &other_store,
                        &other,
                        reserved,
                        requested,
                        now_unix_seconds,
                    )? {
                        continue;
                    }
                    return Err(V2Error::new(
                        ErrorCode::ClaimCollision,
                        format!(
                            "protected path '{}' overlaps requested '{}' from issue {} in phase {:?}",
                            reserved, requested, other.issue, other.phase
                        ),
                    ));
                }
            }
        }
    }
    request
        .claim
        .validate(&request.claim.id, now_unix_seconds)?;
    let created = !issue_local && !wanted.exists();
    if !issue_local && !created {
        let target = Store::new(wanted.clone());
        if let Ok(target_record) = target.load_record(request.issue) {
            let source_record = store.load_record(request.issue)?;
            if target_record.phase == crate::LifecyclePhase::Bound
                && target_record.claim.as_ref() == Some(&request.claim)
            {
                if target_record.issue != request.issue
                    || target_record.repository != source_record.repository
                    || target_record.initialization_digest != source_record.initialization_digest
                {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "bound worktree already contains different issue lifecycle state",
                    ));
                }
                let source_prepared = store
                    .root()
                    .join(".csdlc/prepared/issues")
                    .join(request.issue.to_string());
                let target_prepared = target
                    .root()
                    .join(".csdlc/prepared/issues")
                    .join(request.issue.to_string());
                require_matching_tree(
                    &source_prepared,
                    &target_prepared,
                    "bound worktree already contains different prepared lifecycle state",
                )?;
                let source_preparation = store
                    .root()
                    .join(".csdlc/preparation/issues")
                    .join(request.issue.to_string());
                let target_preparation = target
                    .root()
                    .join(".csdlc/preparation/issues")
                    .join(request.issue.to_string());
                require_matching_tree(
                    &source_preparation,
                    &target_preparation,
                    "bound worktree already contains different claim-free preparation state",
                )?;
                let source_evidence = store
                    .root()
                    .join(".csdlc/evidence")
                    .join(request.issue.to_string());
                let target_evidence = target
                    .root()
                    .join(".csdlc/evidence")
                    .join(request.issue.to_string());
                require_matching_tree(
                    &source_evidence,
                    &target_evidence,
                    "bound worktree already contains different evidence lifecycle state",
                )?;
                return Ok(BindResult {
                    created: false,
                    branch: request.branch,
                    worktree: request.worktree,
                    claim_id: request.claim.id,
                });
            }
        }
    }
    let mut record = store.load_record(request.issue)?;
    let expected_digest = record.digest.clone();
    let was_bound = record.phase == crate::LifecyclePhase::Bound;
    if record.claim.as_ref() != Some(&request.claim) {
        return Err(V2Error::new(
            ErrorCode::ClaimCollision,
            "bind requires the exact claim reserved by init",
        ));
    }
    if record.phase == crate::LifecyclePhase::Initialized {
        if !crate::doctor::diagnose_canonical(store, request.issue).ready {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "issue is not design/card ready for binding",
            ));
        }
        record.advance(
            crate::LifecyclePhase::Ready,
            request.claim.owner.clone(),
            "automatic readiness verification".into(),
        )?;
    }
    if record.phase == crate::LifecyclePhase::Ready {
        record.advance(
            crate::LifecyclePhase::Bound,
            request.claim.owner.clone(),
            "verified Git worktree binding".into(),
        )?;
    } else if record.phase != crate::LifecyclePhase::Bound {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "issue phase cannot be bound",
        ));
    }
    if created {
        let base = request.base_branch.as_str();
        let branch = request.branch.as_str();
        let path = request.worktree.as_str();
        git::run(store.root(), &["worktree", "add", "-b", branch, path, base])?;
    }
    if !was_bound {
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: request.claim.owner.clone(),
            reason: "bind branch/worktree and activate claim".into(),
            operation: "bind".into(),
        });
        record.digest = crate::store::record_digest(&record)?;
        let commit_result = (|| {
            let commit_store = if issue_local {
                Store::new(store.root().to_path_buf())
            } else {
                materialize_bound_issue_state(store, &wanted, request.issue)?
            };
            commit_store.replace_record(request.issue, &expected_digest, &record)
        })();
        if let Err(error) = commit_result {
            if created {
                let remove = git::run(
                    store.root(),
                    &["worktree", "remove", "--force", request.worktree.as_str()],
                );
                let branch = git::run(store.root(), &["branch", "-D", request.branch.as_str()]);
                if remove.is_err() || branch.is_err() {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        format!(
                            "state commit failed ({error}); Git compensation failed (worktree: {}; branch: {})",
                            remove.err().map(|e| e.message).unwrap_or_else(|| "removed".into()),
                            branch.err().map(|e| e.message).unwrap_or_else(|| "removed".into())
                        ),
                    ));
                }
            }
            return Err(error);
        }
    }
    Ok(BindResult {
        created,
        branch: request.branch,
        worktree: request.worktree,
        claim_id: request.claim.id,
    })
}

pub fn heartbeat_claim(
    store: &Store,
    issue: u64,
    claim_id: &str,
    expected_generation: u64,
    now: u64,
    extend_seconds: u64,
) -> Result<()> {
    let mut record = store.load_record(issue)?;
    let expected_digest = record.digest.clone();
    let claim = record
        .claim
        .as_mut()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?;
    if claim.id != claim_id
        || record.generation != expected_generation
        || now >= claim.expires_unix_seconds
        || now < claim.heartbeat_unix_seconds
    {
        return Err(V2Error::new(
            ErrorCode::InvalidClaim,
            "heartbeat compare-and-swap failed",
        ));
    }
    claim.heartbeat_unix_seconds = now;
    claim.expires_unix_seconds = now.saturating_add(extend_seconds);
    record.digest = crate::store::record_digest(&record)?;
    store.replace_record(issue, &expected_digest, &record)
}

pub fn amend_claim_scope(store: &Store, request: AmendClaimScopeRequest) -> Result<Claim> {
    if request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
        || request.add_protected_paths.is_empty()
        || request
            .add_protected_paths
            .iter()
            .any(|path| !clean_relative(path))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "actor, reason, and clean protected paths are required",
        ));
    }
    let _binding_lock = store.binding_lock()?;
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
    if !matches!(
        record.phase,
        crate::LifecyclePhase::Bound | crate::LifecyclePhase::Implemented
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "claim scope may only be amended during bound implementation",
        ));
    }
    record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
        .validate(&request.claim_id, request.now_unix_seconds)?;

    for (other_store, other) in active_issue_records_across_worktrees(store)? {
        if other.issue == request.issue {
            continue;
        }
        if let Some(claim) = other.claim.as_ref() {
            if let Some((reserved, candidate)) = claim.protected_paths.iter().find_map(|reserved| {
                request
                    .add_protected_paths
                    .iter()
                    .find(|candidate| overlaps(reserved, candidate))
                    .map(|candidate| (reserved, candidate))
            }) {
                if terminal_projection_overlap_is_released(
                    store,
                    &other_store,
                    &other,
                    reserved,
                    candidate,
                    request.now_unix_seconds,
                )? {
                    continue;
                }
                return Err(V2Error::new(
                    ErrorCode::ClaimCollision,
                    format!(
                        "protected path '{}' overlaps requested '{}' from issue {} in phase {:?}",
                        reserved, candidate, other.issue, other.phase
                    ),
                ));
            }
        }
    }

    let expected_digest = record.digest.clone();
    let claim = record.claim.as_mut().expect("validated claim");
    claim
        .protected_paths
        .extend(request.add_protected_paths.iter().cloned());
    claim.protected_paths.sort();
    claim.protected_paths.dedup();
    let amended = claim.clone();
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor,
        reason: request.reason,
        operation: serde_json::json!({
            "operation": "amend_claim_scope",
            "add_protected_paths": request.add_protected_paths,
        })
        .to_string(),
    });
    record.digest = crate::store::record_digest(&record)?;
    store.replace_record(request.issue, &expected_digest, &record)?;
    Ok(amended)
}

pub fn transition_active_claim(
    store: &Store,
    request: TransitionActiveClaimRequest,
) -> Result<Claim> {
    if request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
        || request.expected_owner.trim().is_empty()
        || request.expected_purpose.trim().is_empty()
        || request.purpose.trim().is_empty()
        || request.add_protected_paths.is_empty()
        || request
            .add_protected_paths
            .iter()
            .any(|path| !clean_relative(path))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "actor, reason, owner, purpose, and clean protected paths are required",
        ));
    }
    let _binding_lock = store.binding_lock()?;
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
    if record.phase != crate::LifecyclePhase::Bound {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "active claim transition requires bound preparation state",
        ));
    }
    let claim = record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?;
    claim.validate(&request.claim_id, request.now_unix_seconds)?;
    if claim.owner != request.expected_owner {
        return Err(V2Error::new(
            ErrorCode::InvalidClaim,
            "active claim owner does not match expected owner",
        ));
    }
    if claim.purpose != request.expected_purpose {
        return Err(V2Error::new(
            ErrorCode::InvalidClaim,
            "active claim purpose does not match expected source purpose",
        ));
    }

    for (other_store, other) in active_issue_records_across_worktrees(store)? {
        if other.issue == request.issue {
            continue;
        }
        if let Some(other_claim) = other.claim.as_ref() {
            if let Some((reserved, candidate)) =
                other_claim.protected_paths.iter().find_map(|reserved| {
                    request
                        .add_protected_paths
                        .iter()
                        .find(|candidate| overlaps(reserved, candidate))
                        .map(|candidate| (reserved, candidate))
                })
            {
                if terminal_projection_overlap_is_released(
                    store,
                    &other_store,
                    &other,
                    reserved,
                    candidate,
                    request.now_unix_seconds,
                )? {
                    continue;
                }
                return Err(V2Error::new(
                    ErrorCode::ClaimCollision,
                    format!(
                        "protected path '{}' overlaps requested '{}' from issue {} in phase {:?}",
                        reserved, candidate, other.issue, other.phase
                    ),
                ));
            }
        }
    }

    let expected_digest = record.digest.clone();
    let claim = record.claim.as_mut().expect("validated claim");
    let previous_purpose = std::mem::replace(&mut claim.purpose, request.purpose.clone());
    claim
        .protected_paths
        .extend(request.add_protected_paths.iter().cloned());
    claim.protected_paths.sort();
    claim.protected_paths.dedup();
    let transitioned = claim.clone();
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor,
        reason: request.reason,
        operation: serde_json::json!({
            "operation": "transition_active_claim",
            "expected_owner": request.expected_owner,
            "expected_purpose": request.expected_purpose,
            "previous_purpose": previous_purpose,
            "purpose": request.purpose,
            "add_protected_paths": request.add_protected_paths,
        })
        .to_string(),
    });
    record.digest = crate::store::record_digest(&record)?;
    store.replace_record(request.issue, &expected_digest, &record)?;
    Ok(transitioned)
}

pub fn recover_claim(store: &Store, request: RecoverClaimRequest) -> Result<ClaimRecovery> {
    if request.issue == 0
        || request.recovery_actor.trim().is_empty()
        || request.reason.trim().is_empty()
        || request.replacement.id.trim().is_empty()
        || request.replacement.owner.trim().is_empty()
        || request.replacement.purpose.trim().is_empty()
        || request.replacement.branch == "main"
        || request.replacement.protected_paths.is_empty()
        || request
            .replacement
            .protected_paths
            .iter()
            .any(|path| !clean_relative(path))
        || (request.replacement.worktree != "." && !clean_relative(&request.replacement.worktree))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "recovery requires complete actor, reason, binding, purpose, and protected paths",
        ));
    }
    request
        .replacement
        .validate(&request.replacement.id, request.now_unix_seconds)?;
    let _binding_lock = store.binding_lock()?;
    let mut record = store.load_record(request.issue)?;
    let expected_digest = record.digest.clone();
    let current = record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?;
    if current.id != request.expected_claim_id
        || record.generation != request.expected_generation
        || request.now_unix_seconds < current.expires_unix_seconds
    {
        return Err(V2Error::new(
            ErrorCode::InvalidClaim,
            "stale recovery compare-and-swap or expiry check failed",
        ));
    }
    if !claim_matches_active_checkout(store, &request.replacement)? {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "replacement claim does not match the active branch/worktree",
        ));
    }
    let evidence = ClaimRecovery {
        previous_owner: current.owner.clone(),
        observed_expiry_unix_seconds: current.expires_unix_seconds,
        recovery_actor: request.recovery_actor.clone(),
        reason: request.reason.clone(),
    };
    if request.replacement.generation != record.generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "replacement claim generation is stale",
        ));
    }
    for (other_store, other) in active_issue_records_across_worktrees(store)? {
        if other.issue == request.issue {
            continue;
        }
        let Some(other_claim) = other.claim.as_ref() else {
            continue;
        };
        if other_claim
            .validate(&other_claim.id, request.now_unix_seconds)
            .is_err()
        {
            continue;
        }
        if let Some((reserved, candidate)) =
            other_claim.protected_paths.iter().find_map(|reserved| {
                request
                    .replacement
                    .protected_paths
                    .iter()
                    .find(|candidate| overlaps(reserved, candidate))
                    .map(|candidate| (reserved, candidate))
            })
        {
            if terminal_projection_overlap_is_released(
                store,
                &other_store,
                &other,
                reserved,
                candidate,
                request.now_unix_seconds,
            )? {
                continue;
            }
            return Err(V2Error::new(
                ErrorCode::ClaimCollision,
                format!(
                    "protected path '{}' overlaps requested '{}' from live issue {}",
                    reserved, candidate, other.issue
                ),
            ));
        }
    }
    record.claim = Some(request.replacement);
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.recovery_actor,
        reason: request.reason,
        operation: serde_json::to_string(&evidence)?,
    });
    record.digest = crate::store::record_digest(&record)?;
    store.replace_record(request.issue, &expected_digest, &record)?;
    Ok(evidence)
}

pub fn reacquire_claim(
    store: &Store,
    request: ReacquireClaimRequest,
) -> Result<ReacquireClaimResult> {
    if request.issue == 0
        || request.expected_digest.trim().is_empty()
        || request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
        || request.replacement.id.trim().is_empty()
        || request.replacement.owner.trim().is_empty()
        || request.replacement.purpose.trim().is_empty()
        || request.replacement.branch == "main"
        || request.replacement.protected_paths.is_empty()
        || request
            .replacement
            .protected_paths
            .iter()
            .any(|path| !clean_relative(path))
        || (request.replacement.worktree != "." && !clean_relative(&request.replacement.worktree))
        || request.replacement.heartbeat_unix_seconds < request.replacement.acquired_unix_seconds
        || request.replacement.expires_unix_seconds <= request.replacement.heartbeat_unix_seconds
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "reacquire requires complete actor, reason, binding, lease, purpose, and protected paths",
        ));
    }
    request
        .replacement
        .validate(&request.replacement.id, request.now_unix_seconds)?;
    let _binding_lock = store.binding_lock()?;
    let mut record = store.load_record(request.issue)?;
    if record.generation != request.expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "claim reacquisition generation is stale",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "claim reacquisition digest is stale",
        ));
    }
    if matches!(
        record.phase,
        crate::LifecyclePhase::Merged | crate::LifecyclePhase::ClosedOut
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "terminal issue cannot reacquire a writer claim",
        ));
    }
    if request.replacement.generation != record.generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "replacement claim generation is stale",
        ));
    }
    if !claim_matches_active_checkout(store, &request.replacement)? {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "replacement claim does not match the active branch/worktree",
        ));
    }
    let previous = record.claim.as_ref();
    if previous.is_some_and(|claim| request.now_unix_seconds < claim.expires_unix_seconds) {
        return Err(V2Error::new(
            ErrorCode::ClaimCollision,
            "live claim must be released before reacquisition",
        ));
    }

    for (other_store, other) in active_issue_records_across_worktrees(store)? {
        if other.issue == request.issue {
            continue;
        }
        let Some(other_claim) = other.claim.as_ref() else {
            continue;
        };
        if other_claim
            .validate(&other_claim.id, request.now_unix_seconds)
            .is_err()
        {
            continue;
        }
        if let Some((reserved, candidate)) =
            other_claim.protected_paths.iter().find_map(|reserved| {
                request
                    .replacement
                    .protected_paths
                    .iter()
                    .find(|candidate| overlaps(reserved, candidate))
                    .map(|candidate| (reserved, candidate))
            })
        {
            if terminal_projection_overlap_is_released(
                store,
                &other_store,
                &other,
                reserved,
                candidate,
                request.now_unix_seconds,
            )? {
                continue;
            }
            return Err(V2Error::new(
                ErrorCode::ClaimCollision,
                format!(
                    "protected path '{}' overlaps requested '{}' from live issue {}",
                    reserved, candidate, other.issue
                ),
            ));
        }
    }

    let expected_digest = record.digest.clone();
    let previous_claim_id = previous.map(|claim| claim.id.clone());
    let previous_owner = previous.map(|claim| claim.owner.clone());
    record.claim = Some(request.replacement.clone());
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor,
        reason: request.reason,
        operation: serde_json::json!({
            "operation": "reacquire_claim",
            "previous_claim_id": previous_claim_id,
            "previous_owner": previous_owner,
            "claim_id": request.replacement.id,
            "owner": request.replacement.owner,
            "branch": request.replacement.branch,
            "worktree": request.replacement.worktree,
            "protected_paths": request.replacement.protected_paths,
            "purpose": request.replacement.purpose,
        })
        .to_string(),
    });
    record.digest = crate::store::record_digest(&record)?;
    record = store.replace_authority_record(request.issue, &expected_digest, &record)?;
    Ok(ReacquireClaimResult {
        schema: "csdlc.reacquire_claim_result.v1".into(),
        issue: request.issue,
        claim: request.replacement,
        previous_claim_id,
        previous_owner,
        phase: record.phase,
        generation: record.generation,
        digest: record.digest,
    })
}

pub fn rehome_claim_authority(
    store: &Store,
    request: RehomeClaimAuthorityRequest,
) -> Result<RehomeClaimAuthorityResult> {
    rehome_claim_authority_with_test_observer(store, request, || Ok(()))
}

/// Explicit synchronization seam for deterministic concurrency tests.
/// Operational callers must use [`rehome_claim_authority`].
#[doc(hidden)]
pub fn rehome_claim_authority_with_test_observer<F>(
    store: &Store,
    request: RehomeClaimAuthorityRequest,
    after_materialization: F,
) -> Result<RehomeClaimAuthorityResult>
where
    F: FnOnce() -> Result<()>,
{
    if request.issue == 0
        || request.expected_digest.trim().is_empty()
        || request.expected_initialization_digest.trim().is_empty()
        || request.expected_source_digest.trim().is_empty()
        || request.source_worktree.trim().is_empty()
        || request.source_branch.trim().is_empty()
        || request.source_commit.len() != 40
        || !request
            .source_commit
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
        || request.current_session_id.trim().is_empty()
        || request.session_ledger_path.trim().is_empty()
        || request.actor.trim().is_empty()
        || request.operator_authority.trim().is_empty()
        || request.reason.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "authority rehome requires exact source, ledger, actor, and operator authority",
        ));
    }
    request
        .replacement
        .validate(&request.replacement.id, request.now_unix_seconds)?;
    if request.replacement.generation != request.expected_source_generation
        || request.replacement.branch == "main"
        || request.replacement.owner.trim().is_empty()
        || request.replacement.purpose.trim().is_empty()
        || request.replacement.protected_paths.is_empty()
        || request
            .replacement
            .protected_paths
            .iter()
            .any(|path| !clean_relative(path))
        || (request.replacement.worktree != "." && !clean_relative(&request.replacement.worktree))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "authority rehome replacement claim is incomplete",
        ));
    }

    // This lock lives below the Git common directory and is shared by every
    // registered checkout. It serializes the scan and authority replacement.
    let _binding_lock = store.binding_lock()?;
    let mut record = store.load_record(request.issue)?;
    if record.generation != request.expected_generation
        || record.digest != request.expected_digest
        || record.initialization_digest != request.expected_initialization_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "authority rehome target identity or compare-and-swap is stale",
        ));
    }
    if matches!(
        record.phase,
        crate::LifecyclePhase::Merged | crate::LifecyclePhase::ClosedOut
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "terminal issue cannot rehome writer authority",
        ));
    }
    if record
        .claim
        .as_ref()
        .is_some_and(|claim| request.now_unix_seconds < claim.expires_unix_seconds)
    {
        return Err(V2Error::new(
            ErrorCode::ClaimCollision,
            "live canonical claim must be explicitly released before authority rehome",
        ));
    }
    if !claim_matches_active_checkout(store, &request.replacement)? {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "authority rehome replacement does not match the active checkout",
        ));
    }

    let current_root = store.root().canonicalize()?;
    let requested_source_root = PathBuf::from(&request.source_worktree)
        .canonicalize()
        .map_err(|error| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("authority source worktree is unavailable: {error}"),
            )
        })?;
    let registered_source = git::worktrees(store.root())?
        .into_iter()
        .any(|(branch, root)| {
            branch == request.source_branch
                && PathBuf::from(root)
                    .canonicalize()
                    .is_ok_and(|candidate| candidate == requested_source_root)
        });
    if !registered_source || requested_source_root == current_root {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "authority source must be an exact distinct registered branch/worktree",
        ));
    }
    let source_store = Store::new(requested_source_root.clone());
    if git::current_branch(source_store.root())? != request.source_branch
        || git::run(source_store.root(), &["rev-parse", "HEAD"])?.stdout != request.source_commit
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authority source branch or commit changed",
        ));
    }
    let source = source_store.load_record(request.issue)?;
    let source_cards = source_store.load_cards(request.issue)?;
    crate::store::verify_cards(&source_store, &source, &source_cards)?;
    source_store.verify_canonical_authority_projection(&source, &source_cards)?;
    let source_review = source
        .review
        .clone()
        .filter(|review| review.completed)
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "source worktree does not contain completed review evidence",
            )
        })?;
    let current_source_revision =
        git::substantive_revision(source_store.root(), &source_review.scope)?;
    let reviewed_commit = source_review
        .reviewed_revision
        .strip_prefix("git-blake3:")
        .and_then(|revision| revision.split(':').next());
    let source_sor_is_prepublication =
        source_cards.get(&crate::CardKind::Sor).is_some_and(|card| {
            matches!(
                &card.content,
                crate::cards::CardContent::Sor(sor)
                    if matches!(
                        sor.integration_state,
                        crate::cards::IntegrationState::NotStarted
                            | crate::cards::IntegrationState::WorktreeOnly
                    )
                        && sor.publication_state == crate::cards::PublicationState::NotPublished
                        && sor.merge_state == crate::cards::MergeState::NotMerged
                        && sor.closeout_state == crate::cards::CloseoutState::NotStarted
            )
        });
    let source_mismatch = if source.issue != record.issue {
        Some("issue")
    } else if source.repository != record.repository {
        Some("repository")
    } else if source.initialization_digest != record.initialization_digest {
        Some("initialization")
    } else if source.generation != request.expected_source_generation {
        Some("generation")
    } else if source.digest != request.expected_source_digest {
        Some("digest")
    } else if crate::store::record_digest(&source)? != source.digest {
        Some("self digest")
    } else if source.claim.is_some() {
        Some("claim")
    } else if source.phase != crate::LifecyclePhase::Reviewed {
        Some("phase")
    } else if source.publication.is_some() {
        Some("publication evidence")
    } else if source.readiness.is_some() {
        Some("readiness evidence")
    } else if source.terminal.is_some() {
        Some("terminal evidence")
    } else if !source_sor_is_prepublication {
        Some("SOR pre-publication state")
    } else if current_source_revision != source_review.reviewed_revision {
        Some("review revision")
    } else if reviewed_commit != Some(request.source_commit.as_str()) {
        Some("reviewed commit")
    } else {
        None
    };
    if let Some(mismatch) = source_mismatch {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!("source worktree is not the exact claim-free reviewed authority: {mismatch}"),
        ));
    }
    let source_fingerprint = authority_projection_fingerprint(&source_store, &source)?;
    for path in [&source.design_path, &source.diagram_path] {
        if read_regular_authority_file(&source_store.root().join(path))?
            != read_regular_authority_file(&store.root().join(path))?
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "source authored artifacts differ from the aggregate checkout",
            ));
        }
    }

    let mut preserved_bindings = Vec::new();
    for (branch, root) in git::worktrees(store.root())? {
        let root_path = PathBuf::from(&root);
        let other_root = root_path.canonicalize().map_err(|error| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("registered worktree is unavailable at {root}: {error}"),
            )
        })?;
        let scoped = Store::new(root_path);
        match fs::metadata(scoped.issue_dir(request.issue)) {
            Ok(metadata) if !metadata.is_dir() => {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    format!("sibling issue authority is not a directory at {root}"),
                ))
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    format!("sibling issue authority is unavailable at {root}: {error}"),
                ))
            }
        }
        let other = scoped.load_record(request.issue).map_err(|error| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("unreadable sibling authority at {root}: {}", error.message),
            )
        })?;
        if other.initialization_digest != source.initialization_digest
            || other.repository != record.repository
            || other.generation > source.generation
            || (other.generation == source.generation
                && other.digest != source.digest
                && other_root != requested_source_root)
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("newer or conflicting authority exists at {root}"),
            ));
        }
        if other_root != current_root && other_root != requested_source_root {
            if other
                .claim
                .as_ref()
                .is_some_and(|claim| request.now_unix_seconds < claim.expires_unix_seconds)
            {
                return Err(V2Error::new(
                    ErrorCode::ClaimCollision,
                    format!("live issue owner remains in registered worktree {root}"),
                ));
            }
            preserved_bindings.push(format!("branch={branch};worktree={root}"));
        }
    }
    preserved_bindings.sort();

    let ledger_path = PathBuf::from(&request.session_ledger_path);
    let ledger: SessionLedgerView = serde_json::from_slice(&fs::read(&ledger_path)?)?;
    if ledger.schema != "adl.session_ledger.v1" {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "authority rehome session ledger schema is unsupported",
        ));
    }
    for claim in ledger.claims.iter().filter(|claim| {
        claim.github.issue == Some(request.issue)
            && claim.released_at.is_none()
            && claim.mode == "active"
    }) {
        let expires = time::OffsetDateTime::parse(
            &claim.expires_at,
            &time::format_description::well_known::Rfc3339,
        )
        .map_err(|error| V2Error::new(ErrorCode::InvalidInput, error.to_string()))?;
        if expires.unix_timestamp() > request.now_unix_seconds as i64
            && claim.session_id != request.current_session_id
        {
            return Err(V2Error::new(
                ErrorCode::ClaimCollision,
                "another live session-ledger owner blocks authority rehome",
            ));
        }
    }

    // Ordinary typed writers take this issue lock. Keep it through target
    // materialization, source revalidation, and any rollback so no writer can
    // advance the staged authority between those steps.
    let _target_lock = store.authority_projection_lock(request.issue)?;
    if git::substantive_content_digest(store.root(), &source_review.scope)?
        != git::substantive_content_digest(source_store.root(), &source_review.scope)?
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "aggregate checkout review scope differs from the reviewed source; use atomic historical materialization instead",
        ));
    }
    let previous = record.claim.clone();
    let original = record.clone();
    let original_cards = store.load_cards(request.issue)?;
    crate::store::verify_cards(store, &original, &original_cards)?;
    record = source;
    record.claim = Some(request.replacement.clone());
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor,
        reason: request.reason,
        operation: serde_json::json!({
            "operation": "rehome_claim_authority",
            "operator_authority": request.operator_authority,
            "source_commit": request.source_commit,
            "source_worktree": request.source_worktree,
            "source_branch": request.source_branch,
            "source_generation": request.expected_source_generation,
            "source_digest": request.expected_source_digest,
            "initialization_digest": request.expected_initialization_digest,
            "previous_claim": previous,
            "preserved_bindings": preserved_bindings,
            "session_ledger": request.session_ledger_path,
            "current_session_id": request.current_session_id,
        })
        .to_string(),
    });
    record.digest = crate::store::record_digest(&record)?;
    record = store.replace_authority_projection_locked(
        request.issue,
        &request.expected_digest,
        &record,
        &source_cards,
    )?;
    let source_unchanged = (|| -> Result<bool> {
        after_materialization()?;
        let still_registered = git::worktrees(store.root())?
            .into_iter()
            .any(|(branch, root)| {
                branch == request.source_branch
                    && PathBuf::from(root)
                        .canonicalize()
                        .is_ok_and(|candidate| candidate == requested_source_root)
            });
        if !still_registered
            || git::current_branch(source_store.root())? != request.source_branch
            || git::run(source_store.root(), &["rev-parse", "HEAD"])?.stdout
                != request.source_commit
            || git::substantive_revision(source_store.root(), &source_review.scope)?
                != source_review.reviewed_revision
        {
            return Ok(false);
        }
        let after = source_store.load_record(request.issue)?;
        let artifacts_equal = [&after.design_path, &after.diagram_path]
            .into_iter()
            .try_fold(true, |equal, path| {
                Ok::<_, V2Error>(
                    equal
                        && read_regular_authority_file(&source_store.root().join(path))?
                            == read_regular_authority_file(&store.root().join(path))?,
                )
            })?;
        Ok(artifacts_equal
            && authority_projection_fingerprint(&source_store, &after)? == source_fingerprint)
    })();
    if !matches!(source_unchanged, Ok(true)) {
        store.replace_authority_projection_locked(
            request.issue,
            &record.digest,
            &original,
            &original_cards,
        )?;
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            match source_unchanged {
                Ok(false) => "source authority identity changed during materialization; target rolled back".into(),
                Err(error) => format!(
                    "source authority became unreadable during materialization; target rolled back: {}",
                    error.message
                ),
                Ok(true) => unreachable!(),
            },
        ));
    }
    let committed_cards = store.load_cards(request.issue)?;
    crate::store::verify_cards(store, &record, &committed_cards)?;
    Ok(RehomeClaimAuthorityResult {
        schema: "csdlc.rehome_claim_authority_result.v1".into(),
        issue: request.issue,
        source_commit: request.source_commit,
        initialization_digest: record.initialization_digest.clone(),
        preserved_bindings,
        claim: request.replacement,
        generation: record.generation,
        digest: record.digest,
    })
}

fn authority_projection_fingerprint(store: &Store, record: &crate::IssueRecord) -> Result<String> {
    let issue_root = format!(".csdlc/issues/{}", record.issue);
    let mut paths = vec![
        record.design_path.clone(),
        record.diagram_path.clone(),
        format!("{issue_root}/index.json"),
        format!("{issue_root}/audit.jsonl"),
    ];
    for card in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        paths.push(format!("{issue_root}/cards/{card}.values.json"));
        paths.push(format!("{issue_root}/cards/{card}.md"));
    }
    let mut hasher = blake3::Hasher::new();
    for relative in paths {
        let path = if Path::new(&relative).is_absolute() {
            PathBuf::from(&relative)
        } else {
            store.root().join(&relative)
        };
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "authority projection contains a non-regular file",
            ));
        }
        hasher.update(relative.as_bytes());
        hasher.update(&fs::read(path)?);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

fn read_regular_authority_file(path: &Path) -> Result<Vec<u8>> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!("authority file is not regular: {}", path.display()),
        ));
    }
    Ok(fs::read(path)?)
}

pub fn release_closed_claim(
    store: &Store,
    request: ReleaseClosedClaimRequest,
) -> Result<ClaimRecovery> {
    if request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
        || request.repository.trim().is_empty()
        || request.observed_issue_state != "closed"
        || request.observed_issue != request.issue
        || request.observation_source.trim().is_empty()
        || request.observation_source
            != format!("github://{}/issues/{}", request.repository, request.issue)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "release actor and reason required",
        ));
    }
    let mut record = store.load_record(request.issue)?;
    if record.repository != request.repository {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "closed-issue claim release repository mismatch",
        ));
    }
    let current = record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?;
    if record.phase != crate::LifecyclePhase::Implemented
        || current.id != request.expected_claim_id
        || record.generation != request.expected_generation
    {
        return Err(V2Error::new(
            ErrorCode::InvalidClaim,
            "closed-issue claim release compare-and-swap failed",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "closed-issue claim release digest is stale",
        ));
    }
    let evidence = ClaimRecovery {
        previous_owner: current.owner.clone(),
        observed_expiry_unix_seconds: current.expires_unix_seconds,
        recovery_actor: request.actor.clone(),
        reason: request.reason.clone(),
    };
    record.claim = None;
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor,
        reason: request.reason,
        operation: serde_json::json!({
            "operation": "release_closed_claim",
            "observed_issue_state": request.observed_issue_state,
            "observation_source": request.observation_source,
        })
        .to_string(),
    });
    record.digest = crate::store::record_digest(&record)?;
    store.replace_record(request.issue, &request.expected_digest, &record)?;
    Ok(evidence)
}

pub fn revoke_active_claim(
    store: &Store,
    request: RevokeActiveClaimRequest,
) -> Result<RevokeActiveClaimResult> {
    if request.issue == 0
        || request.repository.trim().is_empty()
        || request.expected_claim_id.trim().is_empty()
        || request.expected_digest.trim().is_empty()
        || request.actor.trim().is_empty()
        || request.operator_authority.trim().is_empty()
        || request.reason.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue, repository, claim, digest, actor, operator authority, and reason are required",
        ));
    }
    let mut record = store.load_record(request.issue)?;
    if record.repository != request.repository {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "active-claim revoke repository mismatch",
        ));
    }
    if record.generation != request.expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "active-claim revoke generation is stale",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "active-claim revoke digest is stale",
        ));
    }
    if record.phase == crate::LifecyclePhase::ClosedOut {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "closed-out issue cannot have an active claim revoked",
        ));
    }
    let current = record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?;
    if current.id != request.expected_claim_id {
        return Err(V2Error::new(
            ErrorCode::InvalidClaim,
            "active-claim revoke claim compare-and-swap failed",
        ));
    }
    let claim_id = current.id.clone();
    let previous_owner = current.owner.clone();
    if request.now_unix_seconds >= current.expires_unix_seconds {
        return Err(V2Error::new(
            ErrorCode::ExpiredClaim,
            "expired claim must use expiry recovery instead of operator revoke",
        ));
    }
    record.claim = None;
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor.clone(),
        reason: request.reason.clone(),
        operation: serde_json::json!({
            "operation": "revoke_active_claim",
            "operator_authority": request.operator_authority,
            "claim_id": claim_id,
            "previous_owner": previous_owner,
        })
        .to_string(),
    });
    record.digest = crate::store::record_digest(&record)?;
    let generation = record.generation;
    let digest = record.digest.clone();
    store.replace_record(request.issue, &request.expected_digest, &record)?;
    Ok(RevokeActiveClaimResult {
        schema: "csdlc.revoke_active_claim_result.v1".into(),
        issue: request.issue,
        claim_id,
        previous_owner,
        actor: request.actor,
        operator_authority: request.operator_authority,
        reason: request.reason,
        generation,
        digest,
        released: true,
    })
}

fn unix_now() -> Result<u64> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| V2Error::new(ErrorCode::InvalidInput, e.to_string()))?
        .as_secs())
}

#[cfg(test)]
mod terminal_projection_authority_tests {
    use super::{
        exact_expired_terminal_projection_overlap, terminal_projection_overlap_is_released,
    };
    use crate::finish::{derive_terminal, retain_cached_terminal, FinishRequest};
    use crate::{Claim, DesignReview, IssueRecord, LifecyclePhase, MergeMethod, Store};
    use std::collections::BTreeMap;
    use std::process::Command;

    #[test]
    fn aggregate_overlap_exception_is_exact_expired_and_projection_only() {
        assert!(exact_expired_terminal_projection_overlap(
            5384,
            10,
            ".csdlc/issues/5384",
            ".csdlc/issues/5384/",
            10,
        ));
        assert!(!exact_expired_terminal_projection_overlap(
            5384,
            11,
            ".csdlc/issues/5384",
            ".csdlc/issues/5384",
            10,
        ));
        assert!(!exact_expired_terminal_projection_overlap(
            5384,
            10,
            "docs/milestones/v0.91.8",
            "docs/milestones/v0.91.8",
            10,
        ));
        assert!(!exact_expired_terminal_projection_overlap(
            5384,
            10,
            ".csdlc/issues/5384",
            ".csdlc/issues",
            10,
        ));
    }

    #[test]
    fn derived_terminal_authority_logically_releases_every_stale_claim_path() {
        let temp = tempfile::tempdir().expect("tempdir");
        assert!(Command::new("git")
            .args(["init", "-q"])
            .current_dir(temp.path())
            .status()
            .expect("git init")
            .success());
        let record = IssueRecord {
            schema: "csdlc.issue.v2".into(),
            issue: 5778,
            repository: "owner/repo".into(),
            initialization_digest: "initialization".into(),
            phase: LifecyclePhase::Reviewed,
            generation: 4,
            digest: "digest".into(),
            claim: Some(Claim {
                id: "claim".into(),
                owner: "finished-session".into(),
                generation: 4,
                acquired_unix_seconds: 1,
                expires_unix_seconds: 10_000,
                heartbeat_unix_seconds: 1,
                branch: "codex/5778".into(),
                worktree: ".".into(),
                protected_paths: vec!["csdlc-v2".into()],
                purpose: "implementation".into(),
            }),
            review_assignment: None,
            review: None,
            publication: None,
            readiness: None,
            terminal: None,
            migration: None,
            design_path: "design.md".into(),
            diagram_path: "diagram.mmd".into(),
            design_review: DesignReview::Pending,
            cards: BTreeMap::new(),
            transitions: Vec::new(),
            audit: Vec::new(),
        };
        let request = FinishRequest {
            schema: "csdlc.finish_request.v1".into(),
            issue: 5778,
            expected_generation: 4,
            expected_digest: "digest".into(),
            claim_id: "claim".into(),
            actor: "finished-session".into(),
            repository: "owner/repo".into(),
            pull_request: None,
            base: None,
            head: None,
            expected_head_sha: None,
            merge_method: MergeMethod::Squash,
            required_checks: Vec::new(),
            require_review: false,
            approved_no_pr_reason: Some("approved closure".into()),
            token_file: None,
        };
        let issue = crate::finish::IssueTerminalObservation {
            state: "closed".into(),
            labels: vec![crate::finish::NO_PR_APPROVAL_LABEL.into()],
            observed_unix_seconds: 2,
        };
        let envelope = derive_terminal(&record, &request, &issue, None)
            .expect("derive")
            .expect("terminal");
        retain_cached_terminal(temp.path(), &envelope).expect("retain");
        let store = Store::new(temp.path());

        assert!(terminal_projection_overlap_is_released(
            &store,
            &store,
            &record,
            "csdlc-v2",
            "csdlc-v2/src/lib.rs",
            2,
        )
        .expect("released"));
    }
}
