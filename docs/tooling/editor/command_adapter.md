# Editor Command Adapter Surface

This document is the canonical near-term editor command adapter contract for v0.85.

It defines what browser/editor surfaces may invoke directly, what they may only prepare or copy for a human to run, and what remains out of scope for direct browser invocation.

## Contract

The supported near-term adapter surface is intentionally narrow:

- supported adapter action:
  - `adl/tools/editor_action.sh start --issue <number> --branch codex/<issue>-<slug> [--slug <slug>] [--dry-run]`
- canonical control-plane mapping:
  - `adl/tools/pr.sh start`
- adapter mode:
  - thin adapter only

The browser/editor may:

- prepare the adapter command
- copy the adapter command for a human to run from the repo root
- validate issue/branch pairing constraints before surfacing the command

The browser/editor may not claim direct browser invocation of:

- `pr create`
- `pr init`
- `pr run`
- `pr finish`

Those commands exist in the repo control plane, but they are not part of the current browser-direct adapter surface for v0.85.

## Why The Surface Is Narrow

The adapter must stay thin so the browser does not duplicate workflow logic already owned by the command/control-plane layer.

That means:

- browser code should not recreate `pr` lifecycle behavior in JavaScript
- browser code should not imply hidden direct execution paths
- browser docs should distinguish:
  - implemented control-plane commands
  - supported browser-direct adapter actions

## Truth Table

| Lifecycle command | Exists in repo | Browser-direct adapter support in v0.85 | Truthful near-term status |
| --- | --- | --- | --- |
| `pr create` | yes | no | control-plane only |
| `pr init` | yes | no | control-plane only |
| `pr start` | yes | yes, via `adl/tools/editor_action.sh start` | supported thin adapter |
| `pr run` | yes | no | control-plane only |
| `pr finish` | yes | no | control-plane only |

## Proof Surface

The contract is backed by:

- `adl/tools/editor_action.sh`
- `adl/tools/test_editor_action.sh`
- `docs/tooling/editor/demo.md`

The adapter surface should only be widened in a follow-on issue with matching docs, validation, and proof updates.
