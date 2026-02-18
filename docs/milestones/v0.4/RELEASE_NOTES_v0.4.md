# ADL v0.4 Release Notes

## Metadata
- Product: `ADL`
- Version: `0.4`
- Release date: `2026-02-18`
- Tag: `v0.4.0`
- Release: https://github.com/danielbaustin/agent-design-language/releases/tag/v0.4.0

## Summary
ADL v0.4 ships real runtime concurrency behavior with deterministic fork/join semantics, bounded fork execution, strengthened runtime wiring through `ExecutionPlan`, and no-network demos that make the behavior directly reproducible.

## Highlights
- Runtime execution now follows validated `ExecutionPlan` dependencies.
- Fork-stage work executes through bounded concurrency in the runtime engine.
- Join barrier behavior is deterministic and reproducible.
- Demo coverage now includes fork/join swarm, bounded parallelism stress, and deterministic replay (no network required).

## What's New In Detail

### Runtime Concurrency Engine
- Landed ExecutionPlan + DAG validation scaffold.
- Landed bounded executor primitive for fork work.
- Landed deterministic join barrier wiring and runtime hardening.

### Determinism and Validation
- Added integration coverage for deterministic concurrent execution.
- Added bounded-parallelism integration coverage.
- Preserved existing v0.3-compatible behavior while strengthening runtime path.

### Demo and Docs
- Added v0.4 demo workflows under `swarm/examples/`.
- Added `swarm/tools/demo_v0_4.sh` and deterministic `mock_ollama_v0_4.sh`.
- Added README v0.4 Demos section and a concise "Why v0.4 matters" summary.

## Upgrade Notes
- No migration action required for existing v0.3 workflows.
- Runtime concurrency limit is fixed at engine level (`MAX_PARALLEL=4`) in this release.

## Known Limitations
- Configurable runtime parallelism is not exposed yet.
- Advanced scheduler policies and richer trace schema are deferred.

## Breaking Changes
None.

## Validation Notes
- Local gates used for shipped PRs: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- CI checks on merged PRs are green (`swarm-ci`, `swarm-coverage`).

## What's Next
- v0.5: configurable concurrency controls and scheduler improvements.
- v0.5: expanded orchestration and observability roadmap items.
