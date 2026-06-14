# Octocrab Mini-Sprint Code Review - 2026-06-04

## Review Summary

This review covers the v0.91.5 octocrab mini-sprint after the first typed GitHub client migration wave.

Reviewed scope:

- `#3636` `[v0.91.5][tools][octocrab] Prepare typed GitHub client mini-sprint`
- `#3637` / PR `#3650` typed GitHub client contract
- `#3638` / PR `#3651` issue metadata parity path
- `#3639` / PR `#3652` PR inspection and closing-linkage path
- `#3640` / PR `#3654` `adl-csdlc` control-plane alignment
- `#3641` / PR `#3656` fallback hardening and migration review packet

Reviewed code and docs:

- `adl/src/cli/pr_cmd/github_client.rs`
- `adl/src/cli/pr_cmd/github.rs`
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd/finish_support.rs`
- `adl/src/cli/mod.rs`
- `adl/Cargo.toml`
- `docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md`
- `docs/tooling/ADL_OCTOCRAB_MIGRATION_REVIEW.md`

Overall result: `needs_followup`.

Supersession note: this packet records the first octocrab mini-sprint review as
historical evidence. Follow-up issue `#3697` addressed the core operational
transport gap by moving covered C-SDLC issue/PR workflow helper paths to
octocrab-backed transport and removing ambient `gh` repo-discovery fallback.
Keep the findings below as historical review input, not current-state truth.

The mini-sprint successfully created a typed interpretation layer and documented the migration boundary. At the time of this historical review, the main remaining problem was that the documented fail-closed shell-fallback policy was not enforced by the live GitHub CLI call sites. Follow-up issue `#3697` supersedes that operational finding.

## Findings

### P1 - `ADL_GITHUB_DISABLE_GH_FALLBACK=1` is not enforced by live `gh` operations

The boundary doc says the shared client layer owns fail-closed shell fallback disablement through `ADL_GITHUB_DISABLE_GH_FALLBACK`, and explicitly says not to silently use `gh` fallback when it is enabled. The migration review also says `ADL_GITHUB_DISABLE_GH_FALLBACK=1` disables shell fallback and fails closed.

At the time of this review, the live GitHub operations still called the GitHub CLI directly and did not instantiate or consult `AdlGithubClient::from_env()` before shell-backed operations.

Evidence:

- `docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md:24` says the shared layer owns GitHub client mode selection.
- `docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md:27` says the shared layer owns fail-closed shell fallback disablement.
- `docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md:38` says not to silently use `gh` fallback when `ADL_GITHUB_DISABLE_GH_FALLBACK` is enabled.
- `docs/tooling/ADL_OCTOCRAB_MIGRATION_REVIEW.md:35` says `ADL_GITHUB_DISABLE_GH_FALLBACK=1` disables shell fallback and fails closed for shell-backed modes.
- `adl/src/cli/pr_cmd/github_client.rs:103` defines `AdlGithubClient::from_env()`, but repository search found no non-test call site using it.
- The first-wave implementation used direct GitHub CLI invocations for current PR lookup, unresolved PR wave detection, PR closing-linkage checks, PR body repair, issue creation, issue metadata repair, and finish PR create/update helper paths.

Impact:

- At the time of this review, an operator could set `ADL_GITHUB_DISABLE_GH_FALLBACK=1` expecting shell fallback to fail closed, while live workflow operations could still execute the GitHub CLI.
- At the time of this review, the mode/fallback contract was true for the isolated typed client tests but not for the operational PR/issue path.
- This undermines the main safety claim of the migration slice: fallback should be explicit, controlled, and testable.

Recommended fix:

- Add a single GitHub client guard at the boundary before every shell-backed GitHub operation, or route shell-backed operations through a wrapper that consults `AdlGithubClient::from_env()`.
- If `backend == GhFallback` and fallback is disabled, fail before spawning `gh`.
- Add focused tests proving `ADL_GITHUB_DISABLE_GH_FALLBACK=1` blocks `current_pr_url`, issue metadata parity, PR closing-linkage repair, and finish PR create/update paths unless an octocrab-backed implementation is actually used.

