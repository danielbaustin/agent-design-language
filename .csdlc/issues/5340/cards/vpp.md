# Validation Planning Prompt

Template: 1.0.0

Issue: 5340

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5340/retained/design.md

Diagram: .csdlc/issues/5340/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Run the typed preparation tool-cache and network-denied contract DAG; prove card/design integrity, exact dependency and scope gates, budgets, PVF classes, rollback, and root safety",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5340/validate-preparation.sh"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "engine-cache-warm",
    "proof_role": "Optional controlled-external setup that fetches only the exact lock closure into FastWork and is not validation proof",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      ".csdlc/prepared/issues/5340/validate-engine.sh",
      "warm-cache"
    ],
    "parallel_group": "engine-setup",
    "defer_reason": "Run only after the #5338 gate opens when the exact locked closure is not already cached; this setup is never acceptance evidence"
  },
  {
    "lane": "engine-focused",
    "proof_role": "Prove state transitions, canonical dispatch, joins, bounded retry/failure/cancellation, saturation, typed ports, and fail-closed protocol behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 120,
    "budget_tokens": 6500,
    "argv": [
      ".csdlc/prepared/issues/5340/validate-engine.sh",
      "focused"
    ],
    "parallel_group": "engine-local",
    "defer_reason": "Execute only after #5338 is GitHub merged and retained typed closed_out with exact ancestry and the engine crate exists"
  },
  {
    "lane": "engine-quality",
    "proof_role": "Prove formatting/lint quality plus structural absence of filesystem, network, process, thread, clock, environment, async, unsafe, Runtime, and adapter authority",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3500,
    "argv": [
      ".csdlc/prepared/issues/5340/validate-engine.sh",
      "quality"
    ],
    "parallel_group": "engine-local",
    "defer_reason": "Execute only after the exact #5338 dependency gate opens and implementation exists"
  },
  {
    "lane": "ordering-resume",
    "proof_role": "Prove completion permutations, saturation, cancellation, retry/join boundaries, duplicates, incompatible checkpoints, and two-process resume equivalence",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 7000,
    "argv": [
      ".csdlc/prepared/issues/5340/validate-engine.sh",
      "determinism"
    ],
    "parallel_group": "engine-determinism",
    "defer_reason": "Execute only after the exact #5338 dependency gate opens and its landed fixtures are mapped"
  },
  {
    "lane": "engine-budgets",
    "proof_role": "Enforce typed receipt/ancestry/claim gates, exact scope and COTS, forbidden source/dependencies, LoC ceilings, and complete offline validation",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4500,
    "argv": [
      ".csdlc/prepared/issues/5340/validate-engine.sh",
      "budgets"
    ],
    "parallel_group": "engine-budget",
    "defer_reason": "Execute at the exact implementation revision with measured dependency, COTS, scope, LoC, and hard-deadline evidence"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "After typed merged-publication observation, validate the captured current-main integration tree in a detached FastWork clone before closeout",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 12000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5340/validate-post-merge.rb"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "Execute only after authorized merge and typed reconcile-merged observation; it is mandatory before closeout"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/5340/validate-preparation.sh`
- `.csdlc/prepared/issues/5340/validate-engine.sh warm-cache`
- `.csdlc/prepared/issues/5340/validate-engine.sh focused`
- `.csdlc/prepared/issues/5340/validate-engine.sh quality`
- `.csdlc/prepared/issues/5340/validate-engine.sh determinism`
- `.csdlc/prepared/issues/5340/validate-engine.sh budgets`
- `ruby .csdlc/prepared/issues/5340/validate-post-merge.rb`

## Failure Semantics

Fail closed without implementation, publication, merge, closeout, or prune on a false or stale #5338 dependency signal; unreviewed landed API drift; nondeterministic ordering; unbounded work; impossible join wait; retry or resume budget reset; ambiguous, duplicate, or mismatched port completion; in-flight checkpoint; incompatible resume; silent #5338 fixture skip; Runtime/adaptor/IO authority leak; forbidden dependency or path; unsupported LoC/time variance; stale review; red CI; absent merge authorization; missing post-merge proof; conflicting terminal receipt; or unsafe prune.

## Handoff

Retain typed evidence before convergence.
