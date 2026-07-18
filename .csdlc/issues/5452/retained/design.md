# #5452 Builder Summary Failure Truth Design

## Scope

Repair the Spot builder-image validation wrapper so its exit status preserves
both primary validation and retained-summary generation truth.

## Approach

- Capture the primary validation status without disabling fail-closed behavior
  for the remainder of the wrapper.
- Run summary generation and capture its status independently.
- Return failure when either stage fails, while preserving the primary status
  when both stages do not succeed.
- Extend the focused shell harness with explicit mixed-result regressions.

## Validation

- Focused builder-image wrapper contract test.
- Shell syntax and diff-hygiene checks.
