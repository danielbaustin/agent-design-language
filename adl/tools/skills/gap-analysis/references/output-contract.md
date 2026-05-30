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
- Gap Buckets
- Missing Evidence
- Uncertainty
- Recommended Follow-up
- Artifact Routing
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
- `gap_buckets`
- `missing_evidence`
- `uncertainty`
- `recommended_follow_up`
- `artifact_routing`
- `stop_boundary`

Required `stop_boundary` fields:

- `fixed_gaps`
- `created_issues`
- `created_prs`
- `approved_closeout`
- `approved_release`
- `mutated_repository`

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
- milestone/release bucket when available

## Rules

- Do not infer intended outcomes without an explicit baseline.
- Distinguish missing evidence from proven failure.
- Keep milestone/release truth buckets explicit:
  - `release_blockers`
  - `durable_proof_gaps`
  - `routed_work`
  - `stale_release_readiness_docs`
  - `non_blocking_quality_concerns`
- Use repo-relative, issue-relative, or packet-relative paths.
- Do not write absolute host paths into report artifacts.
- Do not claim approval, release-readiness, merge-readiness, compliance,
  publication approval, or remediation completion.
- Do not create issues, PRs, tests, fixes, docs changes, or closeout edits.
