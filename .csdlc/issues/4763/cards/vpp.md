# Validation Planning Prompt

Template: 1.0.0

Issue: 4763

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Validation remains preparation-bounded. Immediate lanes prove packet hygiene and review capture; future execution lanes are intentionally deferred and fail closed until #4762 actual retained implementation proof and typed lifecycle recovery are available.

## Lane Inputs

Design: .csdlc/prepared/issues/4763/design.md

Diagram: .csdlc/prepared/issues/4763/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prep-diff-hygiene",
    "proof_role": "Confirm the preparation branch has no whitespace/path hygiene errors before commit.",
    "acceptance_ids": [
      "AC-1",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local-prep",
    "defer_reason": null
  },
  {
    "lane": "prep-card-render-integrity",
    "proof_role": "Check the complete issue-local card, design, diagram, and index packet for current C-SDLC v2 doctor consistency.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "run",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--bin",
      "csdlc-doctor",
      "--",
      "--repo",
      ".",
      "--issue",
      "4763"
    ],
    "parallel_group": "local-prep",
    "defer_reason": null
  },
  {
    "lane": "typed-lifecycle-reacquire-doctor",
    "proof_role": "Reacquire the expired #4763 writer claim and run csdlc-doctor through typed v2 lifecycle authority.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "run",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--bin",
      "csdlc-bind",
      "--",
      "--root",
      ".",
      "--reacquire-request",
      ".csdlc/prepared/issues/4763/reacquire-claim-20260731.json"
    ],
    "parallel_group": "typed-lifecycle",
    "defer_reason": "Blocked in this preparation session by unrelated #5332 terminal-authority reconciliation; must pass before later execution is considered lifecycle-clean."
  },
  {
    "lane": "dependency-proof-gate",
    "proof_role": "Before implementation, inspect #4762 retained implementation evidence for actual birth-witness and receipt artifacts.",
    "acceptance_ids": [
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "test",
      "-e",
      ".csdlc/evidence/4762"
    ],
    "parallel_group": "future-execution",
    "defer_reason": "Deferred until #4762 produces actual retained implementation proof; claim/receipt/closeout bookkeeping is not a substitute."
  },
  {
    "lane": "future-doc-proof",
    "proof_role": "After execution, prove first-birthday docs and external launch surfaces changed only intended paths and cite retained evidence.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "git",
      "diff",
      "--name-only",
      "origin/main...HEAD"
    ],
    "parallel_group": "future-execution",
    "defer_reason": "Deferred because this branch performs preparation only, not documentation implementation."
  },
  {
    "lane": "public-claim-redaction",
    "proof_role": "After execution, scan launch/birthday copy for unsupported legal/personhood/consciousness/autonomy/public-readiness claims.",
    "acceptance_ids": [
      "AC-2",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "rg",
      "-n",
      "legal personhood|consciousness|sentient|autonomous public agent|publicly launched",
      "docs/milestones/v0.92",
      "docs/milestones/v0.91.8"
    ],
    "parallel_group": "future-execution",
    "defer_reason": "Deferred until later implementation creates or changes public-facing launch copy."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 2400

Tokens: 30000

## Commands

- `git diff --check`
- `cargo run --manifest-path csdlc-v2/Cargo.toml --bin csdlc-doctor -- --repo . --issue 4763`
- `cargo run --manifest-path csdlc-v2/Cargo.toml --bin csdlc-bind -- --root . --reacquire-request .csdlc/prepared/issues/4763/reacquire-claim-20260731.json`
- `test -e .csdlc/evidence/4762`
- `git diff --name-only origin/main...HEAD`
- `rg -n legal personhood|consciousness|sentient|autonomous public agent|publicly launched docs/milestones/v0.92 docs/milestones/v0.91.8`

## Failure Semantics

Fail closed. Preparation may commit only issue-local cards/design/diagram/review truth. Later execution must not start if #4762 actual retained implementation proof is absent, typed lifecycle reacquire/doctor remains blocked, public claims are unsupported, paths widen without replan, or new COTS dependencies are introduced.

## Handoff

Retain typed evidence before convergence.
