# v0.91.4 Demo Matrix

## Status

Tracked Sprint 4 demo and proof matrix for `v0.91.4` release closeout.

This matrix now distinguishes three classes of surface:

- release-blocking C-SDLC proof needed for Sprint 4 closeout
- bounded showcase/demo surfaces that help reviewers understand the system but do not by themselves close the milestone
- sidecar or bridge surfaces routed to `v0.91.5` or later so they do not distort `v0.91.4` release truth

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Date: `2026-05-31`
- Owner: ADL maintainers
- Related issues / work packages: `#3363` through `#3371`, sidecar evidence
  lanes `#3372` through `#3382`, bridge routing to `v0.91.5`

## Purpose

Define the canonical v0.91.4 demo and proof program for release-tail review:
which proof surfaces are release-blocking, which surfaces are explanatory
showcases, and which sidecar or bridge surfaces are explicitly non-blocking.

## How To Use

- Use release-blocking rows to decide what Sprint 4 must finish before release.
- Use landed core proof rows as supporting evidence for the C-SDLC default path.
- Use showcase rows to choose the best reviewer-facing walkthrough without
  upgrading demos into release gates.
- Use routed/non-blocking rows to prevent sidecars or bridge work from becoming
  hidden v0.91.4 release scope.

## Scope

`v0.91.4` closes on Sprint 4 (`#3362` through `#3371`).

Earlier core C-SDLC work packages remain valid proof inputs for this release.
Remaining bridge work outside Sprint 4, including multi-agent stabilization and sidecar product follow-on work, is not release-blocking here unless explicitly called back into Sprint 4.

## Runtime Preconditions

No single runtime command proves this whole matrix. Reviewer proof is assembled
from the tracked evidence packets, demo/showcase packets, quality-gate records,
and release-tail review artifacts named below.

When a row names a runnable demo or validator, use that row's linked proof
surface for its local preconditions and commands.

## Demo Coverage Summary

For reviewer-facing demo packaging, use `review/demo_showcase/DEMO_SHOWCASE_INDEX_v0.91.5.md` first. That index records the final demo order and separates landed, preserved, parked, active-in-v0.91.5, and next-milestone surfaces.

Current demo-showcase order:

1. Creative Room static C-SDLC showcase.
2. Starharvest browser/gameplay proof.
3. D17 / `#3419` multi-agent workcell proof lane, preserved as sidecar evidence.
4. Celestial Rescue Unity game, active in v0.91.5 and pending Unity editor import/build validation.
5. WildClawBench benchmark sidecar, parked.
6. Unity Observatory, next-milestone work.

## Release-Blocking Proof Surfaces

| ID | Surface | Owning WP / Issue | Status | Reviewer value | Evidence |
| --- | --- | --- | --- | --- | --- |
| D13 | Demo matrix and proof coverage refresh | WP-13 / `#3363` | closed | makes the release proof surface legible and scoped | this matrix, `FEATURE_PROOF_COVERAGE_v0.91.4.md`, and `review/demo_showcase/BEST_AVAILABLE_CSDLC_DEMO_SHOWCASE_v0.91.4.md` |
| D14 | Coverage / quality gate | WP-14 / `#3364` | closed | proves lifecycle, tools, tests, traces, and blocker truth are gated before release | `QUALITY_GATE_v0.91.4.md` |
| D15 | Docs + adoption review pass | WP-15 / `#3365` | closed | proves the default path is documented honestly enough for maintainers and reviewers | `review/docs_adoption/WP15_DOCS_ADOPTION_REVIEW_2026-05-31.md` |
| D16 | Internal review | WP-16 / `#3366` | closed | proves the code/docs/tests/process slice has internal reviewer scrutiny | WP-16 closeout comment plus closed remediation issues `#3542` through `#3546` |
| D17 | External / third-party review | WP-17 / `#3367` | closed | proves the release survives outside scrutiny | external review handoff and returned packet |
| D18 | Review findings remediation | WP-18 / `#3368` | closed | proves findings are fixed, routed, or truthfully deferred | remediation packet and disposition record |
| D19 | Next milestone planning refresh | WP-19 / `#3369` | closed | proves release closeout does not strand the next planning wave | `NEXT_MILESTONE_HANDOFF_v0.91.4.md` refresh |
| D20 | Next milestone review pass | WP-20 / `#3370` | closed | proves the next milestone plan has review before ceremony | `review/next_milestone/V0914_NEXT_MILESTONE_REVIEW_2026-06-01.md` |
| D21 | Release ceremony | WP-21 / `#3371` | closed | proves all release-tail evidence converged in order | `RELEASE_EVIDENCE_v0.91.4.md`, `RELEASE_READINESS_v0.91.4.md`, and `END_OF_MILESTONE_REPORT_v0.91.4.md` |

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
| S03 | Best available default-operation showcase note | WP-13 / `#3363` | closed | explains which existing demo should be shown to reviewers first and why | `review/demo_showcase/BEST_AVAILABLE_CSDLC_DEMO_SHOWCASE_v0.91.4.md` |
| S04 | Demo showcase index and proof map | `#3461` | active_in_v0_91_5 | packages the final reviewer-facing demo order and proof/routing links without adding new demo claims | `review/demo_showcase/DEMO_SHOWCASE_INDEX_v0.91.5.md` |
| S05 | Celestial Rescue Unity game | `#3460` / PR `#3680` | active_in_v0_91_5_pending_unity_import | source-created Unity game scaffold; not a v0.91.4 release gate and not Unity editor/build proven yet | draft PR `#3680` |

