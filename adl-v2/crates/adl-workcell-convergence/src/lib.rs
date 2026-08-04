//! Pure convergence of conductor assignments and bounded task outputs.

mod hygiene;
mod model;

use hygiene::{normalize_path, normalize_paths, path_contains, paths_overlap, reject_secret};
use hygiene::{validate_digest, validate_revision};
pub use model::*;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

const DECISION_DOMAIN: &[u8] = b"adl.workcell-convergence.decision.v1\0";

pub fn converge(input: ConvergenceInput) -> Result<ConvergenceEnvelope, ConvergenceError> {
    validate_envelope(&input)?;
    let assignments = normalize_assignments(&input.assignments)?;
    let outputs_by_issue = normalize_outputs(input.outputs.clone())?;
    let mut outputs = outputs_by_issue.clone();
    let active_claims = normalize_active_claims(&input.active_claims)?;
    let mut blockers = Vec::new();
    let mut changed_assumptions = Vec::new();
    let mut integrated = Vec::new();
    let mut partial_successes = Vec::new();
    let mut remaining = BTreeSet::new();

    if !decision_authority_is_read_only(&input.authority) {
        blockers.push(blocker(
            BlockerCode::HiddenMutationAuthority,
            0,
            "convergence authority includes mutation capability",
        ));
    }

    validate_active_claims(&active_claims, &assignments, &mut blockers)?;

    for (issue, assignment) in &assignments {
        let matched = outputs.remove(issue);
        let Some(output) = matched else {
            blockers.push(blocker(
                BlockerCode::MissingOutput,
                *issue,
                "assignment has no bounded task output",
            ));
            remaining.insert(*issue);
            continue;
        };

        let mut issue_blockers =
            validate_output_binding(&input.source_revision, assignment, &output)?;
        if output.status != OutputStatus::Succeeded
            && output.blockers.is_empty()
            && output.changed_assumptions.is_empty()
        {
            issue_blockers.push(blocker(
                BlockerCode::ResidualBlocker,
                *issue,
                "non-succeeded output requires remaining work",
            ));
        }
        let has_blockers = !issue_blockers.is_empty() || !output.blockers.is_empty();
        blockers.extend(issue_blockers);
        blockers.extend(output.blockers.clone());
        changed_assumptions.extend(output.changed_assumptions.clone());

        if output.status == OutputStatus::Succeeded
            && !has_blockers
            && output.changed_assumptions.is_empty()
        {
            let step = step_from_output(&output);
            partial_successes.push(step.clone());
            integrated.push(step);
        } else if output.status == OutputStatus::Partial && !output.artifacts.is_empty() {
            partial_successes.push(step_from_output(&output));
        } else {
            remaining.insert(*issue);
        }
        if output.status == OutputStatus::Partial {
            remaining.insert(*issue);
        }
    }

    for issue in outputs.keys() {
        blockers.push(blocker(
            BlockerCode::ForgedBinding,
            *issue,
            "task output has no conductor assignment",
        ));
        remaining.insert(*issue);
    }

    integrated.sort_by_key(|step| order_key(&assignments, step.issue));
    partial_successes.sort_by_key(|step| order_key(&assignments, step.issue));
    partial_successes.dedup();
    blockers.sort();
    blockers.dedup();
    changed_assumptions.sort();
    changed_assumptions.dedup();
    let remaining_issues: Vec<u64> = remaining.into_iter().collect();

    let projection = ReadOnlyProjection {
        source_revision: input.source_revision.clone(),
        partial_successes,
        integrated: integrated.clone(),
        residual_blockers: blockers.clone(),
        remaining_issues: remaining_issues.clone(),
    };

    let decision = if !blockers.is_empty() {
        ConvergenceDecision::Blocked(BlockedRecord {
            source_revision: input.source_revision.clone(),
            blockers,
            integrated_issues: integrated.iter().map(|step| step.issue).collect(),
        })
    } else if !changed_assumptions.is_empty() {
        ConvergenceDecision::Replan(ReplanRecord {
            source_revision: input.source_revision.clone(),
            changed_assumptions,
            admissible_remaining_work: remaining_issues,
            integrated_issues: integrated.iter().map(|step| step.issue).collect(),
        })
    } else {
        ConvergenceDecision::Integrate(IntegrationPlan {
            source_revision: input.source_revision.clone(),
            authority: input.authority.declared_integration_authority.clone(),
            steps: integrated,
        })
    };

    let decision_id = decision_id(
        &input,
        &assignments,
        &outputs_by_issue,
        &active_claims,
        &decision,
        &projection,
    )?;
    Ok(ConvergenceEnvelope {
        contract: CONVERGENCE_CONTRACT_VERSION.into(),
        decision_id,
        decision,
        projection,
    })
}

