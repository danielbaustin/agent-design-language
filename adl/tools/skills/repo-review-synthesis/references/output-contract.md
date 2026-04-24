# Output Contract

Default `repo-review-synthesis` artifacts are Markdown review packets with these sections in this order:

```md
## Metadata
- Skill: repo-review-synthesis
- Target: <repo/path/branch/diff/packet>
- Date: <UTC timestamp or calendar date>
- Specialist Artifacts:
  - code: <path or missing>
  - security: <path or missing>
  - tests: <path or missing>
  - docs: <path or missing>
  - architecture: <path or missing>
  - dependency: <path or missing>

## Findings
- <priority>: <title>
  Severity: <P0 | P1 | P2 | P3>
  Source Roles: <role list>
  File: <repo-relative path or none>
  Scenario: <trigger or review condition>
  Impact: <behavioral consequence>
  Evidence: <merged evidence without hiding disagreement>

## Coverage Matrix
- Code: present | missing | skipped
- Security: present | missing | skipped
- Tests: present | missing | skipped
- Docs: present | missing | skipped
- Architecture: present | missing | skipped
- Dependency: present | missing | skipped

`Docs` must be `present`, `missing`, or `skipped`. If `skipped`, include a skip
reason, owner, and follow-up owner in `Residual Risk` and a concrete follow-up
issue candidate unless the skip was pre-approved.

## Dedupe Notes
- <dedupe decision or explicit none>

## Disagreements
- <disagreement or explicit none>

## Validation Performed
- <commands run by specialists or explicit not-run rationale>

## Residual Risk
- <missing coverage, uncertainty, or explicit none>
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
