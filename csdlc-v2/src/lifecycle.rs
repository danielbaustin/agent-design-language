use std::fs;
use std::path::{Component, Path};

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

pub fn initialize_issue(
    store: &Store,
    mut request: BootstrapRequest,
) -> Result<crate::IssueRecord> {
    if !clean_relative(&request.design_path) || !clean_relative(&request.diagram_path) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design and diagram paths must be repository-relative",
        ));
    }
    validate_bootstrap_request(&request)?;
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
    let _binding_lock = store.binding_lock()?;
    let issues = store.root().join(".csdlc/issues");
    if issues.exists() {
        for entry in fs::read_dir(&issues)? {
            let path = entry?.path().join("index.json");
            if !path.exists() {
                continue;
            }
            let other: crate::IssueRecord = serde_json::from_slice(&fs::read(path)?)?;
            if other.issue != request.issue {
                if let Some(claim) = other.claim {
                    if claim
                        .protected_paths
                        .iter()
                        .any(|a| request.claim.protected_paths.iter().any(|b| overlaps(a, b)))
                    {
                        return Err(V2Error::new(
                            ErrorCode::ClaimCollision,
                            format!("protected path overlaps issue {}", other.issue),
                        ));
                    }
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

pub fn bind_issue(store: &Store, request: BindRequest) -> Result<BindResult> {
    if request.branch == "main"
        || request.branch == request.base_branch
        || request.claim.branch != request.branch
        || request.claim.worktree != request.worktree
        || !clean_relative(&request.worktree)
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
    if git::current_branch(store.root())? != request.base_branch {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "primary checkout is not on the declared base branch",
        ));
    }
    let wanted = store.root().join(&request.worktree);
    let wanted_compare = if wanted.exists() {
        fs::canonicalize(&wanted)?
    } else {
        wanted.clone()
    };
    let wanted_text = wanted_compare.to_string_lossy();
    let listed = git::worktrees(store.root())?;
    if let Some((branch, _)) = listed.iter().find(|(_, path)| path == &wanted_text) {
        if branch != &request.branch {
            return Err(V2Error::new(
                ErrorCode::ClaimCollision,
                "worktree is bound to a different branch",
            ));
        }
    }
    if let Some((_, path)) = listed.iter().find(|(branch, _)| branch == &request.branch) {
        if path != &wanted_text {
            return Err(V2Error::new(
                ErrorCode::ClaimCollision,
                "branch is bound to a different worktree",
            ));
        }
    }
    let issues = store.root().join(".csdlc/issues");
    if issues.exists() {
        for entry in fs::read_dir(issues)? {
            let path = entry?.path().join("index.json");
            if !path.exists() {
                continue;
            }
            let other: crate::IssueRecord = serde_json::from_slice(&fs::read(path)?)?;
            if other.issue != request.issue {
                if let Some(claim) = other.claim {
                    if claim
                        .protected_paths
                        .iter()
                        .any(|a| request.claim.protected_paths.iter().any(|b| overlaps(a, b)))
                    {
                        return Err(V2Error::new(
                            ErrorCode::ClaimCollision,
                            format!("protected path overlaps issue {}", other.issue),
                        ));
                    }
                }
            }
        }
    }
    request.claim.validate(&request.claim.id, unix_now()?)?;
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
        if !crate::diagnose(store, request.issue).ready {
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
    let created = !wanted.exists();
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
        if let Err(error) = store.replace_record(request.issue, &expected_digest, &record) {
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

pub fn recover_claim(store: &Store, request: RecoverClaimRequest) -> Result<ClaimRecovery> {
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
    if request.reason.trim().is_empty() || request.recovery_actor.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "recovery actor and reason required",
        ));
    }
    let evidence = ClaimRecovery {
        previous_owner: current.owner.clone(),
        observed_expiry_unix_seconds: current.expires_unix_seconds,
        recovery_actor: request.recovery_actor.clone(),
        reason: request.reason.clone(),
    };
    request
        .replacement
        .validate(&request.replacement.id, request.now_unix_seconds)?;
    if request.replacement.generation != record.generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "replacement claim generation is stale",
        ));
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

fn unix_now() -> Result<u64> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| V2Error::new(ErrorCode::InvalidInput, e.to_string()))?
        .as_secs())
}
