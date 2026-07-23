use crate::model::{
    ConductorDecision, ConductorInput, ConductorPlan, ConductorRefusal, IssueSnapshot, Lane,
    RefusalCode, SerializedGate, TaskAssignment, ValidationLane, CONDUCTOR_CONTRACT_VERSION,
};
use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

const EXECUTION_PLAN_CONTRACT: &str = "adl.execution-plan.v1";
const CORRELATION_DOMAIN: &[u8] = b"adl.workcell-conductor.correlation.v1\0";

pub fn plan(input: ConductorInput) -> Result<ConductorDecision, ConductorRefusal> {
    validate_envelope(&input)?;
    let mut issues = normalize_issues(&input)?;
    let waves = dependency_waves(&issues, &input.resolved_dependencies)?;
    validate_paths(&issues)?;
    validate_wip(&input, issues.len())?;

    let plan_digest = input.execution_plan.source_digest.clone();
    let canonical_input = serde_json::to_vec(&CanonicalIdentity::from_input(&input, &issues))
        .map_err(|error| {
            ConductorRefusal::new(
                RefusalCode::InvalidInput,
                "input",
                format!("canonical input encoding failed: {error}"),
            )
        })?;
    let root_correlation = digest(&input.correlation_seed, &canonical_input);

    let mut assignments = Vec::with_capacity(issues.len());
    for (issue_id, issue) in &mut issues {
        let wave = waves[issue_id];
        let wave_size = waves
            .values()
            .filter(|candidate| **candidate == wave)
            .count();
        let claim = issue.claim.as_ref().expect("claim validated");
        let correlation_id = digest(&root_correlation, format!("{issue_id}:{wave}").as_bytes());
        assignments.push(TaskAssignment {
            issue: *issue_id,
            claim_id: claim.id.clone(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            source_revision: issue.source_revision.clone(),
            execution_plan_digest: plan_digest.clone(),
            dependencies: issue.dependencies.clone(),
            protected_paths: claim.protected_paths.clone(),
            write_paths: issue.write_paths.clone(),
            validation_lanes: issue.validation_lanes.clone(),
            expected_outputs: issue.expected_outputs.clone(),
            lane: if wave_size == 1 {
                Lane::Serial
            } else {
                Lane::Parallel
            },
            wave,
            correlation_id,
        });
    }
    assignments.sort_by_key(|assignment| (assignment.wave, assignment.issue));

    Ok(ConductorDecision {
        plan: ConductorPlan {
            contract: String::from(CONDUCTOR_CONTRACT_VERSION),
            source_revision: input.source_revision,
            execution_plan_digest: plan_digest,
            assignments,
            serialized_gates: vec![
                SerializedGate::Review,
                SerializedGate::Publication,
                SerializedGate::Merge,
                SerializedGate::PostMergeValidation,
                SerializedGate::Closeout,
            ],
        },
    })
}

fn validate_envelope(input: &ConductorInput) -> Result<(), ConductorRefusal> {
    if input.contract != CONDUCTOR_CONTRACT_VERSION {
        return Err(ConductorRefusal::new(
            RefusalCode::InvalidContract,
            "input.contract",
            "unsupported conductor contract",
        ));
    }
    if input.execution_plan.contract != EXECUTION_PLAN_CONTRACT {
        return Err(ConductorRefusal::new(
            RefusalCode::InvalidContract,
            "input.execution_plan.contract",
            "unsupported ADL execution-plan contract",
        ));
    }
    if input.source_revision.trim().is_empty()
        || input.correlation_seed.trim().is_empty()
        || input.execution_plan.source_digest.trim().is_empty()
        || input.execution_plan.node_ids.is_empty()
        || input.issues.is_empty()
    {
        return Err(ConductorRefusal::new(
            RefusalCode::InvalidInput,
            "input",
            "source revision, correlation seed, plan identity, nodes, and issues are required",
        ));
    }
    if input.max_writable_assignments == 0
        || input.active_writable_assignments > input.max_writable_assignments
    {
        return Err(ConductorRefusal::new(
            RefusalCode::WipOverflow,
            "input.max_writable_assignments",
            "invalid writable-assignment limit",
        ));
    }
    Ok(())
}

fn normalize_issues(
    input: &ConductorInput,
) -> Result<BTreeMap<u64, IssueSnapshot>, ConductorRefusal> {
    let mut issues = BTreeMap::new();
    for mut issue in input.issues.clone() {
        if issue.issue == 0 || issue.source_revision.trim().is_empty() {
            return Err(ConductorRefusal::new(
                RefusalCode::InvalidInput,
                "input.issues",
                "issue id and source revision are required",
            ));
        }
        if !issue.ready {
            return Err(ConductorRefusal::for_issue(
                RefusalCode::NotReady,
                issue.issue,
                format!("issues.{}.ready", issue.issue),
                "issue is not ready",
                &issue.source_revision,
            ));
        }
        if issue.cards != crate::model::CardKind::required() {
            return Err(ConductorRefusal::for_issue(
                RefusalCode::MissingCards,
                issue.issue,
                format!("issues.{}.cards", issue.issue),
                "exactly the six required cards must be present",
                &issue.source_revision,
            ));
        }
        let claim = issue.claim.as_mut().ok_or_else(|| {
            ConductorRefusal::for_issue(
                RefusalCode::MissingClaim,
                issue.issue,
                format!("issues.{}.claim", issue.issue),
                "active claim is required",
                &issue.source_revision,
            )
        })?;
        if claim.expires_unix_seconds <= input.observed_unix_seconds {
            return Err(ConductorRefusal::for_issue(
                RefusalCode::StaleClaim,
                issue.issue,
                format!("issues.{}.claim.expires_unix_seconds", issue.issue),
                "claim is stale",
                &issue.source_revision,
            ));
        }
        if claim.id.trim().is_empty()
            || claim.owner.trim().is_empty()
            || claim.branch.trim().is_empty()
            || claim.worktree.trim().is_empty()
            || claim.purpose.trim().is_empty()
        {
            return Err(ConductorRefusal::for_issue(
                RefusalCode::AmbiguousAuthority,
                issue.issue,
                format!("issues.{}.claim", issue.issue),
                "claim authority fields must be explicit",
                &issue.source_revision,
            ));
        }
        issue.dependencies.sort_unstable();
        issue.dependencies.dedup();
        issue
            .validation_lanes
            .sort_by(|left, right| left.name.cmp(&right.name));
        issue.expected_outputs.sort();
        issue.expected_outputs.dedup();
        issue.write_paths = normalize_path_set(issue.issue, "write_paths", &issue.write_paths)?;
        claim.protected_paths =
            normalize_path_set(issue.issue, "claim.protected_paths", &claim.protected_paths)?;
        validate_lanes(input, &issue)?;
        if issues.insert(issue.issue, issue.clone()).is_some() {
            return Err(ConductorRefusal::for_issue(
                RefusalCode::DuplicateIssue,
                issue.issue,
                "input.issues",
                "issue appears more than once",
                &issue.source_revision,
            ));
        }
    }
    Ok(issues)
}

fn validate_lanes(input: &ConductorInput, issue: &IssueSnapshot) -> Result<(), ConductorRefusal> {
    let mut seen = BTreeSet::new();
    for lane in &issue.validation_lanes {
        if lane.name.trim().is_empty()
            || lane.argv.is_empty()
            || lane.argv.iter().any(|part| part.trim().is_empty())
            || !seen.insert(lane.name.clone())
            || !input.known_validation_lanes.contains(&lane.name)
        {
            return Err(ConductorRefusal::for_issue(
                RefusalCode::UnknownValidationLane,
                issue.issue,
                format!("issues.{}.validation_lanes", issue.issue),
                "validation lane is unknown, duplicated, or malformed",
                &issue.source_revision,
            ));
        }
    }
    Ok(())
}

fn dependency_waves(
    issues: &BTreeMap<u64, IssueSnapshot>,
    resolved: &BTreeSet<u64>,
) -> Result<BTreeMap<u64, usize>, ConductorRefusal> {
    let mut graph = DiGraphMap::<u64, ()>::new();
    for issue in issues.keys() {
        graph.add_node(*issue);
    }
    for (issue_id, issue) in issues {
        for dependency in &issue.dependencies {
            if issues.contains_key(dependency) {
                graph.add_edge(*dependency, *issue_id, ());
            } else if !resolved.contains(dependency) {
                return Err(ConductorRefusal::for_issue(
                    RefusalCode::UnresolvedDependency,
                    *issue_id,
                    format!("issues.{issue_id}.dependencies"),
                    format!("dependency {dependency} is unresolved"),
                    &issue.source_revision,
                ));
            }
        }
    }
    toposort(&graph, None).map_err(|cycle| {
        let issue = cycle.node_id();
        ConductorRefusal::for_issue(
            RefusalCode::DependencyCycle,
            issue,
            "input.issues.dependencies",
            "dependency graph contains a cycle",
            &issues[&issue].source_revision,
        )
    })?;

    let mut waves = BTreeMap::new();
    let mut remaining: BTreeSet<u64> = issues.keys().copied().collect();
    let mut wave = 0;
    while !remaining.is_empty() {
        let ready: Vec<u64> = remaining
            .iter()
            .copied()
            .filter(|issue_id| {
                issues[issue_id].dependencies.iter().all(|dependency| {
                    resolved.contains(dependency) || waves.contains_key(dependency)
                })
            })
            .collect();
        debug_assert!(!ready.is_empty());
        for issue_id in ready {
            remaining.remove(&issue_id);
            waves.insert(issue_id, wave);
        }
        wave += 1;
    }
    Ok(waves)
}

fn validate_wip(input: &ConductorInput, planned: usize) -> Result<(), ConductorRefusal> {
    let available = input
        .max_writable_assignments
        .saturating_sub(input.active_writable_assignments);
    if planned > available {
        return Err(ConductorRefusal::new(
            RefusalCode::WipOverflow,
            "input.issues",
            format!("{planned} assignments exceed {available} available writable slots"),
        ));
    }
    Ok(())
}

fn normalize_path_set(
    issue: u64,
    field: &str,
    paths: &[String],
) -> Result<Vec<String>, ConductorRefusal> {
    let mut normalized = BTreeSet::new();
    for value in paths {
        let path = Path::new(value);
        if path.is_absolute() || value.trim().is_empty() {
            return Err(invalid_path(issue, field, value));
        }
        let mut parts = Vec::new();
        for component in path.components() {
            match component {
                Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
                _ => return Err(invalid_path(issue, field, value)),
            }
        }
        if parts.is_empty() {
            return Err(invalid_path(issue, field, value));
        }
        normalized.insert(parts.join("/"));
    }
    Ok(normalized.into_iter().collect())
}

fn invalid_path(issue: u64, field: &str, value: &str) -> ConductorRefusal {
    ConductorRefusal::new(
        RefusalCode::InvalidPath,
        format!("issues.{issue}.{field}"),
        format!("path `{value}` is not a normalized repository-relative path"),
    )
}

fn validate_paths(issues: &BTreeMap<u64, IssueSnapshot>) -> Result<(), ConductorRefusal> {
    let mut owned = Vec::new();
    for (issue_id, issue) in issues {
        let claim = issue.claim.as_ref().expect("claim validated");
        for write_path in &issue.write_paths {
            if !claim
                .protected_paths
                .iter()
                .any(|protected_path| path_contains(protected_path, write_path))
            {
                return Err(ConductorRefusal::for_issue(
                    RefusalCode::AmbiguousAuthority,
                    *issue_id,
                    format!("issues.{issue_id}.write_paths"),
                    format!("write path `{write_path}` is outside the active claim"),
                    &issue.source_revision,
                ));
            }
        }
        for path in claim.protected_paths.iter().chain(issue.write_paths.iter()) {
            owned.push((*issue_id, path));
        }
    }
    for left in 0..owned.len() {
        for right in (left + 1)..owned.len() {
            if owned[left].0 != owned[right].0 && paths_overlap(owned[left].1, owned[right].1) {
                return Err(ConductorRefusal::new(
                    RefusalCode::PathCollision,
                    "input.issues",
                    format!(
                        "issues {} and {} overlap at `{}` and `{}`",
                        owned[left].0, owned[right].0, owned[left].1, owned[right].1
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn path_contains(parent: &str, child: &str) -> bool {
    let parent: Vec<_> = parent.split('/').collect();
    let child: Vec<_> = child.split('/').collect();
    child.starts_with(&parent)
}

fn paths_overlap(left: &str, right: &str) -> bool {
    let left: Vec<_> = left.split('/').collect();
    let right: Vec<_> = right.split('/').collect();
    left == right || left.starts_with(&right) || right.starts_with(&left)
}

fn digest(seed: &str, bytes: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(CORRELATION_DOMAIN);
    hasher.update(seed.as_bytes());
    hasher.update(&[0]);
    hasher.update(bytes);
    hasher.finalize().to_hex().to_string()
}

#[derive(Serialize)]
struct CanonicalIdentity<'a> {
    contract: &'a str,
    source_revision: &'a str,
    execution_plan: &'a crate::model::ExecutionPlanSnapshot,
    issues: &'a BTreeMap<u64, IssueSnapshot>,
}

impl<'a> CanonicalIdentity<'a> {
    fn from_input(input: &'a ConductorInput, issues: &'a BTreeMap<u64, IssueSnapshot>) -> Self {
        Self {
            contract: &input.contract,
            source_revision: &input.source_revision,
            execution_plan: &input.execution_plan,
            issues,
        }
    }
}

#[allow(dead_code)]
fn _assert_validation_lane_is_typed(_: &ValidationLane) {}
