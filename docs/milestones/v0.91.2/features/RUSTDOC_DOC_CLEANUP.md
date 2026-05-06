# Rustdoc And Documentation Cleanup

## Metadata

- Feature Name: Rustdoc And Documentation Cleanup
- Milestone Target: `v0.91.2`
- Status: planned
- Planned WP Home: WP-13
- Source Docs: `.adl/docs/TBD/RUSTDOC_GAP_ANALYSIS.md`; `.adl/docs/TBD/ADL_DOC_CLEANUP_LEDGER.md`
- Proof Modes: docs, checks, review

## Purpose

Close rustdoc and documentation hygiene gaps that remain visible in local TBD
tracking. This is a repo-truth cleanup lane, not a cosmetic rewrite.

## Scope

In scope:

- Rustdoc gap remediation plan and patches.
- Doc cleanup ledger update.
- Stale milestone claim cleanup.
- Validation for changed docs.

Out of scope:

- Broad rewrite without issue scope.
- Retiring source packets before their underlying gaps are closed.
- Unsupported implementation claims.

## Acceptance Criteria

- Rustdoc/doc claims match current code.
- No host paths or unresolved scaffold language remain in promoted docs.
- Cleanup evidence is recorded.
