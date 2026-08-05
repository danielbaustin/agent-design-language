# Issue 5833 Design: Birth Witnesses And Citizen Receipt

## Outcome And Sources

Define WP-15 witness and receipt contracts from `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`, the #4762 retained handoff under `docs/milestones/v0.91.8/review/v092_handoff_4762/`, and existing anti-equivocation/private-state witness contracts in `adl/src/runtime_v2/private_state_witness.rs`.

## Owned Surface

Candidate protected paths are the witness feature contract, narrowly named Runtime witness/receipt schemas and validators, matching fixtures/tests, and `.csdlc/evidence/5833/`. The witness set binds witness identity/role, observed evidence digest, decision, time/sequence anchor, signature or integrity reference, and redaction policy. The receipt explains accepted evidence, caveats, rejection reasons, and claim boundary to the subject without exposing private state.

## Contract

Witnesses must be distinct where policy requires, bind the exact birthday candidate digest, and agree on the reviewed evidence set. Missing, duplicate, stale, forged, equivocal, unauthorized, or candidate-mismatched witnesses fail closed. The receipt is deterministically derived from the validated decision and cannot claim a birth while `birth_event_status` remains `not_claimed`.

## Dependencies And Invariants

WP-09/#5826 through WP-13/#5830 must be terminal as required by sprint gate 3, and #4762 evidence remains an input rather than current birth proof. Receipts are review surfaces, not authority substitutes.

## Validation And Rollback

Focused schema/fixture tests prove valid witness sets and deterministic receipts. Negative/security/privacy lanes cover equivocation, duplicate identity, stale digest, missing authority, forged integrity ref, redaction leakage, and premature birth claims. Rollback removes new schemas while retaining #4762 and all emitted audit evidence unchanged.

## Non-Goals

Public launch, birthday decision ownership, legal attestation, citizenship, governance authority, raw private-state disclosure, and rewriting #4762 are excluded.
