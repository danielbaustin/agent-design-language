# `swarm` Runtime (ADL)

`swarm` is the Rust reference runtime + CLI for Agent Design Language (ADL).

This README is intentionally scoped to local development and runtime usage:
- build/test commands
- key CLI surfaces
- example/demo entry points

For architecture and milestone narrative, use canonical docs:
- `../docs/milestones/v0.6/`
- `../docs/adr/`

## Build And Test

From `swarm/`:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Quick CLI Usage

From `swarm/`:

```bash
cargo run -q --bin swarm -- examples/v0-5-primitives-minimal.adl.yaml --print-plan
```

Useful commands:

```bash
# execute
cargo run -q --bin swarm -- examples/v0-5-primitives-minimal.adl.yaml --run --trace

# canonical v0.6 resume surface
cargo run -q --bin swarm -- resume <run_id>

# print assembled prompts
cargo run -q --bin swarm -- examples/v0-5-primitives-minimal.adl.yaml --print-prompts

# instrumentation
cargo run -q --bin swarm -- instrument graph examples/v0-5-pattern-fork-join.adl.yaml --format dot
cargo run -q --bin swarm -- instrument replay /tmp/trace.json
cargo run -q --bin swarm -- instrument diff-plan /tmp/plan-a.json /tmp/plan-b.json
cargo run -q --bin swarm -- instrument diff-trace /tmp/trace-a.json /tmp/trace-b.json
```

## Demos And Examples

- Examples index: `examples/README.md`
- v0.6 demo matrix: `../docs/milestones/v0.6/DEMOS_v0.6.md`
- v0.6 release notes: `../docs/milestones/v0.6/RELEASE_NOTES_v0.6.md`

No-network demo style (mock provider):

```bash
SWARM_OLLAMA_BIN=tools/mock_ollama_v0_4.sh cargo run -q --bin swarm -- examples/v0-6-hitl-no-pause.adl.yaml --run --trace --out ./out
```

## Current v0.6 Runtime Surface (Short)

- Deterministic scheduler ordering + bounded concurrency
- Pattern registry/compiler boundary (`run.pattern_ref`)
- HITL pause/resume (step-boundary, strict validation)
- Streaming output as observational behavior
- Provider profiles (resolve-time expansion)
- Delegation metadata (schema + trace only, no policy enforcement)
- Signing/verification commands (`keygen`, `sign`, `verify`)

For deeper semantics, see:
- `../docs/milestones/v0.6/DESIGN_v0.6.md`
- `../docs/milestones/v0.6/DECISIONS_v0.6.md`
- `../docs/adr/0001-determinism.md`
- `../docs/adr/0004-provider-profiles.md`
- `../docs/adr/0005-hitl-pause-resume.md`
