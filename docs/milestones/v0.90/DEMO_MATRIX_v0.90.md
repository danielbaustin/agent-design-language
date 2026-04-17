# Demo Matrix - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: tracked planning package

## Purpose

Define the planned proof surfaces for v0.90 before implementation starts.

## Demo Coverage Summary

| Demo ID | Demo title | Milestone claim | Command entry point | Primary proof surface | Status |
| --- | --- | --- | --- | --- | --- |
| D1 | Long-lived supervisor heartbeat | Supervisor can keep bounded agent state across cycles | TBD | supervisor state and heartbeat/lease artifacts | planned |
| D2 | Cycle contract replay packet | Each cycle emits reviewable artifacts | TBD | cycle manifest, observations, decision records, run refs | planned |
| D3 | Operator stop and guardrail controls | Operators remain authoritative over long-lived execution | TBD | status output, stop marker, guardrail report | planned |
| D4 | Stock league long-lived demo | A bounded demo can show recurring supervised cycles safely | TBD | fixture-backed or delayed/public stock league run packet | planned |
| D5 | Demo extension lane | Selected new or extended demos can be added without weakening the primary stock-league proof | TBD | per-demo proof packets, commands, and explicit non-goals | planned |
| D6 | Repo visibility proof packet | ADL can map one milestone or feature slice from canonical docs to implementation, tests, demos, and review surfaces | TBD | manifest and code-doc-demo linkage report | planned |
| D7 | Milestone compression pilot | ADL can detect milestone drift from canonical state without silently mutating release truth | TBD | canonical state file, drift-check output, and generated status summary | planned |

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

## Validation Expectations

The `v0.89.1` WP-19 promotion gate should keep this matrix as planning
until the issue wave creates real commands and proof artifacts. Implementation
WPs should replace TBD command entry points with runnable commands only after
they exist.
