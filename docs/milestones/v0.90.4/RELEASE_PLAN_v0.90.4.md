# Release Plan - v0.90.4

v0.90.4 should release only when the contract-market substrate has a coherent
schema, lifecycle, authority, fixture, runner, demo, and review-summary story.

## Current State

- Issue wave opened by WP-01 as #2420.
- WP-02 through WP-14A are closed as #2421 through #2434.
- WP-15 through WP-20 remain open as #2435 through #2440.
- The tracked milestone package now records the live issue map and execution
  order.
- Demo/proof coverage has been carried through WP-14A and is now recorded in
  DEMO_MATRIX_v0.90.4.md.
- Active crate/package version-reporting surfaces should read `0.90.4` during
  the live release tail, even while the latest published tag remains `v0.90.3`
  until ceremony.

## Release Gates

The milestone is releasable when:

- v0.90.3 closes with no blocking economics-facing carryover from its review
  tail
- v0.90.3 citizen-state dependencies are explicitly inherited or safely
  fixture-backed
- contracts that mention tool-mediated work express tool needs as constraints,
  not execution authority
- contracts and bids have validation fixtures
- evaluation and selection are reviewable
- lifecycle transitions are authority-checked
- external counterparty participation is bounded
- delegation preserves parent responsibility
- the contract-market runner emits deterministic proof artifacts
- the demo matrix distinguishes landed, skipped, failed, and non-proving claims
- the feature proof coverage WP is complete before quality/docs convergence
- internal and external review findings are fixed or dispositioned
- architecture refresh evidence is complete for the milestone period (`architecture-update`
  or explicit `architecture-reviewed-unchanged`).
- release notes describe actual shipped scope
- end-of-milestone report is written before ceremony

## Quality Bar

The release should prefer small, truthful proof over broad claims. If payment,
inter-polis economics, or production counterparty verification is not built, the
release must say so plainly.

## Handoff

The closeout should record a zero-leftover handoff:

- no open carry-forward issue backlog from `v0.90.4`
- v0.90.5 governed-tools follow-up for UTS, ACC, tool registry binding,
  executor authority, redaction, replay, denial records, and model testing
- any contract-market requirement that recorded tool needs only as a constraint
  and must be picked up by v0.90.5
- payment, legal/billing, and inter-polis items remain explicit non-goals
  unless some later milestone deliberately plans them from scratch
- whether the canonical architecture packet changed and was updated, or was
  explicitly reviewed as unchanged with a tracked owner and rationale

The detailed handoff record should live in
`NEXT_MILESTONE_HANDOFF_v0.90.4.md`.
