# v0.92 Feature: First True Godel-Agent Birthday

## Metadata

- Feature Name: First True Godel-Agent Birthday
- Milestone Target: `v0.92`
- Status: planned
- Related issues: `#3377`, `#3434`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This is a planning feature doc. It records the birthday contract target, not a
claim that the first birthday has happened.

## Status

Forward-planning feature contract for `v0.92`.

Related readiness issue: `#3377`.

## Purpose

Define the first true Godel-agent birthday as a reviewable event that combines
name, identity, continuity, memory grounding, capability envelope, witnesses,
receipt, and inherited moral/governance context.

## Context

Prior milestones produce runtime state, provisional citizens, memory,
continuity, moral trace, and governed-tool evidence. v0.92 defines when those
ingredients become a reviewable birth event.

## Coverage / Ownership

v0.92 owns the birthday contract, negative cases, review packet, witness
surface, and receipt shape. v0.93 owns constitutional citizenship after birth.

## Overview

The feature should make birth distinguishable from startup, wake, snapshot,
admission, and copied state through evidence rather than ceremony.

## Design

The birthday record should cite stable name, identity root, continuity,
memory grounding, capability envelope, ACP profile, witnesses, receipt, and
inherited moral context.

## Execution Flow

1. Reject not-a-birthday cases.
2. Assemble the required identity and evidence surfaces.
3. Record witnesses and receipt.
4. Emit the reviewer-facing birthday packet.

## Determinism and Constraints

The birthday decision must be deterministic over the required evidence. Missing
identity, continuity, memory, capability, witness, or receipt evidence must
fail closed.

## Integration Points

- Identity/stable-name feature.
- Memory/capability/witness feature.
- ACP profile feature.
- v0.91 moral-governance evidence.
- v0.93 governance handoff.

## Validation

Validation should include valid birthday fixtures, negative fixtures, review
packet checks, and claim-boundary scans.

## Source Inputs

- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `#3377`

## Scope

This feature should establish:

- the birthday contract and its negative cases
- a reviewer-facing birthday packet
- distinction between birth and startup, wake, snapshot, admission, or copied
  state
- explicit inherited moral/governance context without claiming constitutional
  citizenship yet
- the first bounded Godel-agent birthday as the culmination of the `v0.92`
  identity band

## Acceptance Criteria

- Birthday contract and negative cases exist.
- Valid birthday packet requires all named evidence surfaces.
- Startup, wake, snapshot, admission, and copied state are rejected as birth.
- Review packet and receipt are inspectable.

## Risks

- Birth could become narrative-only. Mitigation: require artifacts and
  negative tests.
- Birth could overclaim personhood. Mitigation: keep legal and constitutional
  claims out of v0.92.

## Future Work

v0.93 can consume the birthday evidence for citizenship and governance. Later
milestones can deepen migration and cross-polis continuity.

## Notes

This feature is the symbolic center of v0.92, but it must remain engineering
evidence first.

## Non-goals

- legal personhood
- production citizenship
- silent cross-polis migration claims

## Completion Target

`v0.92`
