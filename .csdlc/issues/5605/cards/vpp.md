# Validation Planning Prompt

Template: 1.0.0

Issue: 5605

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5605/retained/design.md

Diagram: .csdlc/issues/5605/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "referenced-path-check",
    "proof_role": "Prove all launch-readiness references point to tracked repo surfaces",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "bash",
      "-lc",
      "test referenced podcast paths exist"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "old-podcast-demo-regression",
    "proof_role": "Prove the existing v0.91.3 podcast packet generator still works",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/demo_v0913_podcast_studio_v2.sh"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "old-podcast-packet-test",
    "proof_role": "Prove the existing v0.91.3 podcast packet contract remains deterministic",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_podcast_studio_v2_packet.sh"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "redaction-and-diff-check",
    "proof_role": "Prove docs contain no secrets or host paths and pass whitespace diff checks",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
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

- `bash -lc test referenced podcast paths exist`
- `bash adl/tools/demo_v0913_podcast_studio_v2.sh`
- `bash adl/tools/test_podcast_studio_v2_packet.sh`
- `git diff --check`

## Failure Semantics

Fail closed if any referenced evidence path is missing, any artifact overclaims live publication, or bounded review finds actionable issues.

## Handoff

Retain typed evidence before convergence.
