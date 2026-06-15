# Native Release And Watcher GitHub Support Proof (#3718)

Status: ready_for_review
Issue: #3718
Milestone: v0.91.5

## Summary

This packet records the focused proof for replacing the release-ceremony GitHub release operations with the Rust/octocrab ADL control-plane path.

The implemented surface adds `adl tooling github-release` with native octocrab-backed operations for:

- `ensure-absent`
- `ensure-present`
- `draft`
- `publish`

`adl/tools/release_ceremony.sh` now delegates covered GitHub release operations to that Rust command instead of calling `gh release` directly. The release ceremony test harness uses a fake `adl tooling github-release` command so the shell workflow proves delegation without touching real GitHub release state.

## Covered Behavior

- Release preflight can confirm that a release tag is absent.
- Release preflight can confirm that a release tag is present.
- Draft release creation is routed through the Rust/octocrab command surface.
- Release publication is routed through the Rust/octocrab command surface.
- The shell ceremony supports an explicit `ADL_RELEASE_GITHUB_CMD` override for tests and controlled environments.
- The shell ceremony supports an explicit `ADL_RELEASE_GITHUB_REPO` override to avoid brittle remote parsing in tests.
- The Rust command reads credentials from `GITHUB_TOKEN` or `GH_TOKEN` and does not log token values.

## Watcher Boundary

The `issue-watcher` skill remains an observation and routing skill. This issue does not introduce a daemon or background watcher.

Watcher-related policy remains: use repo-native GitHub metadata surfaces where available, do not rely on `gh` as hidden authority, and fail closed or route follow-up when a covered operation lacks a native path.

## Non-Claims

- No real GitHub release was created, published, or deleted by this proof.
- Release asset upload is not covered by this issue.
- Release deletion is not covered by this issue.
- This issue does not remove every unrelated legacy `gh` call in the repository.
- This issue does not create a generalized GitHub event watcher.

## Validation

Focused validation was run from the bound #3718 worktree.

```bash
cargo test --manifest-path adl/Cargo.toml github_release -- --nocapture
```

Result: passed.

```bash
bash adl/tools/test_release_ceremony.sh
```

Result: passed.

## Review Notes

The proof is intentionally narrow. It shows the release ceremony's covered GitHub release actions now have a typed Rust/octocrab path and that the shell wrapper delegates to it. Remaining GitHub transport work should be routed as follow-on issues rather than hidden inside #3718.
