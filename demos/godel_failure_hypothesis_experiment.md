# v0.8 Demo: Gödel CLI Failure -> Hypothesis -> Experiment

## Purpose

This runbook gives reviewers a deterministic, runnable path through the bounded Gödel CLI surfaces:

1. `adl godel run`
2. `adl godel inspect`
3. `adl godel evaluate`

It exercises the failure -> hypothesis -> mutation -> evaluation -> record flow that now persists the bounded v0.8 runtime/schema artifacts, including a first-class persisted hypothesis artifact.

## Integration Surfaces

The demo aligns with:

- `docs/milestones/v0.8/EXPERIMENT_RECORD_V1.md`
- `docs/milestones/v0.8/CANONICAL_EVIDENCE_VIEW_V1.md`
- `docs/milestones/v0.8/MUTATION_FORMAT_V1.md`
- `docs/milestones/v0.8/EVALUATION_PLAN_V1.md`
- `docs/milestones/v0.8/GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md`
- `docs/milestones/v0.8/OBSMEM_INDEXING_SURFACES_V1.md`
- `swarm/src/cli/godel_cmd.rs`

## Commands

Run from repository root.

### 1. Generate the bounded Gödel runtime artifacts

```bash
swarm/tools/demo_godel_hypothesis_engine.sh
```

### 2. Inspect the persisted runtime artifacts

```bash
cargo run --manifest-path swarm/Cargo.toml --bin adl -- godel inspect \
  --run-id review-godel-cli-001 \
  --runs-dir ./out/godel-cli-demo/runs
```

### 3. Exercise the bounded evaluator directly

```bash
cargo run --manifest-path swarm/Cargo.toml --bin adl -- godel evaluate \
  --failure-code tool_failure \
  --experiment-result ok \
  --score-delta 1
```

## Deterministic Demo Artifacts

- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/canonical_evidence_view.v1.json`
- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/godel_hypothesis.v1.json`
- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/mutation.v1.json`
- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/evaluation_plan.v1.json`
- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/experiment_record.v1.json`
- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/experiment_record.runtime.v1.json`
- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/obsmem_index_entry.runtime.v1.json`

## Loop Walkthrough

### Stage 1: Failure

`adl godel run` starts from a bounded failure description:

- stable identifiers (`run_id`, `workflow_id`)
- structured failure class (`failure_code`)
- canonical evidence references (`--evidence-ref` values)

### Stage 2: Hypothesis

The persisted canonical evidence, hypothesis, and mutation artifacts capture the deterministic bridge from failure to proposed change:

- one bounded hypothesis ID
- one structured deterministic hypothesis artifact
- one deterministic mutation ID
- stable references back to the supplied evidence

### Stage 3: Experiment

The evaluation artifacts and CLI summaries expose the bounded decision surface:

- one evaluation plan reference
- one deterministic adoption/block decision
- one persisted experiment record
- one persisted ObsMem index entry for later retrieval

## Replay and Audit Notes

- All artifact references are repo-relative.
- No raw logs, prompts, tool arguments, or secrets are stored.
- Artifact shape is deterministic for identical input conditions.
- This demo is intentionally bounded to the current CLI/runtime surfaces; it does not claim autonomous runtime mutation execution.

## Non-goals

- Autonomous hypothesis generation
- Policy-learning or scheduler changes
- Cross-run adaptive policy updates
