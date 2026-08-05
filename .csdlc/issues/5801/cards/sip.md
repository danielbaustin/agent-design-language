# Structured Intent Prompt

Template: 1.0.0

Issue: 5801

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete WP-02A: CI and coverage reliability.

## Required Outcome

reliable focused and slow test routing, coverage aggregation, and platform parity

## Scope

- .github/workflows/ci.yaml lane graph, concurrency, required checks, and coverage aggregation
- adl/tools/ci_path_policy.sh and path/PVF classification contracts
- adl/tools/run_pr_fast_test_lane.sh, run_pr_fast_coverage_lane.sh, run_authoritative_coverage_lane.sh, and check_coverage_impact.sh
- Focused contract tests under adl/tools/test_ci_path_policy.sh, test_ci_runtime_contracts.sh, test_check_coverage_impact.sh, and related runner tests
- csdlc-v2 exact-head metadata-lineage behavior where required to prevent needless source revalidation
- .csdlc/issues/5801, .csdlc/prepared/issues/5801, and .csdlc/evidence/5801

## Authority

- Issue 5801 owns deterministic CI/PVF lane selection, coverage deduplication, stale-run cancellation, and bounded metadata-lineage reuse
- Source and product exact-head validation remains authoritative
- WP-02 owns repository migration and WP-02B owns runner-size experimentation
- No AWS, branch-protection weakening, check-name churn, or test deletion is authorized

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
