# v0.91.5 WP-15 Input From Gap Review

Date: `2026-06-17`
Prepared from: `.adl/docs/TBD/v0.91.5_gap_review.md`
Intended consumer: `#3579` WP-15 docs and review alignment

## Purpose

Turn the gap-review conclusions into one bounded WP-15 input so the next docs
alignment pass can consume the findings directly instead of reconstructing them
from scattered milestone surfaces.

## Source-Grounded Baseline

The gap review's baseline is sound:

- Sprint 1 through Sprint 3 delivery is materially landed
- the first internal-review remediation mini-sprint `#3899` is complete
- Sprint 4 `#3574` is the active frontier
- the remaining problem is mostly release-tail truth maintenance, not missing
  bridge substance

WP-15 should preserve that framing. Do not rewrite the milestone as hollow or
as already release-complete.

## Highest-Priority WP-15 Targets

### 1. Normalize stale release-tail status language

The gap review correctly flags opening-era status drift across key milestone
surfaces.

At minimum, WP-15 should reconcile the remaining stale status language in:

- `docs/milestones/v0.91.5/README.md`
- `docs/milestones/v0.91.5/RELEASE_PLAN_v0.91.5.md`
- `docs/milestones/v0.91.5/RELEASE_NOTES_v0.91.5.md`
- `docs/milestones/v0.91.5/VISION_v0.91.5.md`
- `docs/milestones/v0.91.5/DESIGN_v0.91.5.md`
- `docs/milestones/v0.91.5/DECISIONS_v0.91.5.md`
- `docs/milestones/v0.91.5/features/README.md`
- `docs/milestones/v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md`
- `docs/milestones/v0.91.5/features/V092_ACTIVATION_READINESS_v0.91.5.md`

WP-15 should make these read like a milestone in the Sprint 4
review/remediation/release tail, not a package still in WP-01 opening.

### 2. Absorb the first remediation wave into the release-tail story

The docs should clearly show that:

- first internal review already happened
- the first remediation wave already ran through `#3899`
- Sprint 4 is resuming after that completed tranche

The point is not to overclaim closure. The point is to stop under-describing the
work already completed.

### 3. Keep the open-tail truth explicit

WP-15 should preserve that these issues remain open and meaningful:

- `#3574`
- `#3575`
- `#3579`
- `#3576`
- `#3580`
- `#3577`
- `#3581`
- `#3578`

Open release-tail work is expected. The docs should present it as active
frontier, not hidden failure and not completed fact.

## Mechanical-Audit Gap To Carry Forward

The gap review is also right about the missing closeout-truth audit
replacement:

- `check_milestone_closed_issue_sor_truth.sh` is retired
- no Rust/PVF replacement lane is yet available as a repo-native audit surface

WP-15 should not pretend this control exists today.

Recommended doc posture:

- explicitly note that the old shell helper is retired
- point to WP-14's sampled substitute and blocker language
- leave the replacement audit path as an active control-plane gap until a real
  repo-native lane exists

## What WP-15 Should Not Do

- do not restate the milestone as release-ready
- do not erase the still-open Sprint 4 tail
- do not turn the missing mechanical audit into a hidden assumption
- do not rewrite bridge delivery as uncertain when the landed work is already
  evidenced
- do not rely on chat memory where the sprint, review, and quality-gate docs
  can carry the truth directly

## Recommended WP-15 Acceptance Lens

WP-15 should be considered successful if, after its doc alignment pass:

1. core milestone docs tell one coherent post-remediation Sprint 4 story
2. no touched release-facing doc still presents `v0.91.5` as `active_wp_01_opening`
3. the completed first remediation wave is visible in the release-tail packet
4. the missing mechanical closeout-truth audit is documented as an active gap,
   not a solved control
5. the remaining open Sprint 4 tail is easy to understand without manual
   cross-file reconstruction

## Relationship To WP-14

WP-14 already established:

- the quality gate is truthfully `blocked`
- closeout truth is not fully clean
- stale milestone-doc status claims remain
- second-pass internal review is a later handoff, not part of WP-14 execution

WP-15 should consume those conclusions rather than re-deriving them.

Primary supporting surfaces:

- `docs/milestones/v0.91.5/QUALITY_GATE_v0.91.5.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_SECOND_PASS_INTERNAL_REVIEW_PLAN_2026-06-17.md`
- `.adl/docs/TBD/v0.91.5_gap_review.md`

## Bottom Line

The gap review should be used as a WP-15 alignment brief, not as a fresh
release judgment. Its best contribution is a tight reminder that the remaining
risk is truth maintenance and control-surface coherence, not lack of underlying
delivery.
