# Demo And Unity Observatory Readiness v0.91.5

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-06-15`
- Owner: ADL maintainers
- Status: `review_ready_wp_13_closeout`
- Related issues: `#3455`, `#3458`, `#3459`, `#3460`, `#3461`, `#3573`

## Template Rules

This is a planning feature doc and readiness ledger. It records what landed for v0.91.5 demo readiness and what remains future work; it is not a replacement for issue, PR, or proof-packet truth.

## Purpose

Define and close the v0.91.5 demo-readiness work that should feed v0.92 first-birthday planning without overstating Unity Observatory readiness.

## Context

The v0.91.5 demo mini-sprint refreshed or packaged the demo story after several child issues landed:

- `#3458` / PR `#3496`: Starharvest browser-game proof refresh, recorded in [Starharvest browser proof](../../v0.91.4/review/demo_showcase/STARHARVEST_BROWSER_PROOF_v0.91.4.md).
- `#3459` / PR `#3500`: ADL Creative Room demo, recorded in [Creative Room proof packet](../../v0.91.4/review/demo_showcase/CREATIVE_ROOM_PROOF_PACKET_v0.91.4.md).
- `#3460` / PR `#3680`: Celestial Rescue Unity-facing game demo, recorded in [Celestial Rescue Unity proof packet](../../v0.91.4/review/demo_showcase/CELESTIAL_RESCUE_UNITY_PROOF_PACKET_v0.91.5.md).
- `#3461` / PR `#3684`: demo showcase index and proof map, recorded in [demo showcase index](../../v0.91.4/review/demo_showcase/DEMO_SHOWCASE_INDEX_v0.91.5.md).

This gives v0.92 a stronger demo substrate, but it does not prove that the future Unity Observatory product is complete.

## Coverage / Ownership

This feature owns demo showcase readiness and Unity Observatory routing truth for v0.91.5. It does not own the WP-18 first-birthday launch packet (`#3377`) or the provider/model proof sprint.

## Overview

v0.91.5 clarifies which demos are landed proof, which are illustrative rehearsal, and which are future substrate:

- Starharvest and ADL Creative Room are landed demo/proof surfaces from the earlier demo mini-sprint lineage.
- Celestial Rescue is a landed Unity-facing demo artifact and useful Observatory substrate.
- The demo showcase index/proof map packages the story for review.
- Unity Observatory remains future v0.92 work unless a later issue proves it directly.

## Design

- Maintain a demo showcase index and proof map through `#3461`.
- Map demo claims to closed issues and merged PRs.
- Treat Celestial Rescue as Unity-facing demo substrate, not as full Observatory proof.
- Keep illustrative demos separate from runtime proof.
- Keep WildClawBench out of this demo mini-sprint; it is parked and should not be reopened here.

## Execution Flow

1. Refresh Starharvest proof through `#3458` / PR `#3496`. Completed.
2. Build ADL Creative Room through `#3459` / PR `#3500`. Completed.
3. Build Celestial Rescue through `#3460` / PR `#3680`. Completed.
4. Package demo showcase index/proof map through `#3461` / PR `#3684`. Completed.
5. Update v0.91.5 demo matrix/readiness truth through `#3455`. This document update is the closeout alignment step.
6. Feed v0.92 demo planning and first-birthday launch packet through `#3377`, which remains separate WP-18/Sprint 4 work.

## Determinism and Constraints

Demo claims must be tied to source-backed artifacts and GitHub issue/PR evidence, not screenshots or marketing language alone. Where runtime commands are not rerun during closeout, the closeout must say so and rely only on existing merged issue/PR proof.

## Integration Points

- [../DEMO_MATRIX_v0.91.5.md](../DEMO_MATRIX_v0.91.5.md)
- [../V092_ACTIVATION_TEST_MAP_v0.91.5.md](../V092_ACTIVATION_TEST_MAP_v0.91.5.md)
- `#3377` first-birthday launch packet
- `#3573` Sprint 3 demo matrix/showcase umbrella

## Validation

For this readiness document, validation is docs-focused:

- Confirm the referenced child issues are closed or explicitly outside this closeout.
- Confirm the merged PRs exist for the landed demo child issues.
- Run Markdown/path sanity checks for links in this doc and the demo matrix.
- Run `git diff --check`.

Runtime demo replay is not required for this closeout unless a reviewer asks to re-prove a specific child demo.

## Acceptance Criteria

- Demo readiness is visible before v0.92 opens.
- Unity Observatory routing is explicit.
- Demo claims do not overstate proof.
- `#3455` can close after this readiness update and the demo matrix update land.
- `#3573` can close only after it records sprint-level closeout truth, including that `#3377` remains open WP-18/Sprint 4 work.

## Risks

- A polished demo could be mistaken for runtime proof.
- Unity work may exceed bridge scope.
- Closing the demo umbrella while `#3377` remains open could be misread as first-birthday readiness; closeout comments must keep those scopes separate.

## Future Work

v0.92 can build the first-birthday Unity Observatory proof if readiness passes. That future work belongs under `#3377` or follow-on v0.92 issues, not under `#3455`.

## Notes

This feature prepares demo infrastructure and closes the v0.91.5 demo-readiness loop. It does not implement the birthday launch packet.
