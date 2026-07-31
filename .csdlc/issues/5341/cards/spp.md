# Structured Planning Prompt

Template: 1.0.0

Issue: 5341

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

After all merged/closed_out/receipt/ancestry proofs pass, integrate current origin/main, bind once with the single disjoint adapter crate, implement the minimal deterministic public-ingress bridge, complete focused FastWork proof, run one exact GPT-5.5 review and fix pass, typed-publish, require focused green CI, self-merge the authorized head, post-merge validate, and typed-closeout with retained receipt.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Prepare and bounded-review all six cards, design, diagram, exact paths, COTS and LoC budgets, no-deferral matrix, negative authority proof, rollback, validation runner, and executable dependency gate without product edits",
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
    "status": "in_progress"
  },
  {
    "id": "S2",
    "action": "Read-only watch #5340, #5342, and #5591 until each is GitHub merged, typed closed_out, receipt-retained, and ancestral to current origin/main; report every changed state upstream",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "After the gate passes, refresh origin/main and live GitHub truth, verify sole-writer disjointness, integrate current main, reconcile exact terminal APIs and budgets, and typed-transition the claim to only adl-v2/crates/adl-runtime-v3-adapter",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement deterministic plan and engine-event mapping to the public canonical Runtime v3 ingress and exact result/error mapping into verified records without owning scheduling, retry, signing, supervision, transport, or state",
    "acceptance_ids": [
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
    "id": "S5",
    "action": "Implement every positive and negative authority case, COTS and LoC check, boundary scan, rollback proof, and complete FastWork validation with no deferral",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Commit the exact implementation, obtain one bounded exact-revision GPT-5.5 subagent review immediately before PR, fix every actionable finding, and rerun affected lanes",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Use typed review and publication, require green CI, self-merge only the authorized exact head, post-merge validate, typed-closeout with retained receipt, and guarded-prune",
    "acceptance_ids": [
      "AC-1",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- no product code until all three dependency merges are ancestral to current origin/main; closeout runs in parallel
- root main remains clean and read-only; all #5341 records and changes stay in the dedicated issue worktree
- one ADL-owned thin adapter crate and one Runtime-owned canonical ingress with no duplicate scheduler, signer, supervisor, transport, or state owner
- identical terminal accepted input yields identical adapter request identity, payload, and mapped result semantics
- Runtime saturation, closure, conflict, unsupported work, and execution failure remain failures unless the #5340 engine explicitly owns a subsequent action
- trust, provenance, digests, signatures, and verification outcomes are preserved without adapter mutation
- no Runtime v2, AWS, C-SDLC product, listener, credential, hard-coded address, deployment, selector, shared-manifest, or sibling-owner dependency or write
- every acceptance claim is exact-revision, local-FastWork-proven, reviewed, CI-green, integrated, post-merge-validated, and receipt-backed

## Risks

- pre-terminal preview contracts could be mistaken for merged interface authority
- a convenience adapter could absorb engine retry, record signing, Runtime supervision, or C-SDLC authority
- shared workspace or Cargo manifest edits could collide with active ADL-core or Runtime owners
- error mapping could convert saturation, closure, conflict, unsupported work, or execution failure into false success
- work identity or payload mapping could become nondeterministic or break idempotency across resume
- dependency or source growth could turn a thin adapter into a third runtime or execution implementation
- negative proof could become static-only and miss behavioral bypasses
- long dependency waits could stale the claim, cards, design, or origin/main assumptions

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5341/design.md

Digest: a85968cdb8685e8d95e05db20b8347223693983fe1d137c09c953cfc2e4d3b83

## Diagram

.csdlc/prepared/issues/5341/diagram.mmd

Digest: 66c62fc10f92513c0589e16b86a34b212c2f733cf08cf8a152faa3292c29da09

## Stop Conditions

- any of #5340, #5342, or #5591 is not merged and ancestral to current origin/main
- the live terminal contracts contradict the prepared mapping, authority, COTS, LoC, test, or validation assumptions and typed replanning has not completed
- sole-writer verification finds an overlapping active product claim or implementation requires any path beyond the exact adapter crate
- implementation would require Runtime v2, adl-runtime, adl-runtime-kernel, sibling ADL v2, shared manifest, C-SDLC product, AWS, listener, credential, deployment, selector, or signing/scheduler ownership edits
- any required positive, negative, COTS, budget, boundary, exact-revision, review, CI, post-merge, or closeout proof is skipped, ignored, pending, degraded, fixture-only, prose-only, CI-only, stale, or failing
- source, module, test, COTS, or 2400-second validation budgets cannot be met without weakening acceptance
- the active claim becomes stale or ownership cannot be recovered truthfully through typed v2

## Handoff

Proceed only after doctor readiness.
