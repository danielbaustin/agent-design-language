# Issue Execution Metrics Prediction

- status: `predicted`
- input packet count: `1`
- issue goal ref: `goal:v0.91.7:issue:4617`
- sprint goal ref: `goal:v0.91.7:wp-04`
- confidence: `medium`
- unknown values policy: `unknown_is_not_zero`

## Predictions

- elapsed_seconds: `321` basis=`known_elapsed_seconds`
- total_tokens: `54321` basis=`known_total_tokens`
- validation_seconds: `32` basis=`heuristic_from_elapsed_seconds`
- pr_wait_risk: `medium` basis=`heuristic_missing_wait_input`
- ci_wait_risk: `medium` basis=`heuristic_missing_wait_input`
- outlier_risk: `medium` basis=`max(scale_risk, pr_wait_risk, ci_wait_risk)`

## Input Availability

- elapsed_seconds: `known`
- active_work_seconds: `known`
- validation_seconds: `unknown`
- pr_wait_seconds: `unknown`
- ci_wait_seconds: `unknown`
- total_tokens: `known`

## Missing Inputs

- `ci_wait_seconds`
- `pr_wait_seconds`
- `validation_seconds`
