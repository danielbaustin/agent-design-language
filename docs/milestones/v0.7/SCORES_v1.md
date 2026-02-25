# Scores v1 (v0.7)

Scores v1 defines deterministic run-level scoring emitted at:

`<repo>/.adl/runs/<run_id>/learning/scores.json`

## Schema

```json
{
  "scores_version": 1,
  "run_id": "example-run",
  "generated_from": {
    "artifact_model_version": 1,
    "run_summary_version": 1
  },
  "summary": {
    "success_ratio": 1.0,
    "failure_count": 0,
    "retry_count": 0,
    "delegation_denied_count": 0,
    "security_denied_count": 0
  },
  "metrics": {
    "scheduler_max_parallel_observed": 1
  }
}
```

## Determinism Rules

- No timestamps or randomness in the artifact.
- Values are derived only from `run_summary.json` and in-memory trace events.
- `success_ratio` is quantized to thousandths.
- Retry count is computed deterministically from repeated `StepStarted` events per step id.
- `scheduler_max_parallel_observed` is derived from a deterministic trace scan.
- JSON is emitted via `serde_json::to_vec_pretty` + `artifacts::atomic_write`.

## Semantics

- Scores are read-only analytics.
- Scoring does not mutate execution state or affect scheduling/provider behavior.
- Scores are advisory inputs for later learning surfaces (`suggestions.json`, overlays), not enforcement.
