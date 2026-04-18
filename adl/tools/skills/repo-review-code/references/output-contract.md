# Output Contract

Default `repo-review-code` artifacts are Markdown review packets with these sections in this order:

```md
## Metadata
- Skill: repo-review-code
- Target: <repo/path/branch/diff/packet>
- Date: <UTC timestamp or calendar date>
- Artifact: <path or none>

## Findings
- <priority>: <title>
  File: <repo-relative path or none>
  Role: code/correctness
  Scenario: <trigger or review condition>
  Impact: <behavioral consequence>
  Evidence: <specific code/doc/test/config observation>

## Reviewed Surfaces
- <bounded list or explicit none>

## Validation Performed
- <command and what it proved, or explicit not-run rationale>

## Residual Risk
- <what this role did not inspect or could not prove>
```

## Rules

- Findings come before explanatory summaries.
- Use repo-relative file paths in findings and reviewed-surface lists.
- Preserve severity and role ownership; do not downgrade findings from other specialists.
- Do not claim validation unless the command was actually run.
- If no findings are found, write `No material findings.` in `Findings` and state residual risk explicitly.
- Do not emit absolute host paths, secrets, raw prompts, or raw tool arguments.
- Stop before remediation, PR creation, issue creation, publication, or approval claims.

## Shared Suite Compatibility

This per-skill contract must remain compatible with:

```text
adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md
```
