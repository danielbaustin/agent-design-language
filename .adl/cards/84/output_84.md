# ADL Output Card

Task ID: issue-0084
Run ID: issue-0084
Version: v0.2
Title: v0-2-burndown-84
Branch: codex/84-v0-2-burndown-84
Status: DONE

Execution:
- Actor: Codex CLI worker
- Model: GPT-5 (Codex)
- Provider: local
- Start Time: 2026-02-13T05:58:00Z
- End Time: 2026-02-13T06:02:28Z

## Summary
Fixed resolver step-id behavior for v0.2 by preserving explicit `run.workflow.steps[].id` values. Kept deterministic fallback only when an explicit id is absent. Added resolver and CLI tests to lock behavior in `--print-plan` and `--trace` outputs.

## Artifacts produced
- `.adl/cards/84/output_84.md`
- `swarm/src/resolve.rs`
- `swarm/tests/resolve_tests.rs`
- `swarm/tests/cli_smoke.rs`
- `swarm/tests/trace_tests.rs`

## Actions taken
- Updated resolver id selection in `resolve_run`:
  - use `StepSpec.id` when present
  - otherwise fallback to existing deterministic logic (`task` id, then `step-{idx}`)
- Added resolver coverage:
  - unit test `resolve_run_preserves_explicit_step_ids` in `swarm/src/resolve.rs`
  - integration test `resolve_preserves_explicit_step_ids_for_v0_2` in `swarm/tests/resolve_tests.rs`
- Added CLI output coverage:
  - `print_plan_preserves_explicit_step_ids_v0_2` in `swarm/tests/cli_smoke.rs`
  - `cli_trace_v0_2_preserves_explicit_step_ids` in `swarm/tests/trace_tests.rs`
- Reproduced and re-verified the issue command sequence before/after fix.

## Validation
- Tests / checks run:
  - `cargo test --test resolve_tests`
  - `cargo test --test cli_smoke`
  - `cargo test --test trace_tests`
  - `cargo run -- examples/v0-2-multi-step-basic.adl.yaml --print-plan`
  - `cargo run -- examples/v0-2-multi-step-basic.adl.yaml --trace`
- Results:
  - All tests passed.
  - Plan output now shows `step-1` and `step-2`.
  - Trace output now emits `step=step-1` and `step=step-2`.

## Decisions / Deviations
- Used the detailed legacy input card (`.adl/cards/issue-0084__input__v0.3.md`) as task source because canonical `.adl/cards/84/input_84.md` had empty Goal/Acceptance sections.

## Follow-ups / Deferred work
- None.
