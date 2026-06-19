# Scheduler Economics Inputs for #4106

## Scope

This packet records the bounded Scheduler v1 economics input model implemented
for `#4106`.

The implementation is intentionally small. It defines a typed input contract
that `#4107` can consume when building the Cognitive Scheduler v1 decision
surface.

## Implemented v1 Inputs

`adl/src/scheduler.rs` defines:

- `SchedulerEconomicsInputV1`
- `SchedulerEconomicsInputBundleV1`
- `SchedulerEconomicsSummaryV1`
- distinct schema ids for single input payloads and input-bundle payloads
- deterministic JSON/YAML parsing helpers
- bounded validation for schema version, required task identity, dependency
  posture, and claim-boundary language
- a deterministic summary/rank key that exposes lifecycle cost, dependency
  posture, confidence, attention pressure, and parallelism for `#4107`

Minimum required fields from the issue are covered:

- estimated effort
- validation cost
- coordination cost
- risk
- expected value
- urgency
- dependency posture
- parallelism potential
- premium capacity pressure
- governor attention pressure
- confidence

## Fixtures

The fixture bundle at
`adl/tests/fixtures/scheduler/economics_inputs_v1.json` includes representative
inputs for:

- low-risk docs/status work
- first-pass review work
- premium code repair
- release/governor authority
- low-urgency delayed cleanup under capacity pressure
- blocked proof work with an unresolved dependency
- partial-dependency review work where upstream evidence is not fully landed

## Included Concepts From The Cognitive Economics Plan

Included in v1:

- lifecycle cost rather than token-only cost
- validation burden
- coordination burden
- scarce premium cognition pressure
- scarce governor attention pressure
- dependency/blocker posture
- expected value as bounded ordinal input
- confidence as bounded ordinal input

## Deferred Concepts

Deferred beyond `#4106`:

- live provider/model price lookup
- measured ROI or speedup
- market or bidding behavior
- autonomous sprint conduction
- full PVF execution
- exact subjective-value or future-cost measurement

These concepts should not block `#4107`. Scheduler v1 can consume the v1
economics input shape without pretending that the harder economics surfaces are
solved.

## Validation

Focused validation for this slice:

- `cargo test --manifest-path adl/Cargo.toml scheduler_economics -- --nocapture`
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`

Publication validation:

- `pr finish` initially failed closed because the new scheduler source and
  fixture paths were not yet classified into a finish-validation lane.
- The bounded remediation classifies `adl/src/scheduler.rs` and
  `adl/tests/fixtures/scheduler/` as a scheduler-focused Rust slice and maps
  them to the same `scheduler_economics` selector.
- The classifier remediation includes
  `finish_scheduler_paths_run_scheduler_economics_focused_validation`.

## Non-Claims

This packet does not claim:

- full Cognitive Scheduler v1 implementation
- lane selection is complete
- live provider capacity visibility exists
- exact cost or value measurement
- measured sprint acceleration
- replacement of governor judgment
