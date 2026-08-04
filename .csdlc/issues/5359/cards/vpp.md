# Validation Planning Prompt

Template: 1.0.0

Issue: 5359

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5359/design.md

Diagram: .csdlc/prepared/issues/5359/diagram.mmd

## Selected Lanes

[
  {
    "lane": "typed-doctor-prep",
    "proof_role": "Validate the generated C-SDLC v2 #5359 preparation record and card projections.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6",
      "AC-7"
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
      "5359"
    ],
    "parallel_group": "preparation-hygiene",
    "defer_reason": null
  },
  {
    "lane": "typed-validate-prep",
    "proof_role": "Run typed validation over the issue-local preparation packet without finalizing implementation.",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "."
    ],
    "parallel_group": "preparation-hygiene",
    "defer_reason": null
  },
  {
    "lane": "routing-source-hygiene",
    "proof_role": "Confirm #5359/WP-22 and #5355/WP-21A routing references remain present in the checked-in v0.91.8 sources and issue-local design.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "rg",
      "-n",
      "5359|WP-22|5355|WP-21A|release-tail",
      "docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml",
      "docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md",
      ".csdlc/prepared/issues/5359/design.md",
      ".csdlc/prepared/issues/5359/diagram.mmd"
    ],
    "parallel_group": "preparation-hygiene",
    "defer_reason": null
  },
  {
    "lane": "path-scope-hygiene",
    "proof_role": "Verify preparation changes remain limited to the protected #5359 surfaces.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "--",
      ".csdlc/issues/5359",
      ".csdlc/prepared/issues/5359",
      ".csdlc/evidence/5359",
      ".csdlc/locks/5359.lock"
    ],
    "parallel_group": "preparation-hygiene",
    "defer_reason": null
  },
  {
    "lane": "future-predecessor-live-gate",
    "proof_role": "At execution time, verify #5355 closure, closing PR, merge SHA, and ancestry before reviewing v0.92 inputs.",
    "acceptance_ids": [
      "AC-2",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "gh",
      "issue",
      "view",
      "5355",
      "--json",
      "state,closedAt,closedByPullRequestsReferences,url"
    ],
    "parallel_group": "future-execution-gate",
    "defer_reason": "Future execution only; 2026-08-04 live check shows #5355 open with no closing PR references."
  },
  {
    "lane": "future-review-packet-hygiene",
    "proof_role": "At execution time, verify the WP-22 review packet contains blocker, stale-assumption, overclaim, non-claim, and WP-23 disposition sections.",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "rg",
      "-n",
      "blocker|stale assumption|overclaim|non-claim|WP-23",
      ".csdlc/evidence/5359"
    ],
    "parallel_group": "future-execution-gate",
    "defer_reason": "Future execution only; the WP-22 review packet is intentionally not authored during preparation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5359`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate --root .`
- `rg -n 5359|WP-22|5355|WP-21A|release-tail docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md .csdlc/prepared/issues/5359/design.md .csdlc/prepared/issues/5359/diagram.mmd`
- `git diff --check -- .csdlc/issues/5359 .csdlc/prepared/issues/5359 .csdlc/evidence/5359 .csdlc/locks/5359.lock`
- `gh issue view 5355 --json state,closedAt,closedByPullRequestsReferences,url`
- `rg -n blocker|stale assumption|overclaim|non-claim|WP-23 .csdlc/evidence/5359`

## Failure Semantics

Fail closed on missing live predecessor merge, stale ancestry, missing review inputs, or unsupported v0.92 claims.

## Handoff

Retain typed evidence before convergence.
