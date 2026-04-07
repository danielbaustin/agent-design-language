# PR Finish Skill Input Schema

```yaml
skill_input_schema: pr_finish.v1
mode: finish_issue | finish_branch | finish_worktree
repo_root: /absolute/path
target:
  issue_number: <u32 or null>
  branch: <string or null>
  worktree_path: <absolute path or null>
title: <string>
policy:
  validation_mode: bounded | repo_native_default
  open_mode: draft | update_only | ready
  stop_after_finish: true
```
