# v0.91.4 Demo Showcase Index packaged in v0.91.5

## Status

`packaged_for_review_from_v0_91_5_issue_3461`

## Purpose

This index is the compact reviewer-facing map for the v0.91.4 demo mini-sprint surfaces that remained active or needed final packaging after the v0.91.4 release-tail review.

It does not create new demo claims. It records the best current order, links each surface to proof or routing evidence, and keeps deferred work out of the v0.91.4 release proof boundary.

## Recommended Demo Order

| Order | Demo / proof surface | Status | What to show | Evidence / artifact |
| --- | --- | --- | --- | --- |
| 1 | Creative Room static C-SDLC showcase | landed | front-stage explanation of roles, transitions, evidence, and non-claims | `demos/v0.91.4/adl_creative_room_demo.html`; `CREATIVE_ROOM_PROOF_PACKET_v0.91.4.md` |
| 2 | Starharvest browser/gameplay proof | landed | browser-backed interaction proof and browser-runbook route | `STARHARVEST_BROWSER_PROOF_v0.91.4.md`; `STARHARVEST_REFRESH_NOTE_v0.91.4.md`; `BROWSER_PROOF_RUNBOOK_v0.91.4.md` |
| 3 | D17 / multi-agent workcell proof lane | preserved_sidecar | bounded multi-agent C-SDLC proof context, not a default-operation release gate | issue `#3419`; `../multi_agent_workcell/MULTI_AGENT_CSDLC_WORKCELL_PROOF_PACKET_2026-05-28.md`; `../multi_agent_workcell/CODEX_ONLY_COMPLETE_ISSUE_WORKCELL_PROOF_PACKET_2026-05-29.md` |
| 4 | Celestial Rescue Unity game | active_in_v0_91_5_pending_unity_import | source-created Unity game scaffold; do not claim editor/build validation yet | issue `#3460`; draft PR `#3680` |
| 5 | WildClawBench sidecar | parked | benchmark-spike lessons and caveats only; do not present as an active WP-13 demo | `../../WILDCLAW_RESULTS.md`; `../../WILDCLAW_SPIKE_CLOSEOUT_v0.91.4.md` |
| 6 | Unity Observatory | next_milestone | future observatory direction, separate from Celestial Rescue | `../../NEXT_MILESTONE_HANDOFF_v0.91.4.md` |

## Claim Boundary

- Creative Room is the first demo to show because it is clear, static, public, and honest about what it proves.
- Starharvest is the interaction-oriented companion proof because it has browser-backed evidence and a browser runbook.
- D17 / `#3419` remains preserved as multi-agent workcell evidence, but it is not converted into a v0.91.4 release gate.
- Celestial Rescue is v0.91.5 continuation work for the deferred Unity game. It is not editor/build-proven until Unity import validation is recorded.
- WildClawBench remains parked sidecar evidence and must not be presented as a benchmark-win or active WP-13 demo.
- The Unity Observatory remains next-milestone work and is not the same as the Celestial Rescue game.

## Non-Claims

This index does not claim:

- Unity editor import succeeded.
- Unity build validation succeeded.
- WildClawBench validates ADL superiority.
- live provider-backed multi-agent execution is complete.
- Observatory is implemented in v0.91.4.

## Reviewer Use

Use this file with:

- `docs/milestones/v0.91.4/DEMO_MATRIX_v0.91.4.md`
- `docs/milestones/v0.91.4/FEATURE_PROOF_COVERAGE_v0.91.4.md`
- `docs/milestones/v0.91.4/review/demo_showcase/BEST_AVAILABLE_CSDLC_DEMO_SHOWCASE_v0.91.4.md`

Together, those files provide the demo order, the proof map, and the claim boundaries.
