# Editor Command Adapter Surface

This document is the current editor command adapter contract for ADL task-bundle handoff.

It defines what browser/editor surfaces may prepare, what remains human-run through repo tooling and workflow skills, and what is explicitly out of scope for direct browser invocation.

## Language Contract Assumptions

This editor stack is a direct consumer of the ADL language story even though it
does not edit full ADL documents end to end.

The browser/editor surfaces should therefore assume the same canonical language
contract as the runtime and published schema:

- six primitives: `providers`, `tools`, `agents`, `tasks`, `workflows`, `run`
- singular `run` at the top level
- `patterns` and `signature` as top-level language features, not additional
  primitives
- packet and control-plane contracts remain outside the six-primitives language
  core

## Contract

The supported adapter surface is intentionally copy-only:

- supported adapter action:
  - `adl/tools/editor_action.sh prepare --phase init|doctor-ready|run|finish --issue <number> --slug <slug> [--version <vN.N[.P]>] [--title <title>] [--paths <paths>]`
- canonical control-plane mapping:
  - `adl/tools/pr.sh init`
  - `adl/tools/pr.sh doctor --mode ready`
  - `adl/tools/pr.sh run`
  - `adl/tools/pr.sh finish`
- adapter mode:
  - browser-prepared, human-run command handoff

The browser/editor may:

- prepare a lifecycle command
- copy that command for a human to run from the repo root
- validate issue, branch, slug, and version constraints before surfacing the command
- surface the canonical ADL language contract for human orientation

The browser/editor may not claim direct browser invocation of:

- `pr create`
- `pr init`
- `pr doctor`
- `pr ready`
- `pr run`
- `pr finish`
- `pr janitor`
- `pr closeout`

Those commands exist in the repo control plane and related operational skills. They are not browser-direct actions.

## Why The Surface Is Narrow

The adapter must stay thin so the browser does not duplicate workflow logic already owned by the command/control-plane layer.

That means:

- browser code should not recreate lifecycle behavior in JavaScript
- browser code should not imply hidden direct execution paths
- browser docs should distinguish implemented repo commands from browser-prepared command handoff
- editor output should remain compatible with `pr-init`, `pr-ready`, `pr-run`, `pr-finish`, `pr-janitor`, `pr-closeout`, and the card editor skills

## Truth Table

| Lifecycle command | Exists in repo | Browser-direct adapter support | Truthful editor status |
| --- | --- | --- | --- |
| `pr create` | yes | no | control-plane only |
| `pr init` | yes | no | copy-only prepared handoff |
| `pr doctor --mode ready` | yes | no | copy-only prepared handoff |
| `pr run` | yes | no | copy-only prepared handoff |
| `pr finish` | yes | no | copy-only prepared handoff |
| `pr janitor` | skill-owned | no | out of browser scope |
| `pr closeout` | skill-owned | no | out of browser scope |
| `pr start` | legacy alias | no | deprecated compatibility only |

## Legacy Compatibility

`adl/tools/editor_action.sh start` remains available for older deterministic editor demos that still validate the v0.85 compatibility path. It is not the taught current workflow.

## Proof Surface

The contract is backed by:

- `adl/tools/editor_action.sh`
- `adl/tools/test_editor_action.sh`
- `docs/tooling/editor/demo.md`
- `docs/tooling/editor/current_skill_wiring_demo.md`

The adapter surface should only be widened in a follow-on issue with matching docs, validation, and proof updates.
