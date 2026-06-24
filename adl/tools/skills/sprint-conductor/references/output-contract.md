# Output Contract

```yaml
status: done | waiting | blocked | failed
sprint:
  issue_number: <u32 or null>
  issue_url: <url or null>
  issue_created_by_skill: true | false
  issue_closed: true | false
  execution_mode: sequential | parallel | hybrid
  execution_packet_path: <path or null>
  follow_up_issue_policy: post_sprint_follow_on | must_land_before_sprint_close
  goal: <string or null>
  goal_policy:
    status: descriptive_only | nested_child_goals_supported
    sprint_goal_role: descriptive_sprint_objective | active_parent_goal
    active_session_goal_required: child_issue_only | nested_child_goal_allowed
    notes:
      - <bounded text>
  ordered_issue_numbers:
    - <u32>
  recommended_execution_order:
    - <bounded text>
  candidate_parallel_lanes:
    - lane_id: <string>
      classification: safe_parallel | serial_gate | speculative_risky | blocked_until_dependency
      issue_numbers:
        - <u32>
      expected_write_sets:
        - <bounded text>
      expected_pvf_lanes:
        - <bounded text>
      validation_lanes:
        - <bounded text>
      dependency_gates:
        - <bounded text>
      collision_risks:
        - <bounded text>
      watcher_assignment: <bounded text>
      subagent_assignment: <bounded text>
      why_parallel_safe: <bounded text>
      required_coordination: <bounded text>
  safe_parallel_lanes:
    - lane_id: <string>
      issue_numbers:
        - <u32>
      why_parallel_safe: <bounded text>
      required_coordination: <bounded text>
  serial_gates:
    - gate_id: <string>
      blocks:
        - <bounded text>
      exit_condition: <bounded text>
      owner: <string>
  pvf_notes:
    immediate_issue_local_proof: <bounded text>
    parallel_validation_lanes:
      - <bounded text>
    serial_validation_gates:
      - <bounded text>
    proof_reuse_criteria: <bounded text>
    fail_closed_rule: <bounded text>
  planned_vs_actual_parallelism:
    planned_summary: <bounded text>
    actual_summary: <bounded text>
    prediction_misses:
      - lane_id: <string>
        issue_numbers:
          - <u32>
        why_wrong: <bounded text>
        corrective_action: <bounded text>
  issue_records:
    - issue_number: <u32>
      status: pending | active | waiting_for_review | closed_out | blocked | deferred
      pr_url: <url or null>
      artifact_paths:
        - <path>
      goal_metrics:
        status: not_recorded | recorded
        raw_log_path: <path or null>
        record_count: <int>
        phases_recorded:
          - issue_init | doctor_readiness | card_repair | execution_ready | issue_start | pr_publication | review_handoff | merge_closeout | sprint_closeout
        segments_recorded:
          - readiness_prep | bound_execution | sprint_rollup
        selected_stage: issue_init | doctor_readiness | card_repair | execution_ready | issue_start | pr_publication | review_handoff | merge_closeout | sprint_closeout | null
        selected_segment: readiness_prep | bound_execution | sprint_rollup | null
        recorded_at: <timestamp or null>
        data_source: codex_goal_tool | manual_entry | derived_sprint_state | unknown
        goal_id: <string or null>
        goal_id_availability: known | unknown | not_available
        started_at: <timestamp or null>
        completed_at: <timestamp or null>
        elapsed_seconds: <int or null>
        elapsed_availability: known | unknown | not_available
        token_usage:
          total_tokens: <int or null>
          prompt_tokens: <int or null>
          completion_tokens: <int or null>
          availability: known | unknown | not_available
          total_availability: known | unknown | not_available
          prompt_availability: known | unknown | not_available
          completion_availability: known | unknown | not_available
        model_ref: <string or null>
        session_ref: <string or null>
        thread_id: <string or null>
      closeout_gate:
        issue_closed: true | false
        pr_state: merged | closed_no_merge | not_applicable | unknown
        root_sor_status: done | failed | unknown
        worktree_status: pruned | retained_with_reason | not_applicable | unknown
        worktree_note: <bounded text or null>
  follow_up_issues:
    - issue_number: <u32>
      disposition: post_sprint_follow_on | must_land_before_sprint_close
      summary: <bounded text>
sequence:
  current_issue_number: <u32 or null>
  completed_issue_numbers:
    - <u32>
  blocked_issue_number: <u32 or null>
  deferred_issue_numbers:
    - <u32>
  continuation: continue | stop | ask_operator
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
          - stp-editor | sip-editor | spp-editor | srp-editor | sor-editor
      notes:
        - <bounded text>
installed_skill_parity:
  status: not_run | matched | drift_detected | blocked
  tracked_skill_dir: <path or null>
  installed_skill_dir: <path or null>
  left_only:
    - <path>
  right_only:
    - <path>
  diff_files:
    - <path>
  notes:
    - <bounded text>
readiness_sweep:
  status: not_run | ready | needs_repair | blocked
  ordered_issue_numbers:
    - <u32>
  execution_mode: sequential | parallel | hybrid
  execution_packet:
    status: not_required | present | needs_repair | blocked
    path: <path or null>
    missing_sections:
      - <heading>
  review_paths:
    status: declared | needs_repair
    paths:
      - <path>
    missing_paths:
      - <path>
  activity_log_paths:
    status: declared | needs_repair
    paths:
      - <path>
    missing_paths:
      - <path>
  issue_repairs:
    - issue_number: <u32>
      status: ready | needs_editor_repair | blocked
      bundle_path: <path or null>
      next_skills:
        - pr-init | workflow-conductor | stp-editor | sip-editor | spp-editor | srp-editor | sor-editor
      rationale: <bounded text>
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
  selected_skill: workflow-conductor | pr-init | pr-ready | pr-run | pr-finish | issue-watcher | pr-janitor | pr-closeout | stp-editor | sip-editor | spp-editor | srp-editor | sor-editor | repo-packet-builder | repo-review-code | repo-review-tests | repo-review-docs | repo-review-security | repo-review-synthesis | none
  current_phase: intake | issue_loop | review | closeout | waiting | watching | blocked
  blocker_reason: none | child_issue_blocked | malformed_cards | review_findings_blocking | missing_metrics | operator_scope_decision | missing_sprint_issue | unknown
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
  readiness: ready_to_close | needs_remediation | blocked | unknown
  closeout_note_path: <path or null>
  closeout_artifact_path: <path or null>
  sprint_issue_close_summary: <bounded text or null>
  planned_vs_actual_parallelism:
    planned_summary: <bounded text or null>
    actual_summary: <bounded text or null>
    prediction_misses:
      - lane_id: <string>
        issue_numbers:
          - <u32>
        why_wrong: <bounded text>
        corrective_action: <bounded text>
  closure_cleanliness: clean | clean_with_post_sprint_followups | residual_debt | unknown
  goal_metrics_rollup:
    issue_count: <int>
    issues_with_recorded_metrics: <int>
    issues_without_recorded_metrics: <int>
    issues_with_known_elapsed: <int>
    issues_with_unknown_elapsed: <int>
    issues_with_known_total_tokens: <int>
    issues_with_unknown_total_tokens: <int>
    total_elapsed_seconds_known_sum: <int>
    total_tokens_known_sum: <int>
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
next_handoff:
  status: continue | wait_and_recheck | ask_operator | stop
  target_issue_number: <u32 or null>
  target_pr_url: <url or null>
  next_skill: workflow-conductor | pr-init | pr-ready | pr-run | pr-finish | issue-watcher | pr-janitor | pr-closeout | stp-editor | sip-editor | spp-editor | srp-editor | sor-editor | repo-packet-builder | repo-review-code | repo-review-tests | repo-review-docs | repo-review-security | repo-review-synthesis | none
  child_session_goal:
    required: true | false
    create_after_bind: true | false
    sprint_issue_number: <u32 or null>
    child_issue_number: <u32 or null>
    bounded_objective: <bounded text or null>
  rationale: <bounded text>
artifact:
  sprint_state_path: <path or null>
  sprint_review_url: <url or null>
  review_artifact_paths:
    - <path>
  closeout_artifact_paths:
    - <path>
```
