# v0.91.3 Quality Gate

## Status

Active quality gate for the opened `v0.91.3` issue wave.

## Primary Run Path

```bash
bash adl/tools/demo_v0913_quality_gate.sh
```

Primary tracked review surface:

- `docs/milestones/v0.91.3/review/quality_gate/QUALITY_GATE_PACKET_v0.91.3.md`

## Current Gate Dimensions

The current gate is a reviewer-facing aggregation lane over the strongest
already-landed first-slice proof surfaces.

Required validation categories:

- transition manifest schema and fixture validation
- transition DAG packet validation
- evidence bundle packet validation
- merge-readiness packet validation
- ObsMem handoff packet validation
- first-proof readiness packet validation
- first-proof demo packet validation and deterministic replay
- demo-coverage and quality-gate tracked doc-surface checks

Heavy checks remain opt-in:

- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`

The default quality-gate lane should stay focused and should not silently turn
into a broad runtime test cycle.

## Review Gate

Before release-tail closeout:

- docs review must inspect the reviewer-facing milestone package
- internal review must inspect code, docs, tests, and process artifacts
- review findings must be fixed or explicitly routed
- sprint and WP closeout cards must match GitHub and PR truth
- the proof demo must state what it proves and does not prove

## Blockers

The milestone is blocked if:

- the bounded `v0.91.3` quality-gate lane fails
- first-slice manifest or evidence omits material actor/role references or
  overclaims full actor-standing enforcement before `v0.91.4`
- `SRP` semantics drift back to Structured Review Policy
- `SOR`s overclaim merge or closeout truth
- the demo bypasses issue/PR/CI/human review discipline
- the milestone depends on local-only `.adl/docs/TBD` notes as canonical
  planning evidence
- trace/proof references are absolute, local-only, or incompatible with
  future signed-trace bundles
- `v0.91.3` claims full C-SDLC adoption instead of first-slice proof

## Non-Claims

This gate does not by itself prove:

- docs review completion
- internal review completion
- external review completion
- release readiness
- broad repo-wide runtime validation
