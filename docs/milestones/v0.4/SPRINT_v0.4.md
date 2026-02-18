# ADL v0.4 Sprint Summary

## Metadata
- Sprint: `v0.4`
- Milestone: `v0.4`
- Start date: `2026-02-17`
- End date: `2026-02-18`
- Owner: Daniel Austin

## Sprint Goal
Ship real runtime concurrency (ExecutionPlan + bounded fork + deterministic join) and publish no-network demos proving behavior.

## Scope Outcome
- Completed: ExecutionPlan + DAG scaffold.
- Completed: bounded executor integration.
- Completed: deterministic join barrier integration.
- Completed: runtime wiring hardening.
- Completed: no-network demo pass and README demo UX.

## Delivery Links
- WP-01: [#299](https://github.com/danielbaustin/agent-design-language/pull/299) (issue #298)
- WP-02: [#300](https://github.com/danielbaustin/agent-design-language/pull/300) (issue #297)
- WP-03: [#301](https://github.com/danielbaustin/agent-design-language/pull/301) (issue #296)
- Burst 2 runtime wiring: [#303](https://github.com/danielbaustin/agent-design-language/pull/303) (issue #302)
- Burst 3 runtime wiring: [#305](https://github.com/danielbaustin/agent-design-language/pull/305) (issue #304)
- Demo pass: [#307](https://github.com/danielbaustin/agent-design-language/pull/307) (issue #306)

## Quality Gates
- `cargo fmt`: pass
- `cargo clippy --all-targets -- -D warnings`: pass
- `cargo test`: pass
- CI on merged PRs: green

## Risks / Follow-ups
- Configurable runtime parallelism is not exposed yet (current engine limit is fixed).
- Advanced scheduling and trace schema expansion deferred to v0.5.

## Retrospective

### What went well
- Deterministic concurrency achieved without regressions.
- CI discipline maintained (green-only merges).
- No-network demo UX landed successfully.
- Human-readable trace improvements.

### What slowed us down
- GitHub API flakiness during release.
- Multiple bursts required to isolate test vs runtime wiring.
- Trace formatting churn.

### What v0.4 proves
ADL now executes real, deterministic fork/join concurrency
with bounded execution and reproducible artifacts.

## Exit
Sprint goal met. Runtime concurrency is shipped, deterministic, and demoable.
