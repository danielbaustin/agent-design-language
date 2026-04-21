# Release Evidence Playbook

Use this playbook to assemble one milestone evidence packet without widening
into release ceremony work.

## Evidence Families

- Issue and PR evidence: WBS, issue wave, merged issue summaries, PR lists, or
  closeout records.
- Demo and proof evidence: demo matrix, proof coverage, proof artifacts, or
  recorded demo outcomes.
- Review evidence: internal review, external review, gap reviews, or review
  closeout notes.
- Remediation evidence: finding-to-issue links, remediation status, follow-up
  issue state, or explicit deferrals.
- Validation evidence: checklist, validation command log, CI summary, or release
  readiness validation notes.
- Non-claims: anything the packet must not assert, especially release approval.
- Residual risks: known partials, open issues, skipped demos, unresolved reviews,
  or future milestone dependencies.

## Classification Guide

- Use `not_run` when the milestone evidence root cannot be read or contains no
  evidence documents.
- Use `blocked` when explicit blockers, unresolved high-priority findings, or
  missing release-readiness evidence prevent truthful packaging.
- Use `partial` when evidence exists but one or more families are absent,
  incomplete, or visibly unchecked.
- Use `ready` only when all required evidence families are present and no open
  checklist or blocker markers are visible.

`ready` means ready to review the evidence package. It is not release approval.