## Routed Or Non-Blocking Surfaces

| ID | Surface | Status | Routing truth |
| --- | --- | --- | --- |
| R01 | Multi-agent C-SDLC proving wave and remaining stabilization | routed_to_v0_91_5 | the old `#3415` mini-sprint remains useful background evidence, but the concrete bridge wave now lives under `#3501`, `#3503`, and `#3504`, with `#3484` retained as already-satisfied proof rather than a `v0.91.4` release blocker |
| R02 | CodeFriend sidecar product setup | non_core_sidecar | product publication work is tracked separately and does not count as default-operation C-SDLC proof |
| R03 | WildClawBench benchmark sidecar | parked_sidecar | benchmark spike evidence may inform later work, but it is not required for Sprint 4 release closeout and should not be presented as active WP-13 demo work |
| R04 | Unity-facing best-demo completion | active_in_v0_91_5_pending_unity_import | no `v0.91.4` release claim depends on Unity completion; `#3460` now owns the Celestial Rescue Unity project under `demos/v0.91.4/celestial-rescue-unity/` with proof packet `review/demo_showcase/CELESTIAL_RESCUE_UNITY_PROOF_PACKET_v0.91.5.md`, but Unity editor/import/build validation is still pending |
| R05 | Unity Observatory | next_milestone | Observatory remains separate next-milestone work and must not be collapsed into the Celestial Rescue game |
| R06 | D17 / multi-agent workcell proof lane | preserved_sidecar | issue `#3419` and the multi-agent workcell proof packets remain linked evidence, not a v0.91.4 release gate |

## Coverage Rules

- Release-blocking proof must remain separate from reviewer-facing showcase
  material.
- Sidecar and bridge surfaces must not become release gates unless a later issue
  explicitly promotes them.
- A row may use a tracked proof packet instead of a runnable command when the
  proof surface is a review, quality, release, or handoff artifact.
- Pending rows must stay pending until the named WP produces evidence.

## Demo Details

This milestone uses compact table-backed proof details rather than one expanded
demo block per row because Sprint 4 is a release-tail review sequence. The
tables above are the canonical row-level detail for:

- release-blocking proof surfaces
- landed core C-SDLC proof already available
- reviewer-facing showcase surfaces
- routed or non-blocking sidecar/bridge surfaces

## Current Release Read

The strongest reviewer-facing story available today is:

1. core C-SDLC control-plane proof already landed through Sprints 1 through 3
2. Creative Room is the best front-stage demo for explaining the default-operation claim boundary
3. Starharvest provides the strongest browser-backed interaction proof in the current showcase lane
4. D17 / `#3419` multi-agent proof remains preserved as sidecar evidence rather than being hidden or promoted into the release gate
5. Celestial Rescue is active v0.91.5 Unity-game work pending Unity editor import/build validation
6. WildClawBench is parked sidecar evidence, and Observatory remains next-milestone work
7. Sprint 4 has closed the demo/proof refresh, quality gate, docs/adoption review, internal review, external review, remediation, next-milestone planning, next-milestone review, and WP-21 release ceremony evidence

`v0.91.4` should not claim live multi-agent completion, Unity completion, or sidecar-product success as required release proof.

## Cross-Demo Validation

Cross-demo validation happens through Sprint 4, not through a single demo
command:

- WP-13 refreshes demo/proof coverage.
- WP-14 records quality-gate and validation posture.
- WP-15 records docs/adoption readiness.
- WP-16 has provided internal review.
- WP-17 provided external review.
- WP-18 recorded remediation or truthful deferral.

## Determinism Evidence

Determinism evidence is supplied by the tracked proof packets and validators
named in the table rows, plus release-tail review of whether those packets are
repo-relative, replayable where applicable, and explicit about non-claims.

## Reviewer Sign-Off Surface

Reviewers should sign off against:

- this matrix
- `FEATURE_PROOF_COVERAGE_v0.91.4.md`
- `QUALITY_GATE_v0.91.4.md`
- `review/docs_adoption/WP15_DOCS_ADOPTION_REVIEW_2026-05-31.md`
- WP-16 closeout evidence and the WP-17 external review packet

## Exit Criteria

- Release-blocking rows have landed evidence, truthful blockers, or routed
  follow-on dispositions.
- Showcase rows remain explanatory and do not become hidden release gates.
- Sidecar and bridge rows are explicitly non-blocking unless promoted by a
  later tracked decision.
- The matrix passes the planning-template structure validator.
