# Control-Plane Logging Proof (`#3996`)

## Scope

Bounded proof for:

- control-plane `adl_event` emission
- JSON stdout / stderr separation on proven surfaces
- compatibility-log behavior
- the `doctor --json` bootstrap-validator pollution fix

## What Changed

- `adl/src/cli/pr_cmd_cards/validation.rs` now routes bootstrap `sip` / `sor`
  validation through the captured-output helper used by other structured-card
  validation paths.
- That keeps validator success text such as `PASS: sor contract valid ...` from
  leaking onto stdout during `pr.sh doctor --json`.

## Evidence

- `adl/src/cli/observability.rs`
- `adl/tools/observability.sh`
- `adl/src/cli/pr_cmd/github.rs`
- `adl/src/cli/pr_cmd_cards/validation.rs`
- `docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md`

## Proof Plan

- `bash adl/tools/test_pr_json_observability.sh`
- `bash adl/tools/test_control_plane_observability.sh`

## Claimed Result

- Machine-readable `pr.sh ready|preflight|doctor --json` surfaces remain
  stdout-only on the proven paths.
- Human-oriented `adl_event` lines remain on stderr by default.
- Compatibility-log mode remains available through
  `ADL_OBSERVABILITY_STDERR=0` plus `ADL_OBSERVABILITY_LOG=<path>`.
- When stderr is explicitly suppressed, a bad compatibility-log sink stays
  quiet and does not corrupt validator stdout.
- Bootstrap card validation no longer pollutes `doctor --json` stdout with
  validator success lines.

## Non-Claims

- This packet does not claim every historical JSON surface in the repository is
  proven parse-safe.
- This packet does not claim GitHub/token/release/projection observability from
  `#4001`.
- This packet does not claim command-wide heartbeat coverage for every control
  path.
