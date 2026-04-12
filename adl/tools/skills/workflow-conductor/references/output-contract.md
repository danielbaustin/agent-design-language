# Output Contract

```yaml
status: done | blocked | failed
target:
  issue_number: <u32 or null>
  task_bundle_path: <path or null>
  branch: <branch or null>
  worktree_path: <path or null>
  pr_number: <u32 or null>
workflow_state:
  detected_phase: bootstrap_missing | card_local_blocker | pre_run | run_bound | execution_done | pr_in_flight | closed_out | unknown
  evidence_used:
    - <doctor_json_or_path_or_state_surface>
selected_skill:
  phase: init | ready | run | finish | janitor | closeout | editor | blocked
  skill_name: pr-init | pr-ready | pr-run | pr-finish | pr-janitor | pr-closeout | stp-editor | sip-editor | sor-editor | none
  editor_skill: stp-editor | sip-editor | sor-editor | none
workflow_compliance:
  skills_required: true | false
  card_editor_skills_required: true | false
  subagent_requirement: required | recommended | optional | forbidden
  subagent_assigned: true | false | not_applicable
  bypasses:
    - component: <policy_component>
      reason: <bounded_reason>
  policy_result: PASS | PARTIAL | FAIL
actions_taken:
  - <routing or policy action>
handoff_state:
  next_phase: pr-init | pr-ready | pr-run | pr-finish | pr-janitor | pr-closeout | stp-editor | sip-editor | sor-editor | human_review | blocked
```
