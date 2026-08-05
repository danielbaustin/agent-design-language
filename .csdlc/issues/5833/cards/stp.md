# Structured Task Prompt

Template: 1.0.0

Issue: 5833

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver only WP-15 witness and receipt schemas, validators, fixtures, security/privacy negatives, and retained exact-revision evidence.

## Deliverables

- Versioned witness-set schema and validator
- Deterministic accepted/rejected citizen receipt contract
- Equivocation, duplicate, stale, forged, mismatch, authority, and redaction fixtures
- Retained focused, security, privacy, and premature-claim evidence

## Acceptance

1. The WP-15 validator accepts only policy-complete distinct witnesses bound to the exact candidate/evidence digest and derives a deterministic caveated receipt from the validated decision.
2. WP-09/#5826 through WP-13/#5830 and retained #4762 evidence are verified before implementation.
3. Implementation is confined to adl-runtime-kernel/src/birth_witness.rs, lib.rs module registration, tests/birth_witness.rs, tests/fixtures/birth_witness/, the feature contract, and .csdlc/evidence/5833/.
4. Equivalent witness sets and decisions produce canonical exact-revision validation and receipt output.
5. Missing, duplicate, stale, forged, equivocal, unauthorized, candidate-mismatched, redaction-leaking witnesses and premature birth claims fail closed.
6. One bounded exact-head SRP review records no unresolved actionable findings.
7. The implementation PR targets the intended base and includes Closes #5833 without claiming completion of downstream Birthday work.
8. Before claim acquisition, reconcile the live issue and canonical wave with the stricter sprint/card dependency on WP-13/#5830; WP-13 remains required.
9. The exact birth_witness nextest target runs at least one test with --no-tests=fail, and native macOS/Linux receipts bind the same fixture digest before portability is claimed.

## Dependencies

- WP-09 / issue #5826 terminal proof
- WP-10 / issue #5827 terminal proof
- WP-11 / issue #5828 terminal proof
- WP-12 / issue #5829 terminal proof
- WP-13 / issue #5830 terminal proof
- Retained issue #4762 witness handoff

## Inputs

- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
- docs/milestones/v0.91.8/review/v092_handoff_4762/
- adl-runtime-kernel/src/private_state.rs
- adl/src/runtime_v2/private_state_witness.rs (read-only compatibility evidence only)

## Non Goals

- Birthday decision ownership, public launch, legal attestation, citizenship, or governance authority
- Raw private-state disclosure or treating a receipt as authority
- Rewriting #4762 or prior witness evidence
