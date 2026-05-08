# Sprint Conductor Skill Input Schema

```yaml
skill_input_schema: sprint_conductor.v1
mode: run_sprint_slow_path | resume_sprint_slow_path | review_and_closeout_sprint
repo_root: /absolute/path
sprint:
  issue_number: <u32>
  ordered_issue_numbers:
    - <u32>
  goal: <string or null>
  version: <string or null>
  slug: <string or null>
  stop_date: <YYYY-MM-DD or null>
  current_issue_number: <u32 or null>
  completed_issue_numbers:
    - <u32>
  blocked_issue_number: <u32 or null>
  review_paths:
    - /absolute/or/repo-relative/path
  closeout_paths:
    - /absolute/or/repo-relative/path
policy:
  require_sequential_closeout: true
  require_existing_issue_skills: true
  require_editor_skills: true
  require_code_review: true
  capture_coverage_at_closeout: true
  capture_rust_tracker_at_closeout: true
  stop_on_blocker: true
resume_from_state_path: /absolute/or/repo-relative/path/to/sprint_state.md
```
