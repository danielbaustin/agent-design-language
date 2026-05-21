# Release Plan - v0.91.2

## Release Theme

Ship the pressure-release milestone that makes later work faster, safer, and
more reviewable without weakening proof or authority boundaries.

## Required Evidence

- benchmark harness and comparison report
- runtime/test-cycle recovery packet
- review-product surfaces
- Workspace bridge packet
- modernization demo packet
- publication and rustdoc/doc packets
- workflow guardrails packet
- demo-proof convergence plus missing code-feature demo follow-ons
- feature-proof coverage and quality gate
- `WP-20B` full internal review packet
- accepted `WP-20B` finding fixes and re-review record
- review and remediation records

## Release Risks

- benchmark work could be mistaken for execution authority
- runtime recovery could be mistaken for permission to weaken proofs
- Workspace bridge could be mistaken for canonical-source relocation
- publication packets could be mistaken for publication itself
- guardrail docs could understate the severity of workflow failures
- the thin `WP-20` internal review packet could be mistaken for controlling
  review truth after `WP-20B`
- external review could start before accepted `WP-20B` findings are fixed and
  rechecked

## Release Rule

Do not mark `v0.91.2` releasable until:

- the remaining Sprint 4 work finishes cleanly beyond the already-executed
  `WP-01` through `WP-19` band plus bounded `WP-17A` follow-on
- the corrective `WP-20B` findings are fixed or explicitly dispositioned and
  rechecked
- quality, review, remediation, and ceremony surfaces are complete
- release readiness and release evidence are complete
