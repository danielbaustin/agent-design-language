# Sprint Conductor Skill Input Schema

```yaml
skill_input_schema: sprint_conductor.v1
mode: run_sprint | resume_sprint | review_and_closeout_sprint
repo_root: /absolute/path
sprint:
  issue_number: <u32 or null>
  ordered_issue_numbers:
    - <u32>
  execution_mode: sequential | parallel | hybrid
  execution_packet_path: /absolute/or/repo-relative/path | null
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
  follow_up_issue_policy: post_sprint_follow_on | must_land_before_sprint_close
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
  activity_log_paths:
    - /absolute/or/repo-relative/path
  closeout_paths:
    - /absolute/or/repo-relative/path
  issue_records:
    - issue_number: <u32>
      status: pending | active | waiting_for_review | closed_out | blocked | deferred
      pr_url: <url or null>
      artifact_paths:
        - /absolute/or/repo-relative/path
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
  structured_prompt_preflight:
    status: not_run | ready | needs_editor_repair | blocked
    required_card_types:
      - stp.md | sip.md | sor.md | spp.md | srp.md
    issue_results:
      - issue_number: <u32>
        bundle_path: /absolute/or/repo-relative/path | null
        status: ready | needs_editor_repair | blocked
        missing_cards:
          - <filename>
        contradictory_cards:
          - <filename>
        required_editor_skills:
          - stp-editor | sip-editor | spp-editor | srp-editor | sor-editor
  truth_check:
    status: not_run | matched | drift_detected | blocked
    source: github_live | sprint_state_only | mixed
    gate_passed: true | false
    checked_issue_numbers:
      - <u32>
    checked_pr_urls:
      - <url>
  installed_skill_parity:
    status: not_run | matched | drift_detected | blocked
    tracked_skill_dir: /absolute/or/repo-relative/path | null
    installed_skill_dir: /absolute/or/repo-relative/path | null
    left_only:
      - <path>
    right_only:
      - <path>
    diff_files:
      - <path>
  readiness_sweep:
    status: not_run | ready | needs_repair | blocked
    execution_packet:
      status: not_required | present | needs_repair | blocked
      path: /absolute/or/repo-relative/path | null
      missing_sections:
        - <heading>
    review_paths:
      status: declared | needs_repair
      paths:
        - /absolute/or/repo-relative/path
    activity_log_paths:
      status: declared | needs_repair
      paths:
        - /absolute/or/repo-relative/path
    goal_policy:
      status: descriptive_only | nested_child_goals_supported
      sprint_goal_role: descriptive_sprint_objective | active_parent_goal
      active_session_goal_required: child_issue_only | nested_child_goal_allowed
      notes:
        - <bounded text>
    issue_repairs:
      - issue_number: <u32>
        status: ready | needs_editor_repair | blocked
        bundle_path: /absolute/or/repo-relative/path | null
        next_skills:
          - pr-init | workflow-conductor | stp-editor | sip-editor | spp-editor | srp-editor | sor-editor
        rationale: <bounded text>
  issue_closed: true | false
policy:
  require_declared_execution_mode: true
  require_sequential_closeout: true | false
  require_sep_for_parallel_or_hybrid: true
  require_existing_issue_skills: true
  require_editor_skills: true
  require_code_review: true
  allow_create_missing_sprint_issue: true | false
  require_full_sprint_structured_prompt_readiness: true
  allow_review_subagent_exception: true
  max_review_subagents_when_exception_enabled: 1
  follow_up_issue_policy: post_sprint_follow_on | must_land_before_sprint_close
  require_github_truth_recheck: true
  github_truth_gate_blocks_progress: true
  require_installed_skill_parity_before_live_run: true | false
  capture_coverage_at_closeout: true
  capture_rust_tracker_at_closeout: true
  stop_on_blocker: true
review_subagent_ids:
  - <id>
resume_from_state_path: /absolute/or/repo-relative/path/to/sprint_state.md
```

Notes:

- `sequential` mode means one child issue executes at a time.
- `parallel` mode means the SEP must name candidate lanes, safe lanes, and
  serial gates before child work is delegated to separate workers or sessions.
- `hybrid` mode means some child issues may execute in parallel, but named
  serial gates control later lanes.
- Sprint-level SEP state does not replace issue-local `SIP -> STP -> SPP ->
  SRP -> SOR` truth.
- The current sprint-state helper remains single-current-issue. SEP records
  safe parallel intent and routing evidence; it does not by itself prove
  automated multi-active sprint execution.
- `candidate_parallel_lanes` is the authoritative planning surface for safe,
  serial, speculative, and blocked lane intent. `safe_parallel_lanes` remains
  a summary/compatibility projection for already-admitted lanes.
- `planned_vs_actual_parallelism` is sprint-closeout truth. It should explain
  when predicted lanes did not execute as planned rather than implying hidden
  scheduler success.
- `readiness_sweep` is the aggregate pre-execution gate. It is expected to
  consume installed-skill parity, structured-prompt preflight, execution-packet
  presence, review-path declaration, and activity-log declaration before the
  first child issue runs.
- Review and activity-log paths are declaration surfaces for future sprint
  artifacts; readiness does not require those files to exist yet.
- `sprint.goal` is descriptive sprint coordination context. It must not be
  treated as the active Codex session goal during child implementation unless a
  later issue proves explicit nested-goal support.
