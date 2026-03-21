# v0.85 Coverage and Quality Gate

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Owner: `Daniel Austin / Agent Logic`
- Canonical issue / WP: `#879` / `WP-19`
- Scope: review/release quality gate surface

## Purpose

This document defines the canonical v0.85 quality gate.

It is the release-truth surface for:

- the required command suite
- CI and coverage posture
- exception / deferral policy
- the minimum evidence expected before release ceremony work proceeds

This document does not itself implement CI behavior. It records the quality gate
that the milestone must satisfy.

## Why This Exists

The v0.85 milestone requires a closeout quality gate with:

- concrete evidence rather than generic "quality improved" language
- explicit linkage to the actual repo-enforced CI and coverage surfaces
- documented exceptions when upstream milestone items are still blocked

Without a canonical gate document, the milestone checklist, release plan, and
closeout issues can drift into soft language instead of auditable release
criteria.

## Gate Structure

The v0.85 gate has two layers:

1. Baseline repository quality gate
2. Milestone release-readiness gate

The first layer proves the current repository command suite is green.
The second layer proves the milestone can proceed toward release without hidden
blockers.

## Required vs Documented Exceptions

- **Required** means the gate item must pass for the relevant phase.
- **Exception documented** means the item is not yet complete, but the blocker,
  owner, and follow-up path are recorded explicitly.

Exceptions are allowed only where the milestone docs already permit explicit
deferrals. Exceptions do not convert a blocker into a success; they make the
state reviewable.

## Baseline Repository Quality Gate

The baseline repo gate must establish the following:

1. formatting is clean
2. linting is clean at the enforced warning level
3. workspace tests pass
4. CI reflects the same required command surfaces
5. coverage policy is defined and not silently red

### Required local command suite

From `swarm/`:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`

From the repository root:

- `bash swarm/tools/check_no_new_legacy_swarm_refs.sh`
- `bash tools/check_release_notes_commands.sh`
- `bash swarm/tools/demo_smoke_v07_story.sh`

These commands mirror the current `adl-ci` job in `.github/workflows/ci.yaml`.

### Coverage gate

The current required CI coverage job is `adl-coverage`.

It enforces:

- workspace line coverage threshold: `90%`
- per-file line coverage threshold: `80%`
- documented exclusions regex:
  - `/swarm/src/bin/swarm.rs$`
  - `/swarm/src/bin/swarm_remote.rs$`
  - `/swarm/src/obsmem_contract.rs$`

Coverage enforcement is currently implemented by:

- `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`
- `cargo llvm-cov report --json --summary-only --output-path coverage-summary.json`
- `cargo llvm-cov --workspace --summary-only | tee coverage-summary.txt`
- `bash tools/enforce_coverage_gates.sh coverage-summary.json`

The nightly watchdog in `.github/workflows/nightly-coverage-ratchet.yaml` is a
reporting / escalation surface, not the canonical merge gate.

## Milestone Release-Readiness Gate

Before release ceremony work, the milestone must also satisfy:

1. milestone checklist and release plan are aligned with this gate
2. required demo proof surfaces are either present or explicitly blocked with a
   documented exception
3. no unresolved blocker-grade findings remain hidden
4. review / release issues can point to concrete evidence instead of generic
   assertions

### Required milestone surfaces

- `docs/milestones/v0.85/MILESTONE_CHECKLIST_v0.85.md`
- `docs/milestones/v0.85/RELEASE_PLAN_v0.85.md`
- `docs/milestones/v0.85/DEMO_MATRIX_v0.85.md`
- issue `#879` output record and validation evidence

## Current CI Posture

The canonical workflow is `.github/workflows/ci.yaml`.

It currently defines two required quality jobs:

- `adl-ci`
  - tooling sanity
  - legacy-name guardrail
  - `fmt`
  - `clippy`
  - docs command check
  - `test`
  - demo smoke
- `adl-coverage`
  - coverage generation
  - coverage summary artifacts
  - workspace and per-file threshold enforcement

This document should be updated if those required jobs or their enforced
commands change.

## Evidence Expectations for WP-19

WP-19 is considered complete when it leaves behind:

- this canonical quality-gate doc
- a task-bundle output record with the command results actually executed for the
  issue branch
- documented exceptions if any milestone-closeout prerequisites remain blocked

### Minimum evidence to capture in the output record

- `cargo fmt` result
- `cargo clippy` result
- `cargo test` result
- whether the current branch is expected to satisfy the same merge gate as CI
- whether coverage is green by current repo policy or requires an explicit
  exception note

## Current Known Exceptions / Upstream Preconditions

At the time WP-19 is being authored, the milestone still depends on upstream
Sprint 4 closeout work.

Known quality-gate-sensitive dependencies include:

- `WP-18` demo-program completion
- `WP-20` docs consistency pass
- later review/remediation/release issues

Those upstream items do not invalidate this gate document, but they do mean the
milestone release-readiness state may still require explicit exceptions until
the closeout queue is complete.

## Deferral Policy

A required milestone gate item may be deferred only when all are true:

1. the blocker is explicit
2. the risk is documented
3. an owner or canonical issue exists
4. the milestone remains bounded
5. the deferral is recorded in the issue output record or adjacent closeout
   surface

## Out of Scope

- redefining the repo coverage thresholds in this issue
- redesigning CI architecture here
- treating tests as optional because the work is documentation-heavy
- marking blocked upstream milestone work complete without explicit rationale

## Exit Criteria

WP-19 is in a good state when:

- the v0.85 quality gate exists as a canonical milestone doc
- the milestone checklist and release plan point to this gate consistently
- the required command suite is explicit and matches the current CI posture
- coverage policy is explicit and not hand-waved
- any remaining exceptions are documented rather than hidden
