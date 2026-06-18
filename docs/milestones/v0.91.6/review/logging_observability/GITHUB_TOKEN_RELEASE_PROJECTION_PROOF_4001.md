# WP-03 GitHub, Token, Release, and Projection Observability Proof

## Metadata

- Issue: `#4001`
- Milestone: `v0.91.6`
- Wave: `WP-03`
- Date: `2026-06-17`
- Scope: GitHub credential handling, release-helper publish proof, and card/GitHub projection convergence status

## Summary

`#4001` completes the remaining WP-03 tooling-observability lane by:

- routing `adl tooling github-release` through the shared ADL GitHub client policy rather than a private ad hoc token lookup;
- proving the native draft/publish release helper still works through octocrab-backed ADL tooling;
- recording that `#3935` first-tranche PR-body projection convergence is already landed and consumed by the WP-03 packet;
- explicitly routing the broader existing-issue metadata repair gap in `#3985` rather than overclaiming that it is solved here.

## Input-Issue Disposition

| Input issue | Disposition under `#4001` | Notes |
| --- | --- | --- |
| `#3935` | `consumed` | `SOR`-driven PR-body convergence and drift repair are already merged and were used directly during WP-03 publication and issue-closeout. |
| `#3963` | `partially_consumed` | The release helper now reuses the shared ADL GitHub credential policy instead of reading `GITHUB_TOKEN` / `GH_TOKEN` privately. Broader cross-command resolver/cache work remains tracked in `#3963`. |
| `#3965` | `consumed` | Native octocrab-backed draft and publish flows are proven together through the existing release-helper transport test plus the shared-client migration in this issue. |
| `#3985` | `routed` | Existing-issue title/label/body repair remains adjacent tooling work and is not required to complete the WP-03 logging/release observability lane. |

## Code Surfaces

- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd/github_client.rs`
- `adl/src/cli/tooling_cmd/github_release.rs`

## What Changed

### Shared credential policy reuse

`adl tooling github-release` now builds octocrab through the same ADL GitHub
client substrate used by the C-SDLC issue/PR transport.

That gives the release helper:

- deterministic `GITHUB_TOKEN` / `GH_TOKEN` precedence aligned with the main GitHub client;
- fail-closed behavior when `ADL_GITHUB_CLIENT=gh` would otherwise request raw-`gh` fallback;
- token-source diagnostics that stay redacted and do not print token bytes.

### Release publish proof

The native release helper already contained an octocrab-backed draft/publish
transport test for the draft-release regression captured in `#3965`. `#4001`
keeps that proof live while moving credential resolution onto the shared policy.

### Projection convergence status

WP-03 publication and closeout already depended on `#3935`’s first-tranche
managed projection contract:

- PR body truth is rendered from `SOR`;
- missing closing linkage is repaired from the canonical `SOR` projection;
- GitHub drift is not silently treated as authoritative lifecycle truth.

`#4001` therefore treats `#3935` as consumed input rather than reopening that
already-merged work.

## Validation

- `cargo test --manifest-path adl/Cargo.toml github_release_octocrab_covers_absent_draft_present_publish -- --nocapture`
  - Proved native octocrab-backed release draft and publish still work through the ADL helper.
- `cargo test --manifest-path adl/Cargo.toml github_release_octocrab_accepts_gh_token_when_github_token_missing -- --nocapture`
  - Proved the shared credential policy accepts `GH_TOKEN` when `GITHUB_TOKEN` is absent.
- `cargo test --manifest-path adl/Cargo.toml github_release_rejects_gh_fallback_mode_even_with_token -- --nocapture`
  - Proved the release helper fails closed instead of honoring raw-`gh` fallback mode.

## Non-Claims

- This issue does not claim a full GitHub credential cache, key-file resolver, or repo-wide credential-manager substrate. That broader work remains tracked in `#3963`.
- This issue does not claim to solve all existing-issue metadata repair semantics. That broader repair path remains tracked in `#3985`.
- This issue does not broaden PR/issue projection ownership beyond the first-tranche `#3935` PR-body / closing-linkage contract already merged.
