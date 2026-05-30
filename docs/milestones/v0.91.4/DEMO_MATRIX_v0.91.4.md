# v0.91.4 Demo Matrix

## Status

Tracked Sprint 4 demo and proof matrix for `v0.91.4` release closeout.

This matrix now distinguishes three classes of surface:

- release-blocking C-SDLC proof needed for Sprint 4 closeout
- bounded showcase/demo surfaces that help reviewers understand the system but do not by themselves close the milestone
- sidecar or bridge surfaces routed to `v0.91.5` or later so they do not distort `v0.91.4` release truth

## Scope Boundary

`v0.91.4` closes on Sprint 4 (`#3362` through `#3371`).

Earlier core C-SDLC work packages remain valid proof inputs for this release.
Remaining bridge work outside Sprint 4, including multi-agent stabilization and sidecar product follow-on work, is not release-blocking here unless explicitly called back into Sprint 4.

## Release-Blocking Proof Surfaces

| ID | Surface | Owning WP / Issue | Status | Reviewer value | Evidence |
| --- | --- | --- | --- | --- | --- |
| D13 | Demo matrix and proof coverage refresh | WP-13 / `#3363` | in_progress | makes the release proof surface legible and scoped | this matrix, `FEATURE_PROOF_COVERAGE_v0.91.4.md`, and `review/demo_showcase/BEST_AVAILABLE_CSDLC_DEMO_SHOWCASE_v0.91.4.md` |
| D14 | Coverage / quality gate | WP-14 / `#3364` | pending | proves lifecycle, tools, tests, traces, and blocker truth are gated before release | `QUALITY_GATE_v0.91.4.md` |
| D15 | Docs + adoption review pass | WP-15 / `#3365` | pending | proves the default path is documented honestly enough for maintainers and reviewers | docs-review packet to be produced by WP-15 |
| D16 | Internal review | WP-16 / `#3366` | pending | proves the code/docs/tests/process slice has internal reviewer scrutiny | internal review packet to be produced by WP-16 |
| D17 | External / third-party review | WP-17 / `#3367` | pending | proves the release survives outside scrutiny | external review handoff and returned packet |
| D18 | Review findings remediation | WP-18 / `#3368` | pending | proves findings are fixed, routed, or truthfully deferred | remediation packet and disposition record |
| D19 | Next milestone planning refresh | WP-19 / `#3369` | pending | proves release closeout does not strand the next planning wave | `NEXT_MILESTONE_HANDOFF_v0.91.4.md` refresh |
| D20 | Next milestone review pass | WP-20 / `#3370` | pending | proves the next milestone plan has review before ceremony | next-milestone review findings packet |
| D21 | Release ceremony | WP-21 / `#3371` | pending | proves all release-tail evidence converges in order | release evidence packet and ceremony closeout |

## Core C-SDLC Proof Already Available

| ID | Surface | Source issue(s) | Status | Why it matters now | Evidence |
| --- | --- | --- | --- | --- | --- |
| D01 | Software Development Polis and shard ownership proof | `#3353` | landed | establishes actor standing and bounded coordination claims for the default path | `review/software_development_polis/SOFTWARE_DEVELOPMENT_POLIS_PROOF_PACKET_v0.91.4.md` |
| D02 | Merge-readiness proof | `#3355` | landed | shows PR gating and GitHub truth preservation are no longer informal | `review/merge_readiness/MERGE_READINESS_GATE_PACKET_v0.91.4.md` |
| D03 | ObsMem transition memory proof | `#3356` | landed | shows tracked handoff truth can survive transition memory packaging | `review/obsmem_transition_memory/OBSMEM_TRANSITION_MEMORY_PACKET_v0.91.4.md` |
| D04 | Five-minute sprint repeatability proof | `#3359` | landed | shows the default path has measured repeatability and visible coordination cost | `FIVE_MINUTE_SPRINT_REPEATABILITY_REPORT_2026-05-27.md` |
| D05 | Process drift regression proof | `#3361` | landed | shows the default path fails closed on known workflow drift modes | `PROCESS_DRIFT_REGRESSION_REPORT_2026-05-28.md` |

## Best Available Reviewer-Facing Showcase Surfaces

| ID | Surface | Source issue(s) | Status | Role in release story | Evidence |
| --- | --- | --- | --- | --- | --- |
| S01 | Creative Room static C-SDLC showcase | `#3459` | landed | strongest available front-stage explanation of the C-SDLC operating model without overclaiming live provider-backed orchestration | `review/demo_showcase/CREATIVE_ROOM_PROOF_PACKET_v0.91.4.md` |
| S02 | Starharvest browser proof | `#3458`, `#3497` | landed | strongest available browser-backed interaction proof for the demo showcase lane | `review/demo_showcase/STARHARVEST_BROWSER_PROOF_v0.91.4.md` and `review/demo_showcase/BROWSER_PROOF_RUNBOOK_v0.91.4.md` |
| S03 | Best available default-operation showcase note | WP-13 / `#3363` | in_progress | explains which existing demo should be shown to reviewers first and why | `review/demo_showcase/BEST_AVAILABLE_CSDLC_DEMO_SHOWCASE_v0.91.4.md` |

## Routed Or Non-Blocking Surfaces

| ID | Surface | Status | Routing truth |
| --- | --- | --- | --- |
| R01 | Multi-agent C-SDLC proving wave and remaining stabilization | routed_to_v0_91_5 | the old `#3415` mini-sprint remains useful background evidence, but the concrete bridge wave now lives under `#3501`, `#3503`, and `#3504`, with `#3484` retained as already-satisfied proof rather than a `v0.91.4` release blocker |
| R02 | CodeFriend sidecar product setup | non_core_sidecar | product publication work is tracked separately and does not count as default-operation C-SDLC proof |
| R03 | WildClawBench benchmark sidecar | non_core_sidecar | benchmark spike evidence may inform later work, but it is not required for Sprint 4 release closeout |
| R04 | Unity-facing best-demo completion | deferred_to_v0_91_5 | no `v0.91.4` release claim depends on Unity completion; the current truthful release posture is best-available showcase plus explicit non-claim |

## Current Release Read

The strongest reviewer-facing story available today is:

1. core C-SDLC control-plane proof already landed through Sprints 1 through 3
2. Creative Room is the best front-stage demo for explaining the default-operation claim boundary
3. Starharvest provides the strongest browser-backed interaction proof in the current showcase lane
4. Sprint 4 now needs to finish the closeout tail: quality gate, docs/adoption review, internal review, external review, remediation, next-milestone planning, and ceremony

`v0.91.4` should not claim live multi-agent completion, Unity completion, or sidecar-product success as required release proof.
