# Structured Task Prompt

Template: 1.0.0

Issue: 5330

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add and prove the Runtime v3 fast path-policy profile without redesigning the entire CI topology.

## Deliverables

- Runtime v3 fast selector/profile
- v3-only, mixed, and unmapped fixtures
- operator documentation and design diagram

## Acceptance

1. Runtime v3-only paths select only the independent v3 lane
2. Mixed Runtime v3 plus legacy paths select both required profiles
3. Unmapped v3 paths fail closed
4. Focused proof runs quickly and does not invoke unrelated broad jobs

## Dependencies

- Existing CI path-policy workflow
- adl-runtime-kernel Cargo manifest

## Inputs

- .github/workflows/ci.yaml
- adl/tools/select_validation_lanes.py
- adl-runtime-kernel/Cargo.toml
- adl/tools/test_run_pr_fast_test_lane.sh

## Non Goals

- Deleting legacy validation
- Changing Runtime v3 behavior
- Redesigning all CI topology
