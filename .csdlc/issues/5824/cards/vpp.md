# Validation Planning Prompt

Template: 1.0.0

Issue: 5824

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5824/design.md

Diagram: .csdlc/prepared/issues/5824/diagram.mmd

## Selected Lanes

[
  {
    "lane": "enum-inventory-contract",
    "proof_role": "Derive the exhaustive restricted-type denominator from current C-SDLC v2 source and require one source-grounded disposition plus a no-duplicate-work or finite-gap decision for every entry. [preexec_rejection exit=1 diagnostic_sha256=1672959818bed15ac083b3a87719231b3fd1071fd8f191eb14ab920a728195f1]",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5824/validate-enum-inventory.rb"
    ],
    "parallel_group": "inventory",
    "defer_reason": null
  },
  {
    "lane": "prompt-card-enum-exact-target",
    "proof_role": "Run the exact prompt_card_enum_typing integration target for round-trip/schema and invalid-value/legacy behavior; zero tests fail.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--no-tests=fail",
      "--test",
      "prompt_card_enum_typing"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and unintended template or sunset-v1 changes.",
    "acceptance_ids": [
      "AC-7",
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

- `ruby .csdlc/prepared/issues/5824/validate-enum-inventory.rb`
- `cargo nextest run --locked --manifest-path csdlc-v2/Cargo.toml --no-tests=fail --test prompt_card_enum_typing`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
