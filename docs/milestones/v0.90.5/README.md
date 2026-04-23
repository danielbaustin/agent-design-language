# v0.90.5 Planning Package

## Status

Draft milestone package. v0.90.5 is planned as the Governed Tools v1.0
milestone under planning issue #2350, with follow-up polish under #2402 and an
additional readiness review during the v0.90.3 WP-19 handoff pass.

The issue wave has not been opened. This package is the reviewable planning
source for a later WP-01 issue-wave creation pass. It is intended to be the
next immediately executable planning package after v0.90.4 closeout, not a
half-managed later idea lane.

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
- Feature index: FEATURE_DOCS_v0.90.5.md
- WP execution readiness: WP_EXECUTION_READINESS_v0.90.5.md
- Milestone checklist: MILESTONE_CHECKLIST_v0.90.5.md
- Release plan: RELEASE_PLAN_v0.90.5.md
- Release notes draft: RELEASE_NOTES_v0.90.5.md
- Issue wave draft: WP_ISSUE_WAVE_v0.90.5.yaml

## Execution Rule

This package is planning truth, not an execution claim. The WP issue wave must
be created from the reviewed YAML before implementation starts.

Once the wave opens, issue work should happen in issue worktrees. Root checkout
edits are not part of the ADL execution model.

WP-01 should use WP_EXECUTION_READINESS_v0.90.5.md as the card-authoring source
so every issue body carries concrete outputs, validation, non-goals, and
demo/proof expectations. This is especially important for v0.90.5 because the
milestone is security-sensitive and likely to become externally visible if UTS
stays public-compatible.

## Planning Mirror Rule

The local planning mirror is an ignored authoring surface used before promotion.
It should match this tracked milestone package unless a deliberate delta is
recorded. The tracked package remains review truth once promoted.

## Dependency On Earlier Milestones

v0.90.5 builds on:

- v0.90.3 citizen-state, standing, access-control, redacted projection, and
  inhabited CSM demo work
- v0.90.4 contract-market planning and authority lessons, if v0.90.4 remains
  focused on citizen economics

v0.90.5 should not own the v0.90.3 inhabited CSM demo. It should own the
Governed Tools v1.0 flagship demo.

It also should not inherit unresolved v0.90.4 ambiguity about whether contract
tool needs are constraints or execution grants. WP-19 of v0.90.4 is expected to
hand that boundary off cleanly.
