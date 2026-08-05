# Issue 5786 Design: Retire Legacy `adl/src`

Status: design-time ready; implementation remains dependency- and date-gated.

## Authority And Sources

Issue #5786 owns WP-21 legacy retirement. Its live issue body is authoritative
for the pinned 355,675-line baseline, the 80% minimum/90% preferred deletion
target, and the rollback-window gate at `2026-08-12T09:04:24Z`. The execution
design also consumes `docs/milestones/v0.92/WBS_v0.92.md`,
`docs/milestones/v0.92/QUALITY_GATE_v0.92.md`, and the retained v0.91.8
baseline, parity, eligibility, and post-deletion records. At preparation head
`92451299651c44725a1951d4101b9cba27cad864`, `adl/src` contains 480 Rust files;
execution must refresh physical LoC and file counts with one documented method.

## Outcome Contract

Produce a machine-readable inventory for every remaining `adl/src` file and
every active Cargo, CI, install, docs, demo, and command reference. Each row
must name exactly one disposition: replaced by ADL v2, Runtime v3,
C-SDLC v2, a named adapter/product owner, temporarily retained with
owner/reason/expiry, or eligible for deletion. Deletion is allowed only after
the replacement route has exact-revision positive, negative, parity, rollback,
and clean-install proof.

The supported `adl` entrypoint must resolve to the thin ADL v2 CLI. Runtime v2
and obsolete compatibility binaries may leave production source only after the
inventory proves no supported behavior depends solely on them. Crate movement
or a reduced build graph without net incumbent deletion is not completion.

## Execution Sequence

1. Pin the execution SHA; verify WP-20 terminal/ancestry truth, rollback-window
   expiry, rollback non-use or disposition, selector health, and clean install.
2. Generate the exhaustive inventory and deletion denominator, including
   reverse references from manifests, workflows, installers, docs, and demos.
3. Close or route every capability gap and reject all unowned rows.
4. Delete by bounded ownership band, validating each band before proceeding.
5. Prove the thin CLI, artifact/trace compatibility, failure behavior, rollback
   posture, and platform install paths at the exact candidate head.
6. Publish before/after counts, retained exceptions, replacement owners, and
   an independently reviewed final manifest.

## Protected-Path Candidates

- `adl/src` only after the inventory narrows deletion bands to exact files
- `adl/Cargo.toml` and `adl/Cargo.lock` only when removed source changes them
- `adl-v2`, `adl-runtime-kernel`, and `csdlc-v2` only for proven capability gaps
- `.github/workflows`, `adl/tools`, active install/docs/demo references only
  when a deleted owner is still named there
- `.csdlc/evidence/5786` for inventory, denominator, parity, and review proof

The implementation claim must enumerate exact files; these candidate roots do
not authorize a broad repository claim.

## Owned Paths

- `.csdlc/evidence/5786/deletion-manifest.json`
- `.csdlc/evidence/5786/repository-denominator.json`
- `.csdlc/evidence/5786/platform-proof.json`
- `.csdlc/evidence/5786/stale-reference-report.json`
- `.csdlc/prepared/issues/5786/validate-reduction.rb`

Product paths are not owned until the exhaustive denominator selects exact
files and the execution claim is amended to list them individually.

## Validation And Failure Policy

Required lanes are: inventory completeness and reverse-reference rejection;
deletion denominator/threshold accounting; focused replacement parity and
negative cases; thin-CLI and clean-install proof; Runtime v2 and stale-route
absence; macOS and Linux CI parity; exact-head review. Any rollback activity,
pre-window execution, missing owner, below-80% reduction, unsupported behavior,
or failed platform lane blocks completion and preserves the incumbent surface.

## Non-Goals

- No deletion during preparation or before the rollback gate.
- No new feature behavior, architecture-by-file-move, or indefinite unowned
  compatibility exception.
- No WP-21A refactoring, WP-22 quality approval, or downstream release claim.
