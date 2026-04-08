# SIP Editor Skill Input Schema

```yaml
skill_input_schema: sip_editor.v1
mode: normalize_sip | prepare_for_ready | repair_lifecycle_drift
repo_root: /absolute/path
target:
  sip_path: /absolute/or/repo-relative/path/to/sip.md
  issue_number: <u32 or null>
  branch: <string or null>
  worktree_path: <absolute path or null>
  source_prompt_path: <path or null>
policy:
  lifecycle_state: pre_run | run_bound | pr_open | completed
  preserve_issue_intent: true
  stop_after_edit: true
```
