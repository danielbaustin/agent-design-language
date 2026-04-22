# Internal Review - v0.90.3

## Metadata

- Milestone: `v0.90.3`
- Version: `v0.90.3`
- Canonical issue / WP: `#2343` / `WP-16`
- Date: 2026-04-22
- Scope: dependency-aware internal review of the v0.90.3 citizen-state
  substrate, private-state safety surfaces, demo/proof truth, release-tail
  readiness, and non-claim boundaries.

## Executive Summary

Recommendation: keep this as a dependency-aware preliminary internal review
until `WP-14A` (`#2341`) and `WP-15` (`#2342`) land. The review found no P0 or
P1 issue in the implemented citizen-state substrate, but it did find P2
release-tail truth gaps that should be fixed before v0.90.3 release closeout.

The core tranche is substantial. v0.90.3 now has real Runtime v2 code, fixtures,
negative cases, and focused tests for canonical private state, signed envelopes,
local sealing, append-only lineage, continuity witnesses and receipts,
anti-equivocation, sanctuary/quarantine, redacted Observatory projections,
standing, access control, challenge/appeal, and the D12 inhabited Observatory
flagship.

The strongest concern is not that the citizen-state implementation is missing.
It is that the final proof/status surfaces are not yet fully converged: D13
feature-proof coverage still emits the older v0.90.2 packet, and the demo
matrix still labels several landed proof rows as planned. That is exactly what
WP-14A/WP-15/WP-18 exist to clean up.

## Readiness Gate

This review followed the WP-16 plan's dependency-aware mode. At review time:

- `WP-01` through `WP-14` were closed.
- `WP-14A` (`#2341`) was open.
- `WP-15` (`#2342`) was open.
- `WP-16` (`#2343`) was running in the issue worktree.

Because final feature-proof coverage and docs/quality convergence are still
open dependencies, this report should not be treated as release readiness,
third-party approval, or final internal-review closeout.

## Findings

### F1. P2 - D13 feature-proof coverage still targets v0.90.2

The `runtime-v2 feature-proof-coverage` command runs, but it emits a v0.90.2
D11 packet rather than the v0.90.3 D13 coverage record required by
`DEMO_MATRIX_v0.90.3.md`.

Evidence:

- `adl/src/runtime_v2/feature_proof_coverage.rs`
- `docs/milestones/v0.90.3/DEMO_MATRIX_v0.90.3.md`
- command: `cargo run --manifest-path adl/Cargo.toml --quiet -- runtime-v2 feature-proof-coverage --out .adl/reviews/v0.90.3/internal/demo-runs/feature-proof-coverage.json`

Impact: v0.90.3 cannot count D13 as final feature-proof coverage until this is
updated or explicitly deferred.

Recommended route: `WP-14A` / `#2341`, or `WP-18` if WP-14A has already closed.

### F2. P2 - Demo matrix D3-D6 remain planned after implementation landed

`DEMO_MATRIX_v0.90.3.md` still labels D3 signed envelope, D4 local sealing, D5
append-only lineage, and D6 witnesses/receipts as `PLANNED`. The corresponding
WPs are closed, and focused Runtime v2 tests for those surfaces pass.

Impact: reviewers may believe core proof rows are still absent even though the
implementation and fixtures exist. This weakens release-tail truth and makes
the proof matrix less reliable.

Recommended route: `WP-14A` / `#2341` or `WP-15` / `#2342`.

### F3. P2 - WP-16 final readiness is dependency-gated

`WP-14A` and `WP-15` remain open. This internal review can usefully catch
issues now, but final WP-16 closeout should wait for the feature-proof and
docs/quality convergence lanes to land or be explicitly dispositioned.

Impact: moving straight to final release-tail review would blur planned,
preliminary, and landed truth.

Recommended route: finish `#2341` and `#2342`, then refresh this review before
external review or release closeout.

### F4. P3 - v0.90.3 demo wrapper scripts are missing

The obvious wrapper names `test_demo_v0903_feature_proof_coverage.sh` and
`test_demo_v0903_observatory_flagship.sh` do not exist. The underlying CLI and
Rust proof surfaces exist, but operators have to know the lower-level command
names.

Impact: lower release-tail ergonomics and a greater chance of skipped or
mis-recorded proof runs.

Recommended route: `WP-14A`, `WP-15`, or `WP-18`.

### F5. P3 - Demo stdout prints local output roots

The D12 and D13 CLI demo commands print local output roots to stdout. These
paths are not present in tracked milestone docs, and the strict scan over
tracked docs plus review artifacts did not find host-path leaks. Still, review
packets should avoid copying raw stdout without redaction.

Impact: low; mostly a redaction hygiene concern for reviewer-facing packets.

Recommended route: optional WP-18 polish if the release owner wants stdout to
prefer repo-relative paths.

### F6. P3 - WP-16 SOR has in-progress template placeholders

`records-hygiene` reported placeholder-like text in the unfinished WP-16 SOR
template. This is expected while WP-16 is in progress, but it must be
normalized before `pr-finish`.

Impact: low if cleaned before finish; misleading if left in the final output
record.

Recommended route: WP-16 finish normalization.

## Explicit No-Finding Statements

- No P0 findings were identified.
- No P1 findings were identified.
- No evidence was found that raw private state is exposed through the tracked
  public, reviewer, operator, or debug projection fixtures reviewed here.
