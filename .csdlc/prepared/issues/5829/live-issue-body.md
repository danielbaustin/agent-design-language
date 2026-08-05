## Summary
Execute **WP-12** in v0.92: the provider, model, tool, skill, authority, and limit capability envelope.

## Dependencies
- WP-08 issue #5825
- WP-09 issue #5826

## Required Outcome
Produce the typed capability envelope, fixtures, enforcement boundary, and validation report on the established birth and identity contracts.

## Acceptance Criteria
- The envelope binds provider, model, tools, skills, authority, limits, revision, and provenance.
- Missing, forged, stale, over-broad, and unsupported capabilities fail closed.
- Shared Runtime registration is serialized behind WP-11 issue #5828.
- Exact tests and native receipts are source-SHA, argv, runner, output-digest, and artifact bound.
- The implementation PR includes `Closes #5829`.

## Non-goals
- No provider implementation, identity redesign, Memory Palace implementation, or governance expansion.

<!-- csdlc-github-operation:v092-wp12-dependency-reconciled -->
