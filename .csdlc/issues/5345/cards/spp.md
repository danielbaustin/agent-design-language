# Structured Planning Prompt

Template: 1.0.0

Issue: 5345

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the thin command boundary, selector/installer transaction and rollback contract, exact dependency gate, COTS closure, protected paths, LoC/test/module/time budgets, PVF classes, no-deferral matrix, and rollback during preparation; bind and review the packet; wait for all six upstream WPs to merge and close out; then implement, prove, review, publish, merge, post-merge validate, and close out without widening authority.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Complete all six typed cards, design, diagram, exact dependency and protected-path gates, COTS, budgets, PVF, no-deferral, rollback, preparation validation, bind, bounded review, durable commit, and push without product implementation",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Maintain a read-only dependency watch and begin implementation only after all six upstream issues are merged, typed closed_out, receipt-backed, and ancestral",
    "acceptance_ids": [
      "AC-7",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the thin typed CLI, stable installer, authoritative selector, atomic selection transaction, deterministic receipts, and explicit verified rollback primitives",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run the complete offline command, selector, installer, interruption, concurrency, rollback, dependency, module, LoC, test-count, and time proof matrix and fix every finding",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Obtain exact-revision review, publish through typed v2, shepherd green required checks, merge only under authorization, run detached post-merge proof, close out, retain the terminal receipt, and release the claim",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- The CLI remains an adapter and never becomes a second implementation of upstream ADL semantics
- No command implicitly performs network access, reads credentials, selects a generation, or mutates selector state
- Every selector mutation verifies exact installation identity, holds an exclusive lock, uses compare-and-swap, persists atomically, and re-reads before success
- Every selector failure preserves the previous record byte-for-byte
- Rollback is explicit, verified, receipt-bearing, and uses the same atomic transaction as selection
- Default cutover, soak, rollback acceptance, Runtime, provider, governed-tool, and lifecycle authority remain outside WP-10
- Runtime v2 and incumbent ADL remain untouched behavioral evidence
- No implementation begins before every upstream dependency is terminal and ancestral

## Risks

- A convenient command handler can silently duplicate parser, compiler, engine, signing, provider, or Runtime behavior
- Selector state can be corrupted or rolled back incorrectly under stale writers, lock contention, interruption, or partial persistence
- Installer and selector receipts can drift from the installed executable or expose machine-local paths
- A default or fallback can accidentally grant cutover authority before #5343/#5344 review
- Cross-platform filesystem semantics can weaken atomicity, permissions, or lock guarantees
- Dependency convenience can regrow the CLI or import network, async, cloud, Runtime, or lifecycle authority
- The landed WP-04 through WP-09 APIs or lock closure may differ from this provisional adapter contract
- Budget targets can pressure proof quality unless variance is explicitly reviewed instead of hidden

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5345/design.md

Digest: 44cb2d60e112dc5269dfd511762399121a61e66a8f6ccfca8868afdf65581d96

## Diagram

.csdlc/prepared/issues/5345/diagram.mmd

Digest: c851e114b6ff79258a1acb435981e8b1604e27926081797a9d6550f3571e2320

## Stop Conditions

- Any upstream dependency is not merged, typed closed_out, receipt-backed, or ancestral when product implementation would begin
- The landed upstream API or lock closure differs materially from the reviewed thin-adapter contract
- Any command requires duplicate domain logic or undeclared network, credential, selector, Runtime, or lifecycle authority
- Selector mutation cannot preserve exact prior bytes on every failure or cannot remain atomic and compare-and-swap protected
- Rollback cannot verify the previous installed generation through the same transaction
- A proposed dependency escapes the reviewed COTS and upstream ADL v2 closure
- Implementation, test, module, or validation-time variance lacks exact evidence-backed review
- Required acceptance or validation would be deferred, skipped, or replaced by metadata-only proof

## Handoff

Proceed only after doctor readiness.
