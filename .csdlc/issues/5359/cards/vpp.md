# Validation Planning Prompt

Template: 1.0.0

Issue: 5359

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5359/design.md

Diagram: .csdlc/prepared/issues/5359/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp22-yaml-and-wp-parity",
    "proof_role": "Parse the issue wave and prove exact WP identifier/title parity with the WBS.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      "-ryaml",
      ".csdlc/prepared/issues/5359/validate-v092-package.rb"
    ],
    "parallel_group": "planning-validation",
    "defer_reason": null
  },
  {
    "lane": "wp22-source-dispositions",
    "proof_role": "Prove every approved, deferred, and later-backlog TBD input has one explicit disposition.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      "-ryaml",
      ".csdlc/prepared/issues/5359/validate-v092-package.rb"
    ],
    "parallel_group": "planning-validation",
    "defer_reason": null
  },
  {
    "lane": "wp22-typed-doctor",
    "proof_role": "Validate the #5359 lifecycle record and all six rendered card projections.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5359"
    ],
    "parallel_group": "planning-validation",
    "defer_reason": null
  },
  {
    "lane": "wp22-diff-hygiene",
    "proof_role": "Prove the bounded planning diff has no whitespace errors.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 15,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "planning-validation",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby -ryaml .csdlc/prepared/issues/5359/validate-v092-package.rb`
- `ruby -ryaml .csdlc/prepared/issues/5359/validate-v092-package.rb`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5359`
- `git diff --check`

## Failure Semantics

Fail closed on missing live predecessor merge, stale ancestry, missing review inputs, or unsupported v0.92 claims.

## Handoff

Retain typed evidence before convergence.
