# Doc Cleanup Reconciliation - Issue 1762

## Purpose

Issue 1762 was originally parked during v0.88 closeout to capture stale
documentation and workflow-surface cleanup debt. This pass converts it from a
rediscovery note into a bounded cleanup record: what was inspected, what was
updated safely, and what should not be treated as disposable cruft.

## What Was Inspected

- The tracked docs tree contains 523 documentation-like files across 88
  directories.
- The oldest tracked docs are mostly milestone archives, ADRs, security notes,
  and early design records.
- No tracked temporary, rejected-patch, backup, or local coverage-profile files
  were found in the issue worktree.
- The root checkout contains ignored local coverage artifacts named
  `default_*.profraw`. They are generated LLVM/Rust coverage profile files,
  already covered by `.gitignore`, and are not tracked docs.

## Safe Cleanup Applied

- `docs/README.md` now names v0.90 as the active milestone package while
  preserving v0.89.1 as the most recently completed milestone.
- `docs/planning/ADL_FEATURE_LIST.md` now distinguishes active milestone
  status from the unreleased crate version. The crate remains 0.89.1 until the
  v0.90 release bump, but v0.90 is the active milestone wave.
- `docs/planning/README.md` now states directory boundaries so living planning,
  milestone records, historical records, tooling docs, local-only workspace
  state, and generated artifacts do not collapse into one undifferentiated
  pile.

## Not Cruft

These surfaces looked old or lightly referenced but should not be removed by a
generic cleanup pass:

- `docs/milestones/v0.2` through `docs/milestones/v0.89.1` are historical
  milestone records.
- `docs/adr/` is an architecture-decision archive.
- `docs/records/` contains historical task mirrors and closeout records kept
  for auditability.
- `docs/templates/` contains reusable milestone templates.
- `docs/security/` contains threat-model and security-reference material.
- Future milestone directories such as `docs/milestones/v0.91` through
  `docs/milestones/v0.95` are roadmap surfaces, not abandoned drafts.

## Candidate Follow-Ups

These are cleanup candidates, but they should be handled by narrower issues
rather than broad deletion:

- Cross-link standalone tooling/demo contracts from the relevant demo matrices
  or move them into clearer tooling subdirectories.
- Review `docs/tooling/editor/pr_run_demo.md` after the current editor-skill
  wiring lands, and either keep it explicitly historical or replace it with a
  current runnable editor demo guide.
- Add a lightweight generated-artifact cleanup command for ignored local files
  such as coverage profiles, if local worktrees keep accumulating them.
- Periodically review `docs/planning/` so it remains a small cross-milestone
  planning index instead of a second milestone archive.

## Boundary Decision

This cleanup intentionally did not delete milestone archives, ADRs, records, or
future-roadmap docs. Most of the apparent age in the docs tree is evidence of
completed milestone history, not unused garbage.
