# v0.95 Sprint Plan

## Status

Forward-planning sprint outline. The final sprint and WP sequence will be
authored during `v0.95` `WP-01`.

## Sprint Goal

Close the MVP convergence band: dashboard/reporting, evaluator/training,
evaluation platform, distributed integration, walkthrough/catalog,
control-plane hardening, and the final editor boundary.

## Planned Scope

- make the `v0.95` feature package executable as one coherent milestone rather
  than a loose checklist of later ideas
- preserve the dependency chain from `v0.92` / `v0.93` / `v0.94` / `v0.94.1`
  instead of re-arguing earlier architecture
- finish with an explicit MVP boundary and a reviewable ceremony package

## Candidate Execution Shape

| Order | Candidate tranche | Milestone role | Status |
| --- | --- | --- | --- |
| 1 | Dashboard/reporting + evaluator/platform convergence | Make non-user-facing MVP surfaces explicit first. | Planned |
| 2 | Distributed integration + walkthrough/catalog + control-plane hardening | Build the integrated launch-shape story. | Planned |
| 3 | Web editor baseline + Zed decision + final convergence/review tail | Resolve the editor boundary and close the MVP packet. | Planned |

## Cadence Expectations

- keep the milestone on the standard ADL WP rhythm, ending in demo, quality,
  docs/review, and ceremony
- prefer bounded integrated proofs over a large number of disconnected demos
- treat user-facing storytelling and reviewer-facing proof posture as equally
  important in this milestone

## Risks / Dependencies

- Dependency: `v0.94` and `v0.94.1` must already have explicit tracked closure
  packages.
  - Risk: MVP convergence can become a silent overflow bucket for earlier work.
  - Mitigation: keep `v0.95` limited to explicit convergence, integration, and
    decision-boundary surfaces.
- Dependency: the control-plane and editor surfaces must remain workflow-truthful.
  - Risk: polished editor or dashboard work could drift from the validated
    lifecycle.
  - Mitigation: keep all operator/editor features explicitly subordinate to the
    existing control-plane truth model.

## Demo / Review Plan

- Demo artifact: one MVP walkthrough plus bounded supporting demos for
  distributed execution, dashboard/reporting, and editor boundary.
- Review posture: reviewer/customer-ready convergence packet with explicit MVP
  non-goals.
- Sign-off expectation: milestone closes only when user-facing, operator-facing,
  and reviewer-facing surfaces all tell the same story.
