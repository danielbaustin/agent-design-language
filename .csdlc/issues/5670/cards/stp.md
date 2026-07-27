# Structured Task Prompt

Template: 1.0.0

Issue: 5670

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement, validate, review, publish, shepherd, merge, and close out the hosted coverage shard fan-out issue.

## Deliverables

- Updated `.github/workflows/ci.yaml` hosted coverage shard topology
- Updated `adl/tools/run_authoritative_coverage_lane.sh` shard controls
- Focused contract tests for runner and workflow topology

## Acceptance

1. AC-1: `adl-coverage-hosted` is a small aggregation/gate job, not the long workspace coverage producer.
2. AC-2: Workspace hosted coverage has deterministic parallel shard producers with isolated run ids, profile roots, output roots, logs, and artifact names.
3. AC-3: Runtime/companion coverage remains explicit and only runs when policy requires it.
4. AC-4: Aggregation fails closed on missing, stale, duplicate, or inconsistent shard evidence.
5. AC-5: Coverage thresholds, ownership filtering, path-policy routing, no-report collection, and no-stale-output protections are preserved.
6. AC-6: Focused contracts prove CI topology, shard command shape, artifact names, and failure behavior.

## Dependencies

- GitHub issue #5670
- Issue #5666 proportional fast-lane policy
- Issue #5602 no partition-local report crash fix
- Issue #5610 coverage summary merge correctness
- WP19-01 run-scoped coverage isolation

## Inputs

- .github/workflows/ci.yaml
- adl/tools/run_authoritative_coverage_lane.sh
- adl/tools/test_run_authoritative_coverage_lane.sh
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_path_policy.sh
- docs/milestones/v0.91.7/review/wp20_remediation_4647/WP19_FINDING_REMEDIATION_MATRIX_4647.md

## Non Goals

- AWS or Spot backend changes
- coverage threshold reduction
- test-scope reduction
- runtime/product behavior changes
- C-SDLC lifecycle tool rewrite
