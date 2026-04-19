# Review Comment Triage Output Contract

Contract fields for `review-comment-triage` structured output.

```yaml
status: pass | partial | blocked | fail
source:
  mode: triage_from_payload | triage_live_pr_comments
  comment_payload_path: <path or null>
  pr_number: <u32 or null>
  pr_url: <url or null>
  source_label: <short label>
triage:
  actionable_now:
    - id: <string>
      file: <path or null>
      line: <u32 or null>
      author: <handle or null>
      link: <url or null>
      summary: <short rationale>
      suggested_action: <implement | verify_repro | request_thread_sync>
  already_fixed:
    - id: <string>
      file: <path or null>
      line: <u32 or null>
      author: <handle or null>
      link: <url or null>
      rationale: <short rationale>
  stale_or_not_reproducible:
    - id: <string>
      file: <path or null>
      line: <u32 or null>
      author: <handle or null>
      link: <url or null>
      rationale: <short rationale>
  follow_on_issue_needed:
    - id: <string>
      file: <path or null>
      line: <u32 or null>
      author: <handle or null>
      link: <url or null>
      rationale: <short rationale>
  blocked_or_operator_decision:
    - id: <string>
      file: <path or null>
      line: <u32 or null>
      author: <handle or null>
      link: <url or null>
      rationale: <short rationale>
execution_order:
  - <category>
  - <category>
handoff:
  pr_janitor_recommended: true | false
  gh_address_comments_recommended: true | false
  finding_to_issue_planner_recommended: true | false
  next_step: <short next-step text>
uncertainty:
  - <blocked or ambiguous reason>
```

## Rules

- Every triage result must include repo-relative file paths or explicit null.
- If `status` is `partial`, include both `already_fixed` and at least one open bucket.
- `execution_order` must be present when actionable items are non-empty.
- Comment records must preserve identity and link if available.
- Do not claim remediation, issue creation, merge-ready status, or PR fix execution.
- Include uncertainty entries when evidence is ambiguous or comment context is missing.
