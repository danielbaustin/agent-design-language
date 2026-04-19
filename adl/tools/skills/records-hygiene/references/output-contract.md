# Records-Hygiene Output Contract

Use this contract whenever the skill emits a machine-readable artifact.

```yaml
status: clean | findings | blocked | error
target:
  issue_number: <u32 or null>
  task_bundle_path: <repo-relative-or-absolute path or null>
  branch: <branch name or null>
  worktree_path: <repo-relative-or-absolute path or null>
  version: <string or null>
findings:
  - severity: info | warning | blocking | ambiguous
    area: status_drift | linkage_drift | placeholder_drift | identity_drift | integration_truth
    message: <short finding text>
    evidence: <short evidence summary>
    files:
      - <path>
    can_auto_fix: true | false
safe_repairs_applied:
  - file: <path>
    action: <repair action name>
    previous: <previous value>
    updated: <updated value>
skipped_files:
  - <path>
ambiguous_findings:
  - area: <area>
    reason: <why a human decision is required>
    files:
      - <path>
recommended_follow_ons:
  - title: <follow-on issue candidate title>
    rationale: <reason this should be handled as a separate issue>
validation_performed:
  - <validation command or check name>
handoff_state:
  ready_for_editor: true | false
  ready_for_execution: true | false
  ready_for_follow_on_implementation: true | false
```

## Output Rules

- `status: clean` means no findings with `severity: blocking`.
- `status: findings` means actionable findings exist with at least one non-ambiguous finding.
- `status: blocked` means no safe pass is possible without operator input.
- `status: error` means the analyzer itself could not complete safely.
- `can_auto_fix` must be true only when the change is deterministic and mechanically safe.
- `safe_repairs_applied` may only include files under the resolved target scope.
- Do not include absolute host paths.
- `ambiguous_findings` entries should have explicit follow-on guidance and must not include fabricated edits.
- If `policy.report_only` is true, `safe_repairs_applied` must be empty.
