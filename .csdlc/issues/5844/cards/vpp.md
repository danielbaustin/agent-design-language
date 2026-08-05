# Validation Planning Prompt

Template: 1.0.0

Issue: 5844

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5844/design.md

Diagram: .csdlc/prepared/issues/5844/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp24-series-contract",
    "proof_role": "Prove all ten complete packets satisfy the Medium writer and series contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 450,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/evidence/5844/validate-article-series.rb"
    ],
    "parallel_group": "articles",
    "defer_reason": null
  },
  {
    "lane": "wp24-claims-negative",
    "proof_role": "Reject outlines, placeholders, fabricated or missing citations, private data, and unsupported current or release claims.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 450,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/evidence/5844/validate-article-series.rb",
      "--negative"
    ],
    "parallel_group": "articles",
    "defer_reason": null
  },
  {
    "lane": "wp24-writer-contract",
    "proof_role": "Prove the repository Medium article writer retains source-packet and stop-before-publish safeguards.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_medium_article_writer_skill_contracts.sh"
    ],
    "parallel_group": "contracts",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/evidence/5844/validate-article-series.rb`
- `ruby .csdlc/evidence/5844/validate-article-series.rb --negative`
- `bash adl/tools/test_medium_article_writer_skill_contracts.sh`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
