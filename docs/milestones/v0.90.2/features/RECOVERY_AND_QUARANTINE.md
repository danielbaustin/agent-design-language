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

## Validation

Focused validation:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_recovery_eligibility -- --nocapture
```

The tests cover golden fixtures, path hygiene, safe-resume polarity,
reject/quarantine polarity, complete rule evaluation, unsafe attempt rejection,
and live-run/no-birthday overclaiming.
