---
issue_card_schema: adl.issue.v1
wp: WP-20
slug: v0-87-1-wp-20-release-ceremony-final-validation-tag-notes-cleanup
title: '[v0.87.1][WP-20] Release ceremony (final validation + tag + notes + cleanup)'
labels:
- track:roadmap
- type:task
- area:release
- version:v0.87.1
status: draft
action: edit
depends_on:
- WP-19
milestone_sprint: Sprint 3
required_outcome_type:
- release
- docs
repo_inputs:
- CHANGELOG.md
- docs/milestones/v0.87.1/README.md
- docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md
- docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md
- docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md
canonical_files:
- CHANGELOG.md
- docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md
- docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md
- docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md
demo_required: false
demo_names: []
issue_graph_notes:
- WP-20 is the final closeout issue for v0.87.1 after remediation and next-milestone handoff are already in place.
- This issue should validate, align, and close the milestone rather than reopening implementation or review scope.
Dependencies: Depends on WP-19 so the milestone release can point to a real next-milestone destination before it closes.
Issue-Graph Notes: Use this issue for final validation, release-note alignment, cleanup, and milestone closeout once the review tail is truly complete.
Demo Expectations: No new standalone demo required. Proof is green closeout validation, aligned release notes, and a milestone that is actually ready to close.
pr_start:
  enabled: false
  slug: v0-87-1-wp-20-release-ceremony-final-validation-tag-notes-cleanup
issue_number: 1498
---

## Summary

Perform the final `v0.87.1` release ceremony: validate the milestone, align release artifacts, clean up closeout surfaces, and leave the milestone genuinely ready to close.

## Goal

Turn the fully reviewed and dispositioned milestone into a clean release-closeout state with aligned notes, checklist, and final validation evidence.

## Required Outcome

- run the final validation and closeout checks for `v0.87.1`
- ensure release notes, changelog, checklist, and milestone docs agree
- leave the milestone in a state that can be tagged or otherwise closed confidently

## Deliverables

- final closeout validation record
- aligned release notes / changelog / checklist surfaces
- milestone closeout state that points to the next milestone rather than dangling loose ends

## Repo Inputs

- `CHANGELOG.md`
- `docs/milestones/v0.87.1/README.md`
- `docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md`
- `docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md`
- `docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md`

## Dependencies

- `WP-19`

## Acceptance Criteria

- final validation runs are complete and recorded
- release notes, changelog, checklist, and release plan are aligned
- milestone closeout state is truthful about what shipped and what moved to the next milestone
- the milestone is ready to tag/close without hidden cleanup debt

## Demo Expectations

- No standalone demo required. Proof is the final validation and aligned release-closeout surfaces.

## Non-goals

- reopening review or remediation scope
- creating speculative new roadmap work inside release closeout
- claiming release readiness without the final validation evidence

## Issue-Graph Notes

- This is the final milestone-closeout issue and should be the last Sprint 3 work package executed.

## Notes

- Prefer truthful closeout over optimistic release language.

## Tooling Notes

- Keep the GitHub issue body and local source prompt aligned.
- The local cards should make the required closeout validations and release surfaces explicit.
