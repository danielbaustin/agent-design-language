# ADL Octocrab Migration Review

This packet records the v0.91.5 octocrab migration state after the first
typed GitHub client migration wave and the follow-on operational transport
slice.

## Scope

The mini-sprint migrated GitHub issue and PR metadata interpretation toward a
shared typed client layer while preserving `adl/tools/pr.sh` as the canonical
agent-facing workflow entrypoint.

This packet is a migration review surface. The current C-SDLC issue and PR
workflow helpers use live octocrab transport for covered GitHub operations
instead of silently spawning `gh`.

## Completed Slices

- `#3637` added the typed GitHub client contract, mode selection, token-source
  discovery, stable error codes, and diagnostic redaction.
- `#3638` moved issue metadata parity interpretation onto typed helpers while
  preserving existing shell-backed issue operations.
- `#3639` moved PR wave filtering and closing-linkage interpretation onto typed
  helpers while preserving existing shell-backed PR operations.
- `#3640` documented the shared C-SDLC GitHub client boundary and proved that
  `adl-csdlc` stays on the shared PR control-plane path.
- `#3641` added fail-closed shell fallback disablement and this migration review
  packet.
- `#3672` wires that fallback policy into live shell-backed issue and PR
  workflow operations so disabled fallback and explicit octocrab mode fail
  before spawning `gh`.
- `#3697` moves the covered C-SDLC issue and PR workflow helper paths onto
  octocrab-backed transport and removes ambient legacy GitHub CLI repo-view discovery.
- The `#3697` live smoke path verifies token-backed
  `ADL_GITHUB_CLIENT=octocrab` doctor execution and emits
  `github_octocrab` start/completed events for each covered GitHub operation.

## Mode Policy

- `ADL_GITHUB_CLIENT=auto` uses octocrab-backed transport when `GITHUB_TOKEN`
  or `GH_TOKEN` is present.
- `ADL_GITHUB_CLIENT=octocrab` requires `GITHUB_TOKEN` or `GH_TOKEN` and fails
  closed without a token.
- `ADL_GITHUB_CLIENT=gh` is no longer an operational fallback for covered
  C-SDLC GitHub workflow operations, including bounded read paths such as live
  current-PR lookup and other covered issue/PR metadata reads.
- Missing token, explicit `gh` fallback, or unsupported operations fail closed
  rather than spawning `gh`.

The stable failure code for disabled shell fallback is
`github_client.fallback_disabled`.

## Covered Octocrab Operations

The following workflow helper operations now route through octocrab:

- issue creation and issue body updates
- issue title and version-label updates
- PR creation, body updates, readiness changes, and merge operations
- live PR list/view calls used by workflow guardrails
- live issue and PR linkage checks where the current code calls `gh`
- operation-level `github_octocrab` start/completed/failed logs around covered
  live GitHub requests

The helper function names still contain `gh` in a few places for compatibility
with the existing PR control-plane call graph, but those helpers no longer
spawn the GitHub CLI for covered operations.

## Follow-On Routes

The next octocrab follow-ons should stay behavior-preserving and separately
reviewable:

- rename legacy `gh_*` helper functions once the transport behavior has settled
- add richer observability for fallback selection, slow GitHub calls, and
  command timeouts beyond the current per-operation Octocrab events
- add an optional release-gate smoke check for token-backed octocrab auth

## Non-Claims

- This packet does not claim non-GitHub shell commands such as `git` have been
  replaced.
- This packet does not remove the canonical `adl/tools/pr.sh` workflow wrapper.
- This packet does not introduce GitHub App authentication.
- This packet does not change generated-card command policy.
