# ADL Output Card

Task ID: issue-0306
Run ID: issue-0306
Version: v0.4
Title: v0-4-demo-pass
Branch: codex/306-v0-4-demo-pass
Status: DONE

Execution:
- Actor: Codex (GPT-5)
- Model: GPT-5 Codex
- Provider: local CLI + repo scripts
- Start Time: 2026-02-17T17:35:00-08:00 (approx)
- End Time: 2026-02-17T17:48:00-08:00 (approx)

## Summary
Implemented a v0.4 demo pass with three no-network demos and a one-command demo runner:
1. Fork/Join swarm demo with deterministic join barrier and stable artifacts.
2. Bounded parallelism stress demo (8 branch steps, bounded executor behavior).
3. Deterministic replay demo (same workflow run twice; matching artifact hash).

Added a deterministic mock `ollama` tool for demo runs, added `swarm/tools/demo_v0_4.sh`, and updated README with a new `v0.4 Demos` section containing copy/paste `cargo run -q` commands.

## Files changed
- `/Users/daniel/git/agent-design-language/README.md`
- `/Users/daniel/git/agent-design-language/swarm/tools/README.md`
- `/Users/daniel/git/agent-design-language/swarm/examples/v0-4-demo-fork-join-swarm.adl.yaml`
- `/Users/daniel/git/agent-design-language/swarm/examples/v0-4-demo-bounded-parallelism.adl.yaml`
- `/Users/daniel/git/agent-design-language/swarm/examples/v0-4-demo-deterministic-replay.adl.yaml`
- `/Users/daniel/git/agent-design-language/swarm/tools/mock_ollama_v0_4.sh`
- `/Users/daniel/git/agent-design-language/swarm/tools/demo_v0_4.sh`

## Commands run
- `sed -n '1,260p' /Users/daniel/git/agent-design-language/.adl/cards/306/input_306.md`
- `git rev-parse --abbrev-ref HEAD`
- `git status --short`
- `rg -n "demo|v0.4|Demos|cargo run -q|demo_v0_4" /Users/daniel/git/agent-design-language/README.md /Users/daniel/git/agent-design-language/swarm/README.md /Users/daniel/git/agent-design-language/swarm/tools -g '!**/target/**'`
- `sed -n '1,260p' /Users/daniel/git/agent-design-language/README.md`
- `ls -1 /Users/daniel/git/agent-design-language/swarm/examples`
- `sed -n '1,260p' /Users/daniel/git/agent-design-language/swarm/examples/v0-3-concurrency-fork-join.adl.yaml`
- `/Users/daniel/git/agent-design-language/swarm/tools/demo_v0_4.sh`
- `cd /Users/daniel/git/agent-design-language/swarm && cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test`

## Validation
- Demo script:
  - `/Users/daniel/git/agent-design-language/swarm/tools/demo_v0_4.sh` -> PASS
  - Deterministic replay hash check -> PASS
  - Bounded-parallelism elapsed report -> produced (`elapsed_seconds=4` in this run)
- Rust quality gates:
  - `cargo fmt` -> PASS
  - `cargo clippy --all-targets -- -D warnings` -> PASS
  - `cargo test` -> PASS

## Repro steps (copy/paste)
From repo root:

- Fork/Join swarm:
  - `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-4-demo-fork-join-swarm.adl.yaml --run --trace --out .adl/reports/demo-v0.4/fork-join-swarm`

- Bounded parallelism:
  - `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-4-demo-bounded-parallelism.adl.yaml --run --trace --out .adl/reports/demo-v0.4/bounded-parallelism`

- Deterministic replay (run twice with same command):
  - `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-4-demo-deterministic-replay.adl.yaml --run --trace --out .adl/reports/demo-v0.4/deterministic-replay`

- Run all demos in sequence:
  - `swarm/tools/demo_v0_4.sh`

## Notes / Deviations
- Runtime parallelism level is currently fixed in engine code (`MAX_PARALLEL=4`), so the bounded demo proves bounded execution with current engine limit rather than a user-configurable 2/3 setting.
- No network calls are required when using `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh`.

## Follow-ups
- Optional: expose configurable concurrency limit in schema/runtime for demos that explicitly set parallelism values.
- Optional: add a CI smoke test for `swarm/tools/demo_v0_4.sh` in dry/mock mode.
