# Structured Task Prompt

Template: 1.0.0

Issue: 5817

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Reconcile and activate the v0.92 planning package and child issue wave.

## Deliverables

- Current canonical v0.92 version and milestone docs
- Evidence-reconciled WP dependency graph
- Final opened child issue wave
- Six typed cards per opened issue
- Focused validation and exact review evidence

## Acceptance

1. Canonical version and v0.92 planning docs agree
2. Every prerequisite is consumed, gap-routed, or explicitly deferred
3. Issue #5104 loop-runtime evidence is requalified against current Runtime v3
4. The final issue wave is acyclic and contains no duplicate live issues
5. Every opened issue has SIP, STP, SPP, VPP, SRP, and SOR cards
6. Focused YAML, link, schema, version, dependency, and diff validation passes
7. One bounded exact pre-PR review has no actionable findings

## Dependencies

- Completed v0.91.8 release and handoff package
- v0.91.8 activation-test map and AEE completion evidence
- Issue 3377 readiness packet
- Issue 5359 WP-22 planning dispositions
- Issue 5104 merged loop-runtime evidence

## Inputs

- docs/milestones/v0.92
- docs/milestones/v0.91.8
- .adl/docs/TBD
- README.md
- docs/README.md
- adl/Cargo.toml
- AGENTS.md

## Non Goals

- Child WP implementation
- Repository transfer
- v0.93 governance implementation
- Unsupported birthday or legal-status claims
