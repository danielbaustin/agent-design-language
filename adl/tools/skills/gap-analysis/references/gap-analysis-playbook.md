# Gap Analysis Playbook

Use this playbook when comparing expected truth to observed truth.

## Evidence Model

- Expected baseline: the source of intent, such as an issue, feature spec,
  milestone plan, review packet, PR scope, or closeout checklist.
- Observed evidence: the artifacts that prove what happened, such as changed
  files, validation results, docs, reports, PR state, or output cards.
- Finding: a source-grounded mismatch or missing proof between expected and
  observed truth.
- Uncertainty: the honest state when evidence is insufficient.

## Good Findings

- Name the expected baseline.
- Name the observed evidence.
- State why the mismatch matters.
- Preserve uncertainty.
- Recommend a bounded follow-up without doing it.

## Bad Findings

- Treating missing evidence as proof of failure.
- Rewriting the baseline to match the implementation.
- Inventing scope not present in the issue, plan, or spec.
- Calling a task complete without closeout or validation evidence.

## Handoffs

- Use `finding-to-issue-planner` after humans approve follow-up issue creation.
- Use `review-to-test-planner` for validation gaps.
- Use `sor-editor`, `sip-editor`, or `pr-closeout` for card or closeout truth
  drift.
- Use `review-quality-evaluator` for report-publication quality gates.
