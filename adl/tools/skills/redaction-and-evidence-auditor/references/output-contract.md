# Output Contract

The redaction and evidence auditor produces a publication-safety gate report for
a CodeBuddy review packet, specialist review bundle, or final report.

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/redaction-audit/
```

## Required Artifacts

### redaction_report.json

Required fields:

- `schema`
- `status`
- `publication_recommendation`
- `audience`
- `artifact_root`
- `files_scanned`
- `findings`
- `counts`
- `started_at`
- `completed_at`
- `notes`

Each finding must include:

- `severity`
- `category`
- `path`
- `line`
- `message`
- `sample`
- `recommendation`

The `sample` field must mask secret-like material.

### redaction_report.md

Required sections:

- `# Redaction And Evidence Audit`
- `## Verdict`
- `## Publication Recommendation`
- `## Scope`
- `## Findings`
- `## Evidence Boundary Notes`
- `## Required Follow-Up`

## Status Rules

- `pass`: no blocker or warning findings.
- `partial`: warning findings exist but no blockers.
- `fail`: one or more blocker findings exist.
- `not_run`: the target could not be audited.

## Publication Recommendation

- `allow_internal`: no blocking or warning issues for internal use.
- `hold_for_review`: warnings exist or publication context is incomplete.
- `block_publication`: blockers exist or manifest already forbids publication.
- `not_run`: audit did not run.

## Rules

- Do not include full secret values.
- Use paths relative to the audited artifact root.
- Do not write absolute host paths into the audit artifacts.
- Do not mutate audited artifacts.
- Do not claim security approval beyond the scanned packet or report.
- If `publication_allowed` is false in `run_manifest.json`, preserve that
  constraint unless an explicit downstream operator changes it.

