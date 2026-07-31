# Issue #5670 design: hosted coverage shard fan-out

## Intent

Issue #5670 turns the current hosted coverage lane from a long producer into a
small aggregation gate backed by independent hosted shard producers. The goal is
faster PR feedback without weakening the authoritative coverage signal.

## Current Shape

The workflow already separates `adl-coverage-runtime-hosted`,
`adl-coverage-workspace-hosted`, and `adl-coverage-hosted`. The final hosted job
currently verifies producer results and downloads artifacts. Prior fixes made
the coverage runner safer:

- #5602 removed partition-local report rendering from the profile collection path.
- #5610 repaired deterministic summary merge behavior.
- WP19-01 isolated LLVM profile/output roots by declared coverage run id.

The remaining long pole is topology. Workspace coverage still behaves as one
hosted producer rather than multiple independent shard producers with a final
aggregation gate.

## Proposed Change

1. Add deterministic hosted workspace coverage shard producers in CI.
   - Each shard has its own coverage run id, profile root, output root, logs,
     and artifact name.
   - Shards run concurrently when path policy requires hosted workspace
     coverage.

2. Keep `adl-coverage-hosted` as the authoritative aggregation/gate job.
   - It verifies path-policy and producer job results.
   - It downloads every required shard artifact.
   - It fails closed on missing, duplicate, stale, or inconsistent shard
     evidence.
   - It preserves the final required coverage check name.

3. Extend the coverage runner only as much as needed for shard operation.
   - Expose shard count/index in `--print-plan` and execution.
   - Run only the selected shard's partition set.
   - Preserve profile isolation, no-report collection, final report behavior,
     coverage thresholds, and ownership filtering.

4. Add focused contracts.
   - Workflow contract proves fan-out producers, artifact names, and final gate
     dependencies.
   - Runner contract proves shard argument validation, run-id isolation, and
     command shape.

## Boundaries

- No AWS or Spot work.
- No coverage threshold reduction.
- No hidden test-scope reduction.
- No Runtime/product behavior changes.
- No broad rewrite of lifecycle tooling.

## Validation

Focused local validation should use `/Volumes/FastWork` for temporary/build
output and include:

- `bash adl/tools/test_run_authoritative_coverage_lane.sh`
- `bash adl/tools/test_ci_runtime_contracts.sh`
- `bash adl/tools/test_ci_path_policy.sh`
- `git diff --check`

Broader hosted validation is deferred to the PR checks and must stay truthful in
SOR.