- No evidence was found that denied access can mutate continuity or disclose
  raw private state in the focused access-control test surface.
- No evidence was found that v0.90.3 claims first true Godel-agent birthday,
  full v0.91 moral/emotional civilization, v0.92 migration/birthday
  completion, full v0.90.4 economics, or mandatory cloud enclaves.

## Demo And Proof Register

| ID | Classification | Evidence summary |
| --- | --- | --- |
| D1 | proving | Inheritance audit targets real v0.90.2 artifacts and preserves non-claims. |
| D2 | proving | Private-state tests and fixtures prove canonical bytes are distinct from JSON projection. |
| D3 | proving, docs-status gap | Envelope/trust-root tests pass, but the demo matrix still says `PLANNED`. |
| D4 | proving, docs-status gap | Sealing tests pass, but the demo matrix still says `PLANNED`. |
| D5 | proving, docs-status gap | Lineage tests pass, but the demo matrix still says `PLANNED`. |
| D6 | proving, docs-status gap | Witness/receipt tests pass, but the demo matrix still says `PLANNED`. |
| D7 | proving | Anti-equivocation tests and fixtures prove conflicting successors cannot both activate. |
| D8 | proving | Sanctuary/quarantine tests and fixtures prove ambiguous wake blocks activation. |
| D9 | proving | Redacted Observatory tests and fixtures preserve non-authoritative projection. |
| D10 | proving | Standing and access-control tests prove rights and inspection boundaries. |
| D11 | proving | Challenge/appeal tests prove freeze, review, threat, and narrow economics placement. |
| D12 | proving | Observatory flagship demo emits proof packet, operator report, walkthrough, projection, access, quarantine, and challenge artifacts. |
| D13 | non-proving for v0.90.3 final coverage | Feature-proof command emits v0.90.2/D11 coverage. |
| D14 | proving as design evidence only | Observatory multimode UI architecture is design evidence, not runtime proof. |

## Validation Evidence

Passed:

- `bash adl/tools/pr.sh doctor 2343 --slug v0-90-3-wp-16-internal-review --version v0.90.3 --mode full --json`
- `bash adl/tools/pr.sh run 2343 --slug v0-90-3-wp-16-internal-review --version v0.90.3`
- `python3 adl/tools/skills/repo-packet-builder/scripts/build_repo_packet.py . --out .adl/reviews/v0.90.3/internal/codebuddy/repo-packet`
- `bash adl/tools/test_skill_documentation_completeness.sh`
- `bash adl/tools/test_multi_agent_repo_review_specialist_skill_contracts.sh`
- `bash adl/tools/test_multi_agent_repo_review_skill_suite_contracts.sh`
- `bash adl/tools/test_repo_code_review_skill_contracts.sh`
- `bash adl/tools/test_repo_architecture_review_skill_contracts.sh`
- `bash adl/tools/test_repo_dependency_review_skill_contracts.sh`
- `bash adl/tools/test_review_quality_evaluator_skill_contracts.sh`
- `bash adl/tools/test_review_to_test_planner_skill_contracts.sh`
- `bash adl/tools/test_review_comment_triage_skill_contracts.sh`
- `bash adl/tools/test_review_readiness_cleanup_skill_contracts.sh`
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_observatory_flagship -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_continuity_challenge -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture`
- `cargo run --manifest-path adl/Cargo.toml --quiet -- runtime-v2 observatory-flagship-demo --out .adl/reviews/v0.90.3/internal/demo-runs/flagship`
- strict host-path and secret scan over `docs/milestones/v0.90.3` and `.adl/reviews/v0.90.3`

Non-proving or blocked:

- `cargo run --manifest-path adl/Cargo.toml --quiet -- runtime-v2 feature-proof-coverage --out .adl/reviews/v0.90.3/internal/demo-runs/feature-proof-coverage.json`
  ran successfully but emitted v0.90.2 coverage.
- `bash adl/tools/test_demo_v0903_feature_proof_coverage.sh` was not run
  because the script does not exist.
- `bash adl/tools/test_demo_v0903_observatory_flagship.sh` was not run because
  the script does not exist.
- Full `cargo test --manifest-path adl/Cargo.toml` was not run in this pass.

## WP-18 Remediation Queue

Recommended order:

1. Fix or replace `runtime-v2 feature-proof-coverage` so it emits v0.90.3 D13
   coverage, not v0.90.2 D11 coverage.
2. Update `DEMO_MATRIX_v0.90.3.md` so D3-D6 reflect landed implementation and
   proof state.
3. Decide whether to add v0.90.3 demo wrapper scripts for feature-proof
   coverage and Observatory flagship proof.
4. Decide whether demo stdout should print repo-relative output roots or keep
   absolute local roots as operator diagnostics.
5. Normalize WP-16 SOR before `pr-finish`.

## WP-17 Handoff

WP-17 should use this packet as preliminary context, not final release
approval. External review should recheck D13 and demo matrix truth after WP-14A
and WP-15 land, then verify that WP-18 accepted or explicitly deferred the P2
findings.

## Release-Tail Disposition

Current assessment: ready for dependency-aware review continuation, not yet
ready for final external review or release closeout.

The v0.90.3 implementation quality is high in the core citizen-state substrate.
The remaining work is to make proof and release-tail truth match the quality of
the underlying implementation.
