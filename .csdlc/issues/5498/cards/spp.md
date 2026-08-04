# Structured Planning Prompt

Template: 1.0.0

Issue: 5498

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare and review the narrow transport contract, hold product scope behind exact #5499 and #5349 terminal ancestry gates, then later implement a small async typed adapter using maintained COTS primitives.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Render, validate, and independently review all six cards, design, diagram, dependencies, protected paths, COTS, budgets, and PVF lanes",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Wait fail-closed for #5499 and final WP-09 gate #5349 merged typed closeout and ancestry before amending product scope",
    "acceptance_ids": [
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the typed task transport adapter, bounded context envelopes, idempotency ledger, sanitized observations, and explicit stop/escalation contracts",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused and full proof, exact-revision review, typed publication, serialized merge, post-merge validation, and closeout",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- no tracked work on main
- no product implementation before #5499 and #5349 are merged typed closed_out
- preparation claim protects issue-local lifecycle paths only
- planned product path adl-v2/crates/adl-workcell-task-adapter is disjoint from #5499 conductor, #5500 dashboard, and #5502 convergence paths
- no Runtime v2 edits or dependencies
- no AWS, provider, network, or live task execution during preparation
- no autonomous merge, publication, closeout, issue creation, or scope widening
- no private transcript body or secret in retained repository evidence

## Risks

- the adapter could accidentally become a second conductor or lifecycle store
- retries could duplicate task creation, messages, handoffs, or cancellation
- private transcripts or credentials could leak into retained evidence
- stale ownership or task identity could let one issue control another task
- transport status could be mistaken for reviewed lifecycle truth
- the adapter could regrow before its interface stabilizes

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5498/retained/design.md

Digest: a2a956ab9bd05655312a28393c2f4b49b37f3cc3801d66acb84ee7ff9ed6a46f

## Diagram

.csdlc/issues/5498/retained/diagram.mmd

Digest: f4467640388701e91213a0208cb96f71e0580910601f43a09aedfbb71cd5d0e1

## Stop Conditions

- #5499 or #5349 lacks a merged typed closeout receipt ancestral to the execution base
- a planned product path overlaps another active typed claim
- an operation lacks exact issue, claim, branch, worktree, path, dependency, freshness, or output binding
- a request is duplicate with conflicting content or a task owner is stale or ambiguous
- implementation requires private transcript retention or lifecycle authority
- the LoC, test, time, or direct-dependency budget is exceeded without reviewed typed exception

## Handoff

Proceed only after doctor readiness.
