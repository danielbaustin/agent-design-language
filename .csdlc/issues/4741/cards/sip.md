# Structured Intent Prompt

Template: 1.0.0

Issue: 4741

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make Unity Observatory editor and batch proof select a safe execution mode, show real progress, and fail quickly with precise evidence when the selected mode cannot complete.

## Required Outcome

Repository-owned Unity proof wrappers deterministically select open-editor, fresh-batch, or skipped-fail-closed mode; isolate staged execution; monitor semantic progress; and retain exact proof or blocker truth.

## Scope

- Exact-project editor liveness classification
- Open-editor versus fresh-batch versus skipped mode selection
- Approved staging and mutable Unity/.NET scratch isolation
- Semantic log-progress watchdog and bounded blocker classification
- Focused wrapper unit and contract tests
- Concise operator proof-mode runbook

## Authority

- #4741 owns editor liveness, proof-mode selection, staging, watchdog, cleanup, and wrapper tests
- #4739 owns Unity-MCP project and endpoint identity
- #5332 owns ILPP GetDomainName root cause and detailed retry classifier
- No scene generation, fallback geometry, asset publication, runtime contract semantics, investor rendering, or walkthrough capture
- No raw process scan, /private/tmp staging, scratch Unity copies, locally built owner binaries, or arbitrary total runtime ceiling

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 lifecycle only
- Work only in the issue-bound v0.91.8 worktree
- Preserve the older dirty #4741 worktree unchanged
- Use /Volumes/FastWork or issue-local .adl for staging and repository-installed binaries only
- No /private/tmp, raw process scans, arbitrary total runtime ceiling, raw gh fallback, or secret exposure
