# v0.90.5 Test Runtime Reduction Plan

## Purpose

Reduce authoritative Rust coverage wall time in `v0.90.5` by rewriting the
remaining heavyweight proof-style tests so they stop paying the same expensive
setup cost once per test process.

This is an execution-supporting planning document only. It does not create a
new canonical work package, and it does not claim that the rewrite work is
already implemented.

## Placement Rule

This plan belongs in the tracked `v0.90.5` `ideas/` lane because it affects how
we execute and stabilize the milestone, not what Governed Tools v1.0 is. The
governed-tools WBS and proof lane remain authoritative for milestone scope.

Use this plan as support material when:

- CI wall time becomes a real milestone risk again
- heavyweight runtime proof families need bounded follow-on slices
- we need to reduce runtime cost without weakening proof boundaries

Do not treat this document as permission to reorder, replace, or compress away
the main governed-tools work packages.

## Opened Get-Well Slices

WP-01 opened the runtime-reduction work as a separate GW wave:

- GW-00 / #2592: parent scheduling issue and recovery-wave coordination
- GW-01 / #2593: collapse external-counterparty proof-family tests
- GW-02 / #2594: collapse private-state observatory proof-family tests
- GW-03 / #2595: collapse delegation-subcontract proof-family tests
- GW-04 / #2596: collapse contract-market and resource-stewardship proof-family tests
- GW-05 / #2597: shrink CLI and demo proof-matrix tail

These slices are intentionally not part of the canonical WP numbering.

GW-00 tracks execution details, runtime budgets, and per-slice validation in:

- [Get-Well Runtime Tracking](../GET_WELL_TRACKING_v0.90.5.md)

## Source Evidence

This plan is based on the merged `#2547` authoritative coverage run:

- PR: `#2547`
- Run: `24914120505`
- Coverage job: `adl-coverage`
- Nextest summary: `660.944s`
- Tests over `60s`: `1`
- Unique tests over `45s`: `39`
- Deduped cumulative runtime for tests over `45s`: `1938.389s`

Interpretation:

- the long-test population is now concentrated rather than broad and chaotic
- the remaining cost is dominated by repeated heavyweight proof families
- the aggregate runtime of those tests is much larger than wall-clock runtime
  because they run in parallel; they still represent the dominant expensive
  work inside the lane

## Hotspot Families

### Runtime Families

- `runtime_v2::tests::external_counterparty`
  - `12` tests over `45s`
- `runtime_v2::tests::private_state_observatory`
  - `8` tests over `45s`
- `runtime_v2::tests::delegation_subcontract`
  - `7` tests over `45s`
- `runtime_v2::tests::contract_market_demo`
  - `5` tests over `45s`
- `runtime_v2::tests::resource_stewardship_bridge`
  - `4` tests over `45s`

### CLI / Demo Tail

- `adl::bin/adl cli::runtime_v2_cmd::tests::runtime_v2_feature_proof_coverage_runs_runtime_v2_cli_regression_matrix`
  - `86.819s`
- `adl::cli_smoke godel::affect_godel_vertical_slice_demo_emits_changed_strategy_artifact`
  - `56.161s`
- `adl::bin/adl cli::runtime_v2_cmd::tests::runtime_v2_contract_market_demo_validates_stdout_help_and_output_path_rules`
  - `46.400s`

## Diagnosis

The timing pattern is the important clue:

- many tests in the same family cluster tightly around `47s` to `51s`
- those tests differ in assertions, but not much in runtime
- this strongly suggests repeated expensive construction, serialization,
  filesystem materialization, or proof setup across separate test processes

So the main problem is not one bad assertion. The problem is duplicated setup
paid once per test.

## Rewrite Strategy

### 1. Collapse Each Runtime Family Into 1-3 Broader Tests

For each hotspot family, replace many heavyweight sibling tests with a small
set of broader tests:

- one contract / golden / stable-surface test
- one negative-case matrix test
- one write / materialization / review-surface test

This keeps behavioral coverage while cutting repeated setup.

