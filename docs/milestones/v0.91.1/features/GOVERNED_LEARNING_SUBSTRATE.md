# Governed Learning Substrate

## Metadata

- Feature Name: Governed Learning Substrate
- Milestone Target: `v0.91.1`
- Status: landed
- Planned WP Home: WP-11
- Source Docs: `.adl/docs/TBD/learning_model/`
- Proof Modes: fixtures, policy, review
- Proof Route:
  - `adl/src/runtime_v2/governed_learning_substrate.rs`
  - `adl/src/runtime_v2/tests/governed_learning_substrate.rs`
  - `adl/tests/fixtures/runtime_v2/learning/governed_learning_substrate.json`
  - `docs/milestones/v0.91.1/review/governed_learning_fixture/`

## Purpose

Bound learning updates, feedback, adaptation, and rollback under explicit
policy. ADL should learn from evidence without allowing hidden self-modification
or unreviewable drift.

## Scope

In scope:

- Learning update and feedback contract.
- Policy boundary for adaptation and rollback.
- Fixtures for accepted feedback, rejected feedback, and unsafe update claims.
- Integration with landed capability, intelligence, and Theory-of-Mind evidence.

Out of scope:

- Autonomous retraining.
- Hidden model mutation.
- Grand unified learning theory.
- ANRM/Gemma placement claims before WP-12 lands.

## Acceptance Criteria

- Unsafe or hidden learning claims fail closed.
- Accepted learning updates preserve evidence and rollback references.
- Reviewers can inspect what changed and why.

## Landed Slice

WP-11 now lands the first bounded governed-learning substrate packet over the
already-landed capability, intelligence, and Theory-of-Mind surfaces. The
slice keeps adaptation reviewable by requiring explicit evidence, visible
review decisions, and preserved rollback references.

The landed packet exposes:

- explicit dependency linkage to the landed capability review bundle,
  intelligence packet, and Theory-of-Mind packet
- a rollback policy that preserves trust, sandbox, and scheduler guardrails
- three reviewable fixture classes:
  - accepted feedback update
  - rejected feedback claim
  - unsafe hidden update claim
- tracked review fixtures for the accepted, rejected, unsafe, and rollback
  cases

## Landed Artifacts

- `adl/src/runtime_v2/governed_learning_substrate.rs`
- `adl/src/runtime_v2/tests/governed_learning_substrate.rs`
- `adl/tests/fixtures/runtime_v2/learning/governed_learning_substrate.json`
- `docs/milestones/v0.91.1/review/governed_learning_fixture/accepted_feedback_update.json`
- `docs/milestones/v0.91.1/review/governed_learning_fixture/accepted_feedback_rollback.json`
- `docs/milestones/v0.91.1/review/governed_learning_fixture/rejected_feedback_claim.json`
- `docs/milestones/v0.91.1/review/governed_learning_fixture/unsafe_hidden_update_claim.json`

## Proof Notes

- WP-11 binds to the existing learning-overlay guardrails in `adl/src/learning_guardrails.rs`
  and the overlay application boundary in `adl/src/overlay.rs` instead of
  inventing a second mutation mechanism.
- Accepted learning updates remain bounded to reviewer-visible overlay
  suggestions with preserved rollback references.
- Hidden self-modification, autonomous retraining, and silent policy drift are
  denied before any update is accepted.
