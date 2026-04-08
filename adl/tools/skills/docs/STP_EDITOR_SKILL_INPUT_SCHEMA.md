# STP Editor Skill Input Schema

```yaml
skill_input_schema: stp_editor.v1
mode: normalize_stp | tighten_for_review | repair_stp_drift
repo_root: /absolute/path
target:
  stp_path: /absolute/or/repo-relative/path/to/stp.md
  issue_number: <u32 or null>
  source_prompt_path: <path or null>
policy:
  preserve_issue_intent: true
  stop_after_edit: true
  allow_scope_reframing: false
```
