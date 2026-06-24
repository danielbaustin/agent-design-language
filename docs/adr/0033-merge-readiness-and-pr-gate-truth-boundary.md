# ADR 0033: Merge-Readiness And PR Gate Truth Boundary

- Status: Accepted
- Date: 2026-06-23
- Accepted in: v0.91.6
- Candidate source: docs/architecture/adr/0033-merge-readiness-and-pr-gate-truth-boundary.md
- Target milestone: v0.91.4
- Related issues: #3444
- Related ADRs: ADR 0024, ADR 0028, ADR 0029
- Source evidence:
  - `docs/milestones/v0.91.4/features/MERGE_READINESS_AND_PR_GATE_HARDENING.md`
  - `docs/tooling/merge_readiness_gate_policy_v0.91.4.md`
  - `docs/milestones/v0.91.4/review/merge_readiness/MERGE_READINESS_GATE_PACKET_v0.91.4.md`

## Context

C-SDLC should make merge readiness easier to inspect, not easier to overclaim.
Past process failures included local validation being mistaken for remote CI,
PR state drifting from card truth, review findings not being resolved before
publication, and closeout overclaiming final state.

Merge readiness is not one fact. It is the convergence of issue state, branch
identity, worktree state, PR state, review state, CI state, validation evidence,
trace evidence, and closeout truth.

## Decision

ADL should treat merge-readiness and PR-gate truth as a distinct architecture
boundary.

The gate requires:

- current `SRP` review truth
- current `SOR` execution and integration truth
- evidence bundle linkage where durable C-SDLC proof is claimed
- branch/worktree/PR identity checks
- clear distinction between local validation and remote protected-branch
  readiness
- review findings fixed, accepted with rationale, or routed before merge
- signed trace linkage where default-operation proof depends on durable trace
- blocked diagnostics when readiness cannot be claimed

## Consequences

### Positive

- Reduces false "ready" states before review, CI, or closeout truth exists.
- Gives PR janitor and release-tail work clearer failure classes.
- Preserves branch protection and human review while allowing focused local
  proof to remain useful.

### Negative

- Publication and merge preparation require more explicit evidence wiring.
- The gate must resist pressure to treat green local checks as full readiness.
- Some small docs issues may still need card and review truth even when code
  tests are not relevant.

## Alternatives Considered

### Treat local validation as merge readiness

This is tempting when tests are expensive, but it confuses proof surfaces and
can bypass protected-branch truth.

### Treat GitHub PR state as the only source of truth

GitHub is necessary but insufficient. It cannot alone prove card truth, review
finding disposition, signed trace linkage, or closeout readiness.

## Validation Notes

This candidate should be reviewed against merge-readiness proof packets, gate
policy, validator behavior, PR janitor behavior, and release evidence.

## Non-Claims

- This ADR does not merge PRs automatically.
- This ADR does not bypass branch protection or human review.
- This ADR does not require broad Rust tests for every docs-only issue.
- This ADR does not let speed become merge permission.
