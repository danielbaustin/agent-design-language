# Output Contract

When ADL expects structured review output, use this shape.

```yaml
status: complete | partial | blocked
target: <repo path or review target>
findings:
  - priority: P0 | P1 | P2 | P3 | P4 | P5
    file: <path>
    line: <1-based line or null>
    title: <short finding title>
    impact: <why it matters>
    trigger: <how the problem appears>
    evidence: <brief concrete explanation>
assumptions:
  - <assumption>
coverage_summary:
  reviewed:
    - <high-signal area>
  skipped:
    - <ignored or unreviewed area>
manifest_and_config_reviewed:
  - <manifest or config path>
validation_performed:
  - inventory
  - static_reading
  - tests_read | targeted_tests_run | no_tests_run
follow_up:
  - <optional next action>
```

## Rules

- `findings` must come first in any human-readable rendering.
- If there are no significant findings, emit `findings: []` and explain residual risk under `coverage_summary` or `follow_up`.
- Do not claim dynamic validation unless commands were actually run.
- If tests were not run, say so explicitly and include the reason.
- Use `P4` and `P5` only for concrete, non-speculative issues.
- Do not omit top-level manifests or dependency/build/toolchain config from `manifest_and_config_reviewed` unless the user explicitly scoped them out.

## Default Artifact Location

When writing the review to disk by default, use:

```text
.adl/reviews/<timestamp>-repo-review.md
```

Use a timestamp formatted like `YYYYMMDD-HHMMSS`.