### 2. Build Once, Assert Many Times

Introduce shared family-level builders/helpers so each broader test can reuse a
single in-memory packet or fixture bundle.

Expected pattern:

- construct the packet once
- serialize once where possible
- assert multiple invariants against the same artifact

### 3. Turn Negative Suites Into Table Tests

Where a family currently has many individually named negative-case tests, fold
them into one table-driven test:

- input fixture
- expected failure fragment
- expected classification / boundary

This preserves reviewability while paying setup once.

### 4. Merge Duplicate Surface Assertions

When these currently hit the same constructor or serializer, merge them:

- `*_contract_is_stable`
- `*_matches_golden_fixture`
- `*_preserves_*`
- `*_rejects_*` where they share the same base packet

### 5. Keep Only One Expensive End-To-End Proof Path Per Family

Each family should keep one true end-to-end proof path. The rest of the suite
should validate lower-level artifacts or model behavior directly.

### 6. Shrink CLI Proof Tests

The worst remaining single test is:

- `runtime_v2_feature_proof_coverage_runs_runtime_v2_cli_regression_matrix`

Rewrite target:

- keep one small CLI wiring smoke
- move the majority of proof-matrix assertions to direct helper/library tests
- keep one explicit full proof path only where CLI behavior itself is the thing
  being proven

### 7. Replace Demo Reruns With Artifact Validation Where Possible

If a test is proving an artifact contract rather than CLI transport or demo
runner behavior, validate the artifact directly instead of rerunning the full
demo.

## Execution Order

Recommended sequence:

1. `external_counterparty`
2. `private_state_observatory`
3. `delegation_subcontract`
4. `contract_market_demo`
5. `resource_stewardship_bridge`
6. CLI / demo proof tail

Why this order:

- it removes the largest repeated `~48s` cluster first
- then attacks the single worst outlier

## Refactor Slice Template

Apply the same bounded sequence to each family:

1. characterize current proof claims and invariants
2. add or tighten shared fixture/builder helpers
3. merge overlapping contract/golden assertions
4. collapse negative tests into a matrix
5. collapse write/materialization checks into one test
6. rerun authoritative coverage and compare hotspot output

## Invariants

The rewrite must preserve:

- no artifact schema changes
- no golden fixture drift unless intentionally reviewed
- no loss of denial-path / trust-boundary / authority-boundary assertions
- no loss of deterministic ordering guarantees
- at least one explicit end-to-end proof path per feature family

## Acceptance Criteria

`v0.90.5` can call this successful when:

- the number of tests over `45s` drops materially from the current `39`
- the number of tests over `60s` stays at `1` or lower and ideally reaches `0`
- `runtime_v2_feature_proof_coverage_runs_runtime_v2_cli_regression_matrix`
  is no longer the dominant single hotspot
- authoritative coverage wall time trends down materially from the current
  `660.944s` baseline
- all existing proof claims remain covered, with less duplicated setup work

## Non-Goals

- changing runtime semantics
- weakening proof or review boundaries
- removing all expensive proof paths
- replacing reviewer-facing test names with opaque helper-only coverage

## Risks

- over-collapsing tests could hide which subcase failed unless table output is
  explicit
- shared builders can accidentally mask state drift if reused carelessly
- CLI proof extraction can remove too much end-to-end signal if done without a
  preserved smoke path

## Validation Plan

During implementation, compare before/after on:

- authoritative coverage wall time
- count of tests over `45s`
- count of tests over `60s`
- any new hotspot families introduced by the rewrite

Recommended command source of truth:

- the authoritative `adl-coverage` GitHub job on the branch under review

## Follow-On Packaging

This plan has been split into bounded `v0.90.5` GW issues:

- one issue per major runtime family cluster
- one issue for the CLI proof-matrix rewrite
- one issue for demo/artifact proof demotion where appropriate

The per-slice issue mapping, runtime budget, and validation command table live
in [Get-Well Runtime Tracking](../GET_WELL_TRACKING_v0.90.5.md).
