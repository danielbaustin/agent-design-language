# Validation Planning Prompt

Template: 1.0.0

Issue: 5838

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5838/design.md

Diagram: .csdlc/prepared/issues/5838/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp18b-real-providers",
    "proof_role": "Require two real-provider positive runs through the identical scenario and ACIP contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 500,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/demo_v092_provider_neutral_birthday.sh",
      "--real-only",
      "--minimum-providers",
      "2"
    ],
    "parallel_group": "provider-live",
    "defer_reason": null
  },
  {
    "lane": "wp18b-no-substitution",
    "proof_role": "Reject malformed, denied, unavailable, lost, interrupted, cached, fixture, and receipt-only substitutions.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 350,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      "adl/tools/test_v092_provider_neutral_proof.sh",
      "--negative"
    ],
    "parallel_group": "provider-negative",
    "defer_reason": null
  },
  {
    "lane": "wp18b-redaction-platform",
    "proof_role": "Validate credentials/redaction, matrix parity, Runtime isolation, and macOS/Linux tooling posture.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_v092_provider_neutral_proof.sh",
      "--platform-contract"
    ],
    "parallel_group": "platform",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/demo_v092_provider_neutral_birthday.sh --real-only --minimum-providers 2`
- `bash adl/tools/test_v092_provider_neutral_proof.sh --negative`
- `bash adl/tools/test_v092_provider_neutral_proof.sh --platform-contract`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
