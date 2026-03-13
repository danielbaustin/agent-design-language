# Swarm Removal Planning (v0.85)

## Metadata
- Milestone: `v0.85`
- Topic: Final removal of legacy `swarm` identity and directory path
- Date: `2026-03-13`
- Status: `Planning`
- Owner: `Daniel Austin / Agent Logic`

## Why This Matters

ADL has already renamed the package and primary binaries, but the repository still presents a split identity:
- canonical runtime/package identity is largely `adl`
- top-level runtime directory is still `swarm/`
- CI, tooling, docs, examples, tests, and compatibility shims still expose legacy `swarm` naming

That split identity is survivable for current contributors but weakens external credibility. For investors, reviewers, and enterprise evaluators, it reads as an unfinished migration and raises questions about repository discipline, release hygiene, and operational polish.

This work is not optional if ADL wants a clean, investable presentation.

## Executive Summary

Recommendation:
- Do this in `v0.85` as a dedicated cleanup/cutover workstream.
- Treat it as an active-surface identity cutover, not a full historical-document rewrite.
- Remove legacy runtime/tooling compatibility shims as part of the cutover.
- Preserve historical milestone docs as historical records unless a specific historical file is externally surfaced and misleading.

Recommended scope:
- `git mv swarm adl`
- update active code, scripts, CI, tests, examples, and current docs
- remove `swarm` / `swarm-remote` shim binaries
- remove legacy `SWARM_*` env-var compatibility where still present only for migration support
- replace the current guardrail with a stricter “no new or remaining active-surface swarm refs” policy

Recommended timing:
- after `v0.8` release stabilization
- before more `v0.85` work lands on top of the old path conventions

## Current State Snapshot (2026-03-13)

Repository scan results from current `main`:
- `129` files still contain `swarm` textual references
- `150` tracked files currently live under `swarm/`
- `81` references to `manifest-path swarm/Cargo.toml`
- `73` references to `swarm/examples`
- `247` references to `swarm/tools`
- `51` references to `swarm/src`
- `41` references to legacy bins (`swarm`, `swarm-remote`, `CARGO_BIN_EXE_swarm`)
- `27` references to legacy env vars (`SWARM_*`)

Remaining references by area:
- `48` files under `docs/milestones/`
- `29` files under `swarm/tools/`
- `14` files under `docs/` outside milestone history
- `9` files under `swarm/src/`
- `9` files under `swarm/examples/`
- `4` files under `swarm/tests/`
- `2` GitHub workflow files

Interpretation:
- most remaining churn is not core Rust implementation risk
- the real risk is operational coupling: scripts, CI, tests, path-sensitive commands, and contributor workflow
- historical milestone docs materially inflate raw grep counts

## What Has Already Been Done

The repo is not at the start of this migration.

Already complete:
- package name is `adl` in `swarm/Cargo.toml`
- library name is `adl`
- primary runtime binary is `adl`
- remote binary `adl-remote` exists
- compatibility shims for `swarm` and `swarm-remote` already exist
- canonical env vars have mostly moved to `ADL_*`

Still incomplete:
- runtime directory path is still `swarm/`
- CI and coverage filters still key off `/swarm/src/`
- contributor tooling still defaults to `swarm/tools/...`
- tests still validate deprecated `swarm` behavior
- docs and examples still point to `swarm/...` paths extensively
- guardrails still explicitly allow legacy references for compatibility

## Recommended Scope

### In Scope

- Rename top-level runtime directory from `swarm/` to `adl/`
- Update all active path references:
  - GitHub Actions
  - shell tooling
  - README / CONTRIBUTING / onboarding
  - current demo docs
  - examples and example docs
  - tests and fixtures
  - schema and artifact references
- Remove compatibility-only runtime shims:
  - `src/bin/swarm.rs`
  - `src/bin/swarm_remote.rs`
- Remove compatibility-only env-var fallback where safe:
  - `SWARM_OLLAMA_BIN`
  - `SWARM_TIMEOUT_SECS`
  - `SWARM_ALLOW_UNSIGNED`
  - remote signing/bearer-token fallbacks still kept only if explicitly justified
- Replace the legacy-name guardrail with an active-surface zero-tolerance guardrail
- Add a migration note that clearly states:
  - old path -> new path
  - old command -> new command
  - old env var -> new env var

### Out Of Scope

- broad editorial rewrite of all historical milestone docs purely for naming consistency
- rewriting old release artifacts that are intentionally historical
- large architecture redesign bundled together with the rename
- any attempt to preserve indefinite dual-path support

## Decision Lock

The following decisions are recommended for acceptance:

- **Directory rename:** `YES` in `v0.85`
- **Compatibility policy:** remove legacy CLI/env-var compatibility instead of extending it again
- **Historical docs policy:** preserve historical milestone docs unless they are directly surfaced to external reviewers and are misleading
- **Execution model:** one focused cutover branch/PR series, not an opportunistic drip of unrelated edits

## Effort Estimate

### Recommended Option A
Active surfaces only:
- runtime directory rename
- code/tooling/CI/tests/docs/examples cleanup
- compatibility removal
- historical docs left historical

Estimated LOE:
- `3-5 engineer-days`

Expected calendar:
- one engineer: `4-6 working days` including validation and review handling

### Option B
Option A plus rewrite of historical milestone docs and older demo documentation:

Estimated LOE:
- `5-8 engineer-days`

Expected calendar:
- one engineer: `1.5 to 2 weeks`

## Why Option A Is Recommended

Option A delivers the investment-facing benefit:
- externally visible identity becomes coherent
- active repo surfaces stop signaling unfinished migration
- contributor workflow becomes consistent

