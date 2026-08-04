# Validation Planning Prompt

Template: 1.0.0

Issue: 5804

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5804/design.md

Diagram: .csdlc/prepared/issues/5804/diagram.mmd

## Selected Lanes

[
  {
    "lane": "review-corpus-contract",
    "proof_role": "Validate the 75-document v0.91.8 corpus, concrete implementation manifest, live issue truth, portable paths, local links, structured files, 122-row feature crosswalk, WP-18 ancestry, and fail-closed undispatched handoff state",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5804/validate-review-corpus.rb"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "wp19-dependency-contract",
    "proof_role": "Verify merged WP-18 first and final review passes are ancestors of the corrective target",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/check-dependencies.rb"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors in the corrective change",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/5804/validate-review-corpus.rb`
- `ruby .csdlc/prepared/issues/5357/check-dependencies.rb`
- `git diff --check`

## Failure Semantics

Fail closed on missing paths, stale current truth, machine-local requirements, broken links, parse failures, or an actionable review finding.

## Handoff

Retain typed evidence before convergence.
