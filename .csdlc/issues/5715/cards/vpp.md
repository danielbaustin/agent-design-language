# Validation Planning Prompt

Template: 1.0.0

Issue: 5715

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5715/retained/design.md

Diagram: .csdlc/issues/5715/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "python_compile",
    "proof_role": "generator and validator syntax",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      "-m",
      "py_compile",
      "adl/tools/generate_podcast_launch_packet.py",
      "adl/tools/validate_podcast_launch_packet.py"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "podcast_packet",
    "proof_role": "end-to-end local podcast generation and validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
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

Seconds: 3600

Tokens: 25000

## Commands

- `python3 -m py_compile adl/tools/generate_podcast_launch_packet.py adl/tools/validate_podcast_launch_packet.py`
- `bash adl/tools/test_podcast_launch_packet.sh`

## Failure Semantics

Fail closed on modified exported HTML bytes, missing route wiring, dirty export filenames, missing audio/RSS, or generator/output drift.

## Handoff

Retain typed evidence before convergence.
