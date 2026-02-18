# ADL Output Card

Task ID: issue-0304
Run ID: issue-0304
Version: v0.4
Title: v0-4-burst-3-runtime-wiring
Branch: codex/304-meta-v0-4-burst-3-runtime-wiring-executionplan-bounded-fork-deterministic-join
Status: DONE

Execution:
- Actor: Codex (GPT-5)
- Model: GPT-5 Codex
- Provider: local CLI + GitHub CLI
- Start Time: 2026-02-17T17:00:00-08:00 (approx)
- End Time: 2026-02-17T17:22:00-08:00

## Summary
Implemented real runtime wiring improvements in `swarm/src` so concurrent execution is plan-driven, bounded, and deterministic for fork/join semantics. Added one runtime-level integration test proving bounded fork execution and deterministic join barrier behavior. Kept existing concurrent tests passing (including prior #302-related behavior expectations).

## Artifacts produced
- `/Users/daniel/git/agent-design-language/.adl/reports/burst/20260217-172113/plan.md`
- `/Users/daniel/git/agent-design-language/.adl/reports/burst/20260217-172113/run_log.md`
- `/Users/daniel/git/agent-design-language/.adl/reports/burst/20260217-172113/final_summary.md`

## Runtime files changed (required)
- `/Users/daniel/git/agent-design-language/swarm/src/execute.rs`
- `/Users/daniel/git/agent-design-language/swarm/src/execution_plan.rs`

## Additional files changed
- `/Users/daniel/git/agent-design-language/swarm/tests/execute_tests.rs`

## Actions taken
- Read issue card and issue #304 scope.
- Inspected runtime path in `swarm/src/execute.rs` and planner in `swarm/src/execution_plan.rs`.
- Updated runtime concurrency mode check to use `resolved.execution_plan.workflow_kind`.
- Enforced deterministic ready-node ordering in concurrent executor loop (`ready_ids.sort()`).
- Added structural fork/join dependency inference in planner for concurrent workflows:
  - `fork.branch.*` depends on `fork.plan` when present.
  - `fork.join` depends on all `fork.branch.*` steps.
- Added planner unit test validating structural dependency wiring.
- Added new integration test:
  - `run_v0_3_fork_join_uses_bounded_executor_with_deterministic_join_barrier`
  - validates bounded runtime window and that `fork.join` starts after all branch `StepFinished` markers.
- Self-healed one transient implementation miss:
  - initial change caused `fork.plan` ordering regression in existing test.
  - fixed by planner structural dependencies and revalidated full suite.

## Commands run
- `sed -n '1,260p' /Users/daniel/git/agent-design-language/.adl/cards/304/input_304.md`
- `git rev-parse --abbrev-ref HEAD`
- `git status --short`
- `rg -n "ExecutionPlan|fork|join|bounded|run_bounded" swarm/src swarm/tests --glob '!target'`
- `sed -n '1,260p' swarm/src/execute.rs`
- `sed -n '260,620p' swarm/src/execute.rs`
- `sed -n '1,220p' swarm/src/resolve.rs`
- `sed -n '1,220p' swarm/src/execution_plan.rs`
- `sed -n '1,240p' swarm/src/prompt.rs`
- `rg -n "fn resolve_state_inputs|missing_prompt_inputs|execute_concurrent_deterministic" swarm/src/execute.rs`
- `sed -n '620,840p' swarm/src/execute.rs`
- `gh issue view 304 --json number,title,body,state,url`
- `sed -n '900,1320p' swarm/tests/execute_tests.rs`
- `cd /Users/daniel/git/agent-design-language/swarm && cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test`
- `cd /Users/daniel/git/agent-design-language/swarm && cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test` (after self-heal)

## Validation
- Tests / checks run:
  - `cargo fmt`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test`
- Results:
  - `cargo fmt`: PASS
  - `cargo clippy --all-targets -- -D warnings`: PASS
  - `cargo test`: PASS
  - New runtime-level test PASS:
    - `run_v0_3_fork_join_uses_bounded_executor_with_deterministic_join_barrier`

## Behavior changed and local repro steps
- What changed:
  - Concurrent runtime now uses execution-plan workflow kind directly.
  - Planner now encodes structural fork/join dependencies for concurrent workflows.
  - Concurrent ready-node execution order is deterministic by step id.
- Reproduce locally:
  1. `cd /Users/daniel/git/agent-design-language/swarm`
  2. `cargo test run_v0_3_fork_join_uses_bounded_executor_with_deterministic_join_barrier -- --nocapture`
  3. `cargo test run_executes_concurrent_workflows_in_v0_3_in_declared_order -- --nocapture`
  4. `cargo test`

## Decisions / Deviations
- Kept change set minimal and localized to runtime/planner plus one integration test.
- Used step-id-based structural fork/join inference as current-compatible approach (no schema expansion in this burst).

## Follow-ups / Deferred work
- Move structural fork/join semantics to explicit schema fields in future v0.4+ if step-id convention is replaced.
- If needed, make max parallelism configurable while preserving deterministic execution ordering.
