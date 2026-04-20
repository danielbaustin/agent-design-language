# v0.90.1 Coverage and Quality Gate

## Metadata

- Milestone: `v0.90.1`
- Version: `v0.90.1`
- Owner: `Daniel Austin / Codex`
- Canonical issue / WP: `#2154` / `WP-14`
- Scope: milestone quality, coverage, Runtime v2 proof posture, and CSM Observatory proof posture

## Purpose

This document defines the canonical `v0.90.1` quality gate.

It is the release-truth surface for:

- the required local and CI quality command suite
- the merge-gate coverage posture
- the Runtime v2 foundation proof hooks
- the CSM Observatory read-only proof checks
- the non-blocking maintainability watch-list posture
- the minimum proof package required before internal review, third-party review,
  remediation, release readiness, and release evidence can speak truthfully

This document records the gate. It does not replace CI implementation, but the
gate and the CI configuration must agree.

## Why This Exists

`v0.90.1` turns the v0.90 long-lived-agent work into a bounded Runtime v2
foundation prototype. The release tail now needs one explicit quality posture
before internal review, third-party review, remediation, readiness, evidence
assembly, and release ceremony can reason from a stable baseline.

For this milestone, "CI is green" is necessary but not sufficient. Reviewers
also need to know:

- which checks are required
- which checks are enforced in CI
- how coverage is judged
- whether any coverage exclusions exist
- which Runtime v2 proof hooks back the milestone claims
- which CSM Observatory surfaces are fixture-backed and read-only
- how maintainability watch-list debt remains visible without becoming a fake
  blocker

## Gate Structure

The `v0.90.1` gate has five layers:

1. Baseline repository quality gate
2. Coverage posture gate
3. Runtime v2 proof-package gate
4. CSM Observatory proof-package gate
5. Maintainability watch-list posture

The first layer proves the ordinary repository merge gate is green. The second
proves coverage is governed by explicit thresholds and visible exceptions. The
third proves the Runtime v2 foundation claims through focused test and demo
hooks. The fourth proves the Observatory package without claiming live mutation.
The fifth keeps large-module debt visible without inventing release blockers.

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

### Required Local Command Suite

From the repository root:

- `bash -n adl/tools/*.sh`
- `sh adl/tools/codex_pr.sh --help`
- `sh adl/tools/codexw.sh --help`
- `bash adl/tools/check_no_new_legacy_swarm_refs.sh`
- `bash adl/tools/check_release_notes_commands.sh`
- `bash adl/tools/test_repo_code_review_skill_contracts.sh`
- `bash adl/tools/test_test_generator_skill_contracts.sh`
- `bash adl/tools/test_demo_operator_skill_contracts.sh`
- `bash adl/tools/test_arxiv_paper_writer_skill_contracts.sh`
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
- `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings`
- `cargo test --manifest-path adl/Cargo.toml`
- `bash adl/tools/demo_smoke_v07_story.sh`

These commands mirror the substantive quality story enforced by `adl-ci`.

### Required CI Merge-Gate Jobs

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
  - arxiv-paper-writer contract check
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
- generated coverage artifacts are local/CI artifacts and should not be
  committed
- `cargo llvm-cov` convenience wrappers may exist, but the canonical merge gate
  is the CI policy above

## 3) Runtime v2 Proof-Package Gate

Before review-tail issues can speak truthfully about Runtime v2, the milestone
must prove the foundation package through focused test hooks.

Required focused proof commands:

- `cargo test --manifest-path adl/Cargo.toml runtime_v2::tests`
- `cargo test --manifest-path adl/Cargo.toml --test demo_tests demo_l_v0901_runtime_v2_foundation_writes_integrated_proof_packet -- --nocapture`

The focused Runtime v2 proof package covers:

- manifold root
- kernel service loop
- provisional citizen lifecycle
- snapshot and rehydration
- invariant violation artifact
- operator control report
- security-boundary proof packet
- integrated Runtime v2 foundation demo packet

The proof package does not claim first true Gödel-agent birth, full
moral/emotional civilization, complete migration, or full red/blue/purple
security ecology.

## 4) CSM Observatory Proof-Package Gate

The CSM Observatory package is an important reviewer-facing proof surface for
this milestone, but its first v0.90.1 surfaces are fixture-backed and read-only.

Required CSM checks:

- `bash adl/tools/test_csm_visibility_packet.sh`
- `bash adl/tools/test_csm_operator_command_packets.sh`
- `bash adl/tools/test_demo_v0901_csm_observatory_operator_report.sh`
- `bash adl/tools/test_demo_v0901_csm_observatory_static_console.sh`
- `bash adl/tools/demo_v0901_csm_observatory.sh artifacts/v0901/quality_gate/csm_observatory`

The CSM checks prove:

- the visibility packet validates against its schema
- command packet fixtures preserve the no-direct-UI-mutation rule
- the operator report is generated deterministically from the packet
- the static console remains aligned with the fixture packet and safety
  boundary
- the CLI bundle can regenerate the packet/report/console-reference manifest

The CSM checks do not prove:

- live Runtime v2 mutation
- live snapshot or wake execution through the Observatory
- v0.92 identity/capability rebinding
- cross-polis migration or full red/blue/purple security ecology

## 5) Large-Module Watch-List Posture

The large-module watch list is governed by:

- `adl/tools/report_large_rust_modules.sh`
- local output generated by the quality-gate walkthrough

This surface is intentionally non-blocking.

Its purpose is to keep maintainability debt visible without converting every
large-file warning into a fake release blocker. For `v0.90.1`, this is
especially important because Runtime v2 and CSM Observatory work landed in a
compressed milestone and follow-up refactors may belong in v0.90.2 rather than
the release tail.

The watch-list posture is:

- treat report output as a local operational snapshot rather than a tracked
  governance file
- let the default report cover both `adl/src` and `adl/tests`
- use the report during review and quality-gate walkthroughs
- keep the report script green-by-default
- require explicit deferral rationale in output cards only when a PR materially
  expands a watched module without improving structure

## D10 Quality-Gate Walkthrough

The canonical bounded proof surface for this gate is:

- `bash adl/tools/demo_v0901_quality_gate.sh`
- primary artifact: `artifacts/v0901/quality_gate/quality_gate_record.json`

D10 is not a substitute for CI. It is the reviewer-facing aggregation surface
that shows the current bounded quality checks, coverage policy check, Runtime v2
proof hooks, CSM Observatory proof checks, and maintainability-watch output in
one place.

The walkthrough writes per-check logs beside the manifest. Generated artifacts
are local proof artifacts and should not be committed.

## Evidence Expectations For WP-14

`WP-14` is in a good state when it leaves behind:

- this canonical quality-gate doc
- aligned checklist, README, release-plan, release-notes, and demo-matrix
  references
- a quality-gate walkthrough that aggregates the current command suite and logs
- a test for the quality-gate walkthrough
- a task-bundle output record with the commands actually executed for the issue

## Current WP-14 Result

As of the WP-14 authoring pass, the intended green state is:

- CI parity commands: required
- coverage gate: required
- Runtime v2 focused proof hooks: required
- CSM Observatory fixture-backed proof hooks: required
- Rust module watch report: non-blocking, required to run
- release-tail review and remediation items: not part of WP-14 and remain open