Option B creates a lot of editorial churn with limited business return. It should only be done if historical docs are being actively shown to external reviewers or if the repo is being positioned as a polished archival knowledge base.

## Highest-Risk Areas

### 1. Contributor Tooling

The biggest single coupling surface is `swarm/tools/pr.sh`, along with the scripts around it.

Risk:
- staging defaults
- path assumptions
- generated card/template paths
- help text and examples
- downstream helper scripts all assume `swarm/tools/...`

Impact:
- breakage here directly hurts daily development flow

### 2. CI And Coverage Wiring

GitHub Actions still rely on `working-directory: swarm`, `swarm -> target` cache keys, and `/swarm/src/` coverage filters.

Risk:
- green local code but red CI
- silent coverage gate misconfiguration

### 3. Tests For Legacy Behavior

Several tests intentionally assert deprecated `swarm` shims and `CARGO_BIN_EXE_swarm` behavior.

Risk:
- expected failures after compatibility removal
- confusion over which tests should be deleted versus renamed

### 4. Active Docs And Demo Commands

README, onboarding, contributor docs, and demo docs still expose many `swarm/...` commands.

Risk:
- external reviewers hitting dead commands
- polished codebase undermined by obviously stale documentation

## Evidence-Backed Hotspots

Files most likely to dominate the cutover:
- `swarm/tools/pr.sh`
- `.github/workflows/ci.yaml`
- `.github/workflows/nightly-coverage-ratchet.yaml`
- `README.md`
- `CONTRIBUTING.md`
- `docs/onboarding.md`
- `swarm/src/env_compat.rs`
- `swarm/src/bin/swarm.rs`
- `swarm/src/bin/swarm_remote.rs`
- `swarm/tests/cli_smoke.rs`

These should be treated as explicit review checkpoints, not incidental cleanup.

## Proposed Execution Plan

### Phase 0: Decision And Freeze (0.5 day)

- Accept Option A scope
- Decide that compatibility shims are removed rather than extended
- Reserve a dedicated cutover window
- Avoid merging unrelated repo-wide path churn during the cutover

Deliverable:
- decision recorded in v0.85 planning docs / decision log

### Phase 1: Mechanical Rename (0.5-1 day)

- `git mv swarm adl`
- update manifest-path and direct path references
- update imports or comments only where path-sensitive

Deliverable:
- repo builds with the new directory name locally

### Phase 2: Tooling And CI Repair (1-1.5 days)

- update `pr.sh` and helper tooling
- update workflows, cache paths, coverage filters, and help text
- update artifact/template paths

Deliverable:
- local tooling works
- CI config is path-correct

### Phase 3: Compatibility Removal (0.5-1 day)

- remove deprecated shim binaries
- remove legacy env-var fallbacks and related deprecation messaging
- delete or rewrite tests that exist only to validate compatibility behavior

Deliverable:
- no active runtime path still depends on `swarm` compatibility

### Phase 4: Docs And Demo Surface Cleanup (0.5-1 day)

- update active root docs and onboarding docs
- update current demo/review entry points
- update active example docs and commands

Deliverable:
- external reviewer can follow current commands without translation

### Phase 5: Guardrails And Validation (0.5-1 day)

- tighten grep guardrail
- allowlist only true historical content if needed
- run:
  - `cargo fmt`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test`
  - representative demo smoke / tooling checks

Deliverable:
- cutover is locked in and hard to regress

## Validation Checklist

- [ ] `adl/` exists and `swarm/` no longer exists as an active top-level runtime directory
- [ ] active scripts no longer require `swarm/tools/...`
- [ ] CI workflows no longer reference `working-directory: swarm`
- [ ] coverage filters no longer depend on `/swarm/src/`
- [ ] no active tests require `CARGO_BIN_EXE_swarm`
- [ ] no active docs instruct users to run `swarm/...` paths
- [ ] compatibility-only env vars removed or explicitly justified
- [ ] guardrail fails on new active-surface `swarm` references
- [ ] historical references, if any, are intentionally preserved and documented

## Risk Register

| Risk | Impact | Likelihood | Mitigation |
|---|---|---|---|
| `pr.sh` / contributor workflow breakage | High | High | Treat tooling as first-class scope; validate start/finish flows explicitly |
| CI red due to path-based assumptions | High | High | Update workflow paths and run targeted CI-equivalent checks locally |
| Coverage gate drift after path rename | High | Medium | Review and test coverage filters deliberately |
| Hidden legacy references missed in active docs | Medium | High | Grep sweeps plus runnable command verification |
| Historical docs create review noise | Medium | High | Freeze historical docs unless externally surfaced |
| Rename PR becomes too broad and hard to review | High | Medium | Keep scope disciplined; avoid bundling unrelated cleanup |

## Commercial Framing

This work is operationally painful but strategically correct.

Benefits:
- cleaner first impression for investors and technical diligence
- lower reviewer confusion during external evaluation
- less daily cognitive friction for contributors
- better signal that ADL finishes migrations rather than living indefinitely in transitional states

Cost:
- several days of concentrated, low-glamour engineering effort
- some temporary disruption while scripts and workflows are repaired

Conclusion:
- the commercial upside justifies the cleanup
- the right way to do it is a focused, active-surface cutover rather than another compatibility extension

## Recommendation

Proceed with `Option A` in `v0.85` as a dedicated cutover.

Do not postpone indefinitely:
- every new feature merged before the cutover adds more `swarm` path debt
- the repo already has enough evidence of partial migration
- extending the compatibility era again would make the codebase look less disciplined, not more

Do not expand to full historical rewrite unless required:
- it adds cost without materially improving the investment story

The right target state is simple:
- `adl/` is the runtime directory
- `adl` is the only active runtime identity
- `swarm` survives only in clearly historical documentation, if at all
