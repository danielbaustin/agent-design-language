# Validation Planning Prompt

Template: 1.0.0

Issue: 5717

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5717/retained/design.md

Diagram: .csdlc/issues/5717/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "podcast_packet",
    "proof_role": "local podcast/studio generation, digest, audio, and RSS validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_podcast_launch_packet.sh"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_podcast_launch_packet.sh`

## Failure Semantics

Fail closed on missing logo, stale generated output, incorrect contact/video/episode truth, broken studio asset path, or audio/RSS regression.

## Handoff

Retain typed evidence before convergence.
