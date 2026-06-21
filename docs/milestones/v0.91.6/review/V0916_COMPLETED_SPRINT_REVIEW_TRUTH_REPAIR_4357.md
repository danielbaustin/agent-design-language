# v0.91.6 Completed Sprint Review Truth Repair

Status: `repair_packet`
Owner issue: `#4357`
Date: 2026-06-20

## Summary

The completed-sprint review record is not clean enough to consume as-is.
The immediate failure was treating child issue closure, closeout packets, and
retained evidence matrices as if they were equivalent to a completed
sprint-level review.

This packet repairs the review truth surface without inventing missing
historical reviews. It distinguishes:

- closed sprints with retained review evidence;
- closed sprints with retained closeout evidence but incomplete review truth;
- closed issues that are not consumable as completed sprint work;
- open sprints that must stay out of the completed-sprint review queue.

## Corrected Sprint Dispositions

| Issue | Current live state | Correct disposition | Required next action |
| --- | --- | --- | --- |
| `#4276` Predictable execution fabric sprint | closed | `closed_needs_retained_review` | Run a real retained sprint-review/closeout repair before consuming this sprint as reviewed. |
| `#4324` ADR mini-sprint | closed | `closed_not_consumable_as_completed_sprint` | Do not consume as completed ADR sprint work; re-open/recreate the ADR mini-sprint work before release-tail ADR claims depend on it. |
| `#4325` Runtime AWS signal bridge mini-sprint | closed | `closed_closeout_present_review_incomplete` | Either add a retained sprint-review packet or explicitly approve the closeout packet as the review surface. |
| `#4241` Runtime resilience follow-on sprint | closed | `closed_issue_local_review_present_retained_packet_missing` | Add a retained review packet or update the retained evidence matrix with the issue-local review caveat. |

## Open Sprints Not Eligible For Completed-Sprint Review

The following issues are open and must not be counted as completed-sprint review
targets:

- `#3974` WP-09 Observatory/Unity
- `#3976` through `#3984` closeout-tail work packages
- `#4310` Build throughput improvements mini-sprint
- `#4332` VPP and PVF lane-template mini-sprint
- `#4343` Runtime AWS and local operations mini-sprint

## Root Cause

The sprint control plane allowed closure without one uniform invariant:

```text
umbrella complete
  requires child closure truth
  requires retained review truth
  requires retained closeout truth
  requires current SOR/SRP disposition or an explicit caveat
```

Instead, the milestone accumulated several weaker forms of evidence:

- child issues were closed;
- closeout packets existed;
- local ignored cards had stale scaffolds;
- retained evidence matrices summarized the situation;
- some issue-local SRP files recorded review truth, while others stayed
  `not_run`.

Those surfaces are useful, but they are not interchangeable.

## What This Packet Fixes

- It stops treating `#4276` as reviewed. The sprint is closed, but its retained
  review/closeout record is incomplete: the sprint review file says review was
  not started, and the closeout readiness record says remediation was still
  needed. This is a review-record defect, not an assessment of the sprint work
  itself.
- It stops treating `#4324` as completed ADR sprint work. The issue is closed,
  but the current evidence is not sufficient to consume it as a completed
  mini-sprint.
- It records that `#4325` has useful closeout truth, but review truth is still
  incomplete unless the closeout packet is explicitly accepted as the review
  surface.
- It records that `#4241` has issue-local review truth, but no tracked retained
  review packet was found in the milestone review directory.

## What This Packet Does Not Fix

- It does not re-execute implementation work.
- It does not claim old pre-PR reviews happened when no retained evidence was
  found.
- It does not normalize ignored `.adl` lifecycle cards by hand.
- It does not close or review currently open sprint umbrellas.

## Required Review Queue After Repair

The next closed-sprint review pass should start with:

1. `#4276` as a real sprint-review repair target.
2. `#4325` as a closeout-present/review-incomplete mini-sprint.
3. `#4241` as retained-review-packet backfill or evidence-matrix update.

`#4324` should not be reviewed as a completed sprint. It should be treated as a
closed but non-consumable state until the ADR mini-sprint is actually executed
or explicitly recreated.

## Non-Claims

- This packet does not certify v0.91.6 sprint closeout readiness.
- This packet does not claim all completed sprints are reviewed.
- This packet does not accept stale local `SRP` / `SOR` cards as harmless.
- This packet does not make `#4324` complete.
