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
- Start Time: 2026-02-18T06:00:00Z (approx)
- End Time: 2026-02-18T06:12:00Z (approx)

## Summary
Implemented full `--run` progress banners on stderr with quiet-mode suppression, while preserving runtime semantics and deterministic execution ordering.

Added:
- `RUN start <iso> run_id=... workflow=...`
- `STEP start (+<n>ms) <step_id> provider=<provider>`
- `STEP done (+<n>ms) <step_id> ok|fail duration_ms=<n>`
- `RUN done (+<n>ms) ok|fail artifacts=<run_dir>`

Also retained the requested trace duration display in seconds (`duration=<s.sss>s`) from the same branch work.

## Files changed
- `swarm/src/main.rs`
- `swarm/src/execute.rs`
- `swarm/src/trace.rs`
- `swarm/tests/execute_tests.rs`
- `swarm/tests/trace_tests.rs`
- `.adl/cards/313/output_313.md`

## Actions taken
- Added run-level progress banners in CLI run path (`swarm/src/main.rs`) printed via `eprintln!`.
- Added per-step progress banners in executor (`swarm/src/execute.rs`) for both sequential and concurrent execution paths.
- Ensured `--quiet` suppresses all progress banners by gating emission with `!quiet`.
- Exposed minimal trace helpers (`current_elapsed_ms`, `current_ts_ms`, `format_iso_utc_ms`) for consistent elapsed/timestamp banner formatting.
- Added tests:
  - `run_emits_progress_banners_on_stderr`
  - Extended `run_quiet_suppresses_step_output` to verify stderr banner suppression.
- Updated one legacy exact-stderr assertion to allow the new banner prefix lines while still validating the error message content.

## Commands run
- `sed -n '1,280p' .adl/cards/313/input_313.md`
- `git rev-parse --abbrev-ref HEAD`
- `git status --short`
- `rg -n -e "--run|quiet|trace|execute_sequential|RUN SUMMARY|print_trace|RunFinished|StepStarted|StepFinished" swarm/src/main.rs swarm/src/execute.rs swarm/tests -g '!**/target/**'`
- `rg -n "execute_sequential\(" swarm/src swarm/tests`
- `cd swarm && cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test`
- `swarm/tools/demo_v0_4.sh`

## Validation
- `cargo fmt`: PASS
- `cargo clippy --all-targets -- -D warnings`: PASS
- `cargo test`: PASS (including new progress-banner tests)
- `swarm/tools/demo_v0_4.sh`: PASS

## Decisions / Deviations
- Kept progress banner duration field in milliseconds (`duration_ms=<n>`) to match #313 acceptance text.
- Trace event display remains separately formatted with second-based duration (`duration=<s.sss>s`) per the explicit follow-up request.

## Follow-ups / Deferred work
- Optional: move progress-banner formatting to a dedicated utility if additional banner variants are added.
