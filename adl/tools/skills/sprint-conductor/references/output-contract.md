# Output Contract

```yaml
status: done | waiting | blocked | failed
sprint:
  issue_number: <u32 or null>
  issue_url: <url or null>
  issue_created_by_skill: true | false
  issue_closed: true | false
  goal: <string or null>
  ordered_issue_numbers:
    - <u32>
  issue_records:
    - issue_number: <u32>
      status: pending | active | waiting_for_review | closed_out | blocked | deferred
      pr_url: <url or null>
      artifact_paths:
        - <path>
sequence:
  current_issue_number: <u32 or null>
  completed_issue_numbers:
    - <u32>
  blocked_issue_number: <u32 or null>
  deferred_issue_numbers:
    - <u32>
  continuation: continue | stop | ask_operator | waiting_for_review
structured_prompt_preflight:
  status: not_run | ready | needs_editor_repair | blocked
  required_card_types:
    - stp.md | sip.md | sor.md | spp.md | srp.md
  issue_results:
    - issue_number: <u32>
      bundle_path: <path or null>
      canonical_slug: <slug or null>
      status: ready | needs_editor_repair | blocked
      missing_cards:
        - <filename>
      contradictory_cards:
        - <filename>
      required_editor_skills:
        - stp-editor | sip-editor | sor-editor | spp-editor
      notes:
        - <bounded text>
truth_check:
  status: not_run | matched | drift_detected | blocked
  source: github_live | sprint_state_only | mixed
  gate_passed: true | false
  checked_issue_numbers:
    - <u32>
  checked_pr_urls:
    - <url>
  notes:
    - <bounded text>
current_state:
  selected_skill: workflow-conductor | pr-init | pr-ready | pr-run | pr-finish | pr-janitor | pr-closeout | stp-editor | sip-editor | sor-editor | repo-packet-builder | repo-review-code | repo-review-tests | repo-review-docs | repo-review-security | repo-review-synthesis | none
  current_phase: intake | issue_loop | review | closeout | waiting | blocked
  blocker_reason: none | child_issue_blocked | malformed_cards | review_findings_blocking | missing_metrics | operator_scope_decision | healthy_pr_waiting_for_review | missing_sprint_issue | unknown
review:
  status: not_started | in_progress | done | blocked
  selected_skills:
    - repo-packet-builder | repo-review-code | repo-review-tests | repo-review-docs | repo-review-security | repo-review-synthesis
  review_subagent_ids:
    - <id>
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
  sprint_issue_close_summary: <bounded text or null>
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
  sprint_review_url: <url or null>
  review_artifact_paths:
    - <path>
  closeout_artifact_paths:
    - <path>
```
