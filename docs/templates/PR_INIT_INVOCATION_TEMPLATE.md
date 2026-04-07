# PR Init Invocation Template

## Purpose

This is the canonical caller-facing template for invoking the `pr-init` skill.

Use it when you want to:
- create and bootstrap a new tracked issue
- bootstrap an existing tracked issue
- invoke `pr-init` from Codex CLI, a sub-agent, or a future wrapper without reconstructing the payload from loose prose

This template is a caller artifact. It does not replace the underlying schema in:
- `adl/tools/skills/docs/PR_INIT_SKILL_INPUT_SCHEMA.md`

## Template Rules

- Keep the payload valid against `pr_init.v1`.
- Use repo-root-stable paths in examples.
- Make the stop boundary explicit.
- Do not imply branch creation, worktree creation, or implementation work in this template.

## Canonical Invocation Envelope

Copy this shape and replace the placeholder values.

```yaml
Use $pr-init at /Users/daniel/git/agent-design-language/adl/tools/skills/pr-init/SKILL.md with this validated input:

skill_input_schema: pr_init.v1
mode: create_and_bootstrap | bootstrap_existing_issue
repo_root: /Users/daniel/git/agent-design-language

issue:
  number: null | <issue number>
  title: null | "[v0.87][tools] Example issue"
  slug: null | "example-issue"
  version: null | "v0.87"
  labels: null | "track:roadmap,type:task,area:tools"
  body: null | "<inline issue body>"
  body_file: null | "path/to/body.md"

policy:
  version_source: explicit | infer
  label_source: explicit | infer | normalize
  body_source: authored | generated | infer
  allow_slug_derivation: true | false
  stop_after_bootstrap: true
```

## Mode Notes

### `create_and_bootstrap`

Use when a new GitHub issue must be created.

Minimum practical payload:
- `mode: create_and_bootstrap`
- `repo_root`
- `issue.title`
- `policy.stop_after_bootstrap: true`

Typical choices:
- `issue.number: null`
- `policy.body_source: generated` when you want the control plane to generate the first readable body

### `bootstrap_existing_issue`

Use when the GitHub issue already exists and only the local bootstrap surfaces must be created or reconciled.

Minimum practical payload:
- `mode: bootstrap_existing_issue`
- `repo_root`
- `issue.number`
- `policy.stop_after_bootstrap: true`

Typical choices:
- `issue.title: null`
- `issue.labels: null`
- `issue.body: null`
- `issue.body_file: null`

## Example: New Issue Bootstrap

```yaml
Use $pr-init at /Users/daniel/git/agent-design-language/adl/tools/skills/pr-init/SKILL.md with this validated input:

skill_input_schema: pr_init.v1
mode: create_and_bootstrap
repo_root: /Users/daniel/git/agent-design-language
issue:
  number: null
  title: "[v0.87][tools] Example issue"
  slug: "v0-87-tools-example-issue"
  version: "v0.87"
  labels: "track:roadmap,type:task,area:tools"
  body: null
  body_file: null
policy:
  version_source: explicit
  label_source: explicit
  body_source: generated
  allow_slug_derivation: false
  stop_after_bootstrap: true
```

## Example: Existing Issue Bootstrap

```yaml
Use $pr-init at /Users/daniel/git/agent-design-language/adl/tools/skills/pr-init/SKILL.md with this validated input:

skill_input_schema: pr_init.v1
mode: bootstrap_existing_issue
repo_root: /Users/daniel/git/agent-design-language
issue:
  number: 1374
  title: null
  slug: "v0-87-tools-add-structured-issue-bootstrap-invocation-template-for-codex-and-subagents"
  version: "v0.87"
  labels: null
  body: null
  body_file: null
policy:
  version_source: explicit
  label_source: infer
  body_source: infer
  allow_slug_derivation: false
  stop_after_bootstrap: true
```

## Stop Boundary

This template is only for `pr-init`.

It must stop after:
- issue creation or resolution
- source-prompt creation
- root bundle creation
- bootstrap validation

It must not continue into:
- qualitative card review
- branch creation
- worktree creation
- `pr run`
- implementation
- `pr finish`

## Caller Checklist

Before invocation, confirm:
- `repo_root` is absolute
- the mode is correct for the task
- exactly one mode contract is satisfied
- at most one of `issue.body` and `issue.body_file` is set
- `policy.stop_after_bootstrap` is `true`
- any omitted slug/version/labels are allowed to be inferred by policy

## Related Docs

- `adl/tools/skills/docs/PR_INIT_SKILL_INPUT_SCHEMA.md`
- `adl/tools/skills/pr-init/SKILL.md`
- `adl/tools/skills/pr-init/adl-skill.yaml`
- `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
