# v0.90.5 Planning Package

## Status

Tracked planning package for Governed Tools v1.0. The milestone direction was
established under planning issue #2350, tightened under #2402, prepared for
clean execution under #2443, and re-affirmed as the immediate governed-tools
follow-on during the previous-milestone handoff pass under #2439.

The issue wave is open under [#2566](https://github.com/danielbaustin/agent-design-language/issues/2566), with [#2567](https://github.com/danielbaustin/agent-design-language/issues/2567) through [#2591](https://github.com/danielbaustin/agent-design-language/issues/2591) carrying the rest of the tracked milestone band. This package is now active execution truth for `v0.90.5`, not a later planning packet.

## Parallel Python Reduction Tranche

v0.90.5 should remain Governed Tools v1.0. It should not be repurposed into a
stop-the-world Python rewrite gate.

Instead, the milestone should reserve a small bounded Python-reduction tranche
alongside the main governed-tools work. The cross-milestone policy is recorded
in [Python Elimination Staged Plan](../../planning/PYTHON_ELIMINATION_STAGED_PLAN.md).

The expected `v0.90.5` Python tranche is:

- freeze and no-new-tracked-Python discipline
- inventory and disposition truth surface
- one high-leverage Rust port or delete wave for a coherent Python tooling
  family

## Thesis

v0.90.5 should make tools a first-class governed ADL primitive.

Current industry tool calling is too weak for ADL-grade agents. A tool call is
often treated as a function invocation emitted by a model, with too little
attention to actor identity, authority, delegation, privacy, visibility,
side effects, replay, trace, and denial behavior.

ADL should instead treat tool use as governed capability exercise:

- models propose tool use
- Universal Tool Schema describes portable tool shape, semantics, and baseline
  risk metadata
- ADL Capability Contracts define runtime authority, identity, privacy,
  visibility, trace, replay, and Freedom Gate requirements
- the tool-to-capability compiler validates and normalizes proposals before
  execution
- the governed executor runs only approved actions
- tests prove unsafe model output cannot become action merely because it is
  persuasive or valid JSON

## Directory Shape

- root planning docs and WP YAML live in this directory
- feature contracts live under features/
- context and later-band backgrounders live under ideas/

## Execution Support Notes

The governed-tools WBS is the milestone scope. Supporting execution plans may
exist alongside it when they help the milestone land without widening the core
story.

Current tracked execution-support note:

- [v0.90.5 Get-Well Plan](GET_WELL_PLAN_v0.90.5.md)
- [v0.90.5 Test Runtime Reduction Plan](ideas/TEST_RUNTIME_REDUCTION_PLAN_v0.90.5.md)

The get-well plan is the milestone-root pointer; the test-runtime reduction
plan is the detailed source note under `ideas/`.

This recovery work is intentionally kept as execution support:

- it is active and worth keeping visible
- it does not define Governed Tools v1.0 semantics
- it is tracked as a separate GW wave instead of surprise extra WPs
- it gives `v0.90.5` a truthful home for the remaining CI/runtime reduction
  work if the wall-time problem stays material

Opened get-well wave:

- GW-00 / #2592 Get-well test runtime reduction wave
- GW-01 / #2593 Collapse external counterparty proof-family tests
- GW-02 / #2594 Collapse private-state observatory proof-family tests
- GW-03 / #2595 Collapse delegation subcontract proof-family tests
- GW-04 / #2596 Collapse contract-market and resource-stewardship proof-family tests
- GW-05 / #2597 Shrink CLI and demo proof-matrix tail

## Scope Boundary

In scope:

- Universal Tool Schema v1.0 public-compatible schema discipline
- ADL Capability Contract v1.0 authority and privacy schema
- UTS fixture and conformance suite
- ACC authority, visibility, delegation, trace, and redaction fixtures
- tool registry and adapter binding rules
- deterministic UTS-to-ACC compiler
- normalization and argument validation
- policy injection and authority evaluation
- Freedom Gate integration
- governed executor
- trace, replay, redaction, and evidence contract
- dangerous tool negative suite
- multi-model proposal benchmark
- local model and Gemma-focused evaluation
- one flagship Governed Tools v1.0 demo

Out of scope:

- public tool marketplace
- arbitrary shell execution by model output
- production secrets integration
- production cloud sandbox requirement
- real destructive filesystem or network side effects in demos
- claim that UTS alone is enough for ADL safety
- claim that JSON compatibility implies permission to execute
- replacing citizen standing, access control, or Freedom Gate semantics

## Canonical Planning Docs

- Vision: VISION_v0.90.5.md
- Design: DESIGN_v0.90.5.md
- Work Breakdown Structure: WBS_v0.90.5.md
- Sprint plan: SPRINT_v0.90.5.md
- Decisions log: DECISIONS_v0.90.5.md
- Demo matrix: DEMO_MATRIX_v0.90.5.md
- Feature proof coverage: FEATURE_PROOF_COVERAGE_v0.90.5.md
- Feature index: FEATURE_DOCS_v0.90.5.md
- WP execution readiness: WP_EXECUTION_READINESS_v0.90.5.md
- Milestone checklist: MILESTONE_CHECKLIST_v0.90.5.md
- Release plan: RELEASE_PLAN_v0.90.5.md
- Release readiness: RELEASE_READINESS_v0.90.5.md
- Third-party review handoff: ADL_v0.90.5_THIRD_PARTY_REVIEW_HANDOFF.md
- Next-milestone handoff: NEXT_MILESTONE_HANDOFF_v0.90.5.md
- Release notes draft: RELEASE_NOTES_v0.90.5.md
- Opened issue wave: WP_ISSUE_WAVE_v0.90.5.yaml
- Get-well plan: GET_WELL_PLAN_v0.90.5.md
- Execution-support idea note: ideas/TEST_RUNTIME_REDUCTION_PLAN_v0.90.5.md

## Execution Rule

This package is execution truth for the opened `v0.90.5` wave.

Issue work should happen in issue worktrees. Root checkout edits are not part
of the ADL execution model.

WP-01 used WP_EXECUTION_READINESS_v0.90.5.md as the card-authoring source so
every issue body carries concrete outputs, validation, non-goals, and
demo/proof expectations. This remains especially important for `v0.90.5`
because the milestone is security-sensitive and likely to become externally
visible if UTS stays public-compatible.

## Closeout Sequence

`v0.90.5` keeps the established ADL closeout state machine after the flagship
demo:

- WP-19 demo matrix and feature proof coverage
- WP-20 coverage / quality gate
- WP-21 docs + review pass
- WP-22 internal review
- WP-23 external / 3rd-party review
- WP-24 review findings remediation
- WP-25 next milestone planning
- WP-26 release ceremony

## WP-25 Handoff Result

WP-25 captures the clean handoff into the next milestone band in
[NEXT_MILESTONE_HANDOFF_v0.90.5.md](NEXT_MILESTONE_HANDOFF_v0.90.5.md).

The important result is not “there is a vague backlog.” The result is:

- `v0.90.5` landed Governed Tools v1.0 and the first ACIP tranche
- `v0.91` is the immediate full implementation milestone for moral governance,
  cognitive-being, secure intra-polis comms, structured planning, and SRP
- `v0.91.1` absorbs the adjacent-systems lane, including the full comparison
  report and A2A implementation hardening

## Planning Mirror Rule

The local planning mirror is an ignored authoring surface used before promotion.
It should match this tracked milestone package unless a deliberate delta is
recorded. The tracked package remains review truth once promoted.

## Dependency On Earlier Milestones

v0.90.5 builds on:

- v0.90.3 citizen-state, standing, access-control, redacted projection, and
  inhabited CSM demo work
- the previous contract-market milestone's authority and bounded-economics
  lessons that bear on governed delegation and action approval

v0.90.5 should not own the v0.90.3 inhabited CSM demo. It should own the
Governed Tools v1.0 flagship demo.

It also should not inherit unresolved ambiguity about whether contract tool
needs are constraints or execution grants. That earlier boundary is now clean:
the previous milestone leaves no loose issue backlog here and hands off only
the governed-tool authority lane that contracts and bids were explicitly not
allowed to absorb.

Current docs-review findings may still sharpen wording, rustdoc references, and
public-spec boundaries, but they should not widen the core milestone scope away
from governed tools. 
