# Validation Planning Prompt

Template: 1.0.0

Issue: 5821

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5821/design.md

Diagram: .csdlc/prepared/issues/5821/diagram.mmd

## Selected Lanes

[
  {
    "lane": "architecture-threat-gate",
    "proof_role": "Validate the frozen architecture, threat model, COTS choices, schemas, trust boundaries, and exact 16-child coverage/ownership ledger.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5821/validate-distributed-program.rb"
    ],
    "parallel_group": "program-gate",
    "defer_reason": "Validator is an implementation deliverable and runs before any child receives implementation credit."
  },
  {
    "lane": "distributed-state-machines",
    "proof_role": "Prove epochs, leases, fencing, membership, topology, snapshot, migration, rollback, replay, and stale-message semantics deterministically.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "continuity",
      "--test",
      "durable_state",
      "--test",
      "control"
    ],
    "parallel_group": "distributed-contract",
    "defer_reason": null
  },
  {
    "lane": "real-multinode-adversarial",
    "proof_role": "Exercise production mTLS/QUIC membership, partition, fencing, migration, rollback, rotation/revocation, recovery, and relocation failures across real nodes.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_guardian",
      "--",
      "--ignored",
      "--exact",
      "real_multinode_program"
    ],
    "parallel_group": "multinode-live",
    "defer_reason": "Requires completed child implementations, real node identities, and issue-local certificate/state roots."
  },
  {
    "lane": "child-terminal-reconciliation",
    "proof_role": "Require all 16 child issues, PRs, exact reviews, validation, merged revisions, receipts, and released claims to agree.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5821/validate-distributed-children.rb"
    ],
    "parallel_group": "program-closeout",
    "defer_reason": "Runs after all child closeout receipts exist."
  },
  {
    "lane": "platform-and-exact-head",
    "proof_role": "Run declared native platform proof, diff hygiene, and exact integrated review.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/run_owner_validation_lane.sh",
      "runtime"
    ],
    "parallel_group": "platform",
    "defer_reason": "Requires native platform runners and the exact integrated candidate."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5821/validate-distributed-program.rb`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test continuity --test durable_state --test control`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_guardian -- --ignored --exact real_multinode_program`
- `ruby .csdlc/prepared/issues/5821/validate-distributed-children.rb`
- `bash adl/tools/run_owner_validation_lane.sh runtime`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
