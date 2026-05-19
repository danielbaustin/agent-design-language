# v0.91.3 Quality Gate

## Status

Planned quality gate.

## Required Validation

The milestone should run the smallest proving validation for each touched
surface, plus one combined C-SDLC lane before closeout.

Required validation categories:

- schema/fixture validation for transition manifests
- validator tests for valid and invalid lifecycle states
- DAG/shard fixture tests
- evidence bundle serialization or snapshot validation
- merge-readiness gate tests
- docs path/reference checks for the milestone package
- combined-lane test for shared state, env vars, fixtures, and closeout truth

## Review Gate

Before release-tail closeout:

- internal review must inspect code, docs, tests, and process artifacts
- review findings must be fixed or explicitly routed
- sprint and WP closeout cards must match GitHub and PR truth
- the proof demo must state what it proves and does not prove

## Blockers

The milestone is blocked if:

- a combined C-SDLC validation lane fails
- SRP semantics drift back to Structured Review Policy
- SORs overclaim merge or closeout truth
- the demo bypasses issue/PR/CI/human review discipline
- v0.91.3 claims full C-SDLC adoption instead of first-slice proof

