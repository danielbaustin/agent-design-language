# Validation Planning Prompt

Template: 1.0.0

Issue: 5836

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5836/design.md

Diagram: .csdlc/prepared/issues/5836/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp18-positive-runtime",
    "proof_role": "Prove the integrated Runtime emits one complete birthday packet.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 400,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_v092_first_birthday_demo.sh",
      "--positive"
    ],
    "parallel_group": "birthday",
    "defer_reason": null
  },
  {
    "lane": "wp18-negative-replay",
    "proof_role": "Prove all not-a-birthday, interruption, replay, and redaction cases fail closed.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 400,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_v092_first_birthday_demo.sh",
      "--negative"
    ],
    "parallel_group": "birthday",
    "defer_reason": null
  },
  {
    "lane": "wp18-native-macos",
    "proof_role": "Run the packet contract on a native macOS host and pin host class, source revision, argv, result, and artifact digests.",
    "acceptance_ids": [
      "AC-3",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_v092_first_birthday_demo.sh",
      "--native-platform",
      "macos"
    ],
    "parallel_group": "platform",
    "defer_reason": "Requires a native macOS runner; absence or a non-native substitute blocks readiness."
  },
  {
    "lane": "wp18-native-linux",
    "proof_role": "Run the packet contract on a native Linux host and pin host class, source revision, argv, result, and artifact digests.",
    "acceptance_ids": [
      "AC-3",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_v092_first_birthday_demo.sh",
      "--native-platform",
      "linux"
    ],
    "parallel_group": "platform",
    "defer_reason": "Requires a native Linux runner; absence or a non-native substitute blocks readiness."
  },
  {
    "lane": "wp18-publication-gate",
    "proof_role": "Validate both canonical launch documents and fail closed on missing accepted proof, stale review, unsupported claims, unresolved negatives, or absent operator authorization without publishing.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/evidence/5836/validate-publication-gate.rb",
      "--check-only"
    ],
    "parallel_group": "publication",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_v092_first_birthday_demo.sh --positive`
- `bash adl/tools/test_v092_first_birthday_demo.sh --negative`
- `bash adl/tools/test_v092_first_birthday_demo.sh --native-platform macos`
- `bash adl/tools/test_v092_first_birthday_demo.sh --native-platform linux`
- `ruby .csdlc/evidence/5836/validate-publication-gate.rb --check-only`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
