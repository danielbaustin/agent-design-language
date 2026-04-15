# Medium Article Writer Skill Input Schema

Schema id: `medium_article_writer.v1`

## Purpose

Provide one structured invocation shape for the bounded `medium-article-writer`
skill.

The skill should turn one concrete article brief into a reviewer-friendly
Medium-style article packet while stopping before publication.

## Supported Modes

- `draft_from_brief_path`
- `draft_from_brief_text`
- `draft_from_demo_doc`

## Top-Level Shape

```yaml
skill_input_schema: medium_article_writer.v1
mode: draft_from_brief_path | draft_from_brief_text | draft_from_demo_doc
repo_root: /absolute/path
target:
  article_brief_path: <path or null>
  article_brief_text: <string or null>
  demo_doc_path: <path or null>
  artifact_root: <path or null>
  audience: <string or null>
  house_style: <string or null>
  forbidden_claims:
    - <string>
  expected_sections:
    - <string>
policy:
  writing_mode: standard | sharp | conservative
  headline_style: clear | curious | essay
  validation_mode: demo_aligned | artifact_only | none
  stop_before_publish: true
```

## Mode Requirements

### `draft_from_brief_path`

Requires:

- `target.article_brief_path`

Use when:

- the brief already exists as a local file

### `draft_from_brief_text`

Requires:

- `target.article_brief_text`

Use when:

- the operator wants to provide the brief inline

### `draft_from_demo_doc`

Requires:

- `target.demo_doc_path`

Use when:

- the skill should start from the existing Medium-writing demo surface and its documented packet shape

## Policy Fields

- `writing_mode`
  - required
  - one of `standard`, `sharp`, or `conservative`
- `headline_style`
  - required
  - one of `clear`, `curious`, or `essay`
- `validation_mode`
  - required
  - one of `demo_aligned`, `artifact_only`, or `none`
- `stop_before_publish`
  - must be `true`

## Example Invocation

```yaml
Use $medium-article-writer at /Users/daniel/git/agent-design-language/adl/tools/skills/medium-article-writer/SKILL.md with this validated input:

skill_input_schema: medium_article_writer.v1
mode: draft_from_brief_path
repo_root: /Users/daniel/git/agent-design-language
target:
  article_brief_path: demos/fixtures/medium_article_writing/v0-89-medium-article-brief.md
  article_brief_text: null
  demo_doc_path: demos/v0.89/medium_article_writing_demo.md
  artifact_root: null
  audience: ADL reviewers
  house_style: thoughtful technical essay
  forbidden_claims:
    - guaranteed virality
    - autonomous publishing
  expected_sections:
    - title_options
    - outline
    - draft
    - editorial_notes
policy:
  writing_mode: sharp
  headline_style: clear
  validation_mode: demo_aligned
  stop_before_publish: true
```

## Notes

- prefer concrete drafts over vague content strategy language
- keep publication outside the skill boundary
- use the existing bounded Medium article writing demo as the truthful baseline when relevant
