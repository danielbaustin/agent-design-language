# v0.91.3 Quality Gate

## Status

Planned quality gate.

## Required Validation

The milestone should run the smallest proving validation for each touched
surface, plus one combined C-SDLC lane before closeout.

Required validation categories:

- schema/fixture validation for transition manifests
- actor-role reference checks for material transition participants
- validator tests for valid and invalid lifecycle states
- DAG/shard fixture tests
- evidence bundle serialization or snapshot validation
- merge-readiness gate tests
- tracked C-SDLC source-package path checks
- trace/proof reference path checks for future signed trace promotion
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
- first-slice manifest or evidence omits material actor/role references or
  overclaims full actor-standing enforcement before v0.91.4
- SRP semantics drift back to Structured Review Policy
- SORs overclaim merge or closeout truth
- the demo bypasses issue/PR/CI/human review discipline
- the milestone depends on local-only `.adl/docs/TBD` notes as canonical
  planning evidence
- trace/proof references are absolute, local-only, or incompatible with
  v0.91.4 signed trace bundles
- v0.91.3 claims full C-SDLC adoption instead of first-slice proof