fn validate_envelope(input: &ConvergenceInput) -> Result<(), ConvergenceError> {
    if input.contract != CONVERGENCE_CONTRACT_VERSION {
        return Err(ConvergenceError::new(
            ConvergenceErrorCode::InvalidContract,
            "unsupported convergence contract",
        ));
    }
    if input.source_revision.trim().is_empty()
        || input.correlation_seed.trim().is_empty()
        || input.authority.subject.trim().is_empty()
        || input
            .authority
            .declared_integration_authority
            .trim()
            .is_empty()
        || input.assignments.is_empty()
    {
        return Err(ConvergenceError::new(
            ConvergenceErrorCode::InvalidInput,
            "source revision, seed, authority, and assignments are required",
        ));
    }
    validate_revision(&input.source_revision)?;
    reject_secret(&input.source_revision)?;
    reject_secret(&input.correlation_seed)?;
    reject_secret(&input.authority.subject)?;
    reject_secret(&input.authority.declared_integration_authority)?;
    Ok(())
}

fn normalize_assignments(
    assignments: &[adl_workcell_conductor::TaskAssignment],
) -> Result<BTreeMap<u64, adl_workcell_conductor::TaskAssignment>, ConvergenceError> {
    let mut normalized = BTreeMap::new();
    for mut assignment in assignments.iter().cloned() {
        validate_revision(&assignment.source_revision)?;
        reject_secret(&assignment.claim_id)?;
        reject_secret(&assignment.branch)?;
        reject_secret(&assignment.worktree)?;
        assignment.protected_paths = normalize_paths(&assignment.protected_paths)?;
        assignment.write_paths = normalize_paths(&assignment.write_paths)?;
        assignment.expected_outputs = normalize_paths(&assignment.expected_outputs)?;
        assignment.artifact_refs_are_in_scope()?;
        if normalized.insert(assignment.issue, assignment).is_some() {
            return Err(ConvergenceError::new(
                ConvergenceErrorCode::InvalidInput,
                "duplicate conductor assignment",
            ));
        }
    }
    Ok(normalized)
}

trait AssignmentScope {
    fn artifact_refs_are_in_scope(&self) -> Result<(), ConvergenceError>;
}

impl AssignmentScope for adl_workcell_conductor::TaskAssignment {
    fn artifact_refs_are_in_scope(&self) -> Result<(), ConvergenceError> {
        for write_path in &self.write_paths {
            if !self
                .protected_paths
                .iter()
                .any(|protected_path| path_contains(protected_path, write_path))
            {
                return Err(ConvergenceError::new(
                    ConvergenceErrorCode::InvalidPath,
                    format!("write path `{write_path}` is outside protected paths"),
                ));
            }
        }
        Ok(())
    }
}

fn normalize_outputs(
    outputs: Vec<TaskOutput>,
) -> Result<BTreeMap<u64, TaskOutput>, ConvergenceError> {
    let mut normalized = BTreeMap::new();
    for mut output in outputs {
        validate_revision(&output.source_revision)?;
        reject_secret(&output.claim_id)?;
        reject_secret(&output.branch)?;
        reject_secret(&output.worktree)?;
        output.protected_paths = normalize_paths(&output.protected_paths)?;
        output.write_paths = normalize_paths(&output.write_paths)?;
        for artifact in &mut output.artifacts {
            artifact.path = normalize_path(&artifact.path)?;
            validate_digest(&artifact.digest)?;
        }
        output.artifacts.sort();
        output.artifacts.dedup();
        output.validation_refs = normalize_refs(&output.validation_refs)?;
        output.review_refs = normalize_refs(&output.review_refs)?;
        output.blockers = normalize_blockers(&output.blockers)?;
        output.changed_assumptions = normalize_changed_assumptions(&output.changed_assumptions)?;
        if let Some(previous) = normalized.insert(output.issue, output.clone()) {
            if previous != output {
                return Err(ConvergenceError::new(
                    ConvergenceErrorCode::InvalidInput,
                    "duplicate task output has conflicting content",
                ));
            }
        }
    }
    Ok(normalized)
}