### P2 - `ADL_GITHUB_CLIENT=octocrab` selects a typed configuration, but no live octocrab transport exists yet

The migration review is careful not to claim all GitHub operations are octocrab-backed, but the mode policy still says `ADL_GITHUB_CLIENT=octocrab` requires a token and fails closed without one. In current code, the selected octocrab backend is a configuration state only; the live operations remain shell-backed.

Evidence:

- `adl/src/cli/pr_cmd/github_client.rs:86` defines `AdlGithubClient`.
- `adl/src/cli/pr_cmd/github_client.rs:89` stores only `PhantomData<octocrab::Octocrab>` and does not construct a live octocrab client.
- `adl/src/cli/pr_cmd/github_client.rs:131` selects `GithubClientBackend::Octocrab` when token-backed `auto` or explicit `octocrab` mode is requested.
- `docs/tooling/ADL_OCTOCRAB_MIGRATION_REVIEW.md:41` correctly lists remaining shell-backed operations.
- The live call sites listed in the P1 finding continue to use `gh`.

Impact:

- `ADL_GITHUB_CLIENT=octocrab` is easy to misread as selecting live octocrab execution, but today it only selects a typed configuration that is not consumed by the live operation layer.
- This is less severe than P1 because the migration review includes non-claims, but it should be made clearer in command behavior and docs.

Recommended fix:

- Until live octocrab read/list/mutate paths exist, either describe the mode as `octocrab_configured` / `typed_contract_only`, or emit an explicit diagnostic when shell-backed operations remain in use.
- Add follow-on issues for trait-backed issue read/list, PR read/list, issue create/edit, and PR create/update before presenting `ADL_GITHUB_CLIENT=octocrab` as an operational transport selector.

## What Looks Good

- The typed helper layer is a real improvement over scattered string interpretation.
- `GithubClientMode`, `GithubClientBackend`, token-source discovery, stable error codes, and redaction helpers are small and understandable.
- Issue metadata parity moved into deterministic helper functions.
- PR wave filtering and closing-linkage interpretation moved into deterministic helper functions.
- The migration docs are mostly honest that the first wave does not remove shell-backed operations.
- The `adl-csdlc` routing boundary remains conservative: `adl/tools/pr.sh` stays the taught operator entrypoint during migration.

## Validation Performed

- `cargo test --manifest-path adl/Cargo.toml --bin adl github_client -- --nocapture`
  - Result: PASS.
  - Coverage: 18 focused `github_client` / C-SDLC GitHub boundary tests passed; 0 failed; 507 filtered out.

- Earlier attempted command `cargo test --manifest-path adl/Cargo.toml github_client --lib -- --nocapture` completed but matched 0 tests, so it is not counted as proof.

No broad validation was run for this review.

## Residual Risk

- I reviewed the code and docs directly but did not run the full CLI/PR workflow under `ADL_GITHUB_DISABLE_GH_FALLBACK=1`.
- The main finding is based on code-path evidence: the live GitHub operations call `gh` directly while the typed client/fallback policy is not consulted outside tests.
- There may be additional migration gaps in lifecycle closeout/reconciliation paths that still call shell-backed GitHub operations outside the reviewed PR metadata slice.

## Recommended Follow-Up Issues

1. `[v0.91.5][tools][octocrab] Enforce GitHub fallback policy on live gh-backed operations`
2. `[v0.91.5][tools][octocrab] Add trait-backed issue and PR read/list parity before live octocrab mutation`
3. `[v0.91.5][tools][octocrab] Add GitHub client observability for backend selection, fallback, and slow calls`

## Conclusion

The octocrab mini-sprint is a useful first migration wave, but it is not yet a complete operational transport migration. The most important fix is to make `ADL_GITHUB_DISABLE_GH_FALLBACK=1` real for live workflow operations, not only for the isolated typed-client constructor.
