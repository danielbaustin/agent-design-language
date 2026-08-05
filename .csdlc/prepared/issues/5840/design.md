# Issue 5840 Design: Demo Matrix, AEE Proof, And Coverage

## Decision

WP-20 is the integrated proof-index issue defined by the live issue wave. It
reconciles every v0.92 demo claim with an exact command, artifact, owner,
status, and accepted revision; routes AEE completion to existing exact evidence
or a clearly blocking packet; and fails closed on uncovered claims. It does not
own repository reduction despite stale prose in the current proof-coverage row.

## Source Baseline

- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`
- `docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.92/QUALITY_GATE_v0.92.md`
- `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md`
- live `WP_ISSUE_WAVE_v0.92.yaml` and issue #5840 override the stale WP-20 reduction row.

## Proposed Artifacts And Protected-Path Candidates

- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`
- `docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md`
- `adl/tools/validate_v092_demo_proof_coverage.py`
- `adl/tools/test_v092_demo_proof_coverage.sh`
- `.csdlc/evidence/5840/`

## Coverage Model

Each claim row records owner issue, dependency state, exact revision, canonical
command, platform/credential posture, positive artifact, negative artifact,
review status, and non-claims. Status is one of `planned`, `blocked`, `failed`,
or `accepted_exact_revision`; no synthetic or receipt-only result can be
promoted to accepted proof.

The AEE row must cite concrete landed evidence and its limits. If current AEE
evidence cannot support the v0.92 claim, WP-20 records a blocker or routes a
bounded packet rather than relabeling planning prose as proof.

## Execution Plan

1. Verify #5836, #5837, #5838, and #5839 are complete at current exact revisions.
2. Correct the stale WP-20 ownership row without changing WP-21/WP-21A reduction ownership.
3. Replace candidate demo entries with exact commands/artifacts only where accepted proof exists.
4. Build the AEE and artifact index with positive, negative, platform, and non-claim columns.
5. Add a validator that rejects missing paths, duplicate ownership, planned-as-passed status, and unsupported claims.
6. Run the validator, focused tests, and exact-head review.

## Negative And Platform Lanes

- Missing artifact, command, exact revision, negative proof, or review remains uncovered.
- Platform-specific proof is labeled; macOS evidence cannot silently satisfy a required Linux/Windows row.
- Credentialed/provider lanes record availability and redaction without embedding secrets.
- A lifecycle receipt, old demo, or fixture cannot substitute for current feature proof.

## Non-Goals

- Implementing feature behavior or rerouting ownership from child issues.
- Repository-wide reduction or Rust refactoring, owned by WP-21/WP-21A.
- Declaring milestone quality, publication, or release complete.
- Editing unrelated milestone or sprint coordination docs.

## Exit Evidence

The matrix, coverage table, AEE disposition, and artifact index agree; the
validator rejects known false-positive states; all accepted rows are exact
revision and independently usable; and review has no actionable finding.
