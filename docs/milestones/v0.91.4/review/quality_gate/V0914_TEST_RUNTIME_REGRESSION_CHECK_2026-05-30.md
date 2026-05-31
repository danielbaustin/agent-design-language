# v0.91.4 Test Runtime Regression Check

## Status

`partial`

## Purpose

Record the current runtime-regression and validation-tail posture for `WP-14`
without pretending that the milestone already has a mature cross-milestone
performance-comparison system.

## Reviewed Surfaces

- `docs/milestones/v0.91.4/FIVE_MINUTE_SPRINT_REPEATABILITY_REPORT_2026-05-27.md`
- `docs/milestones/v0.91.4/features/FIVE_MINUTE_SPRINT_REPEATABILITY.md`
- `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md`
- current `WP-14` PR state for `#3527` at `2026-05-30T20:48:53Z`

## Verdict

`v0.91.4` already has meaningful validation-tail evidence, but not a full
"compare slowest test families and total lane times against the previous
milestone" packet.

So the runtime-regression story is partially present:

- enough to show where the long-tail risk lives
- not enough to claim mature comparative regression tracking

## Current Evidence

The tracked repeatability report already shows three important timing classes:

- a fast docs/process lane (`#3423` / `#3427`) with `1m39s` open-to-merge
- a fast infrastructure/product-sidecar lane (`#3375` / `#3420`) with `7m36s`
  open-to-merge
- a core C-SDLC tools lane (`#3358` / `#3440`) with a much longer
  `35m54s` open-to-merge tail due to broad validation/runtime proof

That is not a full regression benchmark, but it is real evidence that
validation-tail cost exists and is already visible in the milestone record.

## Findings

### P2 - no dedicated cross-milestone runtime regression comparison packet exists yet

No separate packet was found that compares:

- slowest test families
- total lane times
- previous milestone vs current milestone runtime posture

This means WP-14 could describe runtime-regression posture, but not yet through
the cleaner "current vs previous" control-tower frame.

Disposition: fixed in part by this packet, but not solved as a general future
workflow.

### P3 - current open Sprint 4 tail is not introducing a fresh runtime-regression signal

At the current snapshot, the only open Sprint 4 PR is `#3527`, and its tracked
delta is limited to the WP-14 quality-gate doc plus the three companion
quality-gate review packets added in this issue.

That means there is no new runtime-facing code change in the active release-tail
PR lane that would create a fresh runtime-regression concern right now.

Disposition: recorded as current posture, not a blocker.

## What Passed

- Validation-tail and proof-latency are already treated as explicit evidence in
  `v0.91.4`, not hidden behind generic success claims.
- The current active release-tail PR posture is docs-only and low-risk.
- No new runtime regression signal was discovered in this manual check.

## Non-Claims

This packet does not claim:

- that `v0.91.4` already has a mature runtime-regression benchmark framework
- that no runtime regressions exist anywhere in the milestone
- that broad runtime or slow-proof lanes have been eliminated
- that comparison against previous milestones is complete

## Recommended Follow-On

- Keep this packet as the current WP-14 runtime-regression note.
- Add a future control-tower artifact that compares slowest test families and
  lane totals across milestones instead of relying only on repeatability prose.
