# v0.91 Coverage and Quality Gate

## Metadata

- Milestone: `v0.91`
- Version: `v0.91`
- Owner: Daniel Austin / Codex
- Canonical issue / WP: `#2753` / `WP-19`
- Scope: milestone quality, coverage, proof-package, review-readiness, and
  exception posture after `WP-18`
- Gate date: 2026-05-07

## Purpose

This document defines the canonical `v0.91` quality gate.

It records the current release-truth surface for:

- tests and CI
- coverage enforcement
- docs validation posture
- demo and feature-proof coverage
- review readiness
- runtime and cost posture for heavy proof lanes
- known exceptions that must remain visible before release closeout

This document does not replace CI. It records what CI, local validators,
milestone docs, and gap-review evidence currently say.

## Gate Summary

`v0.91` has a strong implementation and proof posture through `WP-18`.

The current main-branch merge gate is green after `WP-18`:

- GitHub Actions run
  [25514295183](https://github.com/danielbaustin/agent-design-language/actions/runs/25514295183)
  for merge commit `3d671af4c246cd88c9d766f4ac52697b8a04394f`
- `adl-ci`: success
- `adl-coverage`: success

The remaining blockers are release-tail blockers, not hidden implementation
passes:

- `WP-20` through `WP-25` remain open.
- Internal review, third-party review, accepted-finding remediation, next
  milestone planning, and ceremony are not complete yet.

The two concrete quality-gate gaps found during `WP-19` are now repaired:

- local closed-issue SOR truth passes for the current `v0.91` closed issue set
- release notes now describe landed behavior instead of remaining a draft
  placeholder

## Required vs Documented Exceptions

- **Required** means the item must pass for the relevant phase.
- **Exception documented** means the owner, rationale, and disposition are
  explicit.
- Exceptions do not convert a release blocker into a pass.

## 1) Baseline Repository Merge Gate

The canonical merge-gate workflow is `.github/workflows/ci.yaml`.

The required jobs are:

- `adl-ci`
- `adl-coverage`

Current evidence:

| Evidence | Result | Notes |
| --- | --- | --- |
| PR `#2806` (`WP-18`) | PASS | `adl-ci` and `adl-coverage` succeeded before merge. |
| Main run `25514295183` | PASS | Merge commit `3d671af4c246cd88c9d766f4ac52697b8a04394f` completed green. |
| `adl-ci` on main run `25514295183` | PASS | fmt, clippy, docs command check, CI contracts, and demo smoke completed successfully. |
| `adl-coverage` on main run `25514295183` | PASS | Full coverage lane completed and enforced workspace/per-file gates. |

### `adl-ci` details

The main run completed the following relevant gates successfully:

- tooling sanity checks
- coverage-impact and path-policy contracts
- PR-fast and authoritative coverage lane contracts
- issue/PR linkage guardrail
- repo-review, test-generator, demo-operator, and arXiv skill contract checks
- Rust fmt and clippy
- docs command check
- CI runtime/cache contract checks
- demo smoke `S-01..S-05`

The ordinary test step was skipped because the full coverage lane covered the
Rust test execution for this run.

## 2) Coverage Posture Gate

The current coverage gate is `adl-coverage`.

It enforces:

- workspace line coverage threshold: `90%`
- per-file line coverage threshold: `80%`
- active exclusion regex: `^$`

Current main-run evidence from run `25514295183`:

| Coverage surface | Result |
| --- | --- |
| Coverage test run | 1813 tests run, 1813 passed, 2 skipped |
| Workspace line coverage | 90.37% |
| Workspace threshold | 90% |
| Per-file threshold | Passed, `>= 80%` after documented exclusions |
| Generated lcov artifact | Verified and uploaded |

### Runtime and cost posture

The authoritative coverage lane is heavy and remains a real runtime cost:

- It ran 1813 tests in 549.294 seconds inside the coverage step.
- The full `adl-coverage` job took about 12 minutes and 13 seconds.
- That cost is acceptable for the main authoritative gate, but it should not be
  multiplied into redundant release-tail ceremonies unless a changed surface
  requires remeasurement.

The release-tail rule for `v0.91` is:

- use the latest green main authoritative coverage run as the current coverage
  authority
- re-run full coverage only when code, tests, demos, runtime behavior, or
  coverage tooling changes
- for docs-only closeout WPs, run docs/card/link/path validations and rely on
  the latest green main coverage evidence unless CI requires more

## 3) Documentation and Version Truth

Current tracked docs evidence:

- `adl/Cargo.toml` is versioned `0.91.0`.
- `CHANGELOG.md` records `v0.91` as active and the crate version as `0.91.0`.
- `README.md` records `v0.91` as the active milestone and `0.91.0` as the
  current crate version.
- `docs/milestones/v0.91/README.md` names the active milestone package and
  issue wave.
- `docs/milestones/v0.91/DEMO_MATRIX_v0.91.md` contains rows `D1` through
  `D14`.
- `docs/milestones/v0.91/FEATURE_PROOF_COVERAGE_v0.91.md` maps every tracked
  `v0.91` feature surface to a demo, proof route, fixture-backed validation, or
  explicit deferral.
- `docs/milestones/v0.91/CARD_BUNDLE_READINESS_v0.91.md` records the
  post-repair card-bundle readiness proof.
- `docs/milestones/v0.91/SPP_READINESS_v0.91.md` records the bounded SPP
  readiness slice and defers mass SPP generation to the editor-skill path.

Release-note posture:

- `docs/milestones/v0.91/RELEASE_NOTES_v0.91.md` has been rewritten from a
  planned placeholder into landed-behavior notes with explicit remaining
  release-tail status.

## 4) Demo and Feature-Proof Gate

`WP-18` materially completed the first proof convergence layer.

Current proof surfaces:

- `docs/milestones/v0.91/DEMO_MATRIX_v0.91.md`
- `docs/milestones/v0.91/FEATURE_PROOF_COVERAGE_v0.91.md`
- `docs/milestones/v0.91/features/README.md`
- `demos/v0.91/cognitive_being_flagship_demo.md`
- `adl/src/runtime_v2/cognitive_being_flagship_demo.rs`
- `adl/src/runtime_v2/tests/cognitive_being_flagship_demo.rs`

Gate interpretation:

- `D1` through `D13` are landed feature/demo proof routes.
- `D14` is the reviewer-facing coverage map added by `WP-18`.
- The matrix explicitly preserves non-claims: no production moral agency, no
  legal personhood, no consciousness claim, no first true birthday, no scalar
  karma/happiness, no public wellbeing surveillance, and no external
  cross-polis communication without TLS/mTLS-equivalent transport.

## 5) Gap-Review Disposition

The local TBD gap analysis used by this gate is:

- `.adl/docs/TBD/v0.91_gap_review.md`

The gap analysis verdict remains substantially correct after `WP-18`:

- core implementation and proof quality are strong
- release-facing closeout is not done
- the remaining risk is concentrated in quality, docs, review, remediation, and
  ceremony work

Disposition:

| Gap item | Current disposition |
| --- | --- |
| `WP-18` proof convergence | Resolved by merged PR `#2806`; demo matrix and feature-proof coverage are now tracked. |
| `WP-19` quality gate missing | Resolved by this document. |
| Closed-issue SOR truth drift | Resolved locally for `#2751`, `#2752`, and `#2797`; validator now passes for 27 closed `v0.91` issues. |
| Draft release notes | Resolved by rewriting release notes to landed behavior and retaining explicit release-tail status. |
| `WP-20` through `WP-25` still open | Active release-tail blockers; proceed in sequence. |
| Internal/external review absent | Assigned to `WP-21` and `WP-22`; not release-ready. |
| Accepted-finding remediation absent | Assigned to `WP-23`; not release-ready. |
| Next milestone and ceremony incomplete | Assigned to `WP-24` and `WP-25`; not release-ready. |

## 6) Exception Register

| ID | Severity | Exception | Owner / Next WP | Release impact |
| --- | --- | --- | --- | --- |
| QG-001 | RESOLVED | Closed-issue SOR truth validator previously failed for `#2751`, `#2752`, and `#2797`; local records now validate and the milestone closed-issue validator passes. | WP-19 | No remaining release impact. |
| QG-002 | P1 | `WP-20` through `WP-25` remain open. | Release-tail train. | Blocks release ceremony. |
| QG-003 | RESOLVED | Release notes previously described intended behavior; they now describe landed behavior and remaining review-tail status. | WP-19 | No remaining release impact beyond normal WP-20 review polish. |
| QG-004 | P2 | Internal and third-party review have not yet run. | `WP-21`, `WP-22`. | Blocks final assurance. |
| QG-005 | P2 | Review-finding remediation is not yet known. | `WP-23`. | Blocks final release if accepted findings exist. |

### Closed-issue SOR truth evidence

Command:

```bash
bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.91
```

Current result: PASS.

The repaired local records are:

- `.adl/v0.91/tasks/issue-2751__v0-91-wp-17-demo-cognitive-being-flagship-demo/sor.md`
- `.adl/v0.91/tasks/issue-2752__v0-91-wp-18-demo-demo-matrix-and-feature-proof-coverage/sor.md`
- `.adl/v0.91/tasks/issue-2797__v0-91-tools-fix-worktree-task-bundle-materialization/sor.md`

The validator now reports:

```text
PASS check_milestone_closed_issue_sor_truth version=v0.91 checked=27
```

## 7) Review Readiness

Review readiness is partial.

Ready:

- core feature docs exist
- demo matrix exists
- feature-proof coverage exists
- latest main CI and coverage are green
- non-claims are explicit in proof docs
- release notes describe landed behavior and retain release-tail status
- closed-issue SOR truth is green

Not ready:

- internal review has not run
- third-party review has not run
- accepted-finding remediation is not complete

## Final WP-19 Judgment

`v0.91` currently passes the main repository merge and coverage gates after
`WP-18`, and the local closed-issue SOR truth gap identified during `WP-19` is
repaired.

It does not yet pass the release-closeout gate.

The next required work is not broad feature reimplementation. It is the
release-tail sequence: docs/review pass, internal review, external review,
accepted-finding remediation, next-milestone planning, and ceremony. The
closed-issue SOR truth exception found during `WP-19` is already fixed.
