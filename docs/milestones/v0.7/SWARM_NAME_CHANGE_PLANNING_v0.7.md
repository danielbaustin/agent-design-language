# Swarm Name Change Planning (v0.7)

## Metadata
- Milestone: `v0.7`
- Topic: Runtime rename from `swarm` to `adl`
- Date: `2026-02-24`
- Status: `Planning`
- Owner: Daniel Austin

## Goal
Rename the runtime/CLI/tooling identity from `swarm` to `adl` with a controlled migration and minimal contributor disruption.

## Recommendation
Execute this as the final v0.7 step with a temporary compatibility window:
- Introduce `adl` as canonical in v0.7.
- Keep `swarm` aliases for one release cycle.
- Remove legacy aliases after the v0.75 compatibility window (exact target release set during release planning).

## Decision Lock (Accepted)

Selected scope and sequencing:
- **Scope option:** A (active runtime/CLI/tooling/CI/active docs only; treat historical milestone docs as historical)
- **Compatibility policy:** C (one-release compatibility window)
- **Timing:** Do last in v0.7 after functional work stabilizes

Naming targets (v0.7):
- **Top-level directory rename:** **NO** (keep the `swarm/` directory path stable in v0.7)
- **Crate/package name:** `swarm` → `adl`
- **Primary CLI binary:** `swarm` → `adl`
- **Remote binary:** `swarm-remote` → `adl-remote` (or equivalent chosen name)

Compatibility window (v0.7):
- Provide shims so legacy entrypoints continue to work for one release cycle:
  - `swarm` remains available as an alias/shim that invokes `adl` and prints a deprecation warning
  - `swarm-remote` remains available as an alias/shim that invokes `adl-remote` and prints a deprecation warning
  - Legacy env vars (e.g. `SWARM_...`) continue to work with a deprecation warning; canonical env vars become `ADL_...`

Out of scope (v0.7):
- Do not rewrite historical milestone docs or historical examples purely for naming consistency.
- Do not rename the `swarm/` directory path in this milestone.

## Baseline Inventory (2026-02-24)
Repository scan snapshot:
- `475` `swarm` textual references across `81` files.
- `311` path/coupling references (`swarm/`, `swarm-remote`, `SWARM_OLLAMA_BIN`, `CARGO_BIN_EXE_swarm`) across `65` files.
- `111` files currently under the `swarm/` directory.

Highest-impact coupling areas:
- Runtime package/binaries: `swarm/Cargo.toml`, `swarm/src/main.rs`, `swarm/src/bin/swarm_remote.rs`
- Tooling scripts: `swarm/tools/pr.sh`, `swarm/tools/batched_checks.sh`, `swarm/tools/check_release_notes_commands.sh`
- CI/coverage wiring: `.github/workflows/ci.yaml`
- User-facing docs/commands: `README.md`, `CONTRIBUTING.md`, `docs/*`

## Scope Options And LOE
| Option | Scope | Estimated LOE | Notes |
|---|---|---:|---|
| A | Runtime + CLI + tooling + CI + active docs; leave historical milestone docs as historical | 6-9 engineer-days | Recommended baseline; includes no historical-doc rewrite |
| B | Option A + rewrite all historical docs/examples to `adl` naming | 9-14 engineer-days | Larger editorial churn |
| C | Option A or B + compatibility alias window (`swarm` still works) | +2-4 engineer-days | Strongly recommended risk control; required (selected) |

Calendar estimate:
- 1 engineer: ~1.5 to 3 weeks depending on selected scope.
- 2 engineers (parallel tracks): ~4 to 8 working days.

## Execution Plan
### Phase 0: Decision Lock (0.5 day)
- Confirm scope: Option A (active surfaces only; historical docs treated as historical).
- Confirm compatibility: Option C (one-release window; remove after v0.75).
- Confirm naming targets for v0.7:
  - Keep the `swarm/` directory path stable.
  - Rename crate/package and binaries to `adl`.
  - Rename env vars to canonical `ADL_...` with legacy `SWARM_...` supported during the window.
