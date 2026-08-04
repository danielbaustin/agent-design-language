# Validation Planning Prompt

Template: 1.0.0

Issue: 5007

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Focused preparation validation only: diff hygiene, card projection integrity, dependency-gate text checks, and bounded review evidence. Runtime/provider/AWS/code validation is out of scope until #4760 execution proof exists.

## Lane Inputs

Design: .csdlc/prepared/issues/5007/design.md

Diagram: .csdlc/prepared/issues/5007/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prep-diff-hygiene",
    "proof_role": "Prove preparation Markdown/JSON changes have no whitespace errors.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "--",
      ".csdlc/issues/5007",
      ".csdlc/prepared/issues/5007",
      ".csdlc/evidence/5007/preparation"
    ],
    "parallel_group": "prep-local",
    "defer_reason": null
  },
  {
    "lane": "prep-card-projection",
    "proof_role": "Prove the six values JSON files parse, render through the C-SDLC v2 card renderer, and match the refreshed card projections.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
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
      "5007"
    ],
    "parallel_group": "prep-local",
    "defer_reason": null
  },
  {
    "lane": "prep-dependency-gate",
    "proof_role": "Verify #4760 remains the explicit execution gate and #4765/#4768/#4771/ADR 0051 are named dependencies without treating receipts or claim reconciliation as preparation blockers.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "rg",
      "-n",
      "#4760|51bc5ae51b57c19dbab693af1c5a45142995f4e5|0058-memory-palace|gpt-5.5|no-deferral|COTS|PVF",
      ".csdlc/issues/5007",
      ".csdlc/prepared/issues/5007"
    ],
    "parallel_group": "prep-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 300

Tokens: 2000

## Commands

- `git diff --check -- .csdlc/issues/5007 .csdlc/prepared/issues/5007 .csdlc/evidence/5007/preparation`
- `cargo run --manifest-path csdlc-v2/Cargo.toml --bin csdlc-doctor -- --repo . --issue 5007`
- `rg -n #4760|51bc5ae51b57c19dbab693af1c5a45142995f4e5|0058-memory-palace|gpt-5.5|no-deferral|COTS|PVF .csdlc/issues/5007 .csdlc/prepared/issues/5007`

## Failure Semantics

Fail closed on missing #4760 gate language, missing exact origin/main SHA, missing intended paths/COTS/budgets/PVF/rollback/no-deferral text, stale projection digests, unrecorded preparation review, `/private/tmp` use, or any ADR drafting/implementation/PR/publication/merge claim. Do not treat stale claim reconciliation or typed closeout receipts as preparation blockers.

## Handoff

Retain typed evidence before convergence.
