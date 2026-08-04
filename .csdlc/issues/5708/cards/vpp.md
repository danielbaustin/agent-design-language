# Validation Planning Prompt

Template: 1.0.0

Issue: 5708

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5708/retained/design.md

Diagram: .csdlc/issues/5708/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "podcast_studio_packet",
    "proof_role": "Focused deterministic Podcast Studio packet validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "bash",
      "adl/tools/test_podcast_studio_v2_packet.sh"
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

- `bash adl/tools/test_podcast_studio_v2_packet.sh`

## Failure Semantics

Fail closed on any identity, review, publication, or terminal-evidence mismatch.

## Handoff

Retain typed evidence before convergence.
