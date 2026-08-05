# Validation Planning Prompt

Template: 1.0.0

Issue: 5837

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5837/design.md

Diagram: .csdlc/prepared/issues/5837/diagram.mmd

## Selected Lanes

[
  {
    "lane": "shared-consumer-contract",
    "proof_role": "Validate one schema, audience, ordering, reconnect, auth, and backpressure matrix plus redaction and denied actions for both clients.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/test_v0916_unity_observatory_contract.sh"
    ],
    "parallel_group": "consumer-contract",
    "defer_reason": null
  },
  {
    "lane": "html-live-runtime",
    "proof_role": "Use the issue-delivered Playwright entrypoint to drive the real HTML Observatory over trusted Runtime HTTPS/WSS and retain read, authorized control, denial, stale, offline, and visible browser evidence.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "node",
      "adl/tools/validate_v092_html_observatory_live.mjs",
      "--browser",
      "chrome",
      "--require-live-runtime",
      "--require-authorized-control",
      "--require-denial-proof"
    ],
    "parallel_group": "consumer-live",
    "defer_reason": "The named validator is an issue 5837 implementation deliverable and requires issue 5800 trust plus the exact Runtime candidate."
  },
  {
    "lane": "unity-live-runtime",
    "proof_role": "Use the issue-delivered native entrypoint to launch the approved Unity Editor/player against the same Runtime revision and retain real reads, controls, denial, failure, and visual interaction evidence.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/validate_v092_unity_observatory_live.sh",
      "--require-editor-player",
      "--require-live-runtime",
      "--require-visual-evidence"
    ],
    "parallel_group": "consumer-live",
    "defer_reason": "The named validator is an issue 5837 implementation deliverable and requires the approved Unity version and native environment."
  },
  {
    "lane": "guardian-restart-both-clients",
    "proof_role": "Use the issue-delivered coordinator to keep both clients active across Guardian-owned Runtime restart and prove bounded replay, no duplicate application, fresh post-restart correlation, and unchanged authorization.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/validate_v092_observatory_restart_reconnect.sh",
      "--clients",
      "html,unity",
      "--require-bounded-replay",
      "--reject-duplicates",
      "--require-unchanged-authority"
    ],
    "parallel_group": "consumer-live",
    "defer_reason": "The named validator is an issue 5837 implementation deliverable and requires both completed consumers and the exact Guardian/Runtime candidate."
  },
  {
    "lane": "exact-head-hygiene",
    "proof_role": "Reject unrelated changes and support exact-head review.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_v0916_unity_observatory_contract.sh`
- `node adl/tools/validate_v092_html_observatory_live.mjs --browser chrome --require-live-runtime --require-authorized-control --require-denial-proof`
- `bash adl/tools/validate_v092_unity_observatory_live.sh --require-editor-player --require-live-runtime --require-visual-evidence`
- `bash adl/tools/validate_v092_observatory_restart_reconnect.sh --clients html,unity --require-bounded-replay --reject-duplicates --require-unchanged-authority`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
