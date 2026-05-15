# Output Contract

```yaml
status: done | blocked | failed
target:
  srp_path: <path>
edit_state:
  review_prompt_semantics_normalized: true | false
  review_results_truthful: true | false
  finding_dispositions_normalized: true | false
  residual_risk_recorded: true | false
findings:
  - severity: info | warning | blocking
    area: semantics | review_scope | findings | dispositions | residual_risk
    message: <short finding>
actions_taken:
  - <edit or normalization>
handoff_state:
  next_phase: qualitative_review | implementation_repair | pr_finish | blocked
```
