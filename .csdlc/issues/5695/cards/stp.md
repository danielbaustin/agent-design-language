# Structured Task Prompt

Template: 1.0.0

Issue: 5695

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair the bounded mergeability mapping and prove its state table before execution.

## Deliverables

- Corrected mergeability mapping
- Focused classification tests
- Exact review and publication evidence

## Acceptance

1. AC-1: Every supported mergeability variant has an explicit classification
2. AC-2: blocked and unstable cannot be reported as stale_base
3. AC-3: behind remains stale_base and dirty remains conflicted
4. AC-4: Focused tests prove the table and csdlc-merge remains fail-closed until clean ancestry and required checks are observed

## Dependencies

- octocrab MergeableState variants
- Existing csdlc-pr-state classification and merge gate tests

## Inputs

- csdlc-v2/src/github.rs
- csdlc-v2/src/merge.rs
- Issue 5695

## Non Goals

- No provider, Runtime, AWS, or CI workflow changes
- No GitHub merge-policy changes
- No reopening of issue 5683
- No broad test-suite expansion
