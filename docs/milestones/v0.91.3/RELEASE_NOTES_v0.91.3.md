# v0.91.3 Release Notes

## Metadata

- Product: `Agent Design Language`
- Version: `v0.91.3`
- Release date: pending release ceremony
- Tag: `v0.91.3`

## How To Use

These are draft release notes for the milestone package. Final release notes
must be refreshed during the release ceremony and must describe only shipped
behavior.

# `Agent Design Language` `v0.91.3` Release Notes

## Summary

`v0.91.3` introduces the first C-SDLC implementation slice: one bounded
Cognitive State Transition represented through public tracked cards, transition
evidence, review synthesis, merge-readiness, and memory handoff planning.

## Highlights

- Establishes the first concrete Cognitive State Transition milestone package.
- Preserves the corrected `SIP -> STP -> SPP -> SRP -> SOR` card lifecycle.
- Defines `SPP` as the Structured Plan Prompt: issue-local operative execution
  plan truth.
- Adds transition manifest, DAG, evidence, merge gate, and ObsMem handoff work
  packages.
- Adds a full release-tail sequence through internal review, external review,
  remediation, next-milestone planning, next-milestone review, and ceremony.

## What's New In Detail

### C-SDLC First Slice

- Transition manifest, actor-role seed, and state model work landed in `WP-02`.
- Transition DAG and shard plan work landed in `WP-04`.
- Evidence bundle and review synthesis landed in `WP-05`.

### Governance And Memory

- Merge-readiness gate work preserves issue, PR, branch, CI, and human-review
  truth.
- SRP review results and SOR outcome truth receive an ObsMem handoff boundary.
- Tracked workflow records are named as the public proof direction.

### Proof And Review Tail

- First proof demo landed in `WP-09`.
- Proof coverage, quality gate, and docs review have completed. Internal
  review, external review, remediation, next planning, next review, and release
  ceremony remain distinct ordered work packages.

## Upgrade Notes

- Future C-SDLC work should use `SPP` as Structured Plan Prompt terminology.
- Local `.adl` execution state is not sufficient public proof by itself.
- `v0.91.4` should be treated as the repeatability and default-operation
  milestone.

## Known Limitations

- `v0.91.3` does not make C-SDLC the default for all future ADL development.
- Signed trace proof is planned for `v0.91.4`; `v0.91.3` records the handoff and
  proof requirements.
- Software Development Polis standing is seeded, not fully enforced.

## Validation Notes

- Final validation must come from the milestone quality gate, demo matrix,
  review packets, and release ceremony.
- This draft must be reconciled against completed issues before release.

## What's Next

- `v0.91.4` hardens C-SDLC into repeatable default operation.
- Post-`v0.91.4` milestones can build broader product and social-cognition work
  on the stabilized process.

## Exit Criteria

- Release notes reflect only shipped behavior after final refresh.
- Known limitations and future work remain explicit.
- Final text is ready for the GitHub Release body.
