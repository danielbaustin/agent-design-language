# Structured Task Prompt

Template: 1.0.0

Issue: 5788

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Use current targets, --locked, and exact pre-invocation byte restoration for invocation-created drift.

## Deliverables

- Current owner-binary inventory
- Lock-preserving installer and validation builds
- Removed-target and dependency-drift regressions

## Acceptance

1. AC-1: Removed target failure leaves Cargo.lock byte-identical
2. AC-2: Dependency-resolution drift is restored fail-closed
3. AC-3: Pre-existing user-owned lock changes remain byte-identical
4. AC-4: Repository-native Cargo validation uses --locked
5. AC-5: Default owner inventory contains only current Cargo targets

## Dependencies

- Issue #5788 is open
- Current origin/main

## Inputs

- adl/Cargo.toml
- adl/tools/install_owner_binaries.sh
- adl/tools/run_owner_validation_lane.sh

## Non Goals

- No dependency upgrades
- No Cargo.lock edits
- No AWS
- No broad Rust rebuild
