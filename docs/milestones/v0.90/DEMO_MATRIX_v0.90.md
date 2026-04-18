# Demo Matrix - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: issue wave open

## Purpose

Define the planned proof surfaces for v0.90 before implementation starts.

## Demo Coverage Summary

| Demo ID | Demo title | Milestone claim | Command entry point | Primary proof surface | Status |
| --- | --- | --- | --- | --- | --- |
| D1 | Long-lived supervisor heartbeat | Supervisor can keep bounded agent state across cycles | To be defined by `#2021` | supervisor state and heartbeat/lease artifacts | planned |
| D2 | Cycle contract replay packet | Each cycle emits reviewable artifacts | To be defined by `#2022` | cycle manifest, observations, decision records, run refs | planned |
| D3 | Operator stop and guardrail controls | Operators remain authoritative over long-lived execution | To be defined by `#2024` / `#2025` | status output, stop marker, guardrail report | planned |
| D4 | Stock league long-lived demo | A bounded demo can show recurring supervised cycles safely | `cargo run --manifest-path adl/Cargo.toml -- demo demo-j-v090-stock-league-recurring --run --trace --out out --no-open`; scaffold remains available as `demo-i-v090-stock-league-scaffold` | recurring integration proof packet, state root, cycle ledger, inspections, continuity proof, and guardrail summary | integrated by `#2027` |
| D5 | Stock league proof expansion | Selected demo extensions can add reviewer evidence without weakening the primary stock-league proof | `cargo run --manifest-path adl/Cargo.toml -- demo demo-k-v090-stock-league-proof-expansion --run --trace --out out --no-open` | selected-demo manifest, extension proof packet, evidence index, replay manifest, non-goals/deferrals, and extension safety scan | landed by `#2028` |
| D6 | Repo visibility proof packet | ADL can map one milestone or feature slice from canonical docs to implementation, tests, demos, and review surfaces | `docs/milestones/v0.90/repo_visibility/` | manifest and code-doc-demo linkage report | landed by `#2031` |
| D7 | Milestone compression pilot | ADL can detect milestone drift from canonical state without silently mutating release truth | `python3 adl/tools/check_v090_milestone_state.py` | canonical state file, drift-check output, and generated status summary | landed by `#2030` |
| D8 | CodeBuddy multi-agent review showcase | ADL can present the CodeBuddy review-engine skill family as a product-style, packet-first repo review workflow without building the web app or mutating customer repos | `bash adl/tools/demo_v090_codebuddy_review_showcase.sh` | `artifacts/v090/codebuddy_review_showcase/run_manifest.json`, specialist reviews, redaction report, diagram artifacts, final report, and demo-operator classification | landed by `#2072`; intentionally `non_proving` until `#2070` lands |

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
- mark `review-quality-evaluator` as staged until `#2070` lands
- avoid live provider calls, customer repositories, billing, and product-app
  assumptions in the default demo path
- block publication until redaction and evidence gates are explicit

## Validation Expectations

The v0.90 issue wave is open. Implementation WPs should replace planned command
entry points with runnable commands only after they exist, and the docs/review
pass should verify that every claimed proof surface is backed by evidence.
