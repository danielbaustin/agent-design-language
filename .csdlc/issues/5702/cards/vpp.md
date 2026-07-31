# Validation Planning Prompt

Template: 1.0.0

Issue: 5702

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5702/design.md

Diagram: .csdlc/prepared/issues/5702/diagram.mmd

## Selected Lanes

[
  {
    "lane": "source-evidence-paths",
    "proof_role": "existing podcast readiness source evidence exists",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 10,
    "budget_tokens": 100,
    "argv": [
      "rg",
      "--files"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "final-plan-smoke",
    "proof_role": "post-authoring required topic coverage smoke check; target file supplied in SOR validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 10,
    "budget_tokens": 250,
    "argv": [
      "rg",
      "audio|RSS|Deepgram|Gemini|guest"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "post-authoring diff hygiene and review-ready plan check",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 10,
    "budget_tokens": 100,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `rg --files`
- `rg audio|RSS|Deepgram|Gemini|guest`
- `git diff --check`

## Failure Semantics

Fail closed if the plan omits required audio/RSS launch blockers, overclaims deployment or guest confirmation, or Gemini review cannot be truthfully recorded.

## Handoff

Retain typed evidence before convergence.
