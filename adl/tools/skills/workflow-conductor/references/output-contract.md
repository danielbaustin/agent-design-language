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
  detected_phase: bootstrap_missing | card_local_blocker | pre_run | run_bound | execution_done | pr_in_flight | closed_out | already_satisfied | tracker_in_flight | unknown
  blocker_class: none | open_pr_wave_only | doctor_failed_or_inconclusive | review_changes_requested | merge_conflict | checks_failed | merge_blocked | healthy_pr_waiting | satisfied_by_child_issue_wave | satisfied_by_related_issue_refs | satisfied_by_sibling_issue_artifact | active_child_issue_wave | related_issue_ref_active | tracked_adl_residue | unsafe_root_checkout_execution | mismatched_publication_surface | rebind_to_issue_worktree_required | open_linkage_only
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
  issue_classification: docs-only | milestone-package-truth | workflow-docs | tooling-focused | rust-focused | demo-focused | review-remediation | release-tail | unknown
  validation_profile_selected: docs-bounded | tooling-focused | rust-focused | repo-native-finish | janitor-focused | demo-focused | review-remediation | release-tail | unknown
  bypasses:
    - component: <policy_component>
      reason: <bounded_reason>
  policy_result: PASS | PARTIAL | FAIL
actions_taken:
  - <routing or policy action>
handoff_state:
  next_phase: pr-init | pr-ready | pr-run | pr-finish | pr-janitor | pr-closeout | stp-editor | sip-editor | sor-editor | human_review | blocked
  continuation: continue | ask_operator | stop
  escalation_reason: none | operator_override_required | ambiguous_live_state | healthy_pr_waiting | manual_review_required | policy_block | child_issue_wave_satisfied | related_issue_ref_satisfied | sibling_issue_artifact_satisfied | child_issue_wave_active | related_issue_ref_active | repo_policy_residue | unsafe_root_checkout_execution | mismatched_publication_surface | rebind_to_issue_worktree_required
dispatch:
  mode: route_only | plan_subtask | invoke_subtask
  selected_skill: pr-init | pr-ready | pr-run | pr-finish | pr-janitor | pr-closeout | stp-editor | sip-editor | sor-editor | none
  skill_file: <absolute skill path or null>
  command_source: none | builtin | override
  command: <argv array or null>
  status: not_requested | planned | invoked | failed | unsupported | blocked
  result: not_applicable | planned | success | failure | timeout | dispatch_unsupported_for_selected_skill | missing_dispatch_placeholders | no_selected_skill
  exit_code: <int or null>
  stdout: <bounded text summary>
  stderr: <bounded text summary>
artifact:
  path: <path or null>
```
