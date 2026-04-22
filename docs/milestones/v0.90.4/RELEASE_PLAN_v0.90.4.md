# Release Plan - v0.90.4

v0.90.4 should release only when the contract-market substrate has a coherent
schema, lifecycle, authority, fixture, runner, demo, and review-summary story.

## Release Gates

The milestone is releasable when:

- v0.90.3 citizen-state dependencies are explicitly inherited or safely
  fixture-backed
- contracts and bids have validation fixtures
- evaluation and selection are reviewable
- lifecycle transitions are authority-checked
- external counterparty participation is bounded
- delegation preserves parent responsibility
- the contract-market runner emits deterministic proof artifacts
- the demo matrix distinguishes landed, skipped, failed, and non-proving claims
- the feature proof coverage WP is complete before quality/docs convergence
- internal and external review findings are fixed or dispositioned
- release notes describe actual shipped scope

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
