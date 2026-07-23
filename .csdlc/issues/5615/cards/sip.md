# Structured Intent Prompt

Template: 1.0.0

Issue: 5615

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make C-SDLC-only CI proof focused, deterministic, truthful, and fast while preserving stable required checks.

## Required Outcome

Lifecycle metadata, standalone C-SDLC v2 Rust, Runtime, ADL workspace, and mixed diffs select exactly their intended proof lanes, and local Cargo validation uses a declared writable external root or FastWork.

## Scope

- Existing CI path classifier outputs and exact regression fixtures
- One standalone C-SDLC v2 hosted job and stable aggregate wiring
- One portable Cargo validation wrapper and focused shell contract tests
- Issue-local typed lifecycle and retained validation evidence

## Authority

- Issue 5615 owns only CI classification, workflow dispatch wiring, its portable validation wrapper, and issue-local records
- Existing required check names and non-C-SDLC routes remain authoritative
- C-SDLC v2 lifecycle semantics and Runtime behavior are outside scope

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- Work only in `/Volumes/FastWork/adl-wp-5615`
- Use external writable Cargo home and target
- No raw gh, AWS, Runtime v2, or root-main edits
- Keep the change small and complete
