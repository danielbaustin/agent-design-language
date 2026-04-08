# SOR Editor Skill Input Schema

```yaml
skill_input_schema: sor_editor.v1
mode: normalize_sor | prepare_for_finish | repair_truth_drift
repo_root: /absolute/path
target:
  sor_path: /absolute/or/repo-relative/path/to/sor.md
  issue_number: <u32 or null>
  branch: <string or null>
  worktree_path: <absolute path or null>
  pr_number: <u32 or null>
evidence:
  commands_run:
    - <command actually executed>
  changed_paths:
    - <tracked path>
policy:
  integration_state: worktree_only | pr_open | merged | blocked
  preserve_issue_intent: true
  stop_after_edit: true
```
