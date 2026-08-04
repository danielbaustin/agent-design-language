# Structured Intent Prompt

Template: 1.0.0

Issue: 5719

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make podcast/demo page and launch packet UI-only PRs select focused validation rather than full hosted runtime plus workspace coverage.

## Required Outcome

A #5716-like podcast studio/static demo path set no longer sets full_coverage_required=true, while Rust/runtime/provider/tooling changes still select their existing coverage requirements.

## Scope

- CI path-policy classifier
- CI path-policy contract tests
- workflow contract assertions if needed
- issue-local C-SDLC lifecycle evidence

## Authority

- GitHub issue #5719
- Observed PR #5716 hosted coverage selection
- Existing CI path-policy contract tests

## Assumptions

- none

## Operator Constraints

- Use only FastWork for tracked issue work.
- Do not write tracked changes on main.
- Do not remove the adl-coverage-hosted aggregator.
- Avoid duplicate expensive hosted producer lanes for static podcast/demo page changes.
- Preserve full coverage for Rust/runtime/provider/tooling policy changes.
