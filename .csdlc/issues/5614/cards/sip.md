# Structured Intent Prompt

Template: 1.0.0

Issue: 5614

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Resolve secret-scanning alert #1 without weakening redaction coverage.

## Required Outcome

No AWS temporary access-key ID literal remains in tracked source and runtime redaction proof still passes.

## Scope

- adl/tools/test_run_aws_spot_ci_profile.sh

## Authority

- Issue #5614 owns only the synthetic redaction fixture and alert resolution
- No AWS access or credential handling

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- No AWS
- Bound issue worktree
