# Structured Intent Prompt

Template: 1.0.0

Issue: 5645

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make authorized C-SDLC v2 merges explicit, typed, and effortless without weakening readiness gates.

## Required Outcome

A separate Rust binary validates canonical merge readiness, performs an exact-head GitHub merge, and returns the merge SHA.

## Scope

- csdlc-v2/src
- csdlc-v2/tests
- docs/architecture/csdlc_merge_command_5645.md
- docs/architecture/csdlc_merge_command_5645.mmd

## Authority

- Merge only the bound PR after canonical readiness; never merge arbitrary PRs or replace review/closeout.

## Assumptions

- none

## Operator Constraints

- No AWS
- No shell or Python lifecycle path
- Keep csdlc-publish non-merging
