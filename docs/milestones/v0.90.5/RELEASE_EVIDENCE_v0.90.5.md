# Release Evidence - v0.90.5

## Purpose

Assemble the canonical release-proof surfaces for `v0.90.5` in one place.

This file does not replace the milestone docs it points to. It is the compact
index that makes the release package auditable without archaeology.

## Core Milestone Package

- `README.md`
- `WBS_v0.90.5.md`
- `SPRINT_v0.90.5.md`
- `DEMO_MATRIX_v0.90.5.md`
- `FEATURE_DOCS_v0.90.5.md`
- `FEATURE_PROOF_COVERAGE_v0.90.5.md`
- `QUALITY_GATE_v0.90.5.md`
- `RELEASE_READINESS_v0.90.5.md`
- `RELEASE_PLAN_v0.90.5.md`
- `RELEASE_NOTES_v0.90.5.md`
- `END_OF_MILESTONE_REPORT_v0.90.5.md`
- `NEXT_MILESTONE_HANDOFF_v0.90.5.md`

## Review Surfaces

- `review/CLAIM_BOUNDARY_REVIEW.md`
- `ADL_v0.90.5_THIRD_PARTY_REVIEW_HANDOFF.md`
- `.adl/reviews/v0.90.5/ADL_v0.90.5_REVIEW_SUMMARY.md`
- `review/WP24_REMEDIATION_QUEUE.md`

## Implementation / Proof Surfaces

- `review/uts-conformance-report.json`
- `review/model-proposal-benchmark-report.json`
- `review/local-gemma-model-evaluation-report.json`
- `review/dangerous-negative-suite-report.json`
- `features/TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md`
- `features/UTS_PUBLIC_SPEC_AND_CONFORMANCE.md`
- `features/ACC_AUTHORITY_AND_VISIBILITY.md`
- `features/TOOL_REGISTRY_AND_COMPILER.md`
- `features/GOVERNED_EXECUTION_AND_TRACE.md`
- `features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md`
- `features/AGENT_COMMS_v1.md`

## Architecture Surfaces

- `../../adr/0014-contract-market-architecture.md`
- `../../adr/0015-governed-tools-execution-authority-architecture.md`

## Version / Repo Truth

- root `README.md`
- `CHANGELOG.md`
- `REVIEW.md`
- `adl/Cargo.toml`
- `adl/Cargo.lock`

## Ceremony Rule

The release package is only considered complete when these surfaces agree on:

- `v0.90.5` as the completed governed-tools release line
- zero-finding external review result
- explicit no-remediation `WP-24` result
- `v0.91` prepared as the next milestone
