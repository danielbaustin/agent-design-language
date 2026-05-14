# Runtime/Test-Cycle Recovery Report - v0.91.2

## Status

WP-04 proof surface for `#3003`.

This report binds the `v0.91.2` runtime/test-cycle recovery claim to the
landed slow-test recovery work from `#3042`, `#3043`, and `#3044`, with
supporting sibling timing diagnostics from `#3045` / `WP-05A`. It is evidence
for the milestone claim that ADL reduced wasted runtime without weakening
authoritative proof.

## Scope

In scope:

- the captured slow-run diagnosis from `.adl/docs/TBD/test-logs.txt`
- the landed Runtime v2 contract-registry slow-test consolidation in PR `#3048`
- the landed proof-materialization slow-lane split in PR `#3049`
- the landed PR-fast selector tightening in PR `#3050`
- supporting sibling nextest timing diagnostics in PR `#3052`

Out of scope:

- `WP-05` coverage ergonomics closeout
- full milestone quality-gate judgment
- claims that every remaining slow Runtime v2 family is already recovered

## Before State

The supplied `#3032` nextest log captured the baseline problem:

- declared run: `1,882` tests across `28` binaries
- completed timing rows parsed: `1,881`
- slow threshold markers: `77`
- total parsed completed runtime: `7364.644s`

The heaviest repeated cluster was the shared Runtime v2 contract-registry and
accessor proof family:

- top completed test: `181.281s`
- hotspot cluster total: `1860.104s`
- repeated sibling tests at roughly `178s` to `181s`

The next clear cluster was proof materialization:

- hotspot cluster total: `566.277s`
- representative completed test: `75.270s`

Those numbers came from:

- `python3 adl/tools/summarize_nextest_timings.py .adl/docs/TBD/test-logs.txt --top 12 --min-seconds 45`
- [SLOW_TEST_TIMING_DIAGNOSTICS_v0.91.2.md](../SLOW_TEST_TIMING_DIAGNOSTICS_v0.91.2.md)

## Landed Recovery Work

### 1. Contract-Registry Slow-Test Consolidation

PR `#3048` collapsed nine duplicated shared-registry/accessor proof tests into
one explicit authoritative slow-lane proof while preserving smaller local
dependency smokes in the ordinary lane.

Key after-state evidence recorded in the landed output record:

- authoritative consolidated proof:
  `runtime_v2_contract_registry_accessors_cover_shared_runtime_registry`
- focused slow-lane runtime: `10.15s` test time (`10.38s` wall)
- remaining default `contract_registry` smokes: `5.63s`

Primary sources:

- `.adl/v0.91.2/tasks/issue-3042__v0-91-2-wp-04a-runtime-v2-contract-registry-slow-tests/sor.md`
- PR `#3048`

### 2. Proof-Materialization Slow-Lane Split

PR `#3049` moved three duplicated `write_to_root_materializes_fixture` tests
behind `slow-proof-tests` while preserving the explicit-path materialization
checks in the default lane.

Key after-state evidence recorded in the landed output record:

- default explicit-path nextest run: `3 passed`, `3.922s` nextest, `4.52s` wall
- authoritative slow-lane root-materialization run: `3 passed`, `3.956s`
  nextest, `4.26s` wall

Primary sources:

- `.adl/v0.91.2/tasks/issue-3043__v0-91-2-wp-04b-runtime-v2-proof-materialization-slow-tests/sor.md`
- PR `#3049`

### 3. PR-Fast Selector Tightening

PR `#3050` tightened `adl/tools/run_pr_fast_test_lane.sh` so bounded Runtime v2
test-file changes no longer fall through to the broad `test(runtime_v2)` family
lane, and bounded `uts_acc_compiler` changes no longer trigger the full default
nextest sweep.

Live selector proof on the current tree:

```text
mode=focused
reason=bounded_rust_surface_runs_focused_nextest
rust_surface_count=3
filter_tokens=theory_of_mind_foundation,intelligence_metric_architecture,governed_learning_substrate
filter_expression=test(theory_of_mind_foundation) or test(intelligence_metric_architecture) or test(governed_learning_substrate)
```

Primary sources:

- `.adl/v0.91.2/tasks/issue-3044__v0-91-2-wp-04c-tighten-pr-fast-test-lane-selection/sor.md`
- PR `#3050`

### 4. Supporting Sibling Evidence: Timing Diagnostics

PR `#3052` added a bounded nextest timing parser and runbook so the team can
classify slow runs without manually scanning thousands of lines of log output.

This is not counted here as WP-04 implementation. It is sibling evidence from
`WP-05A` that strengthens the reviewability of the WP-04 before-state and any
future slow-run follow-up.

Primary sources:

- [SLOW_TEST_TIMING_DIAGNOSTICS_v0.91.2.md](../SLOW_TEST_TIMING_DIAGNOSTICS_v0.91.2.md)
- `adl/tools/summarize_nextest_timings.py`
- PR `#3052`

## Proof Preservation

This recovery work does not claim that expensive proof vanished. It claims that
the expensive proof moved to the correct lane.

Preserved boundaries:

- one explicit authoritative shared-registry proof path still exists
- one explicit authoritative root-materialization path still exists
- default-lane materialization and local dependency smokes still remain
- authoritative coverage and slow-proof lanes were not waived or deleted
- broad or ambiguous Rust changes still fail closed

## Current Validation

The following focused validation was rerun for this WP-04 packet:

- `python3 adl/tools/summarize_nextest_timings.py .adl/docs/TBD/test-logs.txt --top 6 --min-seconds 45`
  Verified the captured `#3032` log still diagnoses the pre-recovery hotspot
  clusters truthfully.
- `bash adl/tools/test_run_pr_fast_test_lane.sh`
  Verified the fast-lane selector contract remains green after the landed
  `#3050` tightening.
- `bash adl/tools/run_pr_fast_test_lane.sh --changed-files docs/milestones/v0.91.2/review/runtime_test_cycle_recovery_changed_files.txt --print-plan`
  Verified a representative bounded Runtime v2 test-file change now selects a
  focused three-token lane instead of the broad Runtime v2 family lane.
- `rg -n "runtime_v2_contract_registry_accessors_cover_shared_runtime_registry|cfg\\(feature = \"slow-proof-tests\"\\)|write_to_root_materializes_fixture" adl/src/runtime_v2/tests/contract_registry_accessors.rs adl/src/runtime_v2/tests/theory_of_mind_foundation.rs adl/src/runtime_v2/tests/intelligence_metric_architecture.rs adl/src/runtime_v2/tests/governed_learning_substrate.rs`
  Verified the consolidated shared-registry proof and the slow-lane
  materialization gates are present in the current source tree.

## Result

WP-04 is satisfied at the milestone-proof level.

The repo now has:

- a concrete before-state diagnosis
- landed code and test-lane reductions
- preserved authoritative proof routes
- a reproducible selector proof surface
- a reusable timing-diagnostics tool for future slow-run incidents

## Non-Claims

- This report does not claim that `WP-05` is complete.
- This report does not claim that all Runtime v2 slow families are fixed.
- This report does not replace the milestone quality gate or release evidence.
