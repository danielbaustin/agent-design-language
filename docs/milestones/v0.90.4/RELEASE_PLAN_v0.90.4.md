# Release Plan - v0.90.4

v0.90.4 should release only when the contract-market substrate has a coherent
schema, lifecycle, authority, fixture, runner, demo, and review-summary story.

## Current State

- Issue wave opened by WP-01 as #2420.
- WP-02 through WP-20 are open as #2421 through #2440.
- The tracked milestone package now records the live issue map and execution
  order.
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

The closeout should hand off:

- payment-settlement options, if still relevant
- reputation and economic memory follow-ons
- inter-polis economics follow-ons
- production contract/legal/billing boundaries
- any deferred authority or trace negative cases
- v0.90.5 governed-tools follow-up for UTS, ACC, tool registry binding,
  executor authority, redaction, replay, denial records, and model testing
- any contract-market requirement that recorded tool needs only as a constraint
  and must be picked up by v0.90.5
- whether the canonical architecture packet changed and was updated, or was explicitly
  reviewed as unchanged with a tracked owner and rationale.
