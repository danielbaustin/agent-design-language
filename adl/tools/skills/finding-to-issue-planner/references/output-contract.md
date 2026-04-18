# Output Contract

The finding-to-issue planner skill produces issue candidates from CodeBuddy
review findings without creating tracker items.

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/issue-planning/
```

## Required Artifacts

### issue_candidates.md

Required sections:

- Metadata
- Scope
- Issue Candidates
- Deferred Findings
- Approval Boundary
- Validation Notes
- Residual Risk

Each issue candidate must include:

- candidate id
- proposed title
- severity
- source finding ids
- source roles
- confidence
- affected paths or artifacts
- evidence summary
- problem statement
- acceptance criteria
- validation plan
- non-goals
- dependencies or ordering notes
- approval status

### issue_candidates.json

Required top-level fields:

- `schema`
- `source`
- `tracker`
- `status`
- `candidate_count`
- `deferred_count`
- `candidates`
- `deferred_findings`
- `approval_boundary`

Each candidate object must include:

- `candidate_id`
- `title`
- `severity`
- `source_finding_ids`
- `source_roles`
- `confidence`
- `affected_paths`
- `evidence`
- `problem`
- `acceptance_criteria`
- `validation_plan`
- `non_goals`
- `dependencies`
- `approval_status`

## Status Values

- `pass`: candidates were produced and no findings were deferred.
- `partial`: candidates were produced and at least one finding was deferred.
- `not_run`: no readable findings were available.
- `blocked`: explicit requested behavior would cross the mutation boundary or
  hide severity, disagreement, or missing evidence.

## Rules

- Use repo-relative or packet-relative paths.
- Do not write absolute host paths into artifacts.
- Do not create issues, PRs, remediation branches, tests, or tracker items.
- Do not claim operator approval unless it is explicitly present in the input.
- Preserve highest severity when grouping findings.
- Preserve specialist disagreement in grouped candidates.
- Mark findings without evidence as deferred rather than ready.
