# Structured Planning Prompt

Template: 1.0.0

Issue: 5501

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Render and validate six cards; freeze the real live-proof boundary, exact dependency gates, preparation-only ownership, zero-new-dependency COTS posture, budgets, PVF, evidence identity, negative case, dashboard, convergence, serialized integration, and fair baseline contracts; obtain bounded review and fix findings; commit and push preparation only.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Complete six cards, design, diagram, dependency and preparation validators, exact protected paths, COTS, budgets, and PVF without live, product, PR, or additional preparation-review work",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Wait fail-closed for #5349, #5499, #5498, #5500, and #5502 live merged heads to be ancestral to the execution revision; typed closeout and retained receipts are audit-only signals, not readiness blockers",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Validate the admitted live-run manifest and execute at least two real disjoint writable Codex shards with bounded provenance-bearing context, negative-case refusal, dashboard observations, output review, and convergence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run the equivalent bounded single-agent baseline, compare timing and coordination overhead, retain exact proof, complete serialized review/publication/checks/merge/post-merge validation/closeout, and hand evidence to #5497, #5350, and #5361",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- no live or product work before every execution dependency has live merge plus ancestry truth
- typed closeout state and retained receipts are audit-only and do not block readiness when live merge plus ancestry truth is satisfied
- preparation claim protects only issue-local lifecycle and evidence paths
- at least two real writable shards have distinct issue claims, branches, worktrees, and disjoint protected paths
- fixture, mock, prose, screenshot, or library-only evidence cannot satisfy live proof
- context and outputs are bounded, redaction-safe, provenance-bearing, exact-revision continuous, and stale-detectable
- dashboard status is observed, never manually asserted
- review, publication, merge, post-merge validation, and closeout remain serialized and independently authorized
- all applicable acceptance and PVF lanes complete without deferral

## Risks

- a staged demo could be mistaken for a real distributed workcell
- task or context identity could drift between conductor admission and shard execution
- overlapping claims or write sets could corrupt parallel output
- dashboard projections could present manually asserted or stale green status
- convergence summaries could hide failures, residual blockers, or review gaps
- the single-agent baseline could be incomparable or timings could omit coordination overhead
- the proof harness could grow into a second scheduler or lifecycle store

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5501/design.md

Digest: 4909f3533378d71d43cac913e87bfa7047e8aa08e6c279087ea5273116f03e7f

## Diagram

.csdlc/prepared/issues/5501/diagram.mmd

Digest: 38aa0234f183017794bea187c7d5664d236c9c2ee40094c89c74834549816031

## Stop Conditions

- any execution dependency lacks a live merged head ancestral to the execution revision
- fewer than two real writable shards can be admitted with disjoint typed claims and paths
- task, context, output, review, dashboard, convergence, or closeout identity cannot be proven
- the negative case or single-agent baseline cannot be executed truthfully
- execution requires out-of-claim product writes, Runtime v2, AWS, credentials, paid services, hidden authority, or deferred acceptance proof

## Handoff

Proceed only after doctor readiness.
