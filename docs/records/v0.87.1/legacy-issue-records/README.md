# Legacy Tracked `.adl` Issue Records

These files were previously tracked under `.adl/.../bodies/issue-*` and
`.adl/.../tasks/issue-*`.

They are preserved here as a historical archive because they contain real
workflow provenance, but they are no longer allowed to live under tracked
`.adl/` paths.

Rules for this archive:

- preserve the original file contents as historical evidence
- preserve the original relative path shape under `legacy-issue-records/`
- do not treat these files as live canonical workflow state
- do not add new issue execution records here unless the repository explicitly
  needs a historical/public proof surface

The active canonical model is:

- local live execution state: `.adl/`
- tracked historical/public examples: `docs/records/`

If future work needs a public example, create it intentionally under
`docs/records/` rather than force-tracking `.adl/`.
