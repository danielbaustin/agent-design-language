# Release Evidence - v0.91

## Purpose

Assemble the canonical release-proof surfaces for `v0.91` in one place.

This file does not replace the milestone docs it points to. It is the compact
index that makes the release package auditable before the final ceremony.

## Status

Ceremony-ready evidence index. The release ceremony, tag, and release
publication are not complete until `WP-25` records the final script result and
publication disposition.

## Core Milestone Package

- `README.md`
- `WBS_v0.91.md`
- `SPRINT_v0.91.md`
- `DEMO_MATRIX_v0.91.md`
- `FEATURE_PROOF_COVERAGE_v0.91.md`
- `QUALITY_GATE_v0.91.md`
- `RELEASE_READINESS_v0.91.md`
- `RELEASE_PLAN_v0.91.md`
- `RELEASE_NOTES_v0.91.md`
- `END_OF_MILESTONE_REPORT_v0.91.md`
- `NEXT_MILESTONE_HANDOFF_v0.91.md`
- `MILESTONE_CHECKLIST_v0.91.md`
- `WP_EXECUTION_READINESS_v0.91.md`
- `WP_ISSUE_WAVE_v0.91.yaml`

## Review Surfaces

- `ADL_v0.91_THIRD_PARTY_REVIEW_HANDOFF.md`
- `.adl/docs/reviews/v0.91/ADL_v0.91_3RD_PARTY_REVIEW_SUMMARY.md`
- `.adl/docs/reviews/v0.91/ADL_v0.91_Comprehensive_Review.pdf`
- `.adl/docs/reviews/v0.91/internal/WP23_REMEDIATION_QUEUE.md`

The external review summary reports `A+` / `100/100` and zero `P0`, `P1`,
`P2`, or `P3` findings.

## Implementation / Proof Surfaces

- `features/MORAL_EVENT_CONTRACT.md`
- `features/MORAL_TRACE_SCHEMA.md`
- `features/OUTCOME_LINKAGE_AND_ATTRIBUTION.md`
- `features/MORAL_METRICS.md`
- `features/MORAL_TRAJECTORY_REVIEW.md`
- `features/ANTI_HARM_TRAJECTORY_CONSTRAINTS.md`
- `features/WELLBEING_AND_HAPPINESS.md`
- `features/KINDNESS.md`
- `features/HUMOR_AND_ABSURDITY.md`
- `features/AFFECT_REASONING_CONTROL.md`
- `features/CULTIVATING_INTELLIGENCE.md`
- `features/MORAL_RESOURCES.md`
- `features/STRUCTURED_PLANNING_AND_PLAN_REVIEW.md`
- `features/STRUCTURED_REVIEW_POLICY_AND_SRP.md`
- `features/A2A_EXTERNAL_AGENT_ADAPTER.md`
- `demos/v0.91/cognitive_being_flagship_demo.md`
- `demos/v0.91/chatgpt_gemini_claude_triad_conversation_demo.md`
- `adl/src/runtime_v2/cognitive_being_flagship_demo.rs`
- `adl/src/runtime_v2/tests/cognitive_being_flagship_demo.rs`
- `adl/src/agent_comms/orchestrate/proof_demo.inc`

## Architecture Surfaces

- `../../adr/0016-moral-evidence-and-cognitive-being-substrate.md`
- `../../adr/0017-secure-local-agent-comms-and-a2a-boundary.md`
- `../../adr/0018-structured-planning-and-review-policy-artifacts.md`

## Version / Repo Truth

- root `README.md`
- `CHANGELOG.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `adl/Cargo.toml`
- `adl/Cargo.lock`

## Ceremony Rule

The release package is only considered complete when these surfaces agree on:

- `v0.91` as the completed moral-governance and cognitive-being release line
- crate version `0.91.0` for the `v0.91` release line
- internal review, third-party review, and remediation closure
- `v0.91.1` and `v0.91.2` prepared as the next implementation packages
- final release ceremony script result, tag disposition, and release
  publication status
