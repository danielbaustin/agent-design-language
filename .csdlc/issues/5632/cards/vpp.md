# Validation Planning Prompt

Template: 1.0.0

Issue: 5632

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/architecture/adl_pr_cycle_v2_skill.md

Diagram: docs/architecture/adl_pr_cycle_v2_skill.mmd

## Selected Lanes

[
  {
    "lane": "skill-frontmatter",
    "proof_role": "structure",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/validate_skill_frontmatter.sh",
      "docs/tooling/adl_pr_cycle_skill.md"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "installer-parity",
    "proof_role": "generated-copy",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/install_adl_pr_cycle_skill.sh"
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

- `bash adl/tools/validate_skill_frontmatter.sh docs/tooling/adl_pr_cycle_skill.md`
- `bash adl/tools/install_adl_pr_cycle_skill.sh`

## Failure Semantics

Fail closed, preserve evidence, and report one typed recovery operation.

## Handoff

Retain typed evidence before convergence.
