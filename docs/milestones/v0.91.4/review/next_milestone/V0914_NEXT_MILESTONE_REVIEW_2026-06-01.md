# v0.91.4 Next-Milestone Review

## Metadata

- Milestone: `v0.91.4`
- Review lane: `WP-20`
- Issue: `#3370`
- Date: `2026-06-01`
- Reviewer: Codex
- Status: `review_clean_after_fixes`

## Scope

This review covers the WP-19 next-milestone handoff and nearby release-tail
planning surfaces before the v0.91.4 ceremony.

Reviewed surfaces:

- `docs/milestones/v0.91.4/NEXT_MILESTONE_HANDOFF_v0.91.4.md`
- `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md`
- `docs/milestones/v0.91.4/FEATURE_PROOF_COVERAGE_v0.91.4.md`
- `docs/milestones/v0.91.4/DEMO_MATRIX_v0.91.4.md`
- `docs/milestones/v0.91.4/MILESTONE_CHECKLIST_v0.91.4.md`
- `docs/milestones/v0.91.4/RELEASE_PLAN_v0.91.4.md`
- `docs/milestones/v0.91.5/WP_ISSUE_WAVE_v0.91.5.yaml`
- `docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`
- `REVIEW.md`
- current GitHub issue truth for `#3369`, `#3370`, `#3371`, `#3558`, `#3564`,
  and open `version:v0.91.5` bridge issues

This review also notes the external provider-plan review discussed during
WP-20. That plan is a v0.91.5 input and should not block v0.91.4 ceremony
unless its unresolved details are promoted into release-tail claims.

## Findings

### WP20-F001: Release-tail docs still treated WP-19 as pending after merge

- Severity: `P2`
- Status: `fixed_in_wp20`
- Evidence:
  - `NEXT_MILESTONE_HANDOFF_v0.91.4.md` still said `WP-19` was in progress.
  - `QUALITY_GATE_v0.91.4.md` still listed `#3369` as open.
  - `FEATURE_PROOF_COVERAGE_v0.91.4.md` and `DEMO_MATRIX_v0.91.4.md` still
    marked WP-19 as pending.
- Disposition:
  - Updated these surfaces to record WP-19 / `#3369` as closed by PR `#3563`.
  - Release-tail blockers now correctly start at WP-20 and WP-21.

### WP20-F002: Release plan did not reflect completed external review, remediation, or planning

- Severity: `P2`
- Status: `fixed_in_wp20`
- Evidence:
  - `RELEASE_PLAN_v0.91.4.md` still left external review, review remediation,
    next-milestone planning, and next-milestone handoff unchecked.
- Disposition:
  - Marked those completed release-tail items checked.
  - Left final next-milestone review, ceremony, and release publication items
    unchecked.

### WP20-F003: Checklist under-recorded completed sidecar and handoff routing

- Severity: `P3`
- Status: `fixed_in_wp20`
- Evidence:
  - `MILESTONE_CHECKLIST_v0.91.4.md` still treated CodeFriend/WildClaw sidecar
    completion or routing as unchecked.
  - The next-milestone handoff entry still described only a scaffold.
- Disposition:
  - Marked the sidecar completion/routing checks as complete.
  - Updated the handoff checklist wording to show WP-19 refresh, not just
    scaffold presence.

### WP20-F004: Provider role/review-provider plan needs v0.91.5 contract cleanup

- Severity: `P3`
- Status: `routed_to_v0_91_5`
- Evidence:
  - External WP-20 review of the C-SDLC role provider profiles and review
    provider plan found route-schema ambiguity between provider families and
    existing provider-profile references, plus minor YAML/authority wording
    cleanup.
- Disposition:
  - Treat as v0.91.5 input for `#3562`, not a v0.91.4 ceremony blocker.
  - Do not let unresolved provider-plan details become v0.91.4 release claims.

### WP20-F005: Root review guide still described WP-17 as active

- Severity: `P2`
- Status: `fixed_in_wp20`
- Evidence:
  - `REVIEW.md` still told reviewers the current posture was WP-17 external
    review and that WP-17 remained open.
  - GitHub issue truth shows WP-17, WP-18, and WP-19 are closed.
- Disposition:
  - Updated `REVIEW.md` to make WP-20 the current release-tail review posture.
  - Kept the third-party handoff as required review input, not as the active
    controlling stage.
  - This resolves the remaining purpose of `#3558`.

### WP20-F006: Live open-issue truth changed during review-tail cleanup

- Severity: `P3`
- Status: `recorded_in_wp20`
- Evidence:
  - `#3558` remains open even though its remaining `REVIEW.md` purpose is
    satisfied by this WP-20 update.
  - `#3564` is open as a post-review-tail closed-issue `SOR` truth sweep while
    closeout normalization proceeds in another session.
- Disposition:
  - Recorded both issues in the quality-gate open-truth section.
  - Treat `#3558` as closing with or immediately after the WP-20 PR.
  - Treat `#3564` as a closeout-normalization input for WP-21, not a blocker to
    the v0.91.5 next-milestone selection.

## What Passed

- `v0.91.5` remains the correct next milestone.
- The handoff preserves WP-20 review before ceremony.
- Root `REVIEW.md` now points reviewers at the WP-20 release-tail posture
  rather than stale WP-17-open truth.
- v0.91.5 bridge work is routed through open v0.91.5 issues rather than hidden
  v0.91.4 release scope.
- AEE routing now distinguishes closed evidence inputs (`#3526`, `#3534`) from
  the live v0.91.5 readiness route (`#3377`).
- Sidecar work remains separated from C-SDLC core release proof.
- v0.92 activation surfaces are represented in
  `V092_ACTIVATION_TEST_MAP_v0.91.5.md`.
- Open `v0.91.4` issue truth is explicit for WP-21: `#3370`, `#3371`, `#3558`,
  and `#3564` are not silently hidden.

## Ceremony Readiness Decision

`conditional_go_for_wp21`

WP-20 does not approve the release ceremony by itself. It finds the
next-milestone handoff review-clean after the fixes above and allows WP-21 to
consume the handoff for final release-evidence convergence.

WP-21 must still verify:

- final release evidence packet exists
- release plan and milestone checklist are ceremony-ready
- no unresolved P1/P0 findings remain
- remaining v0.91.4 open issue truth is explicit
- `#3558` is closed as satisfied by the WP-20 `REVIEW.md` update or explicitly
  no-op closed after merge
- `#3564` closeout-normalization results are consumed before final closed
  issue/card truth is claimed
- release notes, tag, and publication steps are not pre-claimed

## Non-Claims

This review does not claim:

- v0.91.4 is released
- WP-21 ceremony is complete
- v0.91.5 is execution-ready without WP-01 start
- provider-role/review-provider implementation is complete
- v0.92 first-birthday readiness is proven

## Validation

Focused validation for WP-20 should include:

- Markdown/diff hygiene for touched docs
- planning-template checks for touched planning docs where templates exist
- YAML parse for `WP_ISSUE_WAVE_v0.91.5.yaml`
- focused scan for stale WP-19 pending/open wording
- focused scan for unresolved local path or placeholder residue in touched docs
