# Validation Planning Prompt

Template: 1.0.0

Issue: 5757

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/evidence/5757/DESIGN.md

Diagram: .csdlc/evidence/5757/diagram.mmd

## Selected Lanes

[
  {
    "lane": "observatory-ui",
    "proof_role": "JavaScript syntax and client guard smoke",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "node",
      "--check",
      "demos/html-observatory/app.js"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "runtime-tls-wss",
    "proof_role": "Browser HTTPS, shared TLS identity, and authenticated WSS proof",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 240,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_v0917_html_observatory_integrated_proof.sh"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Whitespace, conflict-marker, and publication-body hygiene",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
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
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `node --check demos/html-observatory/app.js`
- `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh`
- `git diff --check`

## Failure Semantics

Fail closed with evidence in .csdlc/evidence/5757 and do not publish if trusted-origin, generation ordering, TLS/WSS proof, review, or diff hygiene fails.

## Handoff

Retain typed evidence before convergence.
