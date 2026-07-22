# Structured Intent Prompt

Template: 1.0.0

Issue: 5632

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Replace sunset lifecycle guidance in the authoring skill with the independent typed C-SDLC v2 route.

## Required Outcome

The canonical and installed skill describe only v2 binaries, typed cards, bounded validation, review-before-publication, and truthful stop boundaries.

## Scope

- docs/tooling/adl_pr_cycle_skill.md
- docs/architecture/adl_pr_cycle_v2_skill.md
- docs/architecture/adl_pr_cycle_v2_skill.mmd

## Authority

- C-SDLC v2 operator skills and Rust binaries are lifecycle authority
- This skill routes and reports but does not own state

## Assumptions

- none

## Operator Constraints

- never write tracked files on main
- no AWS
- no raw GitHub CLI
- do not invoke sunset v1 wrappers
