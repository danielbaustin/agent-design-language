# Active TBD Document Routing for #4234

## Status

Routing record for issue `#4234`.

This packet records operator dispositions for active-looking `.adl/docs/TBD/`
material found during a read-only scan of the primary checkout. These source
TBD drafts are local ignored planning inputs, not tracked PR evidence in this
issue worktree. This packet therefore captures the durable routing decisions
without modifying the source TBD files, reviewing their full contents, or
claiming that downstream implementation is complete.

## Routing Table

| Source | Disposition | Owner / Target | v0.91.6 urgency | Notes |
| --- | --- | --- | --- | --- |
| `.adl/docs/TBD/LAUNCH_PLAN_JULY_2026.md` | schedule | `v0.91.7` planning / company launch planning | not v0.91.6 | Time-sensitive, but operator directed this into `v0.91.7`. |
| `.adl/docs/TBD/RUSTDOC_GAP_ANALYSIS.md` | leave in place | standing Rust refactoring documentation | no new route | Operator classified this as the standing Rust refactoring doc. Do not duplicate. |
| `.adl/docs/TBD/ADL_GOAL_STATE.md` | schedule | `v0.91.6`-`v0.91.7` runtime/agent-state planning | yes | Needs explicit owner before the runtime/continuity work consumes goal state. |
| `.adl/docs/TBD/ADL_AND_GUILDS.md` | schedule | `v0.91.7` Polis / civilization-model planning | not v0.91.6 | Operator directed this into `v0.91.7`. |
| `.adl/docs/TBD/workflow_tooling/PARALLEL_EXECUTION_LANES_AND_COMPRESSION_MODEL.md` | account in validation sprint | validation/test-tax sprint | yes | Current validation issues cover mechanics, but this source doc is not clearly named as a sprint source input. Gap recorded below. |
| `.adl/docs/TBD/workflow_tooling/planning/SPRINT_CYCLE_TIME_REDUCTION_PLAN.md` | account in validation sprint | validation/test-tax sprint | yes | Current validation issues cover mechanics, but this source doc is not clearly named as a sprint source input. Gap recorded below. |
| `.adl/docs/TBD/tools/VALIDATION_MANAGER_TEST_TAX_RECOVERY_PLAN.md` | account in validation sprint | validation/test-tax sprint | yes | Current validation manager work exists, but this source doc should be explicitly cited by the validation sprint. Gap recorded below. |
| `.adl/docs/TBD/Test_Tax_Prompt_2.md` | retire as scratch | no implementation owner | no | Operator directed retirement as scratch after useful details are captured elsewhere. |
| `.adl/docs/TBD/csm_observatory/UNITY_OBSERVATORY_DEMO.md` | aligned | Observatory sprint `#3974`, children `#4030`-`#4035` | active under WP-09 | Current Observatory feature and SEP surfaces already route Unity/Observatory work. |
| `.adl/docs/TBD/runtime_v2/RUNTIME_V2_MINIMAL_PROTOTYPE.md` | reconcile | new runtime fire-up plan in this packet set | yes | Old milestone wording remains useful, but runtime fire-up needs one current plan. |

## Validation Sprint Accounting Check

Observed current owners:

- `#4214` creates the validation surface manifest and moves metadata such as
  behavior surfaces, proof roles, resource profile, and validation DAG into the
  tracked selector authority.
- `#4215` implements validation-manager profiles.
- `#4216` splits issue and validation hot paths into smaller binaries.

Gap:

The current validation sprint work accounts for the implementation mechanics,
but this routing pass did not find durable sprint-level references to these
three planning sources:

- `.adl/docs/TBD/workflow_tooling/PARALLEL_EXECUTION_LANES_AND_COMPRESSION_MODEL.md`
- `.adl/docs/TBD/workflow_tooling/planning/SPRINT_CYCLE_TIME_REDUCTION_PLAN.md`
- `.adl/docs/TBD/tools/VALIDATION_MANAGER_TEST_TAX_RECOVERY_PLAN.md`

Recommendation:

Add those three source references to the validation/test-tax sprint closeout or
follow-up planning record so the implementation is tied back to the compression,
cycle-time, and test-tax rationale. Do not create duplicate implementation
issues if `#4214`-`#4216` already cover the code path.

## Observatory Alignment Check

Observed current owners:

- WP-09 umbrella `#3974`
- Observatory children `#4030` through `#4035`
- Feature doc `docs/milestones/v0.91.6/features/OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md`
- Sprint execution packet `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_ACTIVE_SPRINT_EXECUTION_PACKETS_2026-06-18.md`

Disposition:

`.adl/docs/TBD/csm_observatory/UNITY_OBSERVATORY_DEMO.md` is aligned with the
current Observatory sprint. No new issue is required from this routing pass.
The important caveat is already present in current docs: do not claim working
Unity Observatory completion until WP-09 child issues have terminal reviewed
closure truth.

## Runtime Fire-Up Follow-Up

Runtime fire-up should consume the current Tokio runtime substrate and route through existing owner `#4185` rather than relying directly on old Runtime v2 milestone wording or creating a duplicate fire-up issue. See:

- `docs/milestones/v0.91.6/RUNTIME_FIRE_UP_PLAN_v0.91.6.md`
- `docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md`
- existing integrated runtime soak owner `#4185`
- `.adl/docs/TBD/runtime_v2/RUNTIME_V2_MINIMAL_PROTOTYPE.md`

## Non-Claims

- This packet does not make the ignored `.adl/docs/TBD/` source drafts
  reviewable as tracked PR evidence; it records routing decisions derived from
  the local planning scan.
- This packet does not implement runtime, scheduler, validation, Observatory,
  launch, or Rustdoc work.
- This packet does not retire or delete any TBD file.
- This packet does not close the validation sprint, Observatory sprint, or
  runtime sprint.
