# Structured Intent Prompt

Template: 1.0.0

Issue: 5330

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make Runtime v3 path changes select a small, independent, reliable CI validation lane.

## Required Outcome

Runtime v3-only diffs run only the v3 lane; mixed diffs retain required legacy validation; unmapped v3 paths fail closed.

## Scope

- .github/workflows/ci.yaml
- adl/tools/select_validation_lanes.py
- adl/tools/run_pr_fast_test_lane.sh
- adl/tools/test_run_pr_fast_test_lane.sh
- docs/architecture/runtime_v3_fast_validation_5330.md
- docs/architecture/runtime_v3_fast_validation_5330.mmd

## Authority

- Path-policy selection is authoritative for lane selection
- Runtime v3 proof commands remain explicit argv and Rust-owned where possible

## Assumptions

- none

## Operator Constraints

- none
