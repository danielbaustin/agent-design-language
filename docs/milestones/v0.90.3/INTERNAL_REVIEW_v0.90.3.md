# Internal Review - v0.90.3

## Metadata

- Milestone: `v0.90.3`
- Version: `v0.90.3`
- Canonical issue / WP: `#2343` / `WP-16`
- Date: 2026-04-22
- Scope: final internal review of the v0.90.3 citizen-state substrate,
  private-state safety surfaces, demo/proof truth, release-tail readiness, and
  non-claim boundaries after `WP-14A` and `WP-15` landed.

## Executive Summary

Recommendation: proceed to `WP-17` third-party review with the P3 notes below
carried as release-tail polish or explicit deferrals. No P0, P1, or P2 finding
remains after refreshing this review against current main.

The core tranche is substantial and well done. v0.90.3 now has real Runtime v2
code, fixtures, negative cases, deterministic proof packets, and focused tests
for canonical private state, signed envelopes, local sealing, append-only
lineage, continuity witnesses and receipts, anti-equivocation,
sanctuary/quarantine, redacted Observatory projections, standing, access
control, challenge/appeal, and the D12 inhabited Observatory flagship.

The earlier preliminary WP-16 concerns were resolved by the now-closed `WP-14A`
and `WP-15` work:

- D13 feature-proof coverage now emits a `v0.90.3` / `D13` packet.
- D3-D6 demo-matrix rows now reflect `LANDED` proof state.
- `RELEASE_READINESS_v0.90.3.md` now records the reviewer entry surface after
  WP-14A feature-proof coverage.

This report does not approve the release by itself. It says the internal review
gate is healthy enough to hand to external review, remediation/deferral, next
milestone handoff, and final ceremony.

## Readiness Gate

At final refresh:

- `WP-01` through `WP-14` were closed.
- `WP-14A` (`#2341`) was closed at 2026-04-22T19:26:32Z.
- `WP-15` (`#2342`) was closed at 2026-04-22T19:40:24Z.
- `WP-16` (`#2343`) is the active internal-review PR.
- `WP-17` through `WP-20` remain the third-party review, accepted-finding
  remediation, next-milestone handoff, and release ceremony tail.

## Findings

### F1. P3 - v0.90.3 demo wrapper scripts are missing

The obvious wrapper names `test_demo_v0903_feature_proof_coverage.sh` and
`test_demo_v0903_observatory_flagship.sh` do not exist. The underlying CLI
commands and focused Rust tests exist and pass, so this is not a proof gap.

Impact: reviewers and operators have to use the documented lower-level commands
instead of one-command wrapper scripts.

Recommended route: `WP-18` polish if the release owner wants wrapper parity
with older demo surfaces; otherwise explicitly defer because the canonical
commands are already documented in `FEATURE_PROOF_COVERAGE_v0.90.3.md` and
`RELEASE_READINESS_v0.90.3.md`.

### F2. P3 - Demo stdout prints local output roots

The D12 and D13 CLI demo commands print local output roots to stdout. The
tracked milestone docs use repo-relative paths, and the strict scan did not
find host-path leakage in tracked review surfaces. This is therefore a review
hygiene concern, not a release blocker.

Impact: raw copied terminal output can contain host paths unless reviewers
redact or summarize it before publication.

Recommended route: optional `WP-18` polish if the release owner wants stdout to
prefer repo-relative output roots for reviewer-facing commands.

## Resolved Preliminary Findings

- Resolved: the preliminary D13 mismatch is fixed. The refreshed
  `runtime-v2 feature-proof-coverage` command emits
  `runtime_v2.feature_proof_coverage.v2`, `D13`, `v0.90.3`, with 14 entries.
- Resolved: the preliminary D3-D6 demo-matrix status gap is fixed. The matrix
  now marks D3 through D6 as `LANDED`.
- Resolved: the preliminary dependency gate is fixed. `#2341` and `#2342` are
  closed.

## Explicit No-Finding Statements

- No P0 findings were identified.
- No P1 findings were identified.
- No P2 findings remain after the final refresh against current main.
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
| D3 | proving | Envelope/trust-root tests and landed docs cover signed envelope and trust-root negative cases. |
| D4 | proving | Sealing tests and landed docs cover local sealed quintessence checkpoint behavior. |
| D5 | proving | Lineage tests and landed docs cover append-only replay and accepted-head authority. |
| D6 | proving | Witness/receipt tests and landed docs cover explainable transition evidence. |
| D7 | proving | Anti-equivocation tests and fixtures prove conflicting successors cannot both activate. |
| D8 | proving | Sanctuary/quarantine tests and fixtures prove ambiguous wake blocks activation. |
| D9 | proving | Redacted Observatory tests and fixtures preserve non-authoritative projection. |
| D10 | proving | Standing and access-control tests prove rights and inspection boundaries. |
| D11 | proving | Challenge/appeal tests prove freeze, review, threat, and narrow economics placement. |
| D12 | proving | Observatory flagship demo emits proof packet, operator report, walkthrough, projection, access, quarantine, and challenge artifacts. |
| D13 | proving | Feature-proof command emits v0.90.3 D13 coverage with D1-D14 mapped to proof, non-runtime design boundary, or explicit evidence. |
| D14 | non-proving design boundary | Observatory multimode UI architecture is landed as design evidence, not runtime UI implementation. |

## Validation Evidence

Passed in the original WP-16 review pass:

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
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_continuity_challenge -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture`

Passed in the final refresh after `#2341` and `#2342` closed:

- `cargo run --manifest-path adl/Cargo.toml --quiet -- runtime-v2 feature-proof-coverage --out .adl/reviews/v0.90.3/internal/demo-runs/feature-proof-coverage-refresh.json`
- `jq '{schema_version,demo_id,milestone,entry_count:(.entries|length),proving_count:([.entries[]|select(.status=="proving")]|length),non_proving:([.entries[]|select(.status!="proving")|.feature_id])}' .adl/reviews/v0.90.3/internal/demo-runs/feature-proof-coverage-refresh.json`
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_observatory_flagship -- --nocapture`
- `cargo run --manifest-path adl/Cargo.toml --quiet -- runtime-v2 observatory-flagship-demo --out .adl/reviews/v0.90.3/internal/demo-runs/flagship-refresh`
- `test -f docs/milestones/v0.90.3/OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md && test -f docs/milestones/v0.90.3/assets/csm_observatory_multimode_ui_mockups.png`
- `gh issue view 2341 --json number,state,closedAt,title`
- `gh issue view 2342 --json number,state,closedAt,title`

Not run:

- Full `cargo test --manifest-path adl/Cargo.toml` was not run in WP-16.
- Full coverage was not rerun in WP-16; WP-15 records the current coverage
  tracker truth and preserves release-tail coverage as a later ceremony or CI
  responsibility.

## WP-18 Remediation Queue

Recommended order:

1. Decide whether to add v0.90.3 demo wrapper scripts for feature-proof
   coverage and Observatory flagship proof.
2. Decide whether demo stdout should print repo-relative output roots or keep
   absolute local roots as operator diagnostics.
3. Preserve any new `WP-17` external-review findings separately so this
   internal-review packet does not blur internal and external review evidence.

## WP-17 Handoff

WP-17 can use this packet as final internal-review context. External review
should recheck the D12 and D13 commands, confirm the two P3 notes are acceptable
or routed to WP-18, and preserve any third-party findings as a separate review
surface.

## Release-Tail Disposition

Current assessment: ready for external review handoff. The v0.90.3
implementation quality is high in the core citizen-state substrate, and the
remaining internal-review notes are release-tail ergonomics and publication
hygiene rather than missing core implementation.
