# Issue #5587 design: production Google Drive context mirror

## Problem

The context-mirror executable selects an in-memory transport even in execute mode, verifies only Drive metadata after writes, and reports recursive mirroring as pending without traversing the configured repository surfaces.

## Design

Keep the existing deterministic planning and write-approval boundary. Select the native OAuth Drive transport only for execute mode; retain the in-memory transport for fixture and dry-run proof. Extend the transport contract with content reads so create/update/unchanged decisions and post-write verification compare bytes. Traverse regular Markdown files beneath `docs/` and `.adl/docs/TBD/`, preserve their repository-relative folder hierarchy beneath the configured Drive root, and fail closed on symlinks, path escapes, ambiguous Drive children, credentials, API errors, or verification mismatch.

Emit a machine-readable report that distinguishes seed, recursive, unchanged, failed, and verified results without exposing credentials or file contents. Automation may archive its Codex task only when the production command exits successfully with a fully verified report; failures remain visible and repeated equivalent failures are deduplicated.

## Boundaries

- No AWS or Spot execution.
- No broad Google Workspace scope; retain `drive.file` and metadata-read scope.
- No deletion of remote content.
- No silent success when recursive traversal or readback is incomplete.
