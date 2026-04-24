# Issue Folding Skill Input Schema

Schema id: `issue_folding.v1`

Use this schema when invoking `issue-folding` with structured input.

## Required Top-Level Fields

- `skill_input_schema`: must be `issue_folding.v1`
- `mode`: one of `analyze_issue_bundle` or `analyze_issue_number`
- `repo_root`: absolute repository root
- `target`: one bounded issue target
- `policy`: stop-boundary and evidence policy

## Target Object

Recommended fields:

- `issue_number`
- `task_bundle_path`
- `source_issue_prompt_path`
- `issue_state`
- `pr_state`
- `closure_hints`

`analyze_issue_bundle` requires:

- `task_bundle_path`

`analyze_issue_number` requires:

- `issue_number`

## Policy Object

Recommended fields:

- `allow_network`
- `stop_before_closeout`
- `stop_before_tracker_mutation`
- `permit_safe_noop_closeout_handoff`
- `require_closure_references_for_linked_dispositions`

## Example

```yaml
skill_input_schema: issue_folding.v1
mode: analyze_issue_bundle
repo_root: /Users/daniel/git/agent-design-language
target:
  task_bundle_path: .adl/v0.90.4/tasks/issue-2481__v0-90-4-backlog-add-issue-folding-operational-skill
  source_issue_prompt_path: .adl/v0.90.4/bodies/issue-2481-v0-90-4-backlog-add-issue-folding-operational-skill.md
  issue_state: open
  pr_state: none
policy:
  allow_network: false
  stop_before_closeout: true
  stop_before_tracker_mutation: true
  permit_safe_noop_closeout_handoff: true
  require_closure_references_for_linked_dispositions: true
```

## Output

Default output root:

```text
.adl/reviews/issue-folding/<run_id>/
```

Required artifacts:

- `issue_folding_report.md`
- `issue_folding_report.json`

The report must preserve issue-graph references, uncertainty, and non-claims. It
must not close the issue, mutate GitHub, merge PRs, or prune worktrees.
