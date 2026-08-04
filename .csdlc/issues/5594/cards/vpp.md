# Validation Planning Prompt

Template: 1.0.0

Issue: 5594

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5594/retained/design.md

Diagram: .csdlc/issues/5594/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "structured-planning",
    "proof_role": "Parse and assert v0.91.8 YAML and JSON topology",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5594/validate_structured_planning.rb"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  },
  {
    "lane": "feature-crosswalk",
    "proof_role": "Pin and classify every canonical ADL feature row with named cutover owners and explicit per-row dispositions",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5594/validate_feature_crosswalk.rb"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  },
  {
    "lane": "local-links",
    "proof_role": "Verify local links across touched v0.91.8 documents",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5594/validate_links.rb"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  },
  {
    "lane": "live-routing",
    "proof_role": "Verify exact live issue labels, parents, dependencies, umbrellas, inventory coverage, and malformed-body repair through ADL owner binaries",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5594/validate_live_routing.rb"
    ],
    "parallel_group": "live-read-only",
    "defer_reason": null
  },
  {
    "lane": "diff-and-lifecycle",
    "proof_role": "Verify full branch diff hygiene against origin/main plus protected scope and typed lifecycle consistency",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "local-control",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5594/validate_structured_planning.rb`
- `ruby .csdlc/prepared/issues/5594/validate_feature_crosswalk.rb`
- `ruby .csdlc/prepared/issues/5594/validate_links.rb`
- `ruby .csdlc/prepared/issues/5594/validate_live_routing.rb`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on stale live truth, missing or overlapping sprint ownership, incomplete cards or contracts, write-scope collisions, unsupported readiness claims, scope expansion, AWS use, or raw gh use.

## Handoff

Retain typed evidence before convergence.
