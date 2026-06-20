# v0.91.6 Validation Manager And Test-Tax Mini-Sprint Review

Status: `retained_review_packet`
Date: 2026-06-20
Sprint umbrella: `#4212`
Retained-review cleanup issue: `#4292`

This review records retained post-closeout truth for the validation manager and
test-tax recovery mini-sprint. It does not re-execute the mini-sprint and does
not widen the validation-manager scope beyond the closed child wave.

## Findings

No P1/P2 findings remain in the retained review surface.

### P3: `#4212` is labeled as a task even though it functioned as a mini-sprint

Live GitHub state shows `#4212` is closed and labeled `type:task`, while the
title, body, and eight-child execution wave make it the mini-sprint umbrella for
validation-manager/test-tax recovery. This does not invalidate the completed
work, but future sprint scans that rely only on `type:mini-sprint` labels will
miss it unless they also consume this retained review packet.

Resolution in `#4292`: retained review and matrix coverage explicitly classify
`#4212` as a mini-sprint-by-role.

### P3: `#4213` closure evidence is indirect

Live GitHub state shows `#4213` is closed, but PR `#4227`, the visible PR whose
closing references include `#4213`, is closed unmerged. The validation inventory
surface is nevertheless present on `main` through later merged work, especially
PR `#4221`, which contains the inventory scripts and finish-path validation
tests.

Resolution in `#4292`: this packet records the indirect closure path so future
reviewers do not mistake PR `#4227` being closed unmerged for missing inventory
implementation.

### P3: The retained closeout view did not cite the planning rationale sources

`docs/milestones/v0.91.6/review/planning/TBD_ACTIVE_DOC_ROUTING_4234.md`
previously recorded that the validation/test-tax sprint should explicitly cite
the following planning inputs:

- `.adl/docs/TBD/workflow_tooling/PARALLEL_EXECUTION_LANES_AND_COMPRESSION_MODEL.md`
- `.adl/docs/TBD/workflow_tooling/planning/SPRINT_CYCLE_TIME_REDUCTION_PLAN.md`
- `.adl/docs/TBD/tools/VALIDATION_MANAGER_TEST_TAX_RECOVERY_PLAN.md`

Resolution in `#4292`: this retained review packet cites those sources as
planning rationale consumed by the `#4212` review surface. No duplicate
implementation issue is needed because the child wave already covers the
mechanics.

## Child Issue Closure Truth

| Issue | Role | Observed state after review |
| --- | --- | --- |
| `#4212` | Validation-manager/test-tax mini-sprint umbrella | closed at 2026-06-20T02:59:03Z |
| `#4213` | Test inventory and attribution report | closed at 2026-06-20T00:30:36Z; implementation evidence is indirect because PR `#4227` closed unmerged and later merged work carries the current inventory surface |
| `#4214` | Validation surface manifest | closed at 2026-06-19T19:02:37Z by merged PR `#4230` |
| `#4215` | Validation manager profiles | closed at 2026-06-19T19:06:50Z by merged PR `#4224` |
| `#4216` | Issue and validation hot-path small binaries | closed at 2026-06-20T02:55:25Z by merged PR `#4221` |
| `#4217` | CI and `pr finish` integration | closed at 2026-06-19T20:18:07Z by merged PR `#4240` |
| `#4218` | Ordinary Rust target reduction | closed at 2026-06-19T19:58:28Z by merged PR `#4249` |
| `#4219` | Slow-proof validation family split | closed at 2026-06-20T00:30:35Z by merged PR `#4258` |
| `#4220` | Validation growth guardrails | closed at 2026-06-20T01:52:39Z by merged PR `#4260` |

## Scope Check

The reviewed mini-sprint covers:

- validation inventory and attribution;
- validation lane manifest/profile selection;
- validation manager profile output for changed paths;
- CI path-policy and `pr finish` profile consumption;
- selected Rust target reduction;
- slow-proof family split;
- validation growth guardrails;
- hot-path small-binary decomposition for issue and validation workflows.

It does not include the future validation-manager agent, FastContext model
experiments, or broader v0.91.7 validation intelligence work.

## Retained Evidence

Primary tracked evidence surfaces:

- `adl/tools/validation_manager.py`
- `adl/tools/test_validation_manager.sh`
- `adl/tools/validation_inventory.py`
- `adl/tools/validation_inventory.sh`
- `adl/tools/test_validation_inventory.sh`
- `adl/config/validation_lane_selector.v0.91.6.json`
- `adl/config/slow_proof_families.v0.91.6.json`
- `adl/tools/ci_path_policy.sh`
- `adl/tools/test_ci_path_policy.sh`
- `adl/tools/test_pr_small_binary_delegation.sh`
- `docs/architecture/VALIDATION_LANE_SELECTOR.md`
- `docs/milestones/v0.91.6/review/PVF_LONG_VALIDATION_LANE_INDEX_4223.md`
- `docs/milestones/v0.91.6/review/planning/TBD_ACTIVE_DOC_ROUTING_4234.md`

Planning rationale consumed by this review:

- `.adl/docs/TBD/workflow_tooling/PARALLEL_EXECUTION_LANES_AND_COMPRESSION_MODEL.md`
- `.adl/docs/TBD/workflow_tooling/planning/SPRINT_CYCLE_TIME_REDUCTION_PLAN.md`
- `.adl/docs/TBD/tools/VALIDATION_MANAGER_TEST_TAX_RECOVERY_PLAN.md`

## Validation Evidence

Focused validation rerun during `#4292`:

```text
bash adl/tools/test_validation_manager.sh
bash adl/tools/test_validation_inventory.sh
bash adl/tools/test_ci_path_policy.sh
bash adl/tools/test_pr_small_binary_delegation.sh
```

Result: all four focused checks passed.

The docs-only profile for the retained matrix path selected `git diff --check`
and did not select Rust, slow-proof, coverage, or release-gate proof. This is
consistent with the validation-manager contract that review-document cleanup
should not pay the full expanded test surface.

## Closeout Position

`#4212` is closed and the validation-manager/test-tax implementation surfaces
are present on `main`. The retained review posture is acceptable with the three
P3 caveats above recorded here. No implementation reopening is recommended from
this review.

## Non-Claims

- This review does not claim the future validation-manager agent exists.
- This review does not claim every validation profile is semantically optimal.
- This review does not claim slow-proof, provider-live, or coverage lanes ran
  during this retained-review cleanup.
- This review does not reinterpret PR `#4227` as merged; it records the
  indirect `#4213` closure path explicitly.
