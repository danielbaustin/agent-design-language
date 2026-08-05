# WP-01B Canonical Documentation And Version Activation Design

## Outcome And Boundary

Issue #5818 makes v0.92 the truthful active-development version across current
repository entrypoints before product implementation starts. It updates current
indexes and version declarations; it does not rewrite historical milestone,
release, review, migration, or evidence artifacts.

The canonical feature inventory remains `docs/planning/ADL_FEATURE_LIST.md`.
The active milestone package under `docs/milestones/v0.92/` supplies planned
feature ownership and must not be converted into implementation-complete truth.
The latest completed release remains v0.91.8 until separate release authority
changes that fact.

## Source-Grounded Surface Inventory

The implementation session must classify every candidate as `update`,
`already_current`, `historical_preserve`, or `not_authoritative` in a retained
machine-readable inventory. Candidate current surfaces are:

- `README.md`, `docs/README.md`, and current package README entrypoints;
- `docs/planning/ADL_FEATURE_LIST.md` and current planning indexes;
- `docs/milestones/v0.92/` feature, quality, demo, review, and execution links;
- root and workspace `Cargo.toml` files, `Cargo.lock`, and user-visible package
  metadata that declare the current ADL version;
- `AGENTS.md`, `REVIEW.md`, `csdlc-v2/operator/skills/`, and current tooling
  runbooks where active lifecycle or milestone wording is exposed.

Historical directories are scan inputs but are protected from version rewriting.
Generated or vendored files are changed only through their owning generator.

## Execution Design

1. Build the checked-surface inventory before editing and identify each
   authoritative current-version declaration.
2. Update the canonical feature inventory and current documentation indexes,
   preserving `planned`, `active`, and `implemented` distinctions.
3. Update authoritative package/version declarations to `0.92.0`; regenerate
   `Cargo.lock` only through Cargo and inspect the resulting diff.
4. Repair current links and lifecycle language only where source evidence shows
   drift from the final C-SDLC v2 authority.
5. Run focused format, link, structured-data, Cargo metadata, version-parity,
   stale-reference, and historical-preservation checks.

The issue-local validator at
`.csdlc/prepared/issues/5818/validate-activation.rb` owns the deterministic
inventory, Markdown-link, YAML/JSON parse, version-parity, and historical-diff
checks. Historical preservation excludes only `.csdlc/evidence/5818/`, which
is new proof owned by this issue; it does not exempt any pre-existing evidence.

## Invariants And Failure Policy

- No v0.92 feature becomes complete merely because its documentation is active.
- Historical evidence retains its original version, dates, and claims.
- No product behavior, repository transfer, release ceremony, or child closeout
  occurs in this issue.
- A broad stale-reference scan is classification evidence, not permission for a
  repository-wide replacement.
- Stop on ambiguous version authority, generated-file ownership, or overlap with
  another active protected path.

## Rollback

The change is a single reviewed documentation/version activation commit. Revert
that commit if parity or historical-preservation checks fail; do not hand-edit
only part of a generated metadata set.

## Proof Design

Proof consists of the retained surface inventory, deterministic version-parity
and historical-preservation checks, Markdown/link and YAML/JSON validation,
Cargo metadata plus locked check, diff hygiene, and an exact-revision bounded
review. Broad runtime tests are not required unless executable version behavior
is changed.
