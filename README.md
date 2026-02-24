# Agent Design Language (ADL)

ADL is a schema-validated language for defining AI workflows as data and executing them with deterministic behavior. Instead of wiring ad hoc scripts and prompts together, ADL lets teams encode providers, agents, tasks, and workflow structure in a reviewable document that resolves into an explicit execution plan.

ADL is built for repeatability and engineering confidence. The runtime enforces strict validation, deterministic ordering, and stable run artifacts so behavior is inspectable in CI, auditable in production-like environments, and easier to debug when failures happen.

[![swarm-ci (main)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)
[![coverage](https://codecov.io/gh/danielbaustin/agent-design-language/graph/badge.svg?branch=main)](https://app.codecov.io/gh/danielbaustin/agent-design-language/tree/main)
![Milestone](https://img.shields.io/badge/milestone-v0.6-green)

## Features By Version

### v0.7 Features
Planned for v0.7 (roadmap; not yet released).

- Runtime delegation policy engine (beyond v0.6 metadata-only delegation)
- Richer checkpoint/recovery beyond step-boundary pause/resume
- Expanded profile and policy surface for stronger operator controls

### v0.6 Features
Shipped in v0.6.

- Deterministic scheduler semantics with bounded concurrency and stable ready-step ordering
- Pattern registry/compiler boundary with deterministic expansion
- HITL pause/resume (minimal): explicit paused state, strict validation, step-boundary resume
- Streaming output semantics that remain observational (artifact semantics unchanged)
- Provider profiles with deterministic resolve-time expansion and fail-fast validation
- Delegation metadata in schema + trace only (no runtime enforcement in v0.6)
- Instrumentation surfaces (`instrument graph`, `replay`, `diff-plan`, `diff-trace`)
- Coverage/quality hardening and v0.6 demo matrix alignment

### v0.5 Features
Brief historical context.

- PatternSchema v0.1 (`linear`, `fork_join`) and deterministic pattern IDs
- Remote execution MVP boundary (`/v1/health`, `/v1/execute`) with scheduler ownership kept local
- Signing/verification CLI baseline (`keygen`, `sign`, `verify`)

## Quickstart

From repo root:

```bash
cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- swarm/examples/v0-5-primitives-minimal.adl.yaml --print-plan
```

For runnable demo coverage and determinism checks, use:

- `docs/milestones/v0.6/DEMOS_v0.6.md`

## Canonical Docs Map

- Milestone documentation: `docs/milestones/v0.6/`
- ADRs: `docs/adr/`
- Runtime/CLI usage (build, test, examples): `swarm/README.md`
- Examples catalog: `swarm/examples/README.md`
- Workflow/process docs: `docs/default_workflow.md`

## Repository Layout

- `swarm/` runtime + CLI
- `adl-spec/` language/spec materials
- `docs/` milestone docs, ADRs, and contributor documentation
- `.adl/` cards, reports, and run artifacts

## License

Apache-2.0
