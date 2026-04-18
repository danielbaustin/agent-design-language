# Output Contract

The gap-analysis skill produces findings-first gap reports from an explicit
expected baseline and observed evidence.

Default artifact root:

```text
.adl/reviews/gap-analysis/<run_id>/
```

## Required Artifacts

### gap_analysis_report.md

Required sections:

- Gap Analysis Summary
- Scope
- Expected Baseline
- Observed Evidence
- Findings
- Missing Evidence
- Uncertainty
- Recommended Follow-up
- Stop Boundary

### gap_analysis_report.json

Required top-level fields:

- `schema`
- `run_id`
- `status`
- `scope`
- `expected_baseline`
- `observed_evidence`
- `findings`
- `missing_evidence`
- `uncertainty`
- `recommended_follow_up`
- `stop_boundary`

## Status Values

- `pass`: no gaps were found for the supplied baseline and evidence.
- `partial`: gaps or missing evidence exist, but the comparison was bounded and
  useful.
- `fail`: a severe gap or closeout/release truth mismatch should block the
  requested closeout or release decision.
- `not_run`: no explicit expected baseline was available.
- `blocked`: the requested action would require fixing, approving, publishing,
  creating issues or PRs, or mutating a repository.

## Finding Shape

Each finding must include:

- stable id
- gap type
- severity
- title
- expected
- observed
- evidence
- uncertainty
- recommended follow-up
- source artifact or path when available

## Rules

- Do not infer intended outcomes without an explicit baseline.
- Distinguish missing evidence from proven failure.
- Use repo-relative, issue-relative, or packet-relative paths.
- Do not write absolute host paths into report artifacts.
- Do not claim approval, release-readiness, merge-readiness, compliance,
  publication approval, or remediation completion.
- Do not create issues, PRs, tests, fixes, docs changes, or closeout edits.
