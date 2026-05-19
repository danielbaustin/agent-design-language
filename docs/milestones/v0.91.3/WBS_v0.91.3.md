# v0.91.3 Work Breakdown Structure

## Status

Planned WBS. Work package issue numbers are `pending` until seeded through the
normal v0.91.3 issue wave.

## WBS Summary

v0.91.3 delivers the first working Cognitive SDLC vertical slice. The milestone
should prove one governance-complete Cognitive State Transition, not a broad
platform rewrite.

## Work Areas

| Area | Work Area | Description | Primary Deliverable | Key Dependencies |
| --- | --- | --- | --- | --- |
| A | Milestone design pass | Promote the v0.91.3 plan into issue cards and sprint umbrellas. | issue wave, cards, and readiness record | v0.91.2 closeout |
| B | Transition schema | Define the initial Cognitive State Transition manifest and state model. | schema doc, fixtures, validator plan | C-SDLC TBD docs |
| C | Artifact lifecycle | Preserve corrected `SIP -> STP -> SPP -> SRP -> SOR` semantics in the slice. | templates, validator expectations, docs | v0.91.2 card migration |
| D | Transition DAG | Represent serial steps, shards, barriers, review, and merge gates. | DAG fixture and proof artifact | B, C |
| E | Shard coordination | Define bounded shard ownership and interface-freeze rules. | shard plan and conflict rules | D |
| F | Evidence bundle | Define transition evidence collection, review packet, and synthesis surface. | evidence bundle schema and demo output | C, D, E |
| G | Review and merge gate | Preserve GitHub issue/PR/CI/human review while adding C-SDLC semantics. | merge-readiness gate record | F |
| H | ObsMem handoff | Route SRP review results and SOR outcome truth into memory handoff shape. | memory handoff contract | F, G |
| I | First proof demo | Run a bounded five-minute-sprint first proof surface. | demo report and metrics snapshot | B-H |
| J | Review, remediation, and handoff | Review the slice and hand v0.91.4 the hardening work. | review records and v0.91.4 handoff | all prior work |

## Candidate WP Sequence

| WP | Title | Queue | Primary Deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Design pass and issue-wave readiness | docs | v0.91.3 issue wave, sprint umbrellas, tracked C-SDLC source package, validated cards | v0.91.2 closeout |
| WP-02 | Cognitive Transition schema | docs/tools | manifest schema, states, fixtures, validation plan | WP-01 |
| WP-03 | Card lifecycle integration | tools | lifecycle validator/doctor expectations for the slice | WP-02; v0.91.2 card migration |
| WP-04 | Transition DAG and shard plan | tools/docs | DAG fixture, shard boundaries, barrier model | WP-02, WP-03 |
| WP-05 | Evidence bundle and review synthesis | tools/docs | evidence bundle schema and review-packet surface | WP-04 |
| WP-06 | Governed merge-readiness gate | tools | merge gate record preserving issue/PR/CI/review truth | WP-05 |
| WP-07 | SRP/SOR ObsMem handoff | docs/tools | memory handoff contract for review and outcome truth | WP-05, WP-06 |
| WP-08 | GWS/C-SDLC expansion lessons | docs/tools | combined-lane and closeout-truth lessons applied to C-SDLC proof criteria | WP-02 through WP-07 |
| WP-09 | Five-minute-sprint first proof demo | demo | bounded transition demo and metrics snapshot | WP-08 |
| WP-10 | Internal review | review | code/docs/test review packet | WP-09 |
| WP-11 | Review findings remediation | review | fixes and follow-on routing | WP-10 |
| WP-12 | v0.91.4 completion planning | docs | concrete v0.91.4 handoff and hardening backlog | WP-11 |
| WP-13 | Release ceremony | release | evidence package and closeout record | WP-12 |

## Sequencing Notes

The milestone should stay vertical. If a proposed work package does not help
prove one Cognitive State Transition, route it to v0.91.4 or later.

The GWS mini-sprint exposed two important C-SDLC lessons:

- combined-lane validation must matter, because isolated issue checks can miss
  integration hazards
- sprint umbrella closeout truth must be treated as part of the product, not an
  optional bookkeeping layer
