# Cognitive SDLC Default Operation

## Metadata

- Feature Name: Cognitive SDLC Default Operation
- Milestone Target: `v0.91.4`
- Status: planned
- Planned WP Home: WP-01 through WP-16

## Purpose

Make C-SDLC the normal software-development path for future ADL issues.

## Acceptance Criteria

- New software-development issues receive correct SIP, STP, SPP, SRP, and SOR
  cards.
- `SPP` is tracked as the issue-local operative execution plan, not as sprint
  orchestration, review truth, or output truth.
- Workflow-conductor routes each lifecycle stage.
- Card editors repair drift without hand edits.
- PR publication requires review truth.
- Closeout records merge/main truth and memory handoff.
- Durable cards, sprint state, review, proof, trace, and release evidence are
  tracked in Git.
- Durable proof includes minimal signed trace bundles before default operation
  is claimed.
- ObsMem consumes tracked evidence, not untracked local artifacts.

## Non-Claims

This feature does not remove human review or protected branch controls.
