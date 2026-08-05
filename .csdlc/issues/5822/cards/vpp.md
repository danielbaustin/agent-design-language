# Validation Planning Prompt

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5822/design.md

Diagram: .csdlc/prepared/issues/5822/diagram.mmd

## Selected Lanes

[
  {
    "lane": "estimation-schema-and-roundtrip",
    "proof_role": "Prove typed observation, forecast, accepted-estimate, and outcome schema and serde round trips.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "estimation"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  },
  {
    "lane": "estimation-negative-and-privacy",
    "proof_role": "Reject missing provenance, schema drift, transcript leakage, target-actual leakage, and estimate enforcement.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "estimation_negative"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  },
  {
    "lane": "forecast-backtest-and-fallback",
    "proof_role": "Prove deterministic cohorts, uncertainty, drift, calibration, and explicit static-profile fallback on insufficient data.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "estimation_backtest"
    ],
    "parallel_group": "analysis",
    "defer_reason": null
  },
  {
    "lane": "cycle-time-comparison",
    "proof_role": "Validate retained equivalent baseline and candidate workflow cohorts and component timing.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      "-rjson",
      "-e",
      "r=JSON.parse(File.read('.csdlc/evidence/5822/cycle-time-comparison.json')); abort('incomparable') unless r['baseline_cohort']&&r['candidate_cohort']&&r['comparison_basis_equal']==true&&r['gates_preserved']==true"
    ],
    "parallel_group": "analysis",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors and support exact-revision review.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
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

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml estimation`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml estimation_negative`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml estimation_backtest`
- `ruby -rjson -e r=JSON.parse(File.read('.csdlc/evidence/5822/cycle-time-comparison.json')); abort('incomparable') unless r['baseline_cohort']&&r['candidate_cohort']&&r['comparison_basis_equal']==true&&r['gates_preserved']==true`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
