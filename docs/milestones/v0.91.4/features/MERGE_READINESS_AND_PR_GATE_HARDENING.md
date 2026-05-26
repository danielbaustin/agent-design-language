# Merge-Readiness And PR Gate Hardening

## Status

Landed in `WP-07`.

## Purpose

Harden the C-SDLC merge-readiness gate so default operation preserves GitHub
issue, PR, branch, CI, review, and closeout truth.

C-SDLC should make merge readiness easier to inspect, not easier to overclaim.
The gate must fail closed when evidence is missing, stale, local-only, or
contradictory.

## Scope

`v0.91.4` should harden:

- transition-aware PR readiness records
- local validation versus remote check status
- review finding disposition requirements
- evidence bundle and signed trace linkage
- branch/worktree/PR identity checks
- closeout preconditions
- blocked-state diagnostics when merge readiness cannot be claimed

## Acceptance Criteria

- A transition cannot be marked merge-ready without current `SRP`, `SOR`,
  evidence bundle, and PR truth.
- Local-only validation is not confused with remote CI or protected-branch
  readiness.
- Review findings must be fixed, accepted with rationale, or routed before the
  gate passes.
- Signed trace proof is linked when durable C-SDLC proof is claimed.
- The demo matrix includes a PR-gate hardening proof surface.

## Proof Surface

- packet:
  `docs/milestones/v0.91.4/review/merge_readiness/MERGE_READINESS_GATE_PACKET_v0.91.4.md`
- report:
  `docs/milestones/v0.91.4/review/merge_readiness/ct_demo_001_merge_gate_profile_report.md`
- snapshot:
  `docs/milestones/v0.91.4/review/merge_readiness/ct_demo_001_merge_gate_snapshot.json`
- tooling policy:
  `docs/tooling/merge_readiness_gate_policy_v0.91.4.md`
- validator:
  `python3 adl/tools/validate_v0914_merge_readiness_gate.py docs/milestones/v0.91.4/review/merge_readiness`
- contract test:
  `bash adl/tools/test_v0914_merge_readiness_gate.sh`

The bounded proof here is intentionally about truthful gate behavior. It proves
that focused merge-readiness validation covers the real `pr_cmd` subtree and
continues to block stale lifecycle truth. It also records, rather than newly
automates, the boundary that remote merge state must not be inferred from local
validation alone.

## Non-Goals

- This feature does not merge PRs automatically.
- This feature does not bypass branch protection or human review.
- This feature does not turn five-minute-sprint speed into merge permission.
