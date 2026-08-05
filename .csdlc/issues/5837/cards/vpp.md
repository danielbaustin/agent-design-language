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
    "proof_role": "Validate one schema/version/audience/order/reconnect/auth/backpressure matrix plus redaction and denied actions for both clients.",
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
    "proof_role": "Run the separate HTML Observatory against the real trusted Runtime HTTP/WSS paths and retain read, authorized control, denial, stale, offline, and visible browser evidence.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/test_v0917_html_observatory_integrated_proof.sh"
    ],
    "parallel_group": "consumer-live",
    "defer_reason": "Requires issue 5800 trust plus the running exact Runtime candidate."
  },
  {
    "lane": "unity-live-runtime",
    "proof_role": "Run native Unity Editor/player integration against the same Runtime revision and retain reads, controls, denial, failure, and visual interaction evidence.",
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
      "adl/tools/test_v0917_unity_observatory_integrated_proof.sh"
    ],
    "parallel_group": "consumer-live",
    "defer_reason": "Requires the approved Unity version, live Runtime endpoint, and native Editor/player environment."
  },
  {
    "lane": "guardian-restart-reconnect",
    "proof_role": "Restart the Guardian-owned Runtime and prove both clients reconnect with bounded replay, no duplicate application, and no authority escalation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/run_runtime_v3_guardian_soak.sh"
    ],
    "parallel_group": "consumer-live",
    "defer_reason": "Requires both completed consumers and the exact Runtime candidate."
  },
  {
    "lane": "exact-head-hygiene",
    "proof_role": "Reject unrelated changes and support exact-head review.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
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
- `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh`
- `bash adl/tools/test_v0917_unity_observatory_integrated_proof.sh`
- `bash adl/tools/run_runtime_v3_guardian_soak.sh`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
