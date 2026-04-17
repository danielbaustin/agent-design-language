# Milestone README - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: issue wave open
- Early planning issue: #1986
- Promotion gate: #1940

## Purpose

Provide the tracked planning entry point for v0.90 after `v0.89.1` WP-19
promoted the package into milestone docs.

This directory intentionally mirrors the tracked milestone layout:

- root planning docs and issue-wave YAML live in this directory
- feature contracts live under features/
- reader-facing idea/backgrounder docs live under ideas/

## Milestone Thesis

v0.90 should make ADL capable of supervising long-lived agents with bounded
cycles, durable continuity handles, operator safety controls, and concrete demo
evidence.

The milestone should not claim to ship the full v0.92 identity/capability
substrate. It should create a practical pre-identity long-lived runtime slice
that later identity work can adopt or migrate.

## Scope Summary

### In Scope

- long-lived supervisor and heartbeat behavior
- bounded cycle contracts and artifact layout
- pre-v0.92 state and continuity handles
- operator stop, safety, and guardrail controls
- status and inspection surfaces for supervised agents
- a stock league demo as a bounded long-lived-agent proof slice
- selected demo additions or extensions that have explicit proof claims and
  validation commands
- a 93 percent coverage ratchet tranche with measurement before threshold
  changes
- a careful milestone-compression pilot based on canonical state and drift
  checks
- a bounded repo visibility prototype with canonical-source and code-doc-demo
  linkage evidence
- explicit Rust refactoring justified by maintainability, testability, or
  review evidence
- a narrow decision on whether minimal trace/status queries belong in v0.90

### Out Of Scope

- full v0.92 identity/capability substrate
- live financial advice or live trading behavior
- unbounded autonomous agents
- full autonomous release approval or silent closeout automation
- full semantic indexing of the entire repository as a repo-visibility platform
- full multi-agent society, accountability, or counterfactual reasoning
- retroactively changing the v0.90 issue wave outside the tracked WP-01 branch

## Document Map

Canonical planning docs:

- Vision: VISION_v0.90.md
- Design: DESIGN_v0.90.md
- Work Breakdown Structure: WBS_v0.90.md
- Feature index: FEATURE_DOCS_v0.90.md
- Sprint plan: SPRINT_v0.90.md
- Decisions log: DECISIONS_v0.90.md
- Demo matrix: DEMO_MATRIX_v0.90.md
- Milestone checklist: MILESTONE_CHECKLIST_v0.90.md
- Release plan: RELEASE_PLAN_v0.90.md
- Release notes draft: RELEASE_NOTES_v0.90.md
- WP execution readiness: WP_EXECUTION_READINESS_v0.90.md
- Issue wave: WP_ISSUE_WAVE_v0.90.yaml
- Milestone compression pilot: milestone_compression/README.md
- Repo visibility prototype: repo_visibility/README.md

Planning control docs:

- EARLY_PLANNING_LANE.md
- V090_PLANNING_INVENTORY_AND_WP19_HANDOFF.md
- LONG_LIVED_STOCK_LEAGUE_ISSUE_BODY.md

Feature and idea lanes:

- features/README.md
- ideas/README.md

## Execution Model

This tracked package assumes a normal ADL milestone wave after `v0.89.1`
release closeout:

- WP-01: promote and finalize the v0.90 milestone package
- WP-02 through WP-05: long-lived runtime feature band
- WP-06: minimal inspection / trace / status boundary
- WP-07 and WP-08: stock league demo and integration proof
- WP-09: demo extensions and proof expansion
- WP-10: coverage ratchet to 93 percent
- WP-11: milestone compression pilot
- WP-12: repo visibility prototype
- WP-13 and WP-14: docs pass and explicit Rust refactoring
- WP-15 onward: reviews, remediation, readiness, next planning, release

The v0.90 issue wave is now open. WP-01 is `#2019`; WP-02 through WP-20
are `#2021` through `#2039`.

WP_EXECUTION_READINESS_v0.90.md records the execution gates for those open
work packages. WP sessions should use it to bind each issue to concrete feature
docs, proof surfaces, validation expectations, and non-goals before changing
code or demos.

## Demo And Validation Surface

Primary planned demo:

- stock league long-lived agents demo

Additional planned demo lane:

- selected new or extended demos, each with a named proof claim, validation
  command, and non-goals

Primary planned proof surfaces:

- bounded supervisor state
- heartbeat / lease status
- cycle manifest and ledger artifacts
- continuity handle files
- operator stop and guardrail reports
- delayed/public or fixture-backed paper market data
- demo extension proof packets
- coverage ratchet report to 93 percent
- milestone state and drift-check report
- repo visibility manifest and linkage report
- Rust refactor validation record

## Status

- Planning: tracked package promoted by `v0.89.1` WP-19 and opened by v0.90 WP-01
- Execution: issue wave open; WP-02 starts from the execution-readiness gate in
  WP_EXECUTION_READINESS_v0.90.md
- Validation: not started
- Release readiness: not started

## WP-19 Promotion Result

The `v0.89.1` WP-19 promotion gate:

- promotes this package into `docs/milestones/v0.90`
- records the official v0.90 issue graph after WP-01 opens the wave
- preserves feature contracts under `features/`
- preserves backgrounder and later-band context under `ideas/`

The `v0.89.1` WP-20 release ceremony treated the v0.90 planning package as
ready, and v0.90 WP-01 now records the opened issue wave.
