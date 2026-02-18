# ADL Output Card

Task ID: issue-0313
Run ID: issue-0313
Version: v0.4
Title: run-step-progress-banners
Branch: codex/313-run-step-progress-banners
Status: DONE

Execution:
- Actor: Codex (GPT-5)
- Model: GPT-5 Codex
- Provider: local CLI
- Start Time: 2026-02-18T03:40:00Z (approx)
- End Time: 2026-02-18T03:46:00Z (approx)

## Summary
Applied a surgical trace-format fix per request: `StepFinished` trace output now reports duration in seconds with 3 decimals using `duration=<s.sss>s` instead of milliseconds (`duration_ms=<n>`).

Example:
- Before: `... StepFinished ... duration_ms=23020`
- After: `... StepFinished ... duration=23.020s`

## Artifacts produced
- Updated output card: `.adl/cards/313/output_313.md`

## Files changed
- `swarm/src/trace.rs`
- `swarm/tests/trace_tests.rs`
- `.adl/cards/313/output_313.md`

## Actions taken
- Updated `swarm/src/trace.rs` StepFinished summary formatting from `duration_ms` to `duration` in seconds with millisecond precision.
- Added helper formatter for seconds output with 3 decimal places.
- Updated trace test assertion in `swarm/tests/trace_tests.rs` to expect `duration=<s.sss>s`.
- Kept all other trace fields and ordering unchanged.

## Validation
- Tests / checks run:
  - `cd swarm && cargo fmt`
  - `cd swarm && cargo clippy --all-targets -- -D warnings`
  - `cd swarm && cargo test`
- Results:
  - `cargo fmt`: PASS
  - `cargo clippy --all-targets -- -D warnings`: PASS
  - `cargo test`: PASS

## Decisions / Deviations
- Deviation from input-card broad scope: per explicit user instruction, only changed duration units on trace output.
- Did not implement progress banners in this pass.

## Follow-ups / Deferred work
- If desired, implement remaining #313 progress-banner scope in a follow-up commit/PR.
