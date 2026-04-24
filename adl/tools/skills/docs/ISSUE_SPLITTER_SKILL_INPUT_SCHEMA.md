# Issue Splitter Skill Input Schema

Schema id: `issue_splitter.v1`

Use this schema when invoking `issue-splitter` with structured input.

## Required Top-Level Fields

- `skill_input_schema`: must be `issue_splitter.v1`
- `mode`: one of `analyze_issue_bundle` or `analyze_issue_number`
- `repo_root`: absolute repository root
- `target`: one bounded issue target
- `policy`: stop-boundary and split-planning policy

## Target Object

Recommended fields:

- `issue_number`
- `task_bundle_path`
- `source_issue_prompt_path`
- `issue_title`
- `split_policy`
- `current_scope_preference`

`analyze_issue_bundle` requires:

- `task_bundle_path`

`analyze_issue_number` requires:

- `issue_number`

## Policy Object

Recommended fields:

- `stop_before_tracker_mutation`
- `stop_before_card_mutation`
- `allow_follow_on_issue_drafts`
- `max_follow_on_count`
- `preserve_issue_graph_links`

## Example

```yaml
skill_input_schema: issue_splitter.v1
mode: analyze_issue_bundle
repo_root: /Users/daniel/git/agent-design-language
target:
  task_bundle_path: .adl/v0.90.4/tasks/issue-2480__v0-90-4-backlog-add-issue-splitter-operational-skill
  source_issue_prompt_path: .adl/v0.90.4/bodies/issue-2480-v0-90-4-backlog-add-issue-splitter-operational-skill.md
  issue_title: "[v0.90.4][backlog][tools] Add issue-splitter operational skill"
policy:
  stop_before_tracker_mutation: true
  stop_before_card_mutation: true
  allow_follow_on_issue_drafts: true
  max_follow_on_count: 2
  preserve_issue_graph_links: true
```

## Output

Default output root:

```text
.adl/reviews/issue-splitter/<run_id>/
```

Required artifacts:

- `issue_splitter_report.md`
- `issue_splitter_report.json`

The report must preserve issue-graph notes, split rationale, and non-claims. It
must not create tracker items, rewrite cards, or claim the scope already changed.
