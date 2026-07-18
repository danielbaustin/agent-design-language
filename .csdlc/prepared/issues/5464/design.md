# #5464 Deterministic cargo-nextest Installation

## Evidence

GitHub-hosted run `29632957768` reported that `cargo-nextest@0.9.140` was
unsupported by the pinned installer and silently fell back to
`cargo-binstall`. The workflow pins `taiki-e/install-action` at commit
`e5c52b603cc5f5e9b52b6a43afad8e9fe0527090`, dated 2026-02-17, while
nextest `0.9.140` was released on 2026-07-05.

Official install-action v2.82.10 commit
`50414676f9f5d50a65992c6dd2ed02641263226c` includes a manifest entry for
nextest `0.9.140`, including the x86_64 Linux release URL and SHA-256.

## Design

Update only cargo-nextest installation steps to the reviewed immutable
v2.82.10 commit. Set `fallback: none` on every such step so an unknown version
or unsupported runner fails clearly instead of changing installation strategy.
Extend the CI runtime contract to require the canonical installer commit,
nextest version, and fail-closed fallback setting for every nextest install.

The PR's GitHub-hosted checks are the live proof surface. Their logs and
annotations must not contain the unsupported-binary or cargo-binstall fallback
warning.

## Boundaries

- Keep nextest pinned at `0.9.140`.
- Preserve immutable full-SHA action pinning and checksum verification.
- No AWS execution.
- No unrelated test-lane or installer changes.
