# Structured Planning Prompt

Template: 1.0.0

Issue: 5357

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Render and validate six cards; freeze exact #5356 terminal gating, issue-local ownership, canonical handoff identity, corpus/receipt schemas, reviewer boundaries, findings synthesis, COTS, budgets, PVF, redaction and rollback; obtain bounded review and fixes; typed approve/bind/doctor; commit and push preparation only.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Reconcile current main, WP-17 inventory, first-pass review, residual coding, live #5791 ownership, and amend the exact non-overlapping #5357 document claim",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Audit and update every claimed canonical v0.91.8 review document and the undispatched external handoff; classify all remaining corpus documents as current or historical",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run exhaustive existence, link, YAML/JSON, stale-truth, redaction, diff, and typed validation; obtain one bounded exact-revision GPT-5.5 documentation review and fix findings",
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
    "id": "S4",
    "action": "Commit and push the reviewed #5357 documentation candidate, then wait for #5791 merge, integrate current main, revalidate exact corpus truth, and leave Target Revision/digest unset until operator freeze",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- #5357 edits only exact canonical review documents and issue-local support paths in its active claim
- #5791 retains sole ownership of its three concurrent release-tail pages until merge; #5357 integrates those pages from main rather than duplicating them
- all tracked documents under docs/milestones/v0.91.8 are included in the external-review digest corpus even when the human manifest groups rather than repeats them
- the first WP-18 review remains historical evidence and cannot approve residual coding; #5791 is the final internal-review authority before WP-19 freeze
- external review remains undispatched and cannot mutate lifecycle state, approve release, or substitute for product proof
- Runtime v2, product changes, dependencies, AWS, credentials, paid services, host paths, private prompts, and raw provider output are forbidden
- one final bounded documentation review occurs after the complete sweep and before the branch is declared ready to freeze

## Risks

- a model selected and prompted by the project could be mislabeled independent
- corpus or implementation proof could drift after dispatch
- a self-hashing or mutable receipt could make review identity unverifiable
- reviewer assertions could be mistaken for observed code evidence
- redaction could remove context or leak sensitive data
- findings could trigger scope sprawl or one-issue-per-finding churn

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5357/design.md

Digest: 0a2cedc5c4b91bb7704ac5e240b38bfabdd1fd8b63e95e7875939231897321a3

## Diagram

.csdlc/prepared/issues/5357/diagram.mmd

Digest: 4868b7e6e805ebf6cbf9326d2aad6e502b7fd52685afd1338b550bd5b5959270

## Stop Conditions

- #5791 has not merged its final internal second pass after residual coding or its merge is not ancestral to the proposed external-review target
- a canonical v0.91.8 document is missing, stale against current issue truth, structurally invalid, unlinked, or outside the digest corpus
- a requested edit overlaps #5791 or another active claim instead of integrating its merged result from main
- the handoff target SHA, PR, corpus digest, reviewer identity, independence, prompt, provider/model, timing, outcome, or output digest cannot be retained truthfully
- evidence contains secrets, private prompts, credentials, host paths, raw provider payloads, personal data, or unverifiable assertions
- a required proof is deferred or substituted, the exact documentation review is stale or has open findings, or the change requires product behavior, Runtime v2, AWS, paid services, a dependency, or out-of-claim writes
- external review would be dispatched before the operator freezes the exact revision and explicitly starts that lane

## Handoff

Proceed only after doctor readiness.
