# GWS Safe Defaults And Scope Checklist

## Default Posture

Use these defaults unless the project has an explicit reason to widen them:

- one Drive folder only
- one document only
- one sheet only
- one bounded sheet range only
- `dry_run` first
- execute mode only after bounded proof surfaces are already in place

## Scope Checklist

- Folder scope is one explicit project folder, not a team-wide workspace root.
- Doc scope is one explicit planning/review doc, not arbitrary Docs access.
- Sheet scope is one explicit content-card sheet, not arbitrary Sheets access.
- Range scope is one explicit bounded range, not an unbounded worksheet.

## Safety Checklist

- `gws` auth is operator-visible.
- Credential material is never copied into tracked artifacts.
- Missing auth is recorded as skipped, not hidden.
- Missing scopes are recorded as skipped, not hidden.
- Unavailable tooling is recorded as skipped, not hidden.
- Dry-run and execute postures are visibly different in the proof artifacts.
- Live mutation stops on revision mismatch.
- Live mutation stops on doc/content-card binding mismatch.
- Workspace output does not silently mutate tracked repo truth.

## Project Policy Checklist

- The project has an issue-backed promotion rule.
- GitHub PRs still gate canonical tracked-doc changes.
- Workspace is used for draft/content-card workflow, not authoritative repo
  state.
- The team agrees on who may run execute mode.
- The team knows where proof artifacts are stored.

## Unsafe Defaults To Reject

- broad shared Drive inventory by default
- multi-doc ambient read scope
- execute mode as the initial setup posture
- silent content-card writes without revision-anchor checks
- promoting Workspace state straight into tracked files without issue/PR review
