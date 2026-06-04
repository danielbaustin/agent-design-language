# ADL Octocrab Migration Review

This packet records the v0.91.5 octocrab mini-sprint state after the first
typed GitHub client migration wave.

## Scope

The mini-sprint migrated GitHub issue and PR metadata interpretation toward a
shared typed client layer while preserving `adl/tools/pr.sh` as the canonical
agent-facing workflow entrypoint.

This packet is a migration review surface, not a claim that every GitHub
operation now uses live octocrab network calls.

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

## Mode Policy

- `ADL_GITHUB_CLIENT=auto` uses an octocrab-backed configuration when
  `GITHUB_TOKEN` or `GH_TOKEN` is present, otherwise it uses `gh` fallback.
- `ADL_GITHUB_CLIENT=octocrab` requires `GITHUB_TOKEN` or `GH_TOKEN` and fails
  closed without a token.
- `ADL_GITHUB_CLIENT=gh` explicitly selects `gh` fallback.
- `ADL_GITHUB_DISABLE_GH_FALLBACK=1` disables shell fallback and fails closed
  for shell-backed modes.
- Live operations that are still shell-backed may use `gh` only when fallback is
  allowed. If `ADL_GITHUB_CLIENT=octocrab` explicitly selects octocrab, or
  fallback is disabled, those live paths fail closed until a trait-backed
  octocrab implementation exists for the operation.

The stable failure code for disabled shell fallback is
`github_client.fallback_disabled`.

## Remaining Shell-Backed Operations

The following operations still route through the existing `gh` command path:

- issue creation and issue body updates
- issue title and version-label updates
- PR creation, body updates, readiness changes, and merge operations
- live PR list/view calls used by workflow guardrails
- live issue and PR linkage checks where the current code calls `gh`

This is intentional for this migration wave. The typed helpers now own shared
interpretation and shell-fallback admission, but live network mutation was not
migrated in this slice.

## Follow-On Routes

The next octocrab follow-ons should stay behavior-preserving and separately
reviewable:

- add a trait-backed live issue read/list path with `gh` parity tests
- add a trait-backed live PR read/list path with `gh` parity tests
- migrate issue create/edit operations after read/list parity is proven
- migrate PR create/update operations after PR read/list parity is proven
- add observability for fallback selection, slow GitHub calls, and command
  timeouts
- add an optional release-gate smoke check for token-backed octocrab auth

## Non-Claims

- This packet does not claim all GitHub operations are octocrab-backed.
- This packet does not remove the canonical `adl/tools/pr.sh` workflow wrapper.
- This packet does not introduce GitHub App authentication.
- This packet does not change generated-card command policy.
