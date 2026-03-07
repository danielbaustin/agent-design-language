# v0.75 Demo Matrix (WP-13)

This matrix defines runnable, deterministic, CI-friendly demo commands for v0.75.

## Scope

In scope for v0.75:
- hierarchical planner demo
- ObsMem indexing demo
- ObsMem retrieval demo
- end-to-end ObsMem pipeline demo

Out of scope for v0.75:
- Rust transpiler demo implementation (v0.8 candidate)

## Runtime Preconditions

Run from `swarm/`.
Use deterministic local provider mode:

```bash
ADL_OLLAMA_BIN=tools/mock_ollama_v0_4.sh
```

No network access is required.

## Related Docs

- Design contract: `docs/milestones/v0.75/DESIGN_0.75.md`
- Demo planning context: `docs/milestones/v0.75/DEMO_PLANNING.md`
- User-facing ObsMem guide: `docs/OBSMEM.md`

## Demo Matrix

### 1) Hierarchical Planner Demo

Description:
- Runs the hierarchical planner workflow and emits deterministic step outputs + trace.

Command to run:

```bash
ADL_OLLAMA_BIN=tools/mock_ollama_v0_4.sh \
cargo run -q --bin adl -- examples/v0-7-hierarchical-planner.adl.yaml --run --trace --allow-unsigned --out ../.tmp/wp13/hier
```

Expected artifacts:
- `../.tmp/wp13/hier/plan.json`
- `../.tmp/wp13/hier/exec-alpha.json`
- `../.tmp/wp13/hier/exec-beta.json`
- `../.tmp/wp13/hier/aggregate.json`
- `../.adl/runs/v0-7-hierarchical-planner/logs/activation_log.json`

Expected trace outputs:
- `TRACE run_id=v0-7-hierarchical-planner workflow_id=workflow version=0.5`
- ordered lifecycle events for `planner.plan`, `executor.alpha`, `executor.beta`, `aggregator.final`

### 2) ObsMem Indexing Demo

Description:
- Executes the same workflow with `ADL_OBSMEM_DEMO=1` and emits deterministic ObsMem index summary.

Command to run:

```bash
ADL_OLLAMA_BIN=tools/mock_ollama_v0_4.sh ADL_OBSMEM_DEMO=1 \
cargo run -q --bin adl -- examples/v0-7-hierarchical-planner.adl.yaml --run --trace --allow-unsigned --out ../.tmp/wp13/obsmem
```

Expected artifacts:
- `../.adl/runs/v0-7-hierarchical-planner/learning/obs_mem_index_summary.json`

Expected trace outputs:
- stdout line: `OBSMEM artifacts index=...obs_mem_index_summary.json query=...obs_mem_query_result.json`
- normal workflow trace banner + ordered step events

### 3) ObsMem Retrieval Demo

Description:
- Uses deterministic retrieval policy (`swarm::obsmem_retrieval_policy`) from the same ObsMem demo run and emits query results.

Command to run:

```bash
ADL_OLLAMA_BIN=tools/mock_ollama_v0_4.sh ADL_OBSMEM_DEMO=1 \
cargo run -q --bin adl -- examples/v0-7-hierarchical-planner.adl.yaml --run --trace --allow-unsigned --out ../.tmp/wp13/obsmem
```

Expected artifacts:
- `../.adl/runs/v0-7-hierarchical-planner/learning/obs_mem_query_result.json`

Expected trace outputs:
- stdout line includes `OBSMEM artifacts ... query=...obs_mem_query_result.json`
- deterministic step trace ordering identical to hierarchical run

### 4) End-to-End ObsMem Pipeline Demo

Description:
- Runs workflow + ObsMem emission, validates replay output generation, and exports learning bundle artifacts.

Commands to run:

```bash
# ObsMem-enabled run
ADL_OLLAMA_BIN=tools/mock_ollama_v0_4.sh ADL_OBSMEM_DEMO=1 \
cargo run -q --bin adl -- examples/v0-7-hierarchical-planner.adl.yaml --run --trace --allow-unsigned --out ../.tmp/wp13/obsmem-e2e

# Replay projection from activation trace
cargo run -q --bin adl -- instrument replay ../.adl/runs/v0-7-hierarchical-planner/logs/activation_log.json > /tmp/wp13-replay.json

# Learning bundle export
cargo run -q --bin adl -- learn export --format bundle-v1 --runs-dir ../.adl/runs --run-id v0-7-hierarchical-planner --out ../.tmp/wp13/learning-bundle
```

Expected artifacts:
- `../.adl/runs/v0-7-hierarchical-planner/learning/obs_mem_index_summary.json`
- `../.adl/runs/v0-7-hierarchical-planner/learning/obs_mem_query_result.json`
- `/tmp/wp13-replay.json`
- `../.tmp/wp13/learning-bundle/learning_export_v1/manifest.json`

Expected trace outputs:
- workflow trace banner for run
- ObsMem artifact path line in stdout
- replay command produces deterministic JSON projection at `/tmp/wp13-replay.json`

## Verification

cargo test --workspace
cargo run -q --bin adl -- demo demo-b-one-command --run --no-open --out ../.tmp/wp13/demo-b
artifact paths validated

Executed/validated form in this WP:

```bash
cargo test --workspace
ADL_OLLAMA_BIN=tools/mock_ollama_v0_4.sh cargo run -q --bin adl -- demo demo-b-one-command --run --no-open --out ../.tmp/wp13/demo-b
# artifact paths validated in ../.tmp/wp13/* and ../.adl/runs/v0-7-hierarchical-planner/learning/*
```
