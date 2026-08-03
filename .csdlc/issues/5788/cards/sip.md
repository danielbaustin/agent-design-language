# Structured Intent Prompt

Template: 1.0.0

Issue: 5788

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make repository-native owner builds and validation lock-preserving and current-target-only.

## Required Outcome

Failed or successful owner build paths leave the caller's exact Cargo.lock state unchanged unless dependency mutation is explicitly requested.

## Scope

- adl/tools/install_owner_binaries.sh
- adl/tools/run_owner_validation_lane.sh
- adl/tools/test_owner_binary_install.sh
- adl/tools/test_owner_validation_lane.sh

## Authority

- Issue #5788 owns only repository lock-preservation and current owner-binary inventory
- No dependency upgrade or lockfile content change is authorized
- No tracked work occurs on main

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- No tracked main edits
- One pre-PR subagent review
