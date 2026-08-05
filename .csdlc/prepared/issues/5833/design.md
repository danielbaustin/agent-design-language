# Issue 5833 Design: Birth Witnesses And Citizen Receipt

## Outcome And Sources

Define WP-15 Runtime v3 witness and receipt contracts from `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`, the #4762 retained handoff under `docs/milestones/v0.91.8/review/v092_handoff_4762/`, and current private-state authority in `adl-runtime-kernel/src/private_state.rs`. Retained `adl/src/runtime_v2/private_state_witness.rs` is read-only compatibility evidence.

## Owned Surface

Protected implementation paths are `adl-runtime-kernel/src/birth_witness.rs`, `adl-runtime-kernel/src/lib.rs` (module registration only), `adl-runtime-kernel/tests/birth_witness.rs`, `adl-runtime-kernel/tests/fixtures/birth_witness/`, `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`, and `.csdlc/evidence/5833/`. The witness set binds witness identity/role, observed evidence digest, decision, time/sequence anchor, signature or integrity reference, and redaction policy. The receipt explains accepted evidence, caveats, rejection reasons, and claim boundary to the subject without exposing private state.

## Contract

Witnesses must be distinct where policy requires, bind the exact birthday candidate digest, and agree on the reviewed evidence set. Missing, duplicate, stale, forged, equivocal, unauthorized, or candidate-mismatched witnesses fail closed. The receipt is deterministically derived from the validated decision and cannot claim a birth while `birth_event_status` remains `not_claimed`.

## Dependencies And Invariants

WP-09/#5826 through WP-13/#5830 must be terminal as required by sprint gate 3, and #4762 evidence remains an input rather than current birth proof. The canonical issue row currently omits the stricter WP-13 gate carried by the sprint gate; pre-execution dependency reconciliation must align the live issue, canonical wave, and cards before claim acquisition, preserving WP-13 as required. Receipts are review surfaces, not authority substitutes.

## Validation And Rollback

The exact `birth_witness` Runtime v3 integration-test target must run a nonzero count proving valid witness sets, deterministic receipts, and equivocation, duplicate identity, stale digest, missing authority, forged integrity ref, redaction leakage, and premature-birth rejection. Native Linux CI and a retained native macOS receipt use the same fixture digest before portability is claimed. Rollback removes new schemas while retaining #4762 and all emitted audit evidence unchanged.

## Non-Goals

Public launch, birthday decision ownership, legal attestation, citizenship, governance authority, raw private-state disclosure, and rewriting #4762 are excluded.
