# v0.91.7 Design

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Created: `2026-06-21`
- Last verified: `2026-07-18`
- Owner: ADL maintainers
- Setup lineage: `#3801`, `#3825`, `#4368`

## Purpose

Define how the v0.91.7 implementation/readiness tranche is organized,
validated, and handed into the required v0.91.8 bridge before v0.92.

Lifecycle note (2026-07-18): this is the retained planning design. WP-01
through WP-15 are now closed; current proof, findings, and non-claims live in
the issue-local packets and `review/V0917_SPRINT_REVIEW_REGISTER.md`.

## Problem Statement

After `v0.91.6`, both cognitive/protocol surfaces and operational substrates still need explicit implementation before `v0.92`: Curiosity, Constructability, reasoning graphs and skill standard,
security requirements, ACIP/A2A/protobuf requirements, affect and happiness
boundaries, Godel mechanics, and economics context.

## Goals

- Capture all source-ledger inputs from `PLANNING_SOURCE_CAPTURE_v0.91.7.md`.
- Complete the canonical planning-doc set for `v0.91.7`.
- Complete all second-tranche feature docs.
- Keep every surface distinct and reviewable.
- Define exactly what `v0.92` may consume.

## Non-Goals

- Runtime implementation.
- Protocol implementation.
- Birthday demo execution.
- Productization of affect, economics, or capability testing.

## Scope

### In Scope

- Planning docs from `docs/templates/planning/1.0.0`, excluding
  `feature_doc.md`.
- Eight second-tranche feature docs.
- Index, README, and proof-surface alignment.

### Out Of Scope

- Runtime code changes.
- `v0.92` activation approval.
- Enterprise security implementation.
- Full skill-standard ratification.

## Requirements

### Functional

- Every required second-tranche doc exists.
- Every feature doc names purpose, scope, non-goals, decisions, dependencies,
  validation, and `v0.92` consumption.
- No second-tranche surface is collapsed into generic future work.

### Non-Functional

- Deterministic document structure and review path.
- No host-local paths or private authoring links.
- Explicit integrated-proven, operator-scoped-out, or blocked-with-evidence states for unready surfaces.

## Proposed Design

`v0.91.7` is an implementation/readiness/control-plane tranche. It consumes `v0.91.6`
readiness truth, prepares the Soak #2 runtime-coherence gate, and produces
implementation/proof docs for `#3780` / `v0.92` activation refresh. Each feature doc
is a boundary and decision surface, not a runtime completion claim.

## Risks And Mitigations

- Risk: Curiosity and Constructability remain aspirational.
  - Mitigation: require artifacts, hooks, validators, and proof expectations.
- Risk: protocol requirements hide behind ACIP prose.
  - Mitigation: separate protobuf/JSON/WebSocket/access-rule decisions.
- Risk: affect and happiness language overclaims.
  - Mitigation: require safe tests and explicit public claim boundaries.

## Validation Plan

- Verify all expected docs exist.
- Run `git diff --check`.
- Scan for placeholder text and host-local paths.
- Run bounded docs review before PR publication.

## Exit Criteria

- Planning package and feature docs are complete enough for review.
- `#3780` can consume second-tranche implementation/proof truth without rediscovery.
