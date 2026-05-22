# Quality Gate Packet v0.91.3

## Scope

`WP-11` proves one bounded claim: the `v0.91.3` first-slice and demo-wave proof
surfaces can be aggregated into one reviewer-facing quality-gate lane before
Sprint 4 moves into docs review, internal review, and remediation work.

## Packet Contents

- `README.md`

## Demo Command

```bash
bash adl/tools/demo_v0913_quality_gate.sh
```

## Focused Validation

```bash
bash adl/tools/test_demo_v0913_quality_gate.sh
bash adl/tools/demo_v0913_quality_gate.sh
```

## Current Gate Dimensions

The current gate covers:

- transition manifest schema proof
- transition DAG packet proof
- evidence bundle packet proof
- merge-readiness packet proof
- ObsMem handoff packet proof
- first-proof readiness packet proof
- first-proof demo packet proof
- demo-coverage and quality-gate tracked doc surfaces

The current gate intentionally does not claim:

- docs review completion
- internal review completion
- external review completion
- release readiness
- broad repo-wide runtime validation by default

## Boundaries

- This packet is a bounded review-entry quality gate, not the full Sprint 4
  release tail.
- This packet must stay repo-relative and must not depend on local-only TBD
  notes.
- Heavy checks are optional and opt-in; the default lane should stay focused on
  the strongest current first-slice proof surfaces.
