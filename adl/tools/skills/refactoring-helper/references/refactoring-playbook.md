# Refactoring Playbook

## Planning Checklist

- Bound the target to named files, modules, diff paths, or issue surfaces.
- State whether behavior must be preserved or behavior change is explicitly in
  scope.
- List invariants before proposing edits.
- List available tests and missing characterization tests.
- Split the plan into independently reviewable slices.
- Attach rollback notes to each slice.
- Record residual risk instead of pretending the plan proves everything.

## Slice Heuristics

- Start with characterization tests when behavior is risky or underspecified.
- Prefer extraction and naming cleanup before data-shape changes.
- Move one boundary at a time.
- Delete only after evidence proves the path is unused or replaced.
- Keep generated artifacts and public formats stable unless a migration is named.

## Validation Heuristics

- Use the smallest command that proves the changed surface.
- Pair each command with the invariant it verifies.
- Record not-run validation as residual risk.
- Do not substitute formatting for behavior proof.

## Unsafe Signals

- target described only as "cleanup everything"
- behavior change is implied but not named
- tests are absent for public or persistence behavior
- refactor spans unrelated subsystems
- rollback would require reconstructing lost behavior from memory
