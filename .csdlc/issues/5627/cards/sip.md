# Structured Intent Prompt

Template: 1.0.0

Issue: 5627

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Reduce routine post-implementation lifecycle ceremony to four typed commands without weakening safety or evidence.

## Required Outcome

Validation finalizes implementation atomically, review records exact scope and advances Reviewed, publication creates a ready PR directly, and closeout remains unchanged.

## Scope

- C-SDLC v2 validation, review, publication, store, and focused operator contracts
- Focused Gate 4-7 regressions and complete four-command lifecycle proof
- Issue-local typed lifecycle, measurement, review, publication, and closeout evidence

## Authority

- Issue 5627 owns only declared csdlc-v2 lifecycle paths and issue-local records
- Existing active draft publications retain compatibility reconciliation
- Runtime, AWS, unrelated milestones, and new dependencies remain outside scope

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- Work only in /Volumes/FastWork/adl-wp-5627 on codex/5627-csdlc-four-command-lifecycle
- No raw gh, AWS, Runtime changes, extra preparation ceremony, or root-main edits
- Execute continuously through authorized merge and typed closeout
