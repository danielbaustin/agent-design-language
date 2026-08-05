# Structured Task Prompt

Template: 1.0.0

Issue: 5853

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver a measured and reversible post-migration build acceleration decision for the standard and 16-core GitHub-hosted runner comparison.

## Deliverables

- Migration-eligibility receipt and frozen workload/environment manifest
- Raw five-cold and ten-warm trial records for each platform and selected workload
- Queue, cache, timing, reliability, proof-parity, and cost analysis
- Restricted runner-group access, concurrency, budget, and rollback evidence
- One production-semantics canary and lane-by-lane adopt/reject/defer decision
- Ten-run post-change observation for adopted routes or cleanup confirmation for rejected routes

## Acceptance

1. WP-02 migration and WP-02A CI reliability gates are verified before paid execution
2. Control and candidate runs use one exact commit, workflow, toolchain, lockfiles, commands, permissions, cache design, and proof input set
3. Five cold and ten warm trials per selected workload and platform retain cache classification, queue delay, failures, retries, cancellations, and justified outliers
4. Result, artifact, validation, and required-check semantics are equivalent before the canary
5. The decision table reports p50, p95, critical-path delta, queue delta, cost, reliability, and proof parity against predeclared thresholds
6. No route is adopted without threshold compliance and a successful canary; adopted routes retain fallback and ten-run observation evidence
7. Rejected or deferred routing is removed without product-code or test-expectation changes
8. Failure, security, privacy, portability, recovery, cost, and claim boundaries are tested or explicitly dispositioned
9. One exact-revision pre-PR review has no unresolved actionable findings

## Dependencies

- WP-02
- WP-02A
- Agent Logic organization-owner approval for budget and selected-repository runner access

## Inputs

- .adl/docs/TBD/POST_GITHUB_MIGRATION_BUILD_ACCELERATION_EXPERIMENT_PLAN.md
- .adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md
- .github/workflows/ci.yaml
- docs/tooling/BUILD_PLATFORM_BENCHMARKS.md
- docs/tooling/VALIDATION_PLATFORM_ROUTING.md
- docs/tooling/DEVELOPER_THROUGHPUT_FAST_LANE.md
- docs/tooling/HARDLINKED_RUST_DEPENDENCY_CACHE.md

## Non Goals

- AWS or a self-hosted runner platform
- Organization-wide larger-runner defaults
- Changes to validation breadth, test semantics, required-check names, or branch protection
- 32-core, coverage-topology, custom-image, ARM64, or self-hosted experiments
- Treating runner provisioning, cache existence, or planning prose as acceleration proof
