# Gap Analysis Skill Input Schema

Schema id: `gap_analysis.v1`

## Purpose

Compare an explicit expected baseline against observed implementation, docs,
tests, review, PR, milestone, or closeout evidence and produce a bounded
findings-first gap report.

For milestone and release work, this schema should support repeatable truth
maintenance: separating real blockers from routed work, stale release/readiness
docs, durable proof gaps, and lower-risk quality concerns.

## Required Top-Level Fields

- `skill_input_schema`: must be `gap_analysis.v1`.
- `mode`: one of `compare_issue_to_implementation`,
  `compare_milestone_to_evidence`, `compare_spec_to_docs`,
  `compare_review_to_closeout`, or `compare_packet_to_report`.
- `expected_baseline`: issue, milestone, spec, PR, review packet, or closeout
  source of intended truth.
- `observed_evidence`: implementation, docs, tests, review, PR, report, or
  closeout evidence to compare.
- `policy`: comparison and stop-boundary policy.

## Optional Fields

- `artifact_root`: report destination.
- `milestone_version`
- `quality_gate_target`
- `gap_review_target`

## Mode-Specific Nested Fields

Place mode-specific source fields under `expected_baseline` and
`observed_evidence`.

- `compare_issue_to_implementation`
  - `expected_baseline.issue_ref`
  - `expected_baseline.acceptance_criteria_path`
  - `observed_evidence.changed_paths`
- `compare_milestone_to_evidence`
  - `expected_baseline.milestone_plan`
  - `observed_evidence.evidence_root`
  - `observed_evidence.truth_sources`
- `compare_spec_to_docs`
  - `expected_baseline.spec_path`
  - `observed_evidence.docs_paths`
- `compare_review_to_closeout`
  - `expected_baseline.review_artifact`
  - `observed_evidence.closeout_record`
- `compare_packet_to_report`
  - `expected_baseline.review_packet_path`
  - `observed_evidence.report_path`

## Policy Fields

- `quality_gate_update_allowed`
- `separate_gap_review_allowed`
- `stop_before_fix`
- `stop_before_mutation`

## Example

```yaml
skill_input_schema: gap_analysis.v1
mode: compare_milestone_to_evidence
expected_baseline:
  milestone_plan: docs/milestones/v0.91.4/SPRINT_v0.91.4.md
  quality_gate_target: docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md
observed_evidence:
  truth_sources:
    - .adl/docs/TBD/v0.91.4_gap_review.md
    - docs/milestones/v0.91.4/FEATURE_PROOF_COVERAGE_v0.91.4.md
    - docs/milestones/v0.91.4/DEMO_MATRIX_v0.91.4.md
    - docs/milestones/v0.91.4/RELEASE_PLAN_v0.91.4.md
  evidence_root: .adl/reviews/gap-analysis/v0.91.4/
policy:
  quality_gate_update_allowed: true
  separate_gap_review_allowed: true
  stop_before_fix: true
  stop_before_mutation: true
```

## Output Contract

Default artifact root:

```text
.adl/reviews/gap-analysis/<run_id>/
```

Required artifacts:

- `gap_analysis_report.md`
- `gap_analysis_report.json`

Statuses:

- `pass`: no gaps found.
- `partial`: gaps or missing evidence exist.
- `fail`: severe gap should block the requested closeout or release decision.
- `not_run`: explicit baseline missing.
- `blocked`: requested action violates stop boundary.

Milestone/release runs should also emit:

- `gap_buckets`
- `artifact_routing`

## Stop Boundary

The skill must not fix implementation, docs, tests, cards, or reports; create
issues or PRs; approve closeout, publication, or release readiness; or mutate
repositories.
