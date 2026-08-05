# Issue 5833 Design: Birth Witnesses And Citizen Receipt

## Outcome And Sources

Define WP-15 Runtime v3 witness and receipt contracts from `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`, the #4762 retained handoff under `docs/milestones/v0.91.8/review/v092_handoff_4762/`, and current private-state authority in `adl-runtime-kernel/src/private_state.rs`. Retained `adl/src/runtime_v2/private_state_witness.rs` is read-only compatibility evidence.

## Owned Paths

The complete writable protected-path set is:

- `adl-runtime-kernel/src/birth_witness.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/birth_witness.rs`
- `adl-runtime-kernel/tests/fixtures/birth_witness/`
- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`
- `.csdlc/prepared/issues/5833/validate-native-receipts.rb`
- `.csdlc/evidence/5833/`

`adl-runtime-kernel/src/lib.rs` is limited to module registration. Private-state, Runtime v2, and #4762 paths are read-only authorities. The witness set binds witness identity/role, observed evidence digest, decision, time/sequence anchor, signature or integrity reference, and redaction policy.

## Contract

Witnesses must be distinct where policy requires, bind the exact birthday candidate digest, and agree on the reviewed evidence set. Missing, duplicate, stale, forged, equivocal, unauthorized, or candidate-mismatched witnesses fail closed. The receipt is deterministically derived from the validated decision and cannot claim a birth while `birth_event_status` remains `not_claimed`.

## Dependencies And Invariants

WP-09/#5826 through WP-13/#5830 must be terminal as required by sprint gate 3, and #4762 evidence remains an input rather than current birth proof. The canonical wave row and live issue must add WP-13 before execution so they exactly match this stricter card contract. Receipts are review surfaces, not authority substitutes.

## Validation And Rollback

The exact `birth_witness` Runtime v3 integration-test target must run a nonzero count proving valid witness sets, deterministic receipts, and equivocation, duplicate identity, stale digest, missing authority, forged integrity ref, redaction leakage, and premature-birth rejection. Native Linux CI and a retained native macOS receipt bind the exact source SHA, test argv, fixture-tree digest, output digest, runner identity, and recomputed native artifact digest before portability is claimed. Rollback removes new schemas while retaining #4762 and all emitted audit evidence unchanged.

## Non-Goals

Public launch, birthday decision ownership, legal attestation, citizenship, governance authority, raw private-state disclosure, and rewriting #4762 are excluded.
