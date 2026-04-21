# Review Readiness Cleanup Playbook

Use this playbook before formal review cycles to remove avoidable structure
noise without changing review substance.

## Inspect For

- Review plan presence and stale status markers.
- Finding register presence and unresolved blocker markers.
- Demo or proof register presence when proof evidence is expected.
- Packet manifest, source paths, and validation notes.
- Placeholder leakage such as `TBD`, `TODO`, `FIXME`, or empty template sections.
- Explicit skipped surfaces and their rationale.
- Follow-on cleanup that should be queued but should not block this review.

## Classification Guide

- Use `safe_mechanical_cleanup` for stale headings, placeholders, missing
  metadata, or small structure drift that can be fixed without changing
  substantive review findings.
- Use `blocker` for missing required surfaces, explicit blocker markers, or
  high-priority review truth that makes the packet misleading.
- Use `skipped` for intentionally absent, unavailable, or out-of-scope surfaces.
- Use `follow_on_needed` for cleanup that is useful but not required before the
  current review cycle.

Do not rewrite findings, hide disagreement, or approve readiness from this
classification.

