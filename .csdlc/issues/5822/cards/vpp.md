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
    "lane": "estimation-contracts-exact-target",
    "proof_role": "Run the exact estimation_contracts integration target for schema, privacy, negatives, cohorts, fallback, calibration, and backtesting; zero tests fail.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 9000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--no-tests=fail",
      "--test",
      "estimation_contracts"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  },
  {
    "lane": "cycle-time-comparison",
    "proof_role": "Validate retained equivalent baseline and candidate cohorts and component timing.",
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

- `cargo nextest run --locked --manifest-path csdlc-v2/Cargo.toml --no-tests=fail --test estimation_contracts`
- `ruby -rjson -e r=JSON.parse(File.read('.csdlc/evidence/5822/cycle-time-comparison.json')); abort('incomparable') unless r['baseline_cohort']&&r['candidate_cohort']&&r['comparison_basis_equal']==true&&r['gates_preserved']==true`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
