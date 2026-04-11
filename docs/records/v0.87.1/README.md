# v0.87.1 Historical Record Surfaces

This directory holds tracked historical record material that we intentionally
preserved during the `v0.87.1` PR-workflow hardening work.

Current repo truth:

- live issue execution records belong in local `.adl/` bundles and should remain
  untracked
- tracked `docs/records/` surfaces are historical/public examples only
- new tracked `.adl/.../bodies/issue-*` and `.adl/.../tasks/issue-*` paths are
  banned by tooling guardrails

The `legacy-issue-records/` subtree preserves older tracked `.adl` issue-record
artifacts verbatim so their provenance is not lost while the active workflow
returns to the local-only model.
