# Demo Matrix - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: pre-third-party-review readiness

## Purpose

Define the proof surfaces for v0.90 and keep their status aligned with the
landed issue wave.

## Demo Coverage Summary

| Demo ID | Demo title | Milestone claim | Command entry point | Primary proof surface | Status |
| --- | --- | --- | --- | --- | --- |
| D1 | Long-lived supervisor heartbeat | Supervisor can keep bounded agent state across cycles | `cargo test --manifest-path adl/Cargo.toml active_lease_blocks_overlapping_tick_and_status_reports_leased -- --nocapture` | supervisor state and heartbeat/lease artifacts in `adl/src/long_lived_agent.rs` tests | landed by `#2021` |
| D2 | Cycle contract replay packet | Each cycle emits reviewable artifacts | `cargo test --manifest-path adl/Cargo.toml tick_creates_state_status_full_cycle_bundle_and_removes_lease -- --nocapture` | cycle manifests, observations, decision records, run refs, ledger, and status artifacts | landed by `#2022` |
| D3 | Operator stop and guardrail controls | Operators remain authoritative over long-lived execution | `cargo test --manifest-path adl/Cargo.toml stop_prevents_next_tick_and_records_reason -- --nocapture` | status output, stop marker, guardrail report, and inspection packet tests | landed by `#2024` / `#2025` |
| D4 | Stock league long-lived demo | A bounded demo can show recurring supervised cycles safely | `cargo run --manifest-path adl/Cargo.toml -- demo demo-j-v090-stock-league-recurring --run --trace --out out --no-open`; scaffold remains available as `demo-i-v090-stock-league-scaffold` | recurring integration proof packet, state root, cycle ledger, inspections, continuity proof, and guardrail summary | integrated by `#2027` |
| D5 | Stock league proof expansion | Selected demo extensions can add reviewer evidence without weakening the primary stock-league proof | `cargo run --manifest-path adl/Cargo.toml -- demo demo-k-v090-stock-league-proof-expansion --run --trace --out out --no-open` | selected-demo manifest, extension proof packet, evidence index, replay manifest, non-goals/deferrals, and extension safety scan | landed by `#2028` |
| D6 | Repo visibility proof packet | ADL can map one milestone or feature slice from canonical docs to implementation, tests, demos, and review surfaces | `docs/milestones/v0.90/repo_visibility/` | manifest and code-doc-demo linkage report | landed by `#2031` |
| D7 | Milestone compression pilot | ADL can detect milestone drift from canonical state without silently mutating release truth | `python3 adl/tools/check_v090_milestone_state.py` | canonical state file, drift-check output, and generated status summary | landed by `#2030` |
| D8 | CodeBuddy multi-agent review showcase | ADL can present the CodeBuddy review-engine skill family as a product-style, packet-first repo review workflow without building the web app or mutating customer repos | `bash adl/tools/demo_v090_codebuddy_review_showcase.sh` | `artifacts/v090/codebuddy_review_showcase/run_manifest.json`, specialist reviews, redaction report, diagram artifacts, final report, and demo-operator classification | landed by `#2072`; review-quality dependency landed by `#2070` |
| D9 | ADL architecture document generation | ADL can maintain a source-grounded first-class architecture packet with diagrams, review automation, generation planning, candidate ADRs, and deterministic validation | `bash adl/tools/demo_v090_architecture_document_generation.sh` | `docs/architecture/`, `docs/architecture/diagrams/`, and `artifacts/v090/adl_architecture_document_generation/architecture_generation_manifest.json` | landed by `#2055` |

## Safety Rules

The stock league demo must:

- avoid live trading
- avoid financial advice
- use fixture-backed or delayed/public data
- label outputs as demo decisions, not investment recommendations
- keep operator stop controls active

Demo extensions must:

- name their exact proof claim before implementation
- provide a validation command or reviewer-readable proof packet
- state non-goals and avoid broad capability claims
- avoid competing with the stock-league demo as the primary long-lived proof

Repo visibility and milestone-compression proof packets must:

- distinguish canonical tracked docs from local planning and historical residue
- avoid broad repo-ingestion claims
- avoid autonomous release approval or silent closeout behavior

The CodeBuddy showcase packet must:

- keep review packet construction, specialist review, diagram review, redaction,
  follow-through planning, and product-report writing as explicit lanes
- preserve severity, evidence, caveats, specialist disagreement, and residual
  risk in the final report
- treat `review-quality-evaluator` as landed by `#2070`
- avoid live provider calls, customer repositories, billing, and product-app
  assumptions in the default demo path
- block publication until redaction and evidence gates are explicit

The architecture document generation packet must:

- keep architecture claims source-grounded in tracked repository evidence
- include evidence, assumptions, and validation notes for every diagram source
- separate machine-checkable invariants from human architecture judgment
- treat documentation-specialist and gap-analysis support as available
- avoid private local trace paths, host-absolute paths, and secret markers in
  public docs or proof artifacts

## Validation Expectations

The v0.90 issue wave is in release-tail readiness. WP-13 and WP-15 have
already aligned the matrix with landed or deferred work. WP-18 keeps this
matrix as pre-third-party-review evidence; final release quality is not complete
until WP-16, WP-17 if needed, WP-19, and WP-20 settle.
