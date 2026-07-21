# Structured Task Prompt

Template: 1.0.0

Issue: 5587

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Correct transport selection, recursive traversal, exact readback verification, and success-only automation semantics.

## Deliverables

- Production native transport route
- Exact content verification
- Recursive Markdown mirroring
- Focused tests and live proof

## Acceptance

1. AC-1: execute mode performs a real Drive create or update and exact readback verification
2. AC-2: docs and .adl/docs/TBD Markdown trees mirror recursively with preserved relative paths
3. AC-3: least-privilege scopes and explicit write approval remain enforced
4. AC-4: automation archives only fully verified successes and retains deduplicated failures
5. AC-5: fixture and focused tests prove deterministic success and fail-closed behavior

## Dependencies

- Google Drive credentials and configured root folder
- Issue #5587 operator authorization

## Inputs

- adl/src/adl_gws_native.rs
- adl/src/adl_gws_drive_sync.rs
- adl/src/adl_gws_context_mirror.rs

## Non Goals

- No remote deletion
- No AWS
- No unrelated Google Workspace providers
