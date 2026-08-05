# Validation Planning Prompt

Template: 1.0.0

Issue: 5847

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5847/design.md

Diagram: .csdlc/prepared/issues/5847/diagram.mmd

## Selected Lanes

[
  {
    "lane": "external-packet-identity",
    "proof_role": "Validate the immutable external packet corpus, digest, target SHA, redaction boundary, and reviewer authority before transfer. [preexec_rejection exit=1 diagnostic_sha256=973b17cbc0a2ca5924335f6f6750d70c2fe5cd25b7b4d57af5b3935f47a062d3]",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 420,
    "budget_tokens": 3500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5847/validate-external-review.rb",
      "packet"
    ],
    "parallel_group": "packet",
    "defer_reason": null
  },
  {
    "lane": "external-report-authority",
    "proof_role": "Validate reviewer identity and authority, report/packet/target digests, complete finding count, full schema/evidence, risk authority, duplicate targets, and dispositions. [preexec_rejection exit=1 diagnostic_sha256=973b17cbc0a2ca5924335f6f6750d70c2fe5cd25b7b4d57af5b3935f47a062d3]",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 540,
    "budget_tokens": 4500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5847/validate-external-review.rb",
      "report"
    ],
    "parallel_group": "report",
    "defer_reason": "Run only after the operator-authorized reviewer response is actually received."
  },
  {
    "lane": "typed-card-doctor",
    "proof_role": "Validate the exact rendered six-card bundle, cross-card references, design approval, digests, statuses, and canonical issue record.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5847"
    ],
    "parallel_group": "typed-readback",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/5847/validate-external-review.rb packet`
- `ruby .csdlc/prepared/issues/5847/validate-external-review.rb report`
- `csdlc-doctor --repo . --issue 5847`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
