# Output Contract

Default `repo-review-docs` artifacts are Markdown review packets with these sections in this order:

```md
## Metadata
- Skill: repo-review-docs
- Target: <repo/path/branch/diff/packet>
- Date: <UTC timestamp or calendar date>
- Artifact: <path or none>

## Findings
- <priority>: <title>
  Severity: <P0 | P1 | P2 | P3>
  File: <repo-relative path or none>
  Role: docs
  Scenario: <trigger or review condition>
  Impact: <behavioral consequence>
  Evidence: <specific code/doc/test/config observation>

## Documentation Objects
- <bounded object> (`schema` | `skill` | `review artifact` | `demo` | `guide` | `runbook` | `release note`)

## Commands Or Claims Checked
- <bounded list or explicit none>

## Validation Performed
- <command and what it proved, or explicit not-run rationale>

## Residual Risk
- <what this role did not inspect or could not prove>

If no docs review was performed, write `Documentation Review: skipped`.
State skip reason, owner, and follow-up action in `Residual Risk`.

If findings are found, preserve the same severity framing used by other
specialist lanes and call out any disagreement from synthesis or peer artifacts.
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
