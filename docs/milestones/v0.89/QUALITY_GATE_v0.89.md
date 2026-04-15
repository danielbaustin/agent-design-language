# v0.89 Coverage and Quality Gate

## Metadata
- Milestone: `v0.89`
- Version: `v0.89`
- Owner: `Daniel Austin / Codex`
- Canonical issue / WP: `#1801` / `WP-14`
- Scope: milestone quality, coverage, and review-surface posture

## Purpose

This document defines the canonical `v0.89` quality gate.

It is the release-truth surface for:

- the required local and CI quality command suite
- the merge-gate coverage posture
- the non-blocking maintainability watch-list posture
- the minimum milestone-proof package required before `WP-15` through `WP-20`
  can claim a green state truthfully

This document records the gate.
It does not replace CI implementation, but the gate and the CI configuration
must agree.

## Why This Exists

`v0.89` ends with a split proof posture:

- the core governed-adaptation band is now implemented
- the release tail still needs one explicit place that says what counts as
  quality evidence for the milestone

That means "CI is green" is not enough by itself.

The milestone also needs one canonical surface that states:

- which checks are required
- which checks are enforced in CI
- how coverage is judged
- how large-module maintainability remains visible without becoming a fake
  blocker
- how the existing `v0.89` proof-entrypoint and review-surface packages
  participate in the quality story

Without this document, the checklist, release plan, release notes, and CI
comments drift into stale milestone references and soft language that is not
auditable.

## Gate Structure

The `v0.89` gate has four layers:

1. Baseline repository quality gate
2. Coverage posture gate
3. Maintainability watch-list posture
4. Milestone proof-package gate

The first layer proves the ordinary repository merge gate is green.
The second proves coverage is governed by explicit thresholds and visible
exceptions.
The third keeps large-module debt visible without inventing fake release
blockers.
The fourth proves the milestone is backed by reviewer-facing proof surfaces
rather than scattered feature claims.

## Required vs Documented Exceptions

- **Required** means the item must pass for the relevant phase.
- **Exception documented** means the blocker, owner, rationale, and follow-up
  path are explicit.
- Exceptions do not convert a blocker into a pass.

## 1) Baseline Repository Quality Gate

The baseline repository gate must establish:

1. formatting is clean
2. linting is clean at the enforced warning level
3. the Rust test suite passes
4. release-note command surfaces remain current
5. legacy-name guardrails still hold
6. contract checks used by CI remain green
7. the bounded demo smoke used by CI remains runnable

### Required local command suite

From the repository root:

- `bash -n adl/tools/*.sh`
- `sh adl/tools/codex_pr.sh --help`
- `sh adl/tools/codexw.sh --help`
- `bash adl/tools/check_no_new_legacy_swarm_refs.sh`
- `bash adl/tools/check_release_notes_commands.sh`
- `bash adl/tools/test_repo_code_review_skill_contracts.sh`
- `bash adl/tools/test_test_generator_skill_contracts.sh`
- `bash adl/tools/test_demo_operator_skill_contracts.sh`
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
- `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings`
- `cargo test --manifest-path adl/Cargo.toml`
- `bash adl/tools/demo_smoke_v07_story.sh`

These commands mirror the substantive quality story enforced by `adl-ci`.

### Required CI merge-gate jobs

The canonical workflow is `.github/workflows/ci.yaml`.

It currently defines two required quality jobs:

- `adl-ci`
  - tooling shell sanity
  - `codex_pr` / `codexw` help sanity
  - no-new-legacy guardrail
  - PR closing-linkage guardrail
  - repo-code-review contract check
  - test-generator contract check
  - demo-operator contract check
  - `fmt`
  - `clippy`
  - release-notes command check
  - `cargo test`
  - demo smoke (`S-01` through `S-05`)
- `adl-coverage`
  - coverage generation
  - coverage summary artifacts
  - workspace and per-file threshold enforcement

If those required jobs or their enforced commands change, this document must be
updated in the same change.

## 2) Coverage Posture Gate

The required coverage job is `adl-coverage`.

It currently enforces:

- workspace line coverage threshold: `90%`
- per-file line coverage threshold: `80%`
- no active per-file exclusion regex

Coverage enforcement is implemented by:

- `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`
- `cargo llvm-cov report --json --summary-only --output-path coverage-summary.json`
- `cargo llvm-cov --workspace --summary-only | tee coverage-summary.txt`
- `bash tools/enforce_coverage_gates.sh coverage-summary.json`

Important posture notes:

- the coverage gate is a merge-gate surface, not a suggestion
- any future exclusion regex must stay documented and justified
- the former `adl/src/obsmem_contract.rs` exclusion was removed here because
  that single-file path no longer exists after the module split in `#1834`, and
  recent green `adl-coverage` runs therefore already prove the live
  `adl/src/obsmem_contract/*` surfaces clear the current floor without an
  exclusion
