# Sprint Conductor Skill Input Schema

```yaml
skill_input_schema: sprint_conductor.v1
mode: run_sprint_slow_path | resume_sprint_slow_path | review_and_closeout_sprint
repo_root: /absolute/path
sprint:
  issue_number: <u32 or null>
  ordered_issue_numbers:
    - <u32>
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
          - stp-editor | sip-editor | sor-editor | spp-editor
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
  issue_closed: true | false
policy:
  require_sequential_closeout: true
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
