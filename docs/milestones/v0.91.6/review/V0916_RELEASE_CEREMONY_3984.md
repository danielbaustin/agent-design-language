# v0.91.6 Release Ceremony Record

## Metadata

- Issue: `#3984`
- Work package: `WP-19`
- Milestone: `v0.91.6`
- Status: preflight-ready
- Date: `2026-06-28`

## Purpose

Record the final v0.91.6 release ceremony truth without converting ceremony
work into hidden implementation. This packet is the WP-19 evidence surface for
release-tail state, release publication boundaries, and handoff into v0.91.7.

## Current Release-Tail Truth

| Surface | Current truth |
| --- | --- |
| WP-11 `#3976` | Closed after demo/proof convergence. |
| WP-12 `#3977` | Closed after quality gate. |
| WP-13 `#3978` | Closed after docs/review-surface alignment. |
| WP-14A `#4582` | Closed after internal review and pre-v0.92 burn-down. |
| WP-15 `#3980` | Closed. External review ran and failed on stale handoff truth; the failure remains retained release evidence. |
| WP-16 `#3981` | Closed. Accepted external-review findings were remediated; `#4620` and `#4621` closed; `#4622` routed to v0.91.7. |
| WP-17 `#3982` | Closed after v0.91.7 handoff refresh. |
| WP-18 `#3983` | Closed after next-milestone readiness review. |
| WP-19 `#3984` | Active release ceremony owner. |
| Umbrella `#4604` | Remains open until WP-19 closes, then should close with a sprint-level note. |

## Release Publication Boundary

The repo-native release ceremony script is
`adl/tools/release_ceremony.sh`. It is safe by default and performs check-only
preflight unless explicit mutation flags are supplied.

WP-19 does not claim that a public tag or GitHub Release has been created until
the script is run with the appropriate operator-approved mutation flags:

- `--create-tag`
- `--push-tag`
- `--draft-release`
- `--publish-release`

The current release notes use `v0.91.6` as the intended tag name, but tag and
GitHub Release publication remain ceremony actions, not documentation claims.

The retired shell closed-issue SOR truth gate at
`adl/tools/check_milestone_closed_issue_sor_truth.sh` is not a valid release
ceremony gate. WP-19 updates `release_ceremony.sh` so the wrapper skips that
retired stub explicitly at runtime when the checker exits with the retired-gate
status and message. This avoids source-text grepping while preventing every
ceremony from failing by default before the Rust/PVF replacement exists.

Residual risk: ceremony preflight does not currently prove closed-issue SOR
bundle truth mechanically. v0.91.6 relies on the retained release-tail evidence,
review packets, and manual issue-state reconciliation for this closeout; a
Rust/PVF replacement gate remains required before future milestones can claim
automated closed-issue SOR bundle truth.

Publication-control residual: `pr.sh finish` currently classifies release-plan
and milestone-checklist edits as a non-runnable release-gate validation-manager
profile, and the selector does not map `adl/tools/release_ceremony.sh` to a
runnable lane. WP-19 records the manual release-gate disposition and local proof
in this packet before publication. Future release-tail work should replace this
manual boundary with a runnable release-gate lane and selector coverage for the
release ceremony wrapper.

CI janitor residual: PR `#4627` initially failed `adl-coverage` on the unrelated
Rust lifecycle fixture
`closeout_closed_completed_issue_bundle_records_prune_result_on_canonical_output`.
The fixture copied `validate_structured_prompt.sh` into a synthetic repository
without also copying its sourced `owner_binary_resolution.sh` helper. WP-19
repaired only that fixture dependency so broad coverage can execute the existing
closeout test; this does not change the release-doc ceremony claim surface.

## v0.91.7 Handoff Boundary

The current v0.91.7 handoff authority is:

- `docs/milestones/v0.91.7/PLANNING_SOURCE_CAPTURE_v0.91.7.md`
- `docs/milestones/v0.91.7/V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`
- `docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml`
- `docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md`

Open routed residuals must be consumed from those sources instead of being
reconstructed from stale release-tail packets.

## Non-Claims

- This packet does not claim `v0.92` activation readiness.
- This packet does not claim runtime/product surfaces are complete from docs,
  seams, mocks, component tests, or local slices alone.
- This packet does not claim EC2 Spot, remote-builder, scheduler, AWS,
  Observatory, provider, C-SDLC, or runtime integration work is fully complete
  unless a cited tracked proof says so.
- This packet does not claim public tag or GitHub Release publication before
  the release ceremony script performs the requested mutating action.

## Validation Plan

WP-19 recorded:

- `bash -n adl/tools/release_ceremony.sh`
- `git diff --check`
- `python3 adl/tools/check_repo_quality_staleness.py --milestone v0.91.6`
- `bash adl/tools/release_ceremony.sh --version v0.91.6 --target-branch <branch> --allow-dirty`
  as a check-only preflight during this PR, because the worktree is intentionally
  dirty before publication.
- `bash adl/tools/release_ceremony.sh --version v0.91.6` from clean `main`
  after merge if the operator wants final check-only ceremony preflight before
  mutating tag or release state.

Local pre-PR results:

- `git diff --check`: pass.
- `python3 adl/tools/check_repo_quality_staleness.py --milestone v0.91.6`: pass.
- `bash -n adl/tools/release_ceremony.sh`: pass.
- `bash adl/tools/release_ceremony.sh --version v0.91.6 --target-branch codex/3984-v0-91-6-wp-19-release-release-ceremony --allow-dirty`: pass, check-only, no tag or GitHub Release mutation; the retired-gate runtime skip path was exercised.
- `pr.sh finish 3984 ... --ready`: blocked before PR publication because the
  validation manager selected a non-runnable release-gate profile and left
  `adl/tools/release_ceremony.sh` unmapped. This packet is the manual
  release-gate disposition for WP-19.
- `cargo test --manifest-path adl/Cargo.toml closeout_closed_completed_issue_bundle_records_prune_result_on_canonical_output -- --nocapture`: pass after the CI janitor fixture repair; validates that synthetic lifecycle closeout repos include the validator helper required by `validate_structured_prompt.sh`.

## v0.92 Consumption Boundary

`v0.92` may consume the v0.91.6 bridge docs, retained review packets, and
v0.91.7 handoff routing as evidence that the first bridge tranche is accounted
for. It may not consume v0.91.6 as proof that integrated runtime/product
surfaces are complete unless the specific surface has integrated proof.

The remaining blocked, deferred, or routed surfaces are named in the v0.91.7
planning source capture and v0.92 activation bridge ledger rather than hidden
inside release ceremony prose.

## Exit

After WP-19 merges and `#3984` closes, close umbrella `#4604` with a note that
the ordered release-tail wave completed and that v0.91.7 must consume routed
residuals from the v0.91.7 planning source capture.

Carry the release-gate validation-manager gap forward: future ceremony PRs
should not require `--no-checks` after manual proof.
