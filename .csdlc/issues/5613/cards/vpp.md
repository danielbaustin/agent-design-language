# Validation Planning Prompt

Template: 1.0.0

Issue: 5613

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5613/retained/design.md

Diagram: .csdlc/issues/5613/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-sor-validation-repair",
    "proof_role": "Prove exact success, CAS, authority, ambiguity, validation, rollback, and receipt parity",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_terminal_sor_validation_repair_5613"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-strict",
    "proof_role": "Prove the complete C-SDLC v2 target remains warning-free and compatible",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  },
  {
    "lane": "terminal-projection-proof",
    "proof_role": "Prove target identity, portable SOR truth, unsupported artifact omission, and receipt parity",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5613/validate_terminal_projections.sh"
    ],
    "parallel_group": "records",
    "defer_reason": null
  },
  {
    "lane": "fresh-checkout-proof",
    "proof_role": "Prove a fresh checkout resolves all targets closed-out, claim-free, doctor-clean, and collision-free",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5613/validate_fresh_checkout.sh"
    ],
    "parallel_group": "records",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate7_terminal_sor_validation_repair_5613`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `bash .csdlc/prepared/issues/5613/validate_terminal_projections.sh`
- `bash .csdlc/prepared/issues/5613/validate_fresh_checkout.sh`

## Failure Semantics

Fail closed without mutation on stale authority, target, or receipt CAS; missing or ambiguous old results; malformed or machine-local replacements; target identity drift; or interrupted receipt update.

## Handoff

Retain typed evidence before convergence.
