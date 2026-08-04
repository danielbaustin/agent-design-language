# Structured Intent Prompt

Template: 1.0.0

Issue: 5763

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Restore deterministic feature-crosswalk validation without weakening the digest guard or widening docs scope.

## Required Outcome

The crosswalk artifact and deterministic validator agree with the current 122-row canonical feature list and retain exact row-by-row parity.

## Scope

- current source-row digest in docs/milestones/v0.91.8/feature_preservation_crosswalk_5594.v1.json
- issue-local lifecycle and evidence records for #5763

## Authority

- Do not edit canonical feature-list prose unless validation proves a source artifact mismatch outside the stale digest
- Do not weaken or remove validate_feature_crosswalk.rb digest, row, source-line, owner, classification, or canonical-field checks
- Do not widen documentation scope beyond the stale digest reconciliation

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 lifecycle only
- Use /Volumes/FastWork/adl-wp-5763 as the issue worktree
- Never use /private/tmp
- Never edit root main
- Run one bounded GPT-5.5 review before PR
- PR body must include Closes #5763
- Do not block on post-merge typed closeout
