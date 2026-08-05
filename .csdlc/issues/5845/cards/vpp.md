# Validation Planning Prompt

Template: 1.0.0

Issue: 5845

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5845/design.md

Diagram: .csdlc/prepared/issues/5845/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp24a-package-positive",
    "proof_role": "Prove all ten packages, final audio manifests, and feed records are complete and internally consistent.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 400,
    "budget_tokens": 4000,
    "argv": [
      "env",
      "TMPDIR=.csdlc/evidence/5845/tmp",
      "bash",
      "adl/tools/test_podcast_launch_packet.sh",
      "--ten-episode-positive"
    ],
    "parallel_group": "podcast",
    "defer_reason": null
  },
  {
    "lane": "wp24a-package-negative",
    "proof_role": "Reject missing or silent audio, digest and metadata mismatch, local feed paths, draft guests, unsafe text, and publication overclaims.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 400,
    "budget_tokens": 4000,
    "argv": [
      "env",
      "TMPDIR=.csdlc/evidence/5845/tmp",
      "bash",
      "adl/tools/test_podcast_launch_packet.sh",
      "--ten-episode-negative"
    ],
    "parallel_group": "podcast",
    "defer_reason": null
  },
  {
    "lane": "wp24a-platform-playback",
    "proof_role": "Record non-synthetic macOS, Linux, desktop-browser, and iOS Safari playback evidence for required episodes.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 400,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      ".csdlc/evidence/5845/validate-platform-playback.py"
    ],
    "parallel_group": "platform",
    "defer_reason": "Run only where the required operating systems and browser/device targets are available; absence blocks publication readiness."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `env TMPDIR=.csdlc/evidence/5845/tmp bash adl/tools/test_podcast_launch_packet.sh --ten-episode-positive`
- `env TMPDIR=.csdlc/evidence/5845/tmp bash adl/tools/test_podcast_launch_packet.sh --ten-episode-negative`
- `python3 .csdlc/evidence/5845/validate-platform-playback.py`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
