# Output Contract

```yaml
status: done | waiting | blocked | failed
sprint:
  issue_number: <u32>
  goal: <string or null>
  ordered_issue_numbers:
    - <u32>
sequence:
  current_issue_number: <u32 or null>
  completed_issue_numbers:
    - <u32>
  blocked_issue_number: <u32 or null>
  deferred_issue_numbers:
    - <u32>
  continuation: continue | stop | ask_operator | waiting_for_review
current_state:
  selected_skill: workflow-conductor | pr-init | pr-ready | pr-run | pr-finish | pr-janitor | pr-closeout | stp-editor | sip-editor | sor-editor | repo-packet-builder | repo-review-code | repo-review-tests | repo-review-docs | repo-review-security | repo-review-synthesis | none
  current_phase: intake | issue_loop | review | closeout | waiting | blocked
  blocker_reason: none | child_issue_blocked | malformed_cards | review_findings_blocking | missing_metrics | operator_scope_decision | healthy_pr_waiting_for_review | unknown
review:
  status: not_started | in_progress | done | blocked
  selected_skills:
    - repo-packet-builder | repo-review-code | repo-review-tests | repo-review-docs | repo-review-security | repo-review-synthesis
  packet_path: <path or null>
  code_review_path: <path or null>
  test_review_path: <path or null>
  docs_review_path: <path or null>
  security_review_path: <path or null>
  synthesis_path: <path or null>
  findings_summary:
    confirmed_findings: <int or null>
    unresolved_questions: <int or null>
closeout:
  status: not_started | in_progress | done | blocked
  closeout_note_path: <path or null>
  coverage:
    source: local_run | ci | existing_quality_gate | not_applicable | missing
    summary: <bounded text>
  rust_tracker:
    source: local_run | ci | existing_quality_gate | not_applicable | missing
    watch_count: <int or null>
    review_count: <int or null>
    rationale_count: <int or null>
actions_taken:
  - <bounded step>
artifact:
  sprint_state_path: <path or null>
  review_artifact_paths:
    - <path>
  closeout_artifact_paths:
    - <path>
```
