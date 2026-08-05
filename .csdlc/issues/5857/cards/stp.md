# Structured Task Prompt

Template: 1.0.0

Issue: 5857

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare and operate only the first-birthday-core sprint umbrella; do not implement its child issues.

## Deliverables

- Issue-specific six-card sprint record
- Sprint Execution Packet with membership, lanes, gates, review path, and activity log
- Integrated sprint review and truthful umbrella closeout record

## Acceptance

1. AC-1: The Sprint Execution Packet records exact child membership and dependency order.
2. AC-2: Safe parallel lanes and serial gates are explicit and do not overlap child ownership.
3. AC-3: The umbrella coordinates only; each child retains implementation, proof, review, publication, and closeout authority.
4. AC-4: Every child handoff requires issue-bound bind, readiness, and session-goal truth before implementation.
5. AC-5: Integrated sprint review and umbrella closeout occur only after every child reaches truthful terminal state.

## Dependencies

- #5825
- #5826
- #5827
- #5828
- #5829
- #5830
- #5831
- #5833
- #5834

## Inputs

- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/SPRINT_v0.92.md
- .csdlc/prepared/issues/5857/sprint-execution-packet.yaml
- #5825
- #5826
- #5827
- #5828
- #5829
- #5830
- #5831
- #5833
- #5834

## Non Goals

- Implementing child issue code
- Replacing child C-SDLC records
- Collapsing child review or publication into the umbrella
- Claiming parallel execution beyond the declared packet
