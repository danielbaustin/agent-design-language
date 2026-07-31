# Structured Intent Prompt

Template: 1.0.0

Issue: 5727

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Allow a deliberately released or expired writer lease to be reacquired through a typed compare-and-swap operation without corrupting or rewinding lifecycle state.

## Required Outcome

Dormant resumable records are truthful read-only state, typed reacquisition restores a validated non-overlapping writer claim, and all write operations remain fail-closed without a live covering claim.

## Scope

- C-SDLC v2 claim lifecycle request and result contracts
- csdlc-bind typed command surface
- doctor classification for dormant nonterminal records
- focused lifecycle and #5354 reproduction tests
- issue-local lifecycle evidence

## Authority

- GitHub issue #5727
- C-SDLC v2 Rust lifecycle and store
- existing claim collision and audit contracts

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 binaries only.
- Keep the primary checkout clean on main.
- Perform all tracked implementation in /Volumes/FastWork/adl-wp-5727.
- Do not weaken overlapping-writer collision checks.
- Do not directly edit existing issue records or rewind lifecycle phases.
- Do not execute WP-15 product or demo work.
