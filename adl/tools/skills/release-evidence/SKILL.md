---
name: release-evidence
description: Assemble milestone release proof surfaces into a bounded reviewable evidence package covering issue and PR evidence, demos, reviews, remediation, validation, non-claims, and residual risks without approving releases, publishing notes, tagging, merging, or closing issues.
---

# Release Evidence

Assemble one milestone release-evidence packet from existing milestone, review,
demo, validation, issue, and PR records. This is an evidence packaging skill,
not a release authority.

Use this skill during milestone closeout preparation, release-readiness review,
or internal/external review handoff when operators need one bounded artifact
that says what is proven, what is partial, what is blocked, and what is not
claimed.

## Quick Start

1. Confirm the milestone identifier and evidence root.
2. Confirm the packet is evidence-only and must stop before release approval.
3. Run the deterministic helper when local filesystem access is available:
   - `scripts/assemble_release_evidence.py --milestone <version> --milestone-root docs/milestones/<version> --out .adl/reviews/release-evidence/<run_id> --run-id <run_id>`
4. Review the Markdown and JSON artifacts.
5. Hand off any gaps to `gap-analysis`, `finding-to-issue-planner`, or the
   appropriate PR lifecycle skill. Do not merge, tag, close, or publish from
   this skill.

## Required Inputs

At minimum, gather:

- `mode`
- `milestone`
- `evidence`
- `policy`

Supported modes:

- `assemble_milestone_evidence`
- `refresh_release_evidence`
- `inspect_release_evidence`

Useful policy fields:

- `write_evidence_artifact`
- `include_open_issues`
- `require_review_records`
- `require_validation_commands`
- `stop_before_release_approval`
- `stop_before_mutation`

If no milestone or evidence root is available, return `not_run`.

## Classification

Classify the package as:

- `ready`: required evidence families are present and no open release checklist
  or blocker markers are visible.
- `partial`: evidence can be assembled but important proof, review, validation,
  or closeout surfaces are missing or still open.
- `blocked`: explicit blocker markers, unresolved high-priority findings, or
  missing release-readiness evidence prevent truthful release-evidence packaging.
- `not_run`: the milestone evidence root is missing, unreadable, or empty.

Do not use `ready` as release approval. It only means the evidence packet itself
is complete enough to review.

## Output

Write Markdown and JSON artifacts when an output root is available.

Default artifact root:

```text
.adl/reviews/release-evidence/<run_id>/
```

Required artifacts:

- `release_evidence_report.md`
- `release_evidence_report.json`

Use the detailed contract in `references/output-contract.md`.

## Stop Boundary

This skill must not:

- approve a release or claim release approval
- merge PRs, close issues, create tags, or publish release notes
- rewrite milestone truth to make evidence look complete
- execute demos beyond reading their recorded evidence
- replace `gap-analysis`, `demo-operator`, `pr-finish`, `pr-closeout`, or
  release ceremony review

Handoff candidates:

- `gap-analysis` when expected release scope needs comparison against evidence.
- `finding-to-issue-planner` when gaps should become reviewable issue
  candidates.
- `demo-operator` when one missing proof surface needs a bounded demo run.
- `pr-closeout` when merged issues need truthful local closeout.

