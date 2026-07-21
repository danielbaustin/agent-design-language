# Structured Intent Prompt

Template: 1.0.0

Issue: 5587

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Run the ADL context mirror against Google Drive with recursive, content-verified, least-privilege behavior.

## Required Outcome

Execute mode uses the native Drive transport, mirrors the declared repository surfaces recursively, and succeeds only after exact remote-content readback.

## Scope

- adl/src/adl_gws_native.rs
- adl/src/adl_gws_drive_sync.rs
- adl/src/adl_gws_context_mirror.rs
- adl/src/bin/demo_adl_gws_context_mirror.rs
- adl/Cargo.toml

## Authority

- Issue #5587 owns production Drive mirror behavior and its focused proof
- Remote deletion and broader Workspace access are excluded
- AWS and Spot are excluded

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 only
- No AWS or Spot
- Use a bound issue worktree