- `cargo llvm-cov` convenience wrappers may exist, but the canonical gate is
  the CI policy above

### Local developer expectation

Local `WP-14` work does not require every unrelated branch to regenerate full
coverage unless the change materially affects:

- `.github/workflows/ci.yaml`
- `adl/tools/enforce_coverage_gates.sh`
- the documented thresholds or exclusions

Any change to those surfaces must keep the documented policy and CI
configuration aligned.

## 3) Large-Module Watch-List Posture

The large-module watch list is governed by:

- local Rust size reports under `.adl/reports/manual/`
- `adl/tools/report_large_rust_modules.sh`

This surface is intentionally non-blocking.

Its purpose is to keep maintainability debt visible without converting every
large-file warning into a fake release blocker.

For `v0.89`, the watch-list posture is:

- treat the current report as a local operational snapshot rather than a tracked
  governance file
- let the default report cover both `adl/src` and `adl/tests` so the largest
  Rust implementation and integration-test surfaces stay visible together
- use the report during review and quality-gate walkthroughs
- keep the report script green-by-default
- require explicit deferral rationale in output cards only when a PR materially
  expands a watched module without improving structure

## 4) Milestone Proof-Package Gate

Before docs/review and release-tail issues can speak truthfully about quality,
the milestone must also establish:

1. the checklist and release plan agree with this gate
2. the proof-entrypoint and integrated review-surface packages are runnable and
   point at real proof rows
3. the demo matrix and quality posture agree on what counts as milestone proof
4. known exceptions remain visible rather than implied

### Required milestone surfaces

- `docs/milestones/v0.89/QUALITY_GATE_v0.89.md`
- `docs/milestones/v0.89/DEMO_MATRIX_v0.89.md`
- `docs/milestones/v0.89/MILESTONE_CHECKLIST_v0.89.md`
- `docs/milestones/v0.89/RELEASE_PLAN_v0.89.md`
- `docs/milestones/v0.89/RELEASE_NOTES_v0.89.md`
- issue `#1801` output record and validation evidence

### D11 quality-gate walkthrough

The canonical bounded proof surface for this gate is:

- `bash adl/tools/demo_v089_quality_gate.sh`
- primary artifact: `artifacts/v089/quality_gate/quality_gate_record.json`

D11 is not a substitute for CI.
It is the reviewer-facing aggregation surface that shows the current bounded
quality checks and their logs in one place.

### Required milestone proof package

The `v0.89` quality posture assumes the reviewer-facing proof package already
exists and stays current:

- `bash adl/tools/demo_v089_proof_entrypoints.sh`
- `bash adl/tools/demo_v089_review_surface.sh`

The quality gate does not replace those packages.
It records how they participate in the release-tail truth story.

## Evidence Expectations For WP-14

`WP-14` is in a good state when it leaves behind:

- this canonical quality-gate doc
- aligned checklist and release-plan references
- aligned release-notes language
- CI/tooling surfaces that point at the current milestone rather than stale
  milestone docs
- a quality-gate walkthrough that aggregates the current command suite and logs
- a task-bundle output record with the commands actually executed for the issue
  branch

### Minimum evidence to capture in the output record

- `cargo fmt` result
- `cargo clippy` result
- `cargo test` result
- release-notes command-check result
- `D11` quality-gate walkthrough result
- whether coverage policy remained aligned with CI thresholds and exclusions
- whether the maintainability watch-list posture remained truthful and
  non-blocking

## Known Bounded Exceptions

At `WP-14` time, later Sprint 3 review-tail work may still be incomplete.

That does not invalidate this gate.
It means only that later checklist/review or release items may still need
explicit owner-bound dispositions until the closeout tail finishes.

As of the initial `WP-14` authoring pass:

- the latest completed `main` CI runs for the landed `WP-09`, `WP-11`, `WP-12`,
  and `WP-13` merges were green
- the newest `main` push from the latest sprint merge may still be in progress
  when this document is reviewed

That state should be recorded explicitly in the issue output record instead of
being flattened into a fake all-green claim.

## Out Of Scope

- redefining the repo-wide coverage thresholds in this issue
- converting the large-module watch list into a build-failing gate
- pretending the integrated review surface replaces CI
- treating milestone demo proof as a substitute for tests / clippy / fmt

## Exit Criteria

`WP-14` is in a good state when:

- the `v0.89` quality gate exists as a canonical milestone doc
- the milestone checklist and release plan point to it consistently
- the required local and CI command suite is explicit and current
- coverage posture is explicit and tied to the active CI configuration
- the maintainability watch-list posture is visible and non-aspirational
- the existing `v0.89` proof packages are referenced as milestone-proof inputs
- any remaining exceptions are documented rather than hidden
