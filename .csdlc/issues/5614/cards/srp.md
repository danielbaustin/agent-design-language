# Structured Review Prompt

Template: 1.0.0

Issue: 5614

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/tools/test_run_aws_spot_ci_profile.sh

## Prompts

- Does the fixture still exercise the same runtime pattern?
- Does any matching literal remain in tracked source?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Historical scanner locations remain until alert #1 is explicitly resolved after the corrected source reaches main.

## Review Result

Revision: Some("git-blake3:01cfd77d10bdf18ae6160bf3b90b2df45f139f90:46bb869b618153dbf9c5021b8f14a484d248296b9f9734adf9b5cccfcbb5d5d9")

Reviewer: Some("subagent:review_5614")

Result: pass
