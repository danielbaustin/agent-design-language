# Validation Planning Prompt

Template: 1.0.0

Issue: 5344

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5344/design.md

Diagram: .csdlc/prepared/issues/5344/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove all six cards, design/diagram, scope, protected paths, dependency predicates, COTS, budgets, PVF, no-deferral, rollback invariants, and root safety",
    "acceptance_ids": [
      "AC-1",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5344/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "dependency-terminal-gate",
    "proof_role": "Fail closed unless #5350 and #5361 are merged, typed closed_out, receipt-backed, claim-free, and ancestral",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5344/check-dependencies.rb"
    ],
    "parallel_group": "execution-gate",
    "defer_reason": "Run only when execution is requested; it must fail closed until both dependencies are terminal and ancestral"
  },
  {
    "lane": "rollback-fault-matrix",
    "proof_role": "Prove isolated-root opt-in, prior-byte preservation, locked CAS selection, successful/failed selection, failed soak, explicit rollback, and exact restoration",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5344/run-validation-lane.rb",
      "rollback-fault-matrix"
    ],
    "parallel_group": "rollback",
    "defer_reason": "Execute after dependency closure and harness implementation; mandatory before publication"
  },
  {
    "lane": "representative-soak",
    "proof_role": "Run the frozen local, CI, Runtime v3, provider-disposition, demo, negative, and rollback scenario manifest and retain normalized evidence",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5344/run-validation-lane.rb",
      "representative-soak"
    ],
    "parallel_group": "soak",
    "defer_reason": "Execute after dependency closure and harness implementation; mandatory before publication"
  },
  {
    "lane": "soak-budgets-and-evidence",
    "proof_role": "Enforce COTS closure, no forbidden dependencies, LoC/module/test/time limits, deterministic evidence schema, redaction, repo-relative paths, test count, and no-deferral",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 3600,
    "budget_tokens": 7000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5344/run-validation-lane.rb",
      "soak-budgets-and-evidence"
    ],
    "parallel_group": "evidence",
    "defer_reason": "Execute at the exact implementation revision after all proving scenarios; mandatory before publication"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Re-run dependency ancestry, rollback, representative soak, evidence integrity, budgets, exact revision identity, and #5343 handoff gate after authorized merge",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 3600,
    "budget_tokens": 12000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5344/run-validation-lane.rb",
      "post-merge-exact"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "Execute only after authorized merge and typed merged-state reconciliation; mandatory before closeout"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `ruby .csdlc/prepared/issues/5344/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5344/check-dependencies.rb`
- `ruby .csdlc/prepared/issues/5344/run-validation-lane.rb rollback-fault-matrix`
- `ruby .csdlc/prepared/issues/5344/run-validation-lane.rb representative-soak`
- `ruby .csdlc/prepared/issues/5344/run-validation-lane.rb soak-budgets-and-evidence`
- `ruby .csdlc/prepared/issues/5344/run-validation-lane.rb post-merge-exact`

## Failure Semantics

Fail closed without soak, selector mutation, publication, cutover, merge, #5343 handoff, deletion, or closeout on an incomplete dependency gate, claim collision, non-isolated root, direct selector edit, prior-byte damage, rollback mismatch, stale or invalid receipt, hidden network/credential/AWS use, Runtime v2 edit, secret or host-path disclosure, overclaim, budget violation, deferred acceptance, stale review, red CI, or absent post-merge proof.

## Handoff

Retain typed evidence before convergence.
