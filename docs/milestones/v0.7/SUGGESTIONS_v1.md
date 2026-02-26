# Suggestions v1 (v0.7)

Suggestions v1 defines deterministic advisory output at:

`<repo>/.adl/runs/<run_id>/learning/suggestions.json`

## Schema

```json
{
  "suggestions_version": 1,
  "run_id": "example-run",
  "generated_from": {
    "artifact_model_version": 1,
    "run_summary_version": 1,
    "scores_version": 1
  },
  "suggestions": [
    {
      "id": "sug-001",
      "category": "retry",
      "severity": "improvement",
      "rationale": "One or more steps failed; consider safer retry policy for transient paths.",
      "evidence": {
        "failure_count": 1,
        "retry_count": 0,
        "delegation_denied_count": 0,
        "security_denied_count": 0,
        "success_ratio": 0.5,
        "scheduler_max_parallel_observed": 1
      },
      "proposed_change": {
        "intent": "increase_step_retry_budget",
        "target": "failed-step-set"
      }
    }
  ]
}
```

## Determinism + Safety Rules

- No timestamps, randomness, host paths, or secrets.
- Stable rule ordering yields stable suggestion IDs (`sug-001`, `sug-002`, ...).
- `proposed_change` uses abstract mutation intent, not direct config keys.
- Suggestions are advisory only and do not change runtime behavior.

## Data Source Rules

- Primary source: `scores.json` + `run_summary.json`.
- Fallback: if `scores.json` is not available, suggestions are derived from `run_summary.json` only with the same deterministic rules.

## Guardrail Compatibility

- Suggestions are compatible with learning guardrails and future overlay mapping.
- No suggestion may imply bypassing envelope, signing/trust policy, or sandbox controls.
