# Output Contract

```yaml
status: done | blocked | failed
target:
  sor_path: <path>
edit_state:
  integration_wording_normalized: true | false
  validation_claims_normalized: true | false
  placeholders_removed: true | false
findings:
  - severity: info | warning | blocking
    area: integration | validation | evidence | summary
    message: <short finding>
actions_taken:
  - <edit or normalization>
handoff_state:
  next_phase: pr_run | pr_finish | blocked
```