fn validate_output_binding(
    source_revision: &str,
    assignment: &adl_workcell_conductor::TaskAssignment,
    output: &TaskOutput,
) -> Result<Vec<Blocker>, ConvergenceError> {
    let mut blockers = Vec::new();
    if assignment.source_revision != source_revision || output.source_revision != source_revision {
        blockers.push(blocker(
            BlockerCode::StaleOutput,
            assignment.issue,
            "assignment and output must match the envelope source revision",
        ));
    }
    if output.claim_id != assignment.claim_id
        || output.branch != assignment.branch
        || output.worktree != assignment.worktree
        || output.source_revision != assignment.source_revision
        || output.assignment_digest != assignment.execution_plan_digest
        || output.protected_paths != assignment.protected_paths
        || output.write_paths != assignment.write_paths
    {
        blockers.push(blocker(
            BlockerCode::ForgedBinding,
            assignment.issue,
            "output does not match conductor assignment binding",
        ));
    }
    validate_artifacts(assignment, output, &mut blockers);
    if output.status == OutputStatus::Succeeded
        && (output.validation_refs.is_empty() || output.review_refs.is_empty())
    {
        blockers.push(blocker(
            BlockerCode::AmbiguousReview,
            assignment.issue,
            "succeeded output requires validation and review references",
        ));
    }
    Ok(blockers)
}

fn validate_artifacts(
    assignment: &adl_workcell_conductor::TaskAssignment,
    output: &TaskOutput,
    blockers: &mut Vec<Blocker>,
) {
    let expected: BTreeSet<_> = assignment.expected_outputs.iter().cloned().collect();
    let observed: BTreeSet<_> = output
        .artifacts
        .iter()
        .map(|artifact| artifact.path.clone())
        .collect();
    for missing in expected.difference(&observed) {
        blockers.push(blocker(
            BlockerCode::MissingArtifact,
            assignment.issue,
            format!("missing declared artifact `{missing}`"),
        ));
    }
    for extra in observed.difference(&expected) {
        blockers.push(blocker(
            BlockerCode::OutOfScopeArtifact,
            assignment.issue,
            format!("undeclared artifact `{extra}`"),
        ));
    }
    for artifact in &output.artifacts {
        if !assignment
            .write_paths
            .iter()
            .any(|write_path| path_contains(write_path, &artifact.path))
        {
            blockers.push(blocker(
                BlockerCode::OutOfScopeArtifact,
                assignment.issue,
                format!(
                    "artifact `{}` is outside declared write paths",
                    artifact.path
                ),
            ));
        }
    }
}

