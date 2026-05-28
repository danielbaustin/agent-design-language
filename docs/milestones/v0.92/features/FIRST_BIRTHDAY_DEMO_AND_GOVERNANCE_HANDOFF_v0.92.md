# v0.92 Feature: First Birthday Demo And Governance Handoff

## Metadata

- Feature Name: First Birthday Demo And Governance Handoff
- Milestone Target: `v0.92`
- Status: planned
- Related issues: `#3377`, `#3434`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This is a planning feature doc. It defines the proof and handoff surface for
the first-birthday milestone without claiming the demo has run or governance is
complete.

## Purpose

Define the flagship demo and handoff surfaces that prove v0.92 produced a
reviewable birthday record and that v0.93 can consume the resulting identity
evidence without redefining birth.

## Context

The first birthday will be judged by its evidence packet and demo quality. A
good milestone needs a demo that is boringly inspectable and a governance
handoff that is explicit about what v0.93 does and does not inherit.

## Coverage / Ownership

v0.92 owns the first-birthday demo, negative suite, proof coverage, birthday
review packet, and v0.93 handoff map. v0.93 owns citizenship and polis
governance implementation.

## Overview

The feature should package birthday behavior into a runnable proof surface and
turn the evidence into a downstream governance input.

Key capabilities:

- run a first-birthday proof demo
- run not-a-birthday negative cases
- assemble the birthday review packet
- map identity evidence to v0.93 governance inputs

## Design

### Core Concepts

- Flagship demo: runnable proof of a valid birthday record.
- Negative suite: cases that must not count as birth.
- Governance handoff map: identity evidence organized for v0.93 consumption.

### Architecture

- Inputs: feature contracts, schemas, fixtures, birthday record, witnesses,
  receipts, validation logs, and review findings.
- Outputs: demo artifacts, negative-case report, review packet, proof coverage
  map, and v0.93 handoff.
- Interfaces: demo commands, review packet docs, release evidence.
- Invariants: demo output must cite evidence; governance remains downstream.

### Data / Artifacts

- Demo command/runbook.
- Demo artifact directory or packet.
- Negative-case report.
- Birthday review packet.
- Governance handoff map.

## Execution Flow

1. Build valid birthday fixtures and negative fixtures.
2. Run the flagship birthday demo.
3. Run negative cases.
4. Assemble review packet and proof coverage.
5. Produce v0.93 governance handoff map.

## Determinism and Constraints

- Demo commands must be repeatable enough for reviewers.
- Negative cases must fail closed.
- Handoff must not claim v0.93 governance is complete.
- Demo language must distinguish evidence, interpretation, and future claims.

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| Demo Matrix | write | Records demo commands, proof surfaces, and status. |
| Release Evidence | write | Supplies birthday proof artifacts for closeout. |
| Governance Planning | handoff | Supplies identity evidence for v0.93. |
| Review Packet | write | Packages the milestone proof for internal and external review. |

## Validation

### Demo

- Demo script(s): final command pending v0.92 implementation.
- Expected behavior: valid birthday packet is accepted; not-a-birthday cases
  are rejected.

### Deterministic / Replay

- Replay requirements: demo commands, fixtures, expected outputs, and allowed
  nondeterminism must be recorded.
- Determinism guarantees: negative cases fail consistently.

### Schema / Artifact Validation

- Schemas involved: identity, continuity, memory grounding, capability, ACP,
  ACIP, witness, receipt, and review-packet artifacts as implemented.
- Artifact checks: required proof artifacts exist and are linked.

### Tests

- Test surfaces: demo runner or fixture validation, negative suite, link and
  claim-boundary checks.

### Review / Proof Surface

- Review method: internal review followed by external review if the milestone
  follows the current cadence.
- Evidence location: demo matrix, proof coverage, release evidence, review
  handoff, and received review packet.

## Acceptance Criteria

- First-birthday demo has a runnable command or explicitly recorded runner.
- Negative suite proves startup, wake, snapshot, admission, and copied state
  are not birth.
- Birthday review packet is complete and evidence-bound.
- v0.93 handoff maps identity evidence without claiming governance completion.

## Risks

- Risk: demo becomes ceremonial instead of evidentiary.
- Mitigation: require fixtures, validation output, and review packet links.
- Risk: handoff overclaims governance.
- Mitigation: require explicit v0.93 non-claim language.

## Future Work

- v0.93 should consume the handoff for constitutional citizenship and polis
  governance.
- Later first-birthday public materials can draw on the demo only after review
  and release evidence are complete.

## Notes

This feature is where the birthday becomes visible to reviewers. It must be
clear, boring, repeatable, and hard to misread.
