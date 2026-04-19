# Review Comment Triage Skill Input Schema

Schema id: `review_comment_triage.v1`

## Purpose

Classify PR review feedback into bounded categories and produce an execution plan
that does not automatically widen issue scope.

## Supported Modes

- `triage_from_payload`
- `triage_live_pr_comments`

## Top-Level Shape

```yaml
skill_input_schema: review_comment_triage.v1
mode: triage_from_payload | triage_live_pr_comments
repo_root: /absolute/path
target:
  comment_payload_path: <path or null>
  pr_number: <u32 or null>
  pr_url: <url or null>
  artifact_root: <path or null>
  max_items: <u32 or null>
  follow_on_template_path: <path or null>
policy:
  allow_network: true | false
  stop_after_triage: true
  max_blocking_items_before_operator: <u32 or null>
  preserve_uncertainty: true | false
  route_follow_on_as_issue_planning: true | false
```

## Mode Requirements

### triage_from_payload

Requires:

- `target.comment_payload_path`

Use this when a local JSON fixture or exported comment surface is already available.

### triage_live_pr_comments

Requires:

- `target.pr_number`

Requires `policy.allow_network=true` and a reachable review source when
`target.pr_url` is not provided.

## Policy Fields

- `allow_network`
  - when false, the skill must only use local payload input.
- `stop_after_triage`
  - must be true.
- `max_blocking_items_before_operator`
  - recommended threshold for operator escalation if unresolved blocking comments grow.
- `preserve_uncertainty`
  - if true, do not auto-collapse ambiguous comments.
- `route_follow_on_as_issue_planning`
  - if true, include explicit follow-on candidates for separate planning.

## Output Notes

When supported, outputs should follow `references/output-contract.md` and include:

- grouped triage categories
- execution order
- unresolved uncertainty
- recommended next bounded skill

If the payload includes comment links and file context, preserve them in the output
artifact.
