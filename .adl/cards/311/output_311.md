# ADL Output Card

Task ID: issue-0311
Run ID: issue-0311
Version: v0.4
Title: trace-human-timestamps
Branch: codex/311-trace-human-timestamps
Status: DONE

Execution:
- Actor: Codex (GPT-5)
- Model: GPT-5 Codex
- Provider: local CLI
- Start Time: 2026-02-18T03:24:00Z (approx)
- End Time: 2026-02-18T03:31:30Z (approx)

## Summary
Updated TRACE output formatting to human-readable ISO-8601 UTC timestamps with run-relative elapsed prefixes, and added per-step `duration_ms` on `StepFinished` lines only. Runtime scheduling/execution semantics were not changed.

New line shape:
- `<iso8601> (+<elapsed_ms>ms) <EventName> key=value ...`
- `StepFinished` additionally appends `duration_ms=<n>`.

## Artifacts produced
- Updated output card: `.adl/cards/311/output_311.md`
- Demo artifacts refreshed by proof run under: `.adl/reports/demo-v0.4/`

## Files changed
- `swarm/src/trace.rs`
- `swarm/src/demo.rs`
- `swarm/tests/trace_tests.rs`
- `.adl/cards/311/output_311.md`

## Actions taken
- Read input card and verified branch context (`codex/311-trace-human-timestamps`).
- Refactored trace emission in `swarm/src/trace.rs`:
  - Added run-scoped `Instant` (`run_started_instant`) to compute deterministic elapsed values.
  - Kept event ordering unchanged; only presentation fields changed.
  - Added elapsed prefix to all events.
  - Added `duration_ms` only for `StepFinished`.
  - Switched to dependency-free ISO UTC formatter from epoch millis.
- Updated `swarm/src/demo.rs` to use new `TraceEvent::summarize()` signature.
- Updated `swarm/tests/trace_tests.rs` with format assertions:
  - ISO-like prefix + `(+<n>ms)` prefix exists.
  - `duration_ms=` appears on `StepFinished` line.
- Self-healed transient failure:
  - Initial attempt to add chrono failed due offline crates index resolution.
  - Replaced with local epoch->UTC conversion in `trace.rs` (no new dependency).

## Commands run
- `sed -n '1,320p' .adl/cards/311/input_311.md`
- `git rev-parse --abbrev-ref HEAD`
- `git status --short`
- `rg -n "timestamp|ts_ms|Trace|run_finished|StepStarted|StepFinished|UTC|localtime|iso" swarm/src swarm/tests -g '!**/target/**'`
- `sed -n '1,320p' swarm/src/trace.rs`
- `sed -n '1,280p' swarm/tests/trace_tests.rs`
- `rg -n "TraceEvent::|elapsed_ms|duration_ms|ts_ms" swarm/src/demo.rs swarm/src/main.rs swarm/tests -g '!**/target/**'`
- `cd swarm && cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test`
- `/Users/daniel/git/agent-design-language/swarm/tools/demo_v0_4.sh`

## Validation
- Tests / checks run:
  - `cd swarm && cargo fmt`
  - `cd swarm && cargo clippy --all-targets -- -D warnings`
  - `cd swarm && cargo test`
  - `swarm/tools/demo_v0_4.sh`
- Results:
  - `cargo fmt`: PASS
  - `cargo clippy --all-targets -- -D warnings`: PASS
  - `cargo test`: PASS
  - `demo_v0_4.sh`: PASS
- Sample TRACE output confirmed:
  - `2026-02-18T03:30:36.438Z (+0ms) StepStarted ...`
  - `... StepFinished ... duration_ms=...`

## Decisions / Deviations
- Did not add external datetime dependencies because crates.io index was temporarily unreachable.
- Implemented ISO-8601 UTC formatting in-project to keep the fix deterministic and offline-safe.

## Follow-ups / Deferred work
- Optional: extract timestamp formatting helper for reuse in non-trace outputs if needed.
- Optional: add a targeted unit test for extreme/negative epoch formatting edge cases (currently not encountered in runtime path).
