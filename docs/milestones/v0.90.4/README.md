# v0.90.4 Planning Package

## Status

Active milestone package. v0.90.4 is the citizen economics and contract-market
substrate milestone. It was initially placed by planning issue #2271, polished
for execution readiness by #2389, reviewed again during the v0.90.3 WP-19
handoff pass, and opened by WP-01 as issue #2420 after the v0.90.3 release
ceremony.

The issue wave is now open as #2420 through #2440. This package is no longer
planning-only truth: it is the tracked execution map for the live v0.90.4 wave.

## Thesis

v0.90.4 should make bounded economic agency legible inside the CSM polis.

v0.90.3 is responsible for citizen-state safety: standing, private state,
access control, projection policy, continuity witnesses, challenge flow, and
quarantine/sanctuary behavior. v0.90.4 consumes that authority substrate and
adds the first practical market layer:

- citizens and authorized agents can publish bounded contracts
- qualified counterparties can submit bids
- evaluations are traceable and reviewable
- awards and acceptance are authority-checked
- delegation and subcontracting preserve parent responsibility
- external counterparties participate through explicit trust and gateway rules
- the contract lifecycle is stateful, signed, and auditable
- reviewers can inspect one bounded contract-market proof without payment rails

The milestone should prove contract-market mechanics before it attempts money,
settlement, pricing markets, or full inter-polis trade.

## Directory Shape

- root planning docs and WP YAML live in this directory
- feature contracts live under features/
- context and later-band backgrounders live under ideas/

## Scope Boundary

In scope:

- inheritance and readiness audit from v0.90.3 citizen-state outputs
- contract schema
- bid schema
- evaluation and selection model
- transition authority model
- contract lifecycle state machine
- external counterparty participation rules
- delegation and subcontracting model
- contract-market fixture set
- deterministic contract-market runner
- reviewer-facing contract-market summary artifact
- one bounded contract-market demo
- release/review evidence that the economics layer consumes citizen standing
  and access-control authority rather than redefining citizenship
- explicit handoff to v0.90.5 for governed tool-call semantics, UTS, ACC, and
  tool execution authority when contracts or bids require tools

Out of scope:

- payment settlement
- Lightning, x402, stablecoin, banking, invoicing, or other payment rails
- full inter-polis economics
- full constitutional governance
- redefining citizen standing, admission, private state, or continuity
- open-ended autonomous markets
- production identity, KYC, billing, tax, or legal contracting systems
- implementing Governed Tools v1.0, Universal Tool Schema, ADL Capability
  Contracts, tool registry semantics, or direct model-to-tool execution

## Canonical Planning Docs

- Vision: VISION_v0.90.4.md
- Design: DESIGN_v0.90.4.md
- Work Breakdown Structure: WBS_v0.90.4.md
- Sprint plan: SPRINT_v0.90.4.md
- Decisions log: DECISIONS_v0.90.4.md
- Demo matrix: DEMO_MATRIX_v0.90.4.md
- Economics inheritance audit:
  ECONOMICS_INHERITANCE_AND_AUTHORITY_AUDIT_v0.90.4.md
- CI runtime policy for execution sessions: CI_RUNTIME_POLICY_v0.90.4.md
- Rust validation acceleration posture:
  RUST_VALIDATION_ACCELERATION_v0.90.4.md
- Finish test topology for validation throughput:
  FINISH_TEST_TOPOLOGY_v0.90.4.md
- Feature index: FEATURE_DOCS_v0.90.4.md
- WP execution readiness: WP_EXECUTION_READINESS_v0.90.4.md
- Milestone checklist: MILESTONE_CHECKLIST_v0.90.4.md
- Release plan: RELEASE_PLAN_v0.90.4.md
- Release notes draft: RELEASE_NOTES_v0.90.4.md
- Issue wave: WP_ISSUE_WAVE_v0.90.4.yaml

## Execution Rule

WP-01 created the issue wave from the reviewed YAML. Treat
WP_ISSUE_WAVE_v0.90.4.yaml as the issue-number source of truth for the live
milestone.

Once the wave opens, issue work should happen in issue worktrees. Root checkout
edits are not part of the ADL execution model.

WP-01 should use WP_EXECUTION_READINESS_v0.90.4.md as the card-authoring source
so every issue body carries concrete outputs, validation, non-goals, and source
docs instead of generic implementation language.

## Open Issue Wave

The official v0.90.4 wave is open:

- WP-01 through WP-20 are #2420 through #2440
- Sprint 1 is #2420 through #2424
- Sprint 2 is #2425 through #2429
- Sprint 3 is #2430 through #2434
- Sprint 4 is #2435 through #2440

WP-14A remains explicit as #2434 so demo/proof coverage does not disappear into
review tail work.

## Compression Rule

v0.90.4 should use the compression model learned in v0.90.1 through v0.90.3:

- make schema, lifecycle, authority, fixture, and runner contracts explicit
  before widening implementation
- keep the first market proof small and reviewer-visible
- front-load negative cases for unauthorized transition, invalid bid,
  unsupported delegation, and trace discontinuity
- keep demo claims narrow
- preserve the demo-matrix WP before quality/docs/review convergence
- let docs-only and fixture-only WPs run with focused validation
- do not compress away authority checks, review summaries, external-party
  boundaries, or release-truth work

## Dependency On v0.90.3

v0.90.4 must not define who is a citizen or what private state authority means.
It depends on the v0.90.3 citizen-state substrate for standing, access-control,
projection, challenge, appeal, and continuity semantics.

If v0.90.3 defers a required authority surface, v0.90.4 should either narrow its
proof to a fixture-backed boundary or delay the affected WP.

The current handoff result is favorable: v0.90.3 closed internal review,
third-party review, and accepted-finding disposition without any blocking
economics-facing carryover.

## Handoff To v0.90.5

Contracts and bids may describe tool requirements, resource estimates, adapter
expectations, or evidence emitted by tool-mediated work. They must not authorize
tool execution by themselves.

v0.90.4 should record tool needs as contract constraints and review evidence.
v0.90.5 owns the governed tool layer: UTS, ACC, tool registry binding,
capability compilation, Freedom Gate mediation for tool use, executor behavior,
redaction, replay, denial records, and multi-model tool-call testing.
