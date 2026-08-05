# Structured Task Prompt

Template: 1.0.0

Issue: 5801

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver reliable focused and slow test routing, coverage aggregation, and platform parity.

## Deliverables

- Retained current-topology review packet and actionable finding disposition
- Single deterministic change-class to PVF/CI lane map
- Separated focused and slow test routing with unknown and mixed changes failing to stronger proof
- Nonduplicated coverage authority with complete shard provenance and one aggregation gate
- Typed metadata-only source-proof reuse boundary
- Stale-run cancellation and final-state required-check regressions
- Linux, macOS, and Windows-facing path/platform contract evidence

## Acceptance

1. A retained Gemini 3.1 Pro packet names prompt, response, and topology paths, recomputed SHA-256 digests, the exact reviewed revision and topology Git blob, findings, and dispositions
2. One deterministic policy classifies docs/review, lifecycle metadata, tooling, ordinary source, runtime-critical, unknown, and mixed changes into explicit PVF lanes
3. Focused and slow test families are separated and unknown changes fail to the stronger proving lane
4. Coverage runs once per PR class with complete exact-SHA shard provenance and one authoritative aggregation gate
5. Metadata-only reconciliation reuses source proof only under typed unchanged-source lineage while substantive drift retains exact-head validation
6. Superseded runs cancel without converting a cancelled or missing final required state into success
7. Focused Gemini-review, policy, coverage, lifecycle-lineage, Linux/macOS/Windows contract, and exact-head CI proof passes

## Dependencies

- WP-02 issue #5819 repository migration verified
- Destination GitHub Actions, required checks, permissions, caches, and branch protections inventoried
- Current exact-head C-SDLC v2 publication and finish contracts remain authoritative

## Inputs

- .github/workflows/ci.yaml
- adl/tools/ci_path_policy.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/check_coverage_impact.sh
- adl/tools/test_check_coverage_impact.sh
- adl/tools/run_pr_fast_test_lane.sh
- adl/tools/run_pr_fast_coverage_lane.sh
- adl/tools/run_authoritative_coverage_lane.sh
- csdlc-v2/src/finish.rs
- csdlc-v2/tests/gate_finish.rs

## Non Goals

- Weakening source or product exact-head validation
- Changing required-check names or branch protection to obtain a speedup
- Deleting tests or lowering coverage thresholds
- AWS or runner-size experimentation
- Broad workflow rewrite without compatibility and rollback proof
- v0.91.8 release-tail expansion
