# Overlays v1 (v0.7)

Overlays v1 provide an explicit, opt-in mechanism for applying deterministic,
restricted runtime configuration changes without mutating source ADL YAML.

## Opt-in apply

Use:

`adl <adl.yaml> --overlay <overlay.json> --run`

Default behavior is unchanged when `--overlay` is not provided.

## Schema

```json
{
  "overlay_version": 1,
  "base_run_id": "optional-run-id",
  "created_by": "adl",
  "created_from": {
    "suggestions_version": 1,
    "artifact_model_version": 1
  },
  "changes": [
    {
      "id": "retry-all",
      "path": "run.workflow.steps.*.retry.max_attempts",
      "op": "set",
      "value": 2,
      "rationale": "example"
    }
  ]
}
```

## Allowed / forbidden surfaces (v1)

- Allowed:
  - `run.workflow.steps.*.retry.max_attempts`
- Forbidden (guardrail-enforced):
  - trust/signing policy fields
  - sandbox policy fields
  - scheduler policy fields
  - delegation policy fields (`run.delegation_policy.*`)

Delegation policy and security envelope behavior are immutable under overlays in
v1. Unknown or forbidden mutation paths are rejected deterministically.

`--overlay` is the only canonical v0.7 apply surface. Converting
`suggestions.json` into an overlay file is an explicit pre-step (out of band),
then the generated overlay is applied with `--overlay`.

## Artifacts

When an overlay is applied, runtime writes:

- `.adl/runs/<run_id>/learning/overlays/applied_overlay.json`
- `.adl/runs/<run_id>/learning/overlays/source_overlay.json`

`applied_overlay.json` records overlay hash and applied fields.
