# Validation Planning Prompt

Template: 1.0.0

Issue: 5363

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5363/design.md

Diagram: .csdlc/prepared/issues/5363/diagram.mmd

## Selected Lanes

[
  {
    "lane": "typed-doctor-5363",
    "proof_role": "Validate canonical typed C-SDLC v2 state for the #5363 preparation packet.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
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
      "5363"
    ],
    "parallel_group": "release-tail-preparation",
    "defer_reason": null
  },
  {
    "lane": "completed-child-5548-ancestry",
    "proof_role": "Prove #5548 completed causal source commit from PR #5598 remains on current origin/main.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 10,
    "budget_tokens": 500,
    "argv": [
      "git",
      "merge-base",
      "--is-ancestor",
      "aac8eaa7dffaa904ed1dfb0ec17fbf667c1ef9f0",
      "origin/main"
    ],
    "parallel_group": "release-tail-preparation",
    "defer_reason": null
  },
  {
    "lane": "completed-child-5558-pr5749-ancestry",
    "proof_role": "Prove #5558 closing PR #5749 merge commit remains on current origin/main.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 10,
    "budget_tokens": 500,
    "argv": [
      "git",
      "merge-base",
      "--is-ancestor",
      "c34f0c9412495039a6374f7ce88fa39e34bb5042",
      "origin/main"
    ],
    "parallel_group": "release-tail-preparation",
    "defer_reason": null
  },
  {
    "lane": "completed-child-5558-pr5769-ancestry",
    "proof_role": "Prove #5558 closing PR #5769 merge commit remains on current origin/main.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 10,
    "budget_tokens": 500,
    "argv": [
      "git",
      "merge-base",
      "--is-ancestor",
      "a5df18f19a4c651eb6594e5690e294c7b7929261",
      "origin/main"
    ],
    "parallel_group": "release-tail-preparation",
    "defer_reason": null
  },
  {
    "lane": "accepted-finding-focused-hygiene",
    "proof_role": "For future execution, prove accepted remediation scope has no whitespace/path hygiene defect before broader preflight.",
    "acceptance_ids": [
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
    "parallel_group": "release-tail-execution",
    "defer_reason": "Run during future execution after accepted findings and changed paths are known."
  },
  {
    "lane": "integrated-owner-release-preflight",
    "proof_role": "For future execution, run the integrated owner validation lane after focused accepted-finding checks pass.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/run_owner_validation_lane.sh",
      "all"
    ],
    "parallel_group": "release-tail-execution",
    "defer_reason": "Run only during future WP-20 execution after WP-19 ancestry and accepted-finding scope are current."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5363`
- `git merge-base --is-ancestor aac8eaa7dffaa904ed1dfb0ec17fbf667c1ef9f0 origin/main`
- `git merge-base --is-ancestor c34f0c9412495039a6374f7ce88fa39e34bb5042 origin/main`
- `git merge-base --is-ancestor a5df18f19a4c651eb6594e5690e294c7b7929261 origin/main`
- `git diff --check`
- `bash adl/tools/run_owner_validation_lane.sh all`

## Failure Semantics

Fail closed on missing live predecessor merge, stale ancestry, unclear finding ownership, or unsupported release claims.

## Handoff

Retain typed evidence before convergence.
