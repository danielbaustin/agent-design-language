# Risks

- Removing receipt writers may accidentally remove the reader needed for legacy compatibility indexing.
- Store terminal-repair code is interleaved with current lifecycle persistence helpers, so broad deletion could break non-terminal operations.
- Public schema and library exports can silently preserve unsupported mutation authority even after the CLI binary is deleted.
- Existing tests may encode obsolete closeout behavior and hide a missing negative guard against its reintroduction.
- Active documentation or installer manifests may continue advertising the deleted binary.
- Historical architecture and evidence files must remain immutable even when they describe the retired design.
