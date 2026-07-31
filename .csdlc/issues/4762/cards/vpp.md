# Validation Planning Prompt

Template: 1.0.0

Issue: 4762

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Validate the #4762 docs/artifact handoff package with deterministic package-shape, diff-hygiene, claim-boundary, typed lifecycle, and exact-head review checks.

## Lane Inputs

Design: .csdlc/prepared/issues/4762/design.md

Diagram: .csdlc/prepared/issues/4762/diagram.mmd

## Selected Lanes

[
  {
    "lane": "receipt-package-validator",
    "proof_role": "Prove required register/receipt fields, witnesses, negative cases, source paths, handoff consumers, and forbidden-claim boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/4762/validate_birth_receipt_package.rb"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove touched docs, cards, evidence, and package artifacts have no diff hygiene failures.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "--",
      ".csdlc/issues/4762",
      ".csdlc/prepared/issues/4762",
      ".csdlc/evidence/4762",
      "docs/milestones/v0.91.8",
      "docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "claim-boundary-scan",
    "proof_role": "Retain searchable evidence for not_claimed and forbidden-claim boundaries.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "rg",
      "birth_event_status|first true Godel-agent birthday has happened|not a birthday occurrence|not_claimed",
      ".csdlc/prepared/issues/4762",
      "docs/milestones/v0.91.8/review/v092_handoff_4762"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "exact-head-review",
    "proof_role": "One exact-head pre-PR review over the changed #4762 package and lifecycle truth.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "codex-review",
      "#4762",
      "--exact-head"
    ],
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1800

Tokens: 8000

## Commands

- `ruby .csdlc/prepared/issues/4762/validate_birth_receipt_package.rb`
- `git diff --check -- .csdlc/issues/4762 .csdlc/prepared/issues/4762 .csdlc/evidence/4762 docs/milestones/v0.91.8 docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
- `rg birth_event_status|first true Godel-agent birthday has happened|not a birthday occurrence|not_claimed .csdlc/prepared/issues/4762 docs/milestones/v0.91.8/review/v092_handoff_4762`
- `codex-review #4762 --exact-head`

## Failure Semantics

Fail closed on validator failure, diff hygiene failure, missing claim-boundary evidence, unresolved exact-head review findings, or stale lifecycle publication truth.

## Handoff

Retain typed evidence before convergence.
