# ADL v0.4 Release Announcement

ADL v0.4 is now complete.
Release: https://github.com/danielbaustin/agent-design-language/releases/tag/v0.4.0

This release marks the shift from planned concurrency to shipped, observable runtime concurrency behavior.

## What v0.4 ships
- Execution through validated `ExecutionPlan` graphs.
- Bounded fork execution in runtime.
- Deterministic join barrier behavior.
- Deterministic replay validation.
- No-network demo harness with one-command demos.

## Why this matters
v0.4 proves the runtime can execute concurrent workflows predictably, with stable artifacts and traceable behavior suitable for engineering teams.

## Key PRs
- [#299](https://github.com/danielbaustin/agent-design-language/pull/299) - WP-01 ExecutionPlan + DAG validation
- [#300](https://github.com/danielbaustin/agent-design-language/pull/300) - WP-02 bounded fork executor
- [#301](https://github.com/danielbaustin/agent-design-language/pull/301) - WP-03 deterministic join barrier
- [#303](https://github.com/danielbaustin/agent-design-language/pull/303) - runtime fork/join integration coverage
- [#305](https://github.com/danielbaustin/agent-design-language/pull/305) - runtime wiring hardening
- [#307](https://github.com/danielbaustin/agent-design-language/pull/307) - demo pass + README updates

## Demo quickstart
From repo root:

```bash
swarm/tools/demo_v0_4.sh
```

This runs fork/join swarm, bounded parallelism, and deterministic replay using a deterministic local mock provider (no network required).

## Notes
Current runtime concurrency is intentionally fixed at `MAX_PARALLEL=4` for v0.4. Configurable parallelism is tracked for the next milestone.
