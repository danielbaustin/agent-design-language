# Sprint Plan - v0.87.1

## Metadata
- Sprint: `v0.87.1-s1`
- Milestone: `v0.87.1`
- Start date: `TBD`
- End date: `TBD`
- Owner: `TBD`

## How To Use
- Keep scope small enough to finish with green CI and merged PRs.
- List work items in planned execution order.
- Track blockers here (not scattered chat notes).

## Sprint Goal
Seed the tracked `v0.87.1` milestone shell and prepare it for later runtime-completion work.

## Planned Scope
- establish canonical milestone docs for `v0.87.1`
- keep feature-doc promotion out of scope for this seed pass
- populate sprint content when `v0.87.1` formally opens

## Work Plan
| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | Design pass for the `v0.87.1` milestone shell | `#1354` | `Daniel / Codex.app` | PR open |
| 2 | Additional runtime-completion work packages | `TBD` | `Daniel / Codex.app` | not yet issued |
| 3 | Additional runtime-completion work packages | `TBD` | `Daniel / Codex.app` | not yet issued |

## Cadence Expectations
- Use issue cards (`input`/`output`) for each item.
- Keep changes scoped per issue; use draft PRs until checks pass.
- Run required quality gates (fmt/clippy/test) for code changes.

## Risks / Dependencies
- Dependency: `#1355` should make doctor/run lifecycle handling fully support dot-suffixed milestone versions.
  - Risk: control-plane tooling still assumes older version formats and can partially fail during bind/materialization.
  - Mitigation: keep `#1355` explicit as the tooling follow-on while continuing bounded docs work on the already-created issue branch.

## Demo / Review Plan
- Demo artifact: none for the seed pass; proof is the tracked milestone shell
- Review date: TBD
- Sign-off owners: Daniel Austin / Codex.app

## Exit Criteria
- `#1354` lands the tracked `v0.87.1` milestone shell with normalized filenames.
- The sprint plan names the docs/design pass explicitly instead of leaving the seed pass implicit.
- Later runtime-completion work remains unissued until the milestone scope is filled in truthfully.
