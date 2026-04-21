# Recovery And Quarantine

## Purpose

Define how Runtime v2 decides whether a failed state can safely resume or must
be quarantined.

## Recovery Allowed When

- the failed action did not mutate committed state
- trace and snapshot evidence are sufficient
- identity and temporal invariants remain valid
- operator policy allows resume

## Quarantine Required When

- committed state may be inconsistent
- identity may have forked or duplicated
- replay evidence is missing
- security-boundary violation risk remains unresolved

## Required Output

- recovery eligibility model
- safe-resume decision record
- reject/quarantine-required decision record
- quarantine artifact
- negative tests for unsafe resume

## WP-11 Recovery Eligibility

WP-11 lands the D8 eligibility boundary. The code-backed recovery model consumes
the WP-08 invalid-action rejection evidence and the WP-09 snapshot/rehydration
wake-continuity evidence, then emits two deterministic decision records:

- `runtime_v2/recovery/safe_resume_decision.json` allows resume only when the
  predecessor snapshot is declared, the wake proof preserves one active head,
  and all eligibility rules pass.
- `runtime_v2/recovery/quarantine_required_decision.json` refuses resume when
  recovery would create ambiguous predecessor linkage or duplicate active-head
  risk, and hands the unsafe case to WP-12.

This is a decision model, not the quarantine state machine. WP-12 still owns the
quarantine artifact, evidence-preservation state, and release path from
quarantine.

## WP-12 Quarantine State Machine

WP-12 lands the D8 quarantine boundary. The quarantine artifacts consume the
WP-11 quarantine-required decision and prove that unsafe recovery does not
silently resume:

- `runtime_v2/quarantine/unsafe_recovery_fixture.json` preserves the ambiguous
  unsafe resume attempt that must not be accepted as recovery.
- `runtime_v2/quarantine/quarantine_artifact.json` records the bounded
  quarantine state machine: accept the quarantine-required decision, preserve
  evidence, then block execution pending operator review.
- `runtime_v2/quarantine/evidence_preservation_artifact.json` lists the
  immutable evidence set that must not be pruned before review.

Quarantine does not release the state back to active execution by itself. Release
requires an operator review record, a new recovery eligibility decision, and
verified preserved evidence.

## Validation

Focused validation:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_recovery_eligibility -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_quarantine -- --nocapture
```

The tests cover golden fixtures, path hygiene, safe-resume polarity,
reject/quarantine polarity, complete rule evaluation, unsafe attempt rejection,
quarantine transition ordering, immutable evidence preservation, and
live-run/no-birthday overclaiming.
