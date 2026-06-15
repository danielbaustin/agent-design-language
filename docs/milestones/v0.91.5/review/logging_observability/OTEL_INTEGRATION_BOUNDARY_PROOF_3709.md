# OTEL Integration Boundary Proof (#3709)

Issue: #3709  
Parent sprint: #3703  
Captured: 2026-06-15  
Status: reviewed_docs_boundary

## Scope

This packet proves the outcome of `#3709` as a bounded boundary decision, not a
collector implementation issue.

The proof question is:

- does ADL now have a truthful, reviewable OpenTelemetry integration boundary
  that preserves deterministic local and CI behavior?

## Evidence Used

- `adl/Cargo.toml`
- `docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md`
- `docs/milestones/v0.91.5/SHARED_OBSERVABILITY_AND_OTEL_CONTRACT_3705.md`
- `docs/milestones/v0.91.5/OPEN_TELEMETRY_INTEGRATION_BOUNDARY_3709.md`
- `docs/milestones/v0.91.5/review/logging_observability/LOGGING_OBSERVABILITY_GAP_MAP_3704.md`

## What This Issue Proves

1. ADL already has the local observability foundations that OTEL would export
   from later:
   - `adl_event`
   - runtime action logs
   - provider JSONL logs
   - long-lived-agent operational ledgers
2. OTEL remains optional and disabled by default.
3. The current repo does not yet add OTEL exporter crates, and that absence is
   now intentional rather than ambiguous.
4. The future exporter path is constrained:
   - explicit feature gate
   - no hidden collector dependency
   - no CI collector requirement
   - no bypass of existing redaction policy

## What This Issue Explicitly Does Not Prove

- live OTEL collector export
- `tracing-opentelemetry` wiring
- OTLP transport behavior
- span instrumentation for every binary or subsystem

## Validation Commands

These commands were used to verify the current decision surface:

```bash
rg -n "tracing|tracing-subscriber|opentelemetry|tracing-opentelemetry|otel" adl/Cargo.toml
rg -n "OTEL|OpenTelemetry|collector|feature gate|disabled by default" docs/milestones/v0.91.5/OPEN_TELEMETRY_INTEGRATION_BOUNDARY_3709.md
rg -n "OTEL|OpenTelemetry|OTEL-ready|collector" docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md docs/milestones/v0.91.5/SHARED_OBSERVABILITY_AND_OTEL_CONTRACT_3705.md
git diff --check
```

## Validation Results

- `adl/Cargo.toml` contains `tracing` and `tracing-subscriber`
- `adl/Cargo.toml` does not currently add OTEL exporter crates
- the new boundary doc records the no-new-crates decision for this issue
- the shared observability contract remains the OTEL mapping authority
- the control-plane contract remains honest that complete OTEL export is not
  yet implemented
- `git diff --check` is the required hygiene check for this docs-boundary slice

## Residual Risk

- Because exporter crates are not added here, later implementation work must
  still prove compile cost, feature isolation, and collector behavior.
- Some existing repo docs may still say "OTEL-ready" at a high level; `#3711`
  should ensure those references point at this boundary decision and do not
  overclaim implementation.

## Reviewer Conclusion

`#3709` should be evaluated as a truthful OTEL boundary decision:

- enough to stop this issue from overclaiming OTEL implementation;
- not enough to claim live exporter support;
- not enough by itself to clean up every repo-wide "OTEL-ready" reference;
- safe for local and CI determinism today.
