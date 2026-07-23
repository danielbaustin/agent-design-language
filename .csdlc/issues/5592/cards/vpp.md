# Validation Planning Prompt

Template: 1.0.0

Issue: 5592

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5592/design.md

Diagram: .csdlc/prepared/issues/5592/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-parity-b-live-graph",
    "proof_role": "Prove guardian-launched canonical-ingress production reasoning graph execution and deterministic retained transitions",
    "acceptance_ids": [
      "AC-1",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5592/run_exact_live_test_lane.rb",
      "--lane",
      "runtime-v3-parity-b-live-graph"
    ],
    "parallel_group": "parity-b-live",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-parity-b-bounded-loop",
    "proof_role": "Prove bounded convergence, exhaustion, cancellation, checkpoint, replay, and resume without budget reset or duplicate effects",
    "acceptance_ids": [
      "AC-2",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5592/run_exact_live_test_lane.rb",
      "--lane",
      "runtime-v3-parity-b-bounded-loop"
    ],
    "parallel_group": "parity-b-live",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-parity-b-adaptation",
    "proof_role": "Prove provenance-bound observation, signed one-shot mutation, atomic durability, deterministic recovery, and rollback positives and negatives",
    "acceptance_ids": [
      "AC-3",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5592/run_exact_live_test_lane.rb",
      "--lane",
      "runtime-v3-parity-b-adaptation"
    ],
    "parallel_group": "parity-b-learning",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-parity-b-affect-isolation",
    "proof_role": "Prove bounded affect reasoning-control, safe non-claims, adversarial task-signal isolation, and monotonic authority",
    "acceptance_ids": [
      "AC-4",
      "AC-6",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5592/run_exact_live_test_lane.rb",
      "--lane",
      "runtime-v3-parity-b-affect-isolation"
    ],
    "parallel_group": "parity-b-cognition",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-parity-b-curiosity-boundary",
    "proof_role": "Prove bounded curiosity and confidence-scored theory-of-mind task models cannot create tool, network, disclosure, mutation, or policy authority",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5592/run_exact_live_test_lane.rb",
      "--lane",
      "runtime-v3-parity-b-curiosity-boundary"
    ],
    "parallel_group": "parity-b-cognition",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-parity-b-authority",
    "proof_role": "Prove cognition, review, adaptation, replay, and restart preserve Freedom Gate, shutdown, cancellation, resource, and review authority under adversarial inputs",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5592/run_exact_live_test_lane.rb",
      "--lane",
      "runtime-v3-parity-b-authority"
    ],
    "parallel_group": "parity-b-negative",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-parity-b-feature-dispositions",
    "proof_role": "Validate one truthful proof-bearing disposition for every owned feature row and reject metadata-only live credit",
    "acceptance_ids": [
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5592/run_exact_live_test_lane.rb",
      "--lane",
      "runtime-v3-parity-b-feature-dispositions"
    ],
    "parallel_group": "parity-b-quality",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-parity-b-independence",
    "proof_role": "Reject Runtime v2 source/execution coupling, premature deletion, AWS, default switch, and unsupported claims while inventorying proven duplicate reasoning paths",
    "acceptance_ids": [
      "AC-8",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "tree",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "parallel_group": "parity-b-quality",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-parity-b-live-kernel",
    "proof_role": "Run the complete canonical Runtime v3 kernel suite after focused live and negative proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "parallel_group": "parity-b-full",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-parity-b-quality",
    "proof_role": "Prove strict warning-free code and preserve the integrated source/module/test budget without reducing behavior or negative proof",
    "acceptance_ids": [
      "AC-8",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--all-features",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "parity-b-quality",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5592/run_exact_live_test_lane.rb --lane runtime-v3-parity-b-live-graph`
- `ruby .csdlc/prepared/issues/5592/run_exact_live_test_lane.rb --lane runtime-v3-parity-b-bounded-loop`
- `ruby .csdlc/prepared/issues/5592/run_exact_live_test_lane.rb --lane runtime-v3-parity-b-adaptation`
- `ruby .csdlc/prepared/issues/5592/run_exact_live_test_lane.rb --lane runtime-v3-parity-b-affect-isolation`
- `ruby .csdlc/prepared/issues/5592/run_exact_live_test_lane.rb --lane runtime-v3-parity-b-curiosity-boundary`
- `ruby .csdlc/prepared/issues/5592/run_exact_live_test_lane.rb --lane runtime-v3-parity-b-authority`
- `ruby .csdlc/prepared/issues/5592/run_exact_live_test_lane.rb --lane runtime-v3-parity-b-feature-dispositions`
- `cargo tree --manifest-path adl-runtime-kernel/Cargo.toml`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets --all-features -- -D warnings`

## Failure Semantics

Fail closed on missing or unreviewed #5591 contract truth, protected-path collision, non-live evidence, unbounded or replay-unsafe loops, invalid or reusable mutation authority, adversarial signal steering, subjective affect overclaim, authority widening, missing feature disposition, Runtime v2 coupling, premature deletion, AWS, publication, budget breach, actionable review findings, or any non-green exact-revision lane.

## Handoff

Retain typed evidence before convergence.