- Record these decisions in `docs/milestones/v0.7/DECISIONS_v0.7.md` (see D-06).

### Phase 1: Mechanical Rename (1-2 days)
- Rename package/binaries and update imports/usages in Rust code/tests.
- Update test binary environment constants (for example `CARGO_BIN_EXE_*`).
- Update file paths and command strings in scripts.

Deliverable:
- Build/test passes locally with new canonical naming.

### Phase 2: Tooling + CI Migration (1-2 days)
- Update workflow job names and coverage filters currently tied to `swarm` paths.
- Update helper scripts and default staged paths in PR automation.
- Add migration-safe behavior for scripts that contributors run frequently.

Deliverable:
- CI green with renamed runtime identity.

### Phase 3: Docs + Developer UX (1-3 days)
- Update root docs, contributor docs, onboarding docs, demo commands.
- If Option B is selected, include historical milestone docs.
- Add migration section: old command -> new command mapping.

Deliverable:
- Docs are internally consistent and runnable.

### Phase 4: Compatibility Window (Required, 2-4 days)
- Keep `swarm` aliases/shims for CLI/scripts/env vars.
- Emit deprecation messaging with explicit post-v0.75 removal target once finalized.
- Add tests that prove both new and old entrypoints work during transition.

Deliverable:
- Backward-compatible transition in v0.7.

### Phase 5: Cutover Validation + Release Readiness (1-2 days)
- Run full checks (`fmt`, `clippy`, `test`, CI, coverage).
- Add/enable grep guardrail to prevent new unintended `swarm` references (allowlist for intentional compatibility/historical content).
- Verify representative workflows and demos end-to-end.
- Add a CI/automation guardrail to prevent new unintended `swarm` references (allowlist: compatibility shims + historical docs).

Deliverable:
- Go/no-go checklist completed with evidence links.

## Parallelization Lanes
Use parallel workstreams after Phase 0:
- Lane 1: Runtime/CLI/Test rename.
- Lane 2: Tooling/CI migration.
- Lane 3: Docs and migration guide.

Integration checkpoint required before merge:
- Rebase all lanes on final naming decisions.
- One integration PR with final pass over scripts/docs.

## Risk Register
| Risk | Impact | Likelihood | Mitigation |
|---|---|---|---|
| Breaking contributor workflows (`pr.sh`, checks scripts) | High | High | Compatibility aliases + explicit migration mapping |
| CI/coverage break due to path-based filters | High | High | Update workflow filters and add validation checks |
| Large doc churn creates review fatigue | Medium | High | Prefer Option A; treat historical docs as frozen unless explicitly in scope |
| Hidden string references missed | Medium | Medium | Guardrail grep checks + focused review of top-hit files |
| Release timing slip at end of milestone | High | Medium | Pre-stage design decisions and reserve dedicated rename window |

## Acceptance Criteria
- `adl` is canonical across runtime package, CLI, tooling, CI, and active docs.
- All required quality gates pass on merge target.
- Migration guide exists with concrete command/env var mappings.
- If compatibility window is enabled:
  - `swarm` entrypoints continue to function in v0.7.
  - Deprecation notices point to the post-v0.75 removal plan.

## Proposed Tracking Checklist (for WBS/issues)
- [ ] Decision recorded: scope option and compatibility policy
- [ ] Runtime/package/binary rename merged
- [ ] Tooling scripts migrated
- [ ] CI and coverage migration complete
- [ ] Docs migration complete
- [ ] Compatibility aliases and tests (if enabled)
- [ ] Guardrail checks enabled
- [ ] Final integration PR merged
- [ ] Release notes include migration guidance

## Open Decisions

- Exact binary naming for the remote runner (default target: `adl-remote`).
- Exact deprecation messaging text and removal date (target: post-v0.75 compatibility window).
