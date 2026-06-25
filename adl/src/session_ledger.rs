use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration as StdDuration, Instant};

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

pub const SESSION_LEDGER_SCHEMA: &str = "adl.session_ledger.v1";
pub const DEFAULT_TTL_SECS: i64 = 900;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionLedger {
    pub schema: String,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub global_freeze: Option<FreezeState>,
    #[serde(default)]
    pub claims: Vec<OccupancyClaim>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FreezeState {
    pub active: bool,
    pub reason: String,
    pub set_by: String,
    pub set_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OccupancyClaim {
    pub claim_id: String,
    pub session_id: String,
    pub owner: String,
    pub resource: ResourceRef,
    pub purpose: String,
    pub mode: ClaimMode,
    pub lifecycle_phase: Option<String>,
    pub policy_ref: Option<String>,
    pub github: GithubRef,
    pub branch: Option<String>,
    pub worktree_path: Option<String>,
    #[serde(default)]
    pub do_not_touch_paths: Vec<String>,
    #[serde(default)]
    pub blockers: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub heartbeat_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
    pub release_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceRef {
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct GithubRef {
    pub issue: Option<u64>,
    pub pull_request: Option<u64>,
    pub repository: Option<String>,
    pub last_state: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ClaimMode {
    Active,
    Watching,
    Paused,
    Stale,
    Released,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LedgerStatus {
    pub schema: String,
    pub ledger_path: String,
    pub now: DateTime<Utc>,
    pub global_freeze_active: bool,
    pub active_claims: usize,
    pub stale_claims: usize,
    pub released_claims: usize,
    pub claims: Vec<ClaimStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimStatus {
    pub claim_id: String,
    pub session_id: String,
    pub owner: String,
    pub resource: ResourceRef,
    pub purpose: String,
    pub mode: ClaimMode,
    pub classification: ClaimClassification,
    pub issue: Option<u64>,
    pub pull_request: Option<u64>,
    pub branch: Option<String>,
    pub worktree_path: Option<String>,
    pub heartbeat_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetClaimMatch {
    pub claim_id: String,
    pub session_id: String,
    pub owner: String,
    pub resource: ResourceRef,
    pub mode: ClaimMode,
    pub classification: ClaimClassification,
    pub issue: Option<u64>,
    pub branch: Option<String>,
    pub worktree_path: Option<String>,
    pub heartbeat_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
    pub matches_issue: bool,
    pub matches_branch: bool,
    pub matches_worktree: bool,
    pub self_claim: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetClaimAssessment {
    pub ledger_path: String,
    pub status: &'static str,
    pub block_kind: &'static str,
    pub guidance: &'static str,
    pub current_session_id: Option<String>,
    pub relevant_claims: Vec<TargetClaimMatch>,
}

#[derive(Debug, Clone)]
pub struct TargetClaimQuery<'a> {
    pub repo_root: &'a Path,
    pub issue_number: u64,
    pub branch: &'a str,
    pub worktree_path: &'a Path,
    pub current_session_id: Option<&'a str>,
    pub current_session_aliases: Vec<&'a str>,
    pub now: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ClaimClassification {
    Active,
    Watching,
    Paused,
    Stale,
    Released,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimInput {
    pub session_id: String,
    pub owner: String,
    pub resource: ResourceRef,
    pub purpose: String,
    pub mode: ClaimMode,
    pub lifecycle_phase: Option<String>,
    pub policy_ref: Option<String>,
    pub github: GithubRef,
    pub branch: Option<String>,
    pub worktree_path: Option<String>,
    pub do_not_touch_paths: Vec<String>,
    pub blockers: Vec<String>,
    pub ttl_secs: i64,
}

#[derive(Debug)]
pub struct LedgerLock {
    path: PathBuf,
}

impl Drop for LedgerLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

impl SessionLedger {
    pub fn empty(now: DateTime<Utc>) -> Self {
        Self {
            schema: SESSION_LEDGER_SCHEMA.to_string(),
            updated_at: now,
            global_freeze: None,
            claims: Vec::new(),
        }
    }

    pub fn status(&self, ledger_path: &Path, now: DateTime<Utc>) -> LedgerStatus {
        let claims = self
            .claims
            .iter()
            .map(|claim| claim_status(claim, now))
            .collect::<Vec<_>>();
        LedgerStatus {
            schema: SESSION_LEDGER_SCHEMA.to_string(),
            ledger_path: ledger_path.display().to_string(),
            now,
            global_freeze_active: self
                .global_freeze
                .as_ref()
                .map(|freeze| freeze.active)
                .unwrap_or(false),
            active_claims: claims
                .iter()
                .filter(|claim| claim.classification == ClaimClassification::Active)
                .count(),
            stale_claims: claims
                .iter()
                .filter(|claim| claim.classification == ClaimClassification::Stale)
                .count(),
            released_claims: claims
                .iter()
                .filter(|claim| claim.classification == ClaimClassification::Released)
                .count(),
            claims,
        }
    }

    pub fn claim(&mut self, input: ClaimInput, now: DateTime<Utc>) -> Result<OccupancyClaim> {
        validate_non_empty("session id", &input.session_id)?;
        validate_non_empty("owner", &input.owner)?;
        validate_non_empty("resource kind", &input.resource.kind)?;
        validate_non_empty("resource id", &input.resource.id)?;
        validate_non_empty("purpose", &input.purpose)?;
        if input.ttl_secs <= 0 {
            return Err(anyhow!("ttl seconds must be greater than zero"));
        }

        let active_conflicts = self
            .claims
            .iter()
            .filter(|claim| {
                claim.resource == input.resource
                    && classify_claim(claim, now) != ClaimClassification::Released
                    && classify_claim(claim, now) != ClaimClassification::Stale
            })
            .map(|claim| claim.claim_id.clone())
            .collect::<Vec<_>>();
        if !active_conflicts.is_empty() {
            return Err(anyhow!(
                "resource '{}:{}' already has active claim(s): {}",
                input.resource.kind,
                input.resource.id,
                active_conflicts.join(", ")
            ));
        }

        let claim_id = next_claim_id(&self.claims, &input, now);
        let claim = OccupancyClaim {
            claim_id,
            session_id: input.session_id,
            owner: input.owner,
            resource: input.resource,
            purpose: input.purpose,
            mode: input.mode,
            lifecycle_phase: input.lifecycle_phase,
            policy_ref: input.policy_ref,
            github: input.github,
            branch: input.branch,
            worktree_path: input.worktree_path,
            do_not_touch_paths: input.do_not_touch_paths,
            blockers: input.blockers,
            created_at: now,
            heartbeat_at: now,
            expires_at: now + Duration::seconds(input.ttl_secs),
            released_at: None,
            release_reason: None,
        };
        self.claims.push(claim.clone());
        self.updated_at = now;
        Ok(claim)
    }

    pub fn heartbeat(
        &mut self,
        claim_id: &str,
        ttl_secs: i64,
        now: DateTime<Utc>,
    ) -> Result<OccupancyClaim> {
        if ttl_secs <= 0 {
            return Err(anyhow!("ttl seconds must be greater than zero"));
        }
        let claim = self
            .claims
            .iter_mut()
            .find(|claim| claim.claim_id == claim_id)
            .ok_or_else(|| anyhow!("claim '{claim_id}' not found"))?;
        match classify_claim(claim, now) {
            ClaimClassification::Released => {
                return Err(anyhow!("claim '{claim_id}' is already released"));
            }
            ClaimClassification::Stale => {
                return Err(anyhow!(
                    "claim '{claim_id}' is stale; create a new claim instead"
                ));
            }
            ClaimClassification::Active
            | ClaimClassification::Watching
            | ClaimClassification::Paused => {}
        }
        claim.heartbeat_at = now;
        claim.expires_at = now + Duration::seconds(ttl_secs);
        claim.released_at = None;
        claim.release_reason = None;
        self.updated_at = now;
        Ok(claim.clone())
    }

    pub fn release(
        &mut self,
        claim_id: &str,
        reason: Option<String>,
        now: DateTime<Utc>,
    ) -> Result<OccupancyClaim> {
        let claim = self
            .claims
            .iter_mut()
            .find(|claim| claim.claim_id == claim_id)
            .ok_or_else(|| anyhow!("claim '{claim_id}' not found"))?;
        claim.mode = ClaimMode::Released;
        claim.released_at = Some(now);
        claim.release_reason = reason;
        self.updated_at = now;
        Ok(claim.clone())
    }
}

pub fn default_ledger_path(repo_root: &Path) -> PathBuf {
    repo_root
        .join(".adl")
        .join("session-ledger")
        .join("ledger.json")
}

pub fn load_ledger(path: &Path, now: DateTime<Utc>) -> Result<SessionLedger> {
    match fs::read_to_string(path) {
        Ok(raw) => {
            let ledger: SessionLedger = serde_json::from_str(&raw)
                .with_context(|| format!("parse session ledger {}", path.display()))?;
            if ledger.schema != SESSION_LEDGER_SCHEMA {
                return Err(anyhow!(
                    "unsupported session ledger schema '{}'",
                    ledger.schema
                ));
            }
            Ok(ledger)
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(SessionLedger::empty(now)),
        Err(err) => Err(err).with_context(|| format!("read session ledger {}", path.display())),
    }
}

pub fn acquire_ledger_lock(path: &Path) -> Result<LedgerLock> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create session ledger dir {}", parent.display()))?;
    }
    let lock_path = lock_path_for(path);
    let started = Instant::now();
    loop {
        match try_acquire_lock_path(&lock_path)? {
            Some(lock) => return Ok(lock),
            None => {
                if started.elapsed() >= StdDuration::from_secs(5) {
                    return Err(anyhow!(
                        "session ledger is locked at {}",
                        lock_path.display()
                    ));
                }
                thread::sleep(StdDuration::from_millis(50));
            }
        }
    }
}

pub fn try_acquire_ledger_lock(path: &Path) -> Result<Option<LedgerLock>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create session ledger dir {}", parent.display()))?;
    }
    try_acquire_lock_path(&lock_path_for(path))
}

pub fn save_ledger(path: &Path, ledger: &SessionLedger) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create session ledger dir {}", parent.display()))?;
    }
    let tmp = path.with_extension("json.tmp");
    let raw = serde_json::to_string_pretty(ledger).context("serialize session ledger")? + "\n";
    fs::write(&tmp, raw).with_context(|| format!("write session ledger temp {}", tmp.display()))?;
    fs::rename(&tmp, path).with_context(|| {
        format!(
            "replace session ledger {} with {}",
            path.display(),
            tmp.display()
        )
    })?;
    Ok(())
}

fn lock_path_for(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("ledger.json");
    path.with_file_name(format!("{file_name}.lock"))
}

fn try_acquire_lock_path(lock_path: &Path) -> Result<Option<LedgerLock>> {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(lock_path)
    {
        Ok(mut file) => {
            writeln!(
                file,
                "pid={} acquired_at={}",
                std::process::id(),
                Utc::now().to_rfc3339()
            )
            .with_context(|| format!("write session ledger lock {}", lock_path.display()))?;
            Ok(Some(LedgerLock {
                path: lock_path.to_path_buf(),
            }))
        }
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => Ok(None),
        Err(err) => {
            Err(err).with_context(|| format!("create session ledger lock {}", lock_path.display()))
        }
    }
}

pub fn parse_resource(raw: &str) -> Result<ResourceRef> {
    let (kind, id) = raw
        .split_once(':')
        .ok_or_else(|| anyhow!("resource must use kind:id form"))?;
    validate_non_empty("resource kind", kind)?;
    validate_non_empty("resource id", id)?;
    Ok(ResourceRef {
        kind: kind.to_string(),
        id: id.to_string(),
    })
}

pub fn parse_mode(raw: Option<&str>) -> Result<ClaimMode> {
    match raw.unwrap_or("active") {
        "active" => Ok(ClaimMode::Active),
        "watching" => Ok(ClaimMode::Watching),
        "paused" => Ok(ClaimMode::Paused),
        "stale" => Ok(ClaimMode::Stale),
        "released" => Ok(ClaimMode::Released),
        other => Err(anyhow!(
            "unknown claim mode '{other}' (expected active, watching, paused, stale, released)"
        )),
    }
}

pub fn classify_claim(claim: &OccupancyClaim, now: DateTime<Utc>) -> ClaimClassification {
    if claim.released_at.is_some() || claim.mode == ClaimMode::Released {
        ClaimClassification::Released
    } else if claim.expires_at < now || claim.mode == ClaimMode::Stale {
        ClaimClassification::Stale
    } else {
        match claim.mode {
            ClaimMode::Active => ClaimClassification::Active,
            ClaimMode::Watching => ClaimClassification::Watching,
            ClaimMode::Paused => ClaimClassification::Paused,
            ClaimMode::Stale => ClaimClassification::Stale,
            ClaimMode::Released => ClaimClassification::Released,
        }
    }
}

pub fn assess_target_claims(
    ledger: &SessionLedger,
    ledger_path: &Path,
    query: &TargetClaimQuery<'_>,
) -> TargetClaimAssessment {
    let normalized_target_worktree = normalize_path(query.worktree_path);
    let issue_id = query.issue_number.to_string();
    let mut relevant_claims = ledger
        .claims
        .iter()
        .filter_map(|claim| {
            let matches_issue = claim.github.issue == Some(query.issue_number)
                || resource_matches_issue(&claim.resource, &issue_id);
            let matches_branch = claim.branch.as_deref() == Some(query.branch);
            let matches_worktree = claim
                .worktree_path
                .as_deref()
                .map(|raw| {
                    normalize_claim_worktree_path(query.repo_root, raw)
                        == normalized_target_worktree
                })
                .unwrap_or(false);
            if !(matches_issue || matches_branch || matches_worktree) {
                return None;
            }
            Some(TargetClaimMatch {
                claim_id: claim.claim_id.clone(),
                session_id: claim.session_id.clone(),
                owner: claim.owner.clone(),
                resource: claim.resource.clone(),
                mode: claim.mode,
                classification: classify_claim(claim, query.now),
                issue: claim.github.issue,
                branch: claim.branch.clone(),
                worktree_path: claim.worktree_path.clone(),
                heartbeat_at: claim.heartbeat_at,
                expires_at: claim.expires_at,
                released_at: claim.released_at,
                matches_issue,
                matches_branch,
                matches_worktree,
                self_claim: query
                    .current_session_aliases
                    .iter()
                    .any(|session_id| claim.session_id == *session_id),
            })
        })
        .collect::<Vec<_>>();
    relevant_claims.sort_by(|left, right| left.claim_id.cmp(&right.claim_id));

    let has_active_conflict = relevant_claims
        .iter()
        .any(|claim| claim.classification == ClaimClassification::Active && !claim.self_claim);
    let has_stale = relevant_claims
        .iter()
        .any(|claim| claim.classification == ClaimClassification::Stale);
    let has_nonblocking_live_claim = relevant_claims.iter().any(|claim| {
        !claim.self_claim
            && matches!(
                claim.classification,
                ClaimClassification::Watching | ClaimClassification::Paused
            )
    });
    let has_self_claim = relevant_claims.iter().any(|claim| claim.self_claim);
    let has_released = relevant_claims
        .iter()
        .any(|claim| claim.classification == ClaimClassification::Released);

    let (status, block_kind, guidance) = if has_active_conflict {
        (
            "BLOCK",
            "session_active_conflict",
            "Another live session claim already owns this issue/worktree. Do not bind or resume blindly; inspect `adl session status`, coordinate the handoff, then use `adl session heartbeat` or `adl session release` as appropriate.",
        )
    } else if has_stale {
        (
            "WARN",
            "session_stale_claim_manual_inspection",
            "Stale session claims exist for this issue/worktree. Treat them as manual-inspection evidence only; do not auto-clean them up. Inspect with `adl session status`, then create, heartbeat, or release a claim deliberately.",
        )
    } else if has_nonblocking_live_claim {
        (
            "WARN",
            "session_nonblocking_live_claim",
            "A relevant watching or paused session claim exists. Inspect `adl session status` before rebinding so ownership and handoff remain explicit.",
        )
    } else if has_self_claim {
        (
            "PASS",
            "session_self_claim",
            "The current session already owns the relevant claim. Keep it fresh with `adl session heartbeat` and release it with `adl session release` when handing off.",
        )
    } else if has_released {
        (
            "PASS",
            "session_released_history",
            "Only released claims were found for this issue/worktree. You may proceed and create a fresh claim if you need active ownership.",
        )
    } else {
        (
            "PASS",
            "none",
            "No relevant session-ledger claims were found for this issue/worktree.",
        )
    };

    TargetClaimAssessment {
        ledger_path: ledger_path.display().to_string(),
        status,
        block_kind,
        guidance,
        current_session_id: query.current_session_id.map(|value| value.to_string()),
        relevant_claims,
    }
}

pub fn load_target_claim_assessment(
    repo_root: &Path,
    issue_number: u64,
    branch: &str,
    worktree_path: &Path,
    current_session_ids: &[String],
    now: DateTime<Utc>,
) -> Result<TargetClaimAssessment> {
    let ledger_path = default_ledger_path(repo_root);
    let ledger = load_ledger(&ledger_path, now)?;
    let query = TargetClaimQuery {
        repo_root,
        issue_number,
        branch,
        worktree_path,
        current_session_id: current_session_ids.first().map(|value| value.as_str()),
        current_session_aliases: current_session_ids
            .iter()
            .map(|value| value.as_str())
            .collect(),
        now,
    };
    Ok(assess_target_claims(&ledger, &ledger_path, &query))
}

pub fn current_codex_session_ids() -> Vec<String> {
    let mut ids = Vec::new();
    if let Ok(value) = std::env::var("CODEX_SESSION_ID") {
        if !value.trim().is_empty() {
            ids.push(value);
        }
    }
    if let Ok(value) = std::env::var("CODEX_THREAD_ID") {
        if !value.trim().is_empty() && !ids.iter().any(|existing| existing == &value) {
            ids.push(value);
        }
    }
    ids
}

pub fn current_codex_session_id() -> Option<String> {
    current_codex_session_ids().into_iter().next()
}

fn claim_status(claim: &OccupancyClaim, now: DateTime<Utc>) -> ClaimStatus {
    ClaimStatus {
        claim_id: claim.claim_id.clone(),
        session_id: claim.session_id.clone(),
        owner: claim.owner.clone(),
        resource: claim.resource.clone(),
        purpose: claim.purpose.clone(),
        mode: claim.mode,
        classification: classify_claim(claim, now),
        issue: claim.github.issue,
        pull_request: claim.github.pull_request,
        branch: claim.branch.clone(),
        worktree_path: claim.worktree_path.clone(),
        heartbeat_at: claim.heartbeat_at,
        expires_at: claim.expires_at,
        released_at: claim.released_at,
    }
}

fn validate_non_empty(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{label} cannot be empty"))
    } else {
        Ok(())
    }
}

fn resource_matches_issue(resource: &ResourceRef, issue_id: &str) -> bool {
    resource.id == issue_id
        && matches!(
            resource.kind.as_str(),
            "issue" | "github_issue" | "csdlc_issue" | "workflow_issue"
        )
}

fn normalize_claim_worktree_path(repo_root: &Path, raw: &str) -> PathBuf {
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        normalize_path(&path)
    } else {
        normalize_path(&repo_root.join(path))
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    path.components().collect()
}

fn next_claim_id(claims: &[OccupancyClaim], input: &ClaimInput, now: DateTime<Utc>) -> String {
    let base = format!(
        "{}-{}-{}",
        sanitize(&input.resource.kind),
        sanitize(&input.resource.id),
        now.format("%Y%m%dT%H%M%SZ")
    )
    .to_ascii_lowercase();
    let mut counts = BTreeMap::<String, usize>::new();
    for claim in claims {
        *counts.entry(claim.claim_id.clone()).or_default() += 1;
    }
    if !counts.contains_key(&base) {
        return base;
    }
    for n in 2.. {
        let candidate = format!("{base}-{n}");
        if !counts.contains_key(&candidate) {
            return candidate;
        }
    }
    unreachable!("unbounded counter should find an id")
}

fn sanitize(raw: &str) -> String {
    raw.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-06-22T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    fn input(issue: u64) -> ClaimInput {
        ClaimInput {
            session_id: "thread-1".to_string(),
            owner: "codex".to_string(),
            resource: ResourceRef {
                kind: "csdlc_issue".to_string(),
                id: issue.to_string(),
            },
            purpose: "implement issue".to_string(),
            mode: ClaimMode::Active,
            lifecycle_phase: Some("implementation".to_string()),
            policy_ref: Some("AGENTS.md".to_string()),
            github: GithubRef {
                issue: Some(issue),
                pull_request: None,
                repository: Some("danielbaustin/agent-design-language".to_string()),
                last_state: Some("open".to_string()),
            },
            branch: Some(format!("codex/{issue}-example")),
            worktree_path: Some(format!(".worktrees/adl-wp-{issue}")),
            do_not_touch_paths: vec!["adl/Cargo.lock".to_string()],
            blockers: Vec::new(),
            ttl_secs: DEFAULT_TTL_SECS,
        }
    }

    #[test]
    fn claim_status_and_release_round_trip() {
        let mut ledger = SessionLedger::empty(now());
        let claim = ledger.claim(input(4412), now()).expect("claim");
        assert_eq!(claim.claim_id, "csdlc-issue-4412-20260622t000000z");
        let status = ledger.status(Path::new(".adl/session-ledger/ledger.json"), now());
        assert_eq!(status.active_claims, 1);
        assert_eq!(status.claims[0].classification, ClaimClassification::Active);

        let released = ledger
            .release(&claim.claim_id, Some("done".to_string()), now())
            .expect("release");
        assert_eq!(released.mode, ClaimMode::Released);
        let status = ledger.status(Path::new(".adl/session-ledger/ledger.json"), now());
        assert_eq!(status.released_claims, 1);
    }

    #[test]
    fn active_claim_blocks_same_resource_until_stale() {
        let mut ledger = SessionLedger::empty(now());
        ledger.claim(input(4412), now()).expect("claim");
        let err = ledger.claim(input(4412), now()).unwrap_err();
        assert!(err.to_string().contains("already has active claim"));

        let later = now() + Duration::seconds(DEFAULT_TTL_SECS + 1);
        let second = ledger
            .claim(input(4412), later)
            .expect("stale allows reclaim");
        assert!(second.claim_id.ends_with("-20260622t001501z"));
    }

    #[test]
    fn heartbeat_renews_expiry() {
        let mut ledger = SessionLedger::empty(now());
        let claim = ledger.claim(input(4412), now()).expect("claim");
        let later = now() + Duration::seconds(60);
        let renewed = ledger
            .heartbeat(&claim.claim_id, 30, later)
            .expect("heartbeat");
        assert_eq!(renewed.heartbeat_at, later);
        assert_eq!(renewed.expires_at, later + Duration::seconds(30));
    }

    #[test]
    fn heartbeat_preserves_non_active_claim_modes() {
        let mut ledger = SessionLedger::empty(now());
        let mut watching = input(4412);
        watching.mode = ClaimMode::Watching;
        let claim = ledger.claim(watching, now()).expect("watching claim");
        let renewed = ledger
            .heartbeat(&claim.claim_id, 30, now() + Duration::seconds(10))
            .expect("watching heartbeat");
        assert_eq!(renewed.mode, ClaimMode::Watching);

        let mut paused = input(4413);
        paused.mode = ClaimMode::Paused;
        let paused_claim = ledger.claim(paused, now()).expect("paused claim");
        let paused_renewed = ledger
            .heartbeat(&paused_claim.claim_id, 45, now() + Duration::seconds(20))
            .expect("paused heartbeat");
        assert_eq!(paused_renewed.mode, ClaimMode::Paused);
    }

    #[test]
    fn stale_claim_cannot_be_resurrected_after_reclaim() {
        let mut ledger = SessionLedger::empty(now());
        let first = ledger.claim(input(4412), now()).expect("first claim");
        let later = now() + Duration::seconds(DEFAULT_TTL_SECS + 1);
        let second = ledger.claim(input(4412), later).expect("reclaim");
        assert_ne!(first.claim_id, second.claim_id);

        let err = ledger
            .heartbeat(&first.claim_id, DEFAULT_TTL_SECS, later)
            .unwrap_err();
        assert!(err.to_string().contains("is stale"));

        let status = ledger.status(Path::new(".adl/session-ledger/ledger.json"), later);
        assert_eq!(status.active_claims, 1);
        assert_eq!(status.stale_claims, 1);
    }

    #[test]
    fn resource_requires_kind_and_id() {
        assert_eq!(
            parse_resource("csdlc_issue:4412").unwrap(),
            ResourceRef {
                kind: "csdlc_issue".to_string(),
                id: "4412".to_string()
            }
        );
        assert!(parse_resource("4412").is_err());
    }

    #[test]
    fn ledger_lock_blocks_second_mutator() {
        let path = std::env::temp_dir()
            .join(format!(
                "adl-session-ledger-lock-test-{}",
                std::process::id()
            ))
            .join("ledger.json");
        let lock = acquire_ledger_lock(&path).expect("first lock");
        let second = try_acquire_ledger_lock(&path).expect("second lock attempt");
        assert!(second.is_none());
        drop(lock);
        let _lock = try_acquire_ledger_lock(&path)
            .expect("third lock attempt")
            .expect("lock released");
        let _ = fs::remove_dir_all(path.parent().expect("temp parent"));
    }

    #[test]
    fn target_claim_assessment_reports_no_claims() {
        let repo_root = PathBuf::from("/repo");
        let ledger = SessionLedger::empty(now());
        let query = TargetClaimQuery {
            repo_root: &repo_root,
            issue_number: 4419,
            branch: "codex/4419-test",
            worktree_path: &repo_root.join(".worktrees/adl-wp-4419"),
            current_session_id: Some("thread-1"),
            current_session_aliases: vec!["thread-1"],
            now: now(),
        };

        let assessment = assess_target_claims(&ledger, &default_ledger_path(&repo_root), &query);

        assert_eq!(assessment.status, "PASS");
        assert_eq!(assessment.block_kind, "none");
        assert!(assessment.relevant_claims.is_empty());
    }

    #[test]
    fn target_claim_assessment_blocks_other_active_claims() {
        let repo_root = PathBuf::from("/repo");
        let mut ledger = SessionLedger::empty(now());
        let mut other = input(4419);
        other.session_id = "thread-2".to_string();
        other.branch = Some("codex/4419-test".to_string());
        other.worktree_path = Some(".worktrees/adl-wp-4419".to_string());
        ledger.claim(other, now()).expect("claim");
        let query = TargetClaimQuery {
            repo_root: &repo_root,
            issue_number: 4419,
            branch: "codex/4419-test",
            worktree_path: &repo_root.join(".worktrees/adl-wp-4419"),
            current_session_id: Some("thread-1"),
            current_session_aliases: vec!["thread-1"],
            now: now(),
        };

        let assessment = assess_target_claims(&ledger, &default_ledger_path(&repo_root), &query);

        assert_eq!(assessment.status, "BLOCK");
        assert_eq!(assessment.block_kind, "session_active_conflict");
        assert_eq!(assessment.relevant_claims.len(), 1);
        assert!(!assessment.relevant_claims[0].self_claim);
    }

    #[test]
    fn target_claim_assessment_warns_for_stale_claims() {
        let repo_root = PathBuf::from("/repo");
        let mut ledger = SessionLedger::empty(now());
        let mut stale = input(4419);
        stale.session_id = "thread-2".to_string();
        stale.branch = Some("codex/4419-test".to_string());
        stale.worktree_path = Some(".worktrees/adl-wp-4419".to_string());
        let claim = ledger.claim(stale, now()).expect("claim");
        let stale_time = claim.expires_at + Duration::seconds(1);
        let query = TargetClaimQuery {
            repo_root: &repo_root,
            issue_number: 4419,
            branch: "codex/4419-test",
            worktree_path: &repo_root.join(".worktrees/adl-wp-4419"),
            current_session_id: Some("thread-1"),
            current_session_aliases: vec!["thread-1"],
            now: stale_time,
        };

        let assessment = assess_target_claims(&ledger, &default_ledger_path(&repo_root), &query);

        assert_eq!(assessment.status, "WARN");
        assert_eq!(
            assessment.block_kind,
            "session_stale_claim_manual_inspection"
        );
        assert_eq!(
            assessment.relevant_claims[0].classification,
            ClaimClassification::Stale
        );
    }

    #[test]
    fn target_claim_assessment_allows_self_claims() {
        let repo_root = PathBuf::from("/repo");
        let mut ledger = SessionLedger::empty(now());
        let mut mine = input(4419);
        mine.branch = Some("codex/4419-test".to_string());
        mine.worktree_path = Some(".worktrees/adl-wp-4419".to_string());
        ledger.claim(mine, now()).expect("claim");
        let query = TargetClaimQuery {
            repo_root: &repo_root,
            issue_number: 4419,
            branch: "codex/4419-test",
            worktree_path: &repo_root.join(".worktrees/adl-wp-4419"),
            current_session_id: Some("thread-1"),
            current_session_aliases: vec!["thread-1"],
            now: now(),
        };

        let assessment = assess_target_claims(&ledger, &default_ledger_path(&repo_root), &query);

        assert_eq!(assessment.status, "PASS");
        assert_eq!(assessment.block_kind, "session_self_claim");
        assert!(assessment.relevant_claims[0].self_claim);
    }

    #[test]
    fn target_claim_assessment_treats_thread_and_session_ids_as_same_current_owner() {
        let repo_root = PathBuf::from("/repo");
        let mut ledger = SessionLedger::empty(now());
        let mut mine = input(4419);
        mine.session_id = "thread-1".to_string();
        mine.branch = Some("codex/4419-test".to_string());
        mine.worktree_path = Some(".worktrees/adl-wp-4419".to_string());
        ledger.claim(mine, now()).expect("claim");
        let query = TargetClaimQuery {
            repo_root: &repo_root,
            issue_number: 4419,
            branch: "codex/4419-test",
            worktree_path: &repo_root.join(".worktrees/adl-wp-4419"),
            current_session_id: Some("session-1"),
            current_session_aliases: vec!["session-1", "thread-1"],
            now: now(),
        };

        let assessment = assess_target_claims(&ledger, &default_ledger_path(&repo_root), &query);

        assert_eq!(assessment.status, "PASS");
        assert_eq!(assessment.block_kind, "session_self_claim");
        assert!(assessment.relevant_claims[0].self_claim);
    }
}