fn validate_active_claims(
    claims: &[ActiveClaim],
    assignments: &BTreeMap<u64, adl_workcell_conductor::TaskAssignment>,
    blockers: &mut Vec<Blocker>,
) -> Result<(), ConvergenceError> {
    for claim in claims {
        let protected_paths = normalize_paths(&claim.protected_paths)?;
        for assignment in assignments.values() {
            if claim.claim_id == assignment.claim_id {
                continue;
            }
            for left in &protected_paths {
                for right in assignment
                    .protected_paths
                    .iter()
                    .chain(assignment.write_paths.iter())
                {
                    if paths_overlap(left, right) {
                        blockers.push(blocker(
                            BlockerCode::PathOverlap,
                            assignment.issue,
                            format!(
                                "active claim {} for issue {} overlaps `{right}`",
                                claim.claim_id, claim.issue
                            ),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn normalize_active_claims(claims: &[ActiveClaim]) -> Result<Vec<ActiveClaim>, ConvergenceError> {
    let mut normalized = Vec::with_capacity(claims.len());
    for claim in claims {
        reject_secret(&claim.claim_id)?;
        normalized.push(ActiveClaim {
            issue: claim.issue,
            claim_id: claim.claim_id.clone(),
            protected_paths: normalize_paths(&claim.protected_paths)?,
        });
    }
    normalized.sort_by(|left, right| {
        (left.issue, &left.claim_id, &left.protected_paths).cmp(&(
            right.issue,
            &right.claim_id,
            &right.protected_paths,
        ))
    });
    normalized.dedup();
    Ok(normalized)
}

fn step_from_output(output: &TaskOutput) -> IntegrationStep {
    IntegrationStep {
        issue: output.issue,
        claim_id: output.claim_id.clone(),
        branch: output.branch.clone(),
        source_revision: output.source_revision.clone(),
        artifacts: output.artifacts.clone(),
        validation_refs: output.validation_refs.clone(),
        review_refs: output.review_refs.clone(),
    }
}

fn decision_authority_is_read_only(authority: &ConvergenceAuthority) -> bool {
    authority.may_decide
        && !authority.may_create_task
        && !authority.may_mutate_github
        && !authority.may_write_filesystem
        && !authority.may_mutate_lifecycle
}

fn decision_id(
    input: &ConvergenceInput,
    assignments: &BTreeMap<u64, adl_workcell_conductor::TaskAssignment>,
    outputs: &BTreeMap<u64, TaskOutput>,
    active_claims: &[ActiveClaim],
    decision: &ConvergenceDecision,
    projection: &ReadOnlyProjection,
) -> Result<String, ConvergenceError> {
    #[derive(Serialize)]
    struct Identity<'a> {
        contract: &'a str,
        source_revision: &'a str,
        correlation_seed: &'a str,
        authority: &'a ConvergenceAuthority,
        assignments: &'a BTreeMap<u64, adl_workcell_conductor::TaskAssignment>,
        outputs: &'a BTreeMap<u64, TaskOutput>,
        active_claims: &'a [ActiveClaim],
        decision: &'a ConvergenceDecision,
        projection: &'a ReadOnlyProjection,
    }
    let bytes = serde_json::to_vec(&Identity {
        contract: &input.contract,
        source_revision: &input.source_revision,
        correlation_seed: &input.correlation_seed,
        authority: &input.authority,
        assignments,
        outputs,
        active_claims,
        decision,
        projection,
    })
    .map_err(|error| {
        ConvergenceError::new(
            ConvergenceErrorCode::Serialization,
            format!("decision identity serialization failed: {error}"),
        )
    })?;
    let mut hasher = blake3::Hasher::new();
    hasher.update(DECISION_DOMAIN);
    hasher.update(&bytes);
    Ok(hasher.finalize().to_hex().to_string())
}

fn normalize_refs(values: &[String]) -> Result<Vec<String>, ConvergenceError> {
    let mut refs = BTreeSet::new();
    for value in values {
        if value.trim().is_empty() || value.contains("://") || value.chars().any(char::is_control) {
            return Err(ConvergenceError::new(
                ConvergenceErrorCode::InvalidInput,
                "evidence references must be local, non-empty, and printable",
            ));
        }
        reject_secret(value)?;
        refs.insert(normalize_path(value)?);
    }
    Ok(refs.into_iter().collect())
}

fn normalize_blockers(blockers: &[Blocker]) -> Result<Vec<Blocker>, ConvergenceError> {
    let mut normalized = Vec::with_capacity(blockers.len());
    for blocker in blockers {
        if blocker.message.trim().is_empty() {
            return Err(ConvergenceError::new(
                ConvergenceErrorCode::InvalidInput,
                "blocker messages must be non-empty",
            ));
        }
        reject_secret(&blocker.message)?;
        normalized.push(Blocker {
            code: blocker.code,
            issue: blocker.issue,
            message: blocker.message.clone(),
            evidence_refs: normalize_refs(&blocker.evidence_refs)?,
        });
    }
    normalized.sort();
    normalized.dedup();
    Ok(normalized)
}

fn normalize_changed_assumptions(
    assumptions: &[ChangedAssumption],
) -> Result<Vec<ChangedAssumption>, ConvergenceError> {
    let mut normalized = Vec::with_capacity(assumptions.len());
    for assumption in assumptions {
        for value in [&assumption.key, &assumption.expected, &assumption.observed] {
            if value.trim().is_empty() || value.chars().any(char::is_control) {
                return Err(ConvergenceError::new(
                    ConvergenceErrorCode::InvalidInput,
                    "changed assumption fields must be non-empty printable strings",
                ));
            }
            reject_secret(value)?;
        }
        normalized.push(assumption.clone());
    }
    normalized.sort();
    normalized.dedup();
    Ok(normalized)
}

fn blocker(code: BlockerCode, issue: u64, message: impl Into<String>) -> Blocker {
    Blocker {
        code,
        issue,
        message: message.into(),
        evidence_refs: Vec::new(),
    }
}

fn order_key(
    assignments: &BTreeMap<u64, adl_workcell_conductor::TaskAssignment>,
    issue: u64,
) -> (usize, u64) {
    assignments
        .get(&issue)
        .map(|assignment| (assignment.wave, assignment.issue))
        .unwrap_or((usize::MAX, issue))
}
