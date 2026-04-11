# v0.87.1 Coverage and Quality Gate

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Owner: `Daniel Austin / Codex`
- Canonical issue / WP: `#1463` / `WP-14`
- Scope: runtime-milestone quality and coverage posture

## Purpose

This document defines the canonical `v0.87.1` quality gate.

It is the release-truth surface for:

- the required local and CI quality command suite
- the merge-gate coverage posture
- the non-blocking maintainability watch-list posture
- the minimum quality evidence required before docs/review and release-tail work can claim a green state

This document does not itself implement CI behavior. It records the gate that
the milestone must satisfy and the bounded exceptions it allows.

## Why This Exists

`v0.87.1` is a runtime-completion milestone with a large demo and review
surface. The quality gate therefore has to do more than say "CI is green."

It must make explicit:

- which checks are actually required
- which checks are enforced in CI
- how coverage is judged
- how large-module maintainability is tracked without turning watch-list
  reporting into a fake release blocker
- how demo and review-tail proof surfaces fit into the quality story

Without this document, the checklist, release plan, and D11 demo surface can
drift into generic quality language that is not auditable.

## Gate Structure

The `v0.87.1` gate has three layers:

1. Baseline repository quality gate
2. Coverage posture gate
3. Milestone runtime-proof gate

The first layer proves that the ordinary repository merge gate is green.
The second proves that coverage is governed by explicit thresholds and visible
exceptions.
The third proves that the runtime milestone is backed by bounded demo and review
surfaces rather than code-only claims.

## Required vs Documented Exceptions

- **Required** means the item must pass for the relevant phase.
- **Exception documented** means the blocker, owner, rationale, and follow-up
  path are explicit.
- Exceptions do not convert a blocker into a pass. They make the state
  reviewable and bounded.

## 1) Baseline Repository Quality Gate

The baseline repository gate must establish:

1. formatting is clean
2. linting is clean at the enforced warning level
3. the Rust test suite passes
4. release-note command surfaces are current and checkable
5. legacy-name guardrails still hold
6. the required runtime proof demos used by the quality gate remain runnable

### Required local command suite

From the repository root:

- `cargo fmt --manifest-path adl/Cargo.toml --all -- --check`
- `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings`
- `cargo test --manifest-path adl/Cargo.toml`
- `bash adl/tools/check_no_new_legacy_swarm_refs.sh`
- `bash adl/tools/check_release_notes_commands.sh`
- `bash adl/tools/test_demo_v0871_runtime_rows.sh`
- `bash adl/tools/test_demo_v0871_operator_surface.sh`
- `bash adl/tools/test_demo_v0871_runtime_state.sh`
- `bash adl/tools/test_demo_v0871_review_surface.sh`

These commands mirror the quality story that `adl-ci`, D11, and the milestone
docs are expected to tell.

### Required CI merge-gate jobs

The canonical workflow is `.github/workflows/ci.yaml`.

It currently defines two required quality jobs:

- `adl-ci`
  - tooling shell sanity
  - workflow helper sanity
  - no-new-legacy guardrail
  - PR closing-linkage guardrail
  - `fmt`
  - `clippy`
  - release-notes command check
  - `cargo test`
  - demo smoke
- `adl-coverage`
  - coverage generation
  - coverage summary artifacts
  - workspace and per-file threshold enforcement

If those required jobs or their enforced commands change, this document must be
updated in the same change.

## 2) Coverage Posture Gate

The current required coverage job is `adl-coverage`.

It enforces:

- workspace line coverage threshold: `90%`
- per-file line coverage threshold: `80%`
- documented exclusion regex for:
  - `adl/src/obsmem_contract.rs`

Coverage enforcement is currently implemented by:

- `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`
- `cargo llvm-cov report --json --summary-only --output-path coverage-summary.json`
- `cargo llvm-cov --workspace --summary-only | tee coverage-summary.txt`
- `bash tools/enforce_coverage_gates.sh coverage-summary.json`

Important posture notes:

- the coverage gate is a merge-gate surface, not a suggestion
- the exclusion regex must stay documented and justified
- nightly coverage reporting may exist, but it is not the canonical merge gate

### Local developer expectation

Local WP-14 work does not require every branch to regenerate coverage unless the
change materially affects the coverage policy or enforcement logic. However, any
change to:

- `.github/workflows/ci.yaml`
- `adl/tools/enforce_coverage_gates.sh`
- the documented thresholds or exclusions

must keep the documented policy and CI configuration aligned.

## 3) Large-Module Watch-List Posture

The large-module watch list is governed by:

- `docs/tooling/rust_module_watch_list.md`
- `adl/tools/report_large_rust_modules.sh`

This surface is intentionally non-blocking.

Its purpose is to keep maintainability debt visible without converting every
large-file warning into a fake release blocker. For `v0.87.1`, the watch-list
posture is:

- use the report during review and quality-gate walkthroughs
- keep the report script green-by-default
- require explicit deferral rationale in output cards only when a PR materially
  expands a watched module without improving structure

## 4) Milestone Runtime-Proof Gate

Before docs/review and release-tail issues can speak truthfully about quality,
the milestone must also establish:

1. the checklist and release plan agree with this gate
2. the D11 quality-gate walkthrough is runnable and points at real logs
3. the demo matrix and quality posture agree on what counts as milestone proof
4. known exceptions are visible rather than implied

### Required milestone surfaces

- `docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md`
- `docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md`
- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- `docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md`
- issue `#1463` output record and validation evidence

### D11 quality-gate walkthrough

The canonical bounded proof surface for this gate is:

- `bash adl/tools/demo_v0871_quality_gate.sh`
- primary artifact: `artifacts/v0871/quality_gate/quality_gate_record.json`

D11 is not a substitute for CI. It is the reviewer-facing aggregation surface
that shows the current bounded quality checks and their logs in one place.
That aggregation must include the active coverage-enforcement step, not merely
reference it in prose.

## Evidence Expectations For WP-14

WP-14 is in a good state when it leaves behind:

- this canonical quality-gate doc
- aligned checklist and release-plan references
- CI/tooling surfaces that point at the current milestone rather than stale
  milestone docs
- a task-bundle output record with the commands actually executed for the issue
  branch

### Minimum evidence to capture in the output record

- `cargo fmt` result
- `cargo clippy` result
- `cargo test` result
- release-notes command-check result
- D11 quality-gate walkthrough result
- whether coverage policy remained aligned with CI thresholds and exclusions
- whether the watch-list posture remained truthful and non-blocking

## Known Bounded Exceptions

At WP-14 time, the milestone may still have upstream release-tail dependencies
that are not yet complete.

That does not invalidate this gate. It means only that later checklist/review or
release items may still need explicit owner-bound dispositions until the
closeout tail finishes.

## Out Of Scope

- redefining the repo-wide coverage thresholds in this issue
- converting the large-module watch list into a build-failing gate
- pretending the demo matrix alone replaces CI
- treating runtime demo proof as a substitute for tests/clippy/fmt

## Exit Criteria

WP-14 is in a good state when:

- the `v0.87.1` quality gate exists as a canonical milestone doc
- the milestone checklist and release plan point to it consistently
- the required local and CI command suite is explicit and current
- coverage posture is explicit and tied to the active CI configuration
- the maintainability watch-list posture is visible and non-aspirational
- any remaining exceptions are documented rather than hidden
