# Issue 5842 Design: v0.92 Quality Gate

Status: design-time ready; gate execution waits for all declared predecessors.

## Authority And Sources

Issue #5842, `docs/milestones/v0.92/QUALITY_GATE_v0.92.md`,
`docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md`, the feature index at
`docs/milestones/v0.92/features/README.md`, the WBS, issue-wave YAML, demo
matrix, checklist, and canonical typed child records define the gate. The
current coverage matrix is planning truth: most rows remain `planned`; it is
not evidence that WP-22 has passed.

## Outcome Contract

Build a machine-readable feature-completion matrix with one row for every
indexed v0.92 product feature and every supporting critical path required by
the quality gate. Each accepted row must bind owner issue, PR, reviewed head,
merge commit, implementation paths, validation and negative proof, integration
evidence, platform evidence where claimed, and typed terminal/receipt truth.
Unknown, planned, open, fixture-only, receipt-only, demo-mode, synthetic,
substituted-provider, stale-review, or non-ancestral evidence is blocking.

The gate produces a findings-first blocker report and may update status only
from exact evidence. It does not repair features, waive scope to preserve a
date, or authorize WP-25 internal review while any required row is unaccepted.

## Execution Sequence

1. Verify WP-04, WP-05, WP-06, WP-07, WP-13A, WP-20, WP-21, and WP-21A at
   current live GitHub and typed terminal truth and pin the gate SHA.
2. Enumerate the feature index and supporting quality/release rows without
   silently dropping planned or blocked entries.
3. Resolve each row to exact implementation, review, validation, integration,
   platform, and claim-boundary evidence.
4. Run negative audits for fixture/synthetic/substitution credit, stale SHAs,
   missing ancestry, missing platform proof, and unsupported public claims.
5. Emit the matrix, quality-gate record, and blocker report; fail closed if any
   row is not accepted.
6. Obtain exact-head independent review of the gate logic and every acceptance
   disposition before WP-25 may begin.

## Protected-Path Candidates

- `docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md`
- `docs/milestones/v0.92/QUALITY_GATE_v0.92.md`
- `docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md`
- `docs/reviews/v0.92/quality-gate-5842`
- `.csdlc/evidence/5842`

Feature implementation paths remain read-only inputs. Any needed repair is a
blocker routed to its owner, not absorbed into the WP-22 claim.

## Validation And Failure Policy

Required lanes are feature-index completeness/schema checks, issue/PR/head/
merge/ancestry and typed-terminal cross-checks, positive proof-link validation,
negative rejection fixtures for every prohibited evidence class, platform and
provider-identity consistency checks, docs/YAML/link validation, and exact-head
review. One missing or ambiguous row makes the gate fail; the report must name
the blocker and owner without advancing internal review.

## Non-Goals

- No product remediation, evidence invention, scope waiver, or issue closure.
- No release readiness inferred from GitHub closure or receipts alone.
- No WP-23 docs rewrite, WP-25 review execution, or downstream ceremony.
