# Validation Planning Prompt

Template: 1.0.0

Issue: 5355

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5355/design.md

Diagram: .csdlc/prepared/issues/5355/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-doctor-5355",
    "proof_role": "Validate the typed C-SDLC v2 issue record, card projections, claim, and design/diagram digests for #5355.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5355"
    ],
    "parallel_group": "wp21a-prep-local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-validate-request-5355",
    "proof_role": "Run the declared request-driven typed validation profile for the #5355 preparation packet.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "--request",
      ".csdlc/prepared/issues/5355/validate-prep-request.json"
    ],
    "parallel_group": "wp21a-prep-local",
    "defer_reason": null
  },
  {
    "lane": "predecessor-live-merge-ancestry",
    "proof_role": "Confirm #5362 is closed by a live merged PR and its observed merge commit is ancestral to refreshed origin/main and the exact #5355 execution base.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "gh",
      "issue/pr live-state plus git merge-base --is-ancestor checks"
    ],
    "parallel_group": "wp21a-execution-gate",
    "defer_reason": "Execution-time gate; current preparation observed #5362 open, so this lane is intentionally blocked before later execution."
  },
  {
    "lane": "focused-docs-yaml-diff-hygiene",
    "proof_role": "Validate canonical v0.91.8 docs/YAML/link surfaces touched by future WP-21A work and verify diff hygiene.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "git",
      "diff --check plus focused docs/YAML/link checks for touched WP-21A paths"
    ],
    "parallel_group": "wp21a-prep-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5355`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate --root . --request .csdlc/prepared/issues/5355/validate-prep-request.json`
- `gh issue/pr live-state plus git merge-base --is-ancestor checks`
- `git diff --check plus focused docs/YAML/link checks for touched WP-21A paths`

## Failure Semantics

Fail closed on missing live predecessor merge, stale ancestry, missing canonical docs, or unsupported handoff claims.

## Handoff

Retain typed evidence before convergence.
