# Sprint Review Skill Input Schema

## Purpose

Define the structured input for `sprint-review`, which reviews one completed sprint, mini-sprint, issue wave, or release-tail bundle by composing the existing ADL review skills.

## Shape

```yaml
skill_input_schema: sprint_review.v1
mode: review_completed_bundle | refresh_existing_review
repo_root: /absolute/path
target:
  scope: sprint | mini_sprint | issue_wave | release_tail
  umbrella_issue: <u32>
  child_issues:
    - <u32>
  pr_urls:
    - <url>
  changed_paths:
    - <path>
  review_docs:
    - <path>
  validation_evidence:
    - <path or command>
  lifecycle_cards:
    - <path>
  closeout_artifacts:
    - <path>
  execution_packet_path: <path or null>
  activity_log_path: <path or null>
  follow_up_issues:
    - <u32>
policy:
  required_lanes:
    - gap_analysis | code | docs | tests | evidence_and_closeout | synthesis | review_quality | security | architecture | dependency | release_evidence
  require_code_review_when_code_changed: true | false
  require_closeout_truth: true | false
  allow_review_subagent_exception: true | false
  stop_before_remediation: true
  stop_before_publication: true
  write_review_artifact: true | false
output_root: <path or null>
run_id: <string or null>
```

## Notes

- `scope` determines the review framing, not the implementation workflow.
- For `review_completed_bundle`, `pr_urls`, `changed_paths`, `validation_evidence`, `lifecycle_cards`, and `closeout_artifacts` are required parts of the evidence bundle, not optional enrichments.
- `child_issues` must be explicit and ordered enough that missing closeout truth can be detected.
- `required_lanes` should reflect the actual changed surfaces; code review is mandatory when code changed.
- This skill stops at review truth and follow-up routing. It is not an approval or remediation skill.
