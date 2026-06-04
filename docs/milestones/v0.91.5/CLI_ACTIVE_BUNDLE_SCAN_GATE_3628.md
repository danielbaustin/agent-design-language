# CLI Active-Bundle Scan Gate Before Shim Deletion (#3628)

Issue: #3628
Status: implemented as planning and proof gate

## Purpose

This document defines the required active-bundle scan gate before any future
issue deletes CLI compatibility shims preserved by the v0.91.5 owner-binary
split.

The gate exists because ADL now has multiple valid command histories:

- current live guidance;
- active issue cards and work packages;
- historical milestone evidence;
- compatibility shims that still execute;
- future owner binaries such as `adl-csdlc`, `adl-runtime`, and `adl-review`.

Deleting old command paths is allowed only after the repo can prove those paths
are no longer active workflow dependencies.

## Source Inputs

- `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md`
- `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md`
- `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md`
- `docs/milestones/v0.91.5/CLI_OWNER_COMMAND_GUIDANCE_AUDIT_3611.md`
- `docs/planning/PR_CONTROL_PLANE_DECRUFT_COMPATIBILITY_CUT_PLAN.md`
- `AGENTS.md`
- `docs/templates/prompts/`
- `adl/tools/skills/`

## Scan Classification Model

Every command reference found by a future deletion scan must be classified into
one of these buckets.

| Class | Meaning | Deletion impact |
| --- | --- | --- |
| `active` | Current workflow state depends on this command form. Examples include open issue cards, current prompt templates, live skill instructions, root `AGENTS.md`, portable adapter contracts, and milestone instructions for unfinished work. | Deletion is blocked until the active surface is migrated or deliberately routed. |
| `historical` | The reference is preserved as evidence from closed work, old reviews, old PRs, archived milestones, or explanation of past behavior. | Deletion may proceed if readability remains intact and the reference is not invoked by current tooling. |
| `unknown` | The scan cannot prove whether the reference is active or historical. | Deletion is blocked until a human routes the reference. |

Historical readability is not the same as executable compatibility. Old records
must remain inspectable, but ADL does not need to keep every old workflow alias
runnable forever.

## Required Scan Inputs

A compatibility-deletion issue must scan at least these surfaces before it
removes executable command support.

| Surface | Why it matters | Expected classification focus |
| --- | --- | --- |
| `.adl/v0.91.5/tasks/` | Active and recent issue bundles can contain executable instructions. | Mostly active until issues close; unknowns block deletion. |
| `.adl/cards/` | Compatibility card links may preserve current or transitional command strings. | Active if linked to open work; historical if tied to closed work only. |
| `AGENTS.md` | Root operating contract for agents. | Active. |
| `docs/templates/prompts/` | New card generation source. | Active. |
| `docs/templates/` | Invocation and lifecycle templates may still teach commands. | Active unless explicitly marked historical. |
| `adl/tools/skills/` | Skill instructions drive actual agent behavior. | Active for installed/live skills; historical references must be clearly marked. |
| `docs/milestones/v0.91.5/` | Current milestone execution and review truth. | Active for open WPs/issues; historical for closed evidence. |
| `docs/planning/` | Future work can fossilize old command names if not routed. | Active if it is an accepted future plan; historical if superseded. |
| portable ADL adapter docs/templates | External repos will copy process truth from these surfaces. | Active once created. |
| CI and tool scripts under `adl/tools/` | Scripts can invoke deprecated commands even when docs are clean. | Active unless test-only historical fixture is explicit. |

## Command Families Requiring Scan Proof

Future deletion issues must name each affected command family and include scan
evidence before removing executable support.

| Command family | Preferred owner after v0.91.5 split | Scan requirement |
| --- | --- | --- |
| direct `adl pr ...` issue-mode commands | `adl/tools/pr.sh` remains public issue-work wrapper; future `adl-csdlc` internals only after migration gate. | Prove no active card, skill, or conductor route relies on direct `adl pr ...`. |
| `adl pr run <adl.yaml>` runtime-through-PR | `adl-runtime run <adl.yaml>` | Prove no active runtime/demo/proof packet still teaches runtime-through-PR. |
| `adl tooling prompt-template ...` | `adl-csdlc tooling prompt-template ...` | Prove live docs, skills, and generated-card templates have migrated. |
| review tooling under `adl tooling ...` | `adl-review ...` | Prove live review skills and output contracts have migrated. |
| runtime umbrella forms such as `adl demo`, `adl provider`, `adl agent`, `adl godel`, `adl identity`, `adl runtime-v2` | `adl-runtime ...` | Prove v0.92 activation docs and active runtime proof packets have migrated. |
| `adl/tools/codex_pr.sh` | `adl/tools/pr.sh` or future wrapper internals | Prove no active operator workflow or closeout path depends on it. |
| legacy card-template fallback paths | Rust-owned active prompt-template registry | Prove current issue creation, doctor, run, finish, and closeout fixtures pass without the fallback. |

## Minimum Report Shape

A future active-bundle scan report should include this table shape.

| Command reference | Path | Line or evidence pointer | Class | Required action before deletion |
| --- | --- | --- | --- | --- |
| `<command>` | `<repo-relative path>` | `<line/ref>` | `active | historical | unknown` | `<migrate | preserve readable | route | none>` |

The report must also include:

- scan command(s) used;
- known exclusions and why they are safe;
- count by class;
- deletion recommendation;
- residual unknowns;
- reviewer sign-off or findings.

## Deletion Blockers

Deletion is blocked when any of these are true:

- an `active` reference remains unmigrated;
- an `unknown` reference remains unrouted;
- generated cards still teach the deprecated command;
- `AGENTS.md` or live skills still teach the deprecated command as current;
- wrapper/conductor equivalence tests do not cover the replacement path;
- closeout or historical-record readability would become misleading;
- no focused regression fixture proves the supported workflow still works.

## Acceptance Bar For Future Deletion Issues

Each future executable shim deletion issue must include:

- a bounded issue and complete C-SDLC card bundle;
- a scan report using this classification model;
- old/new command equivalence or fail-closed tests;
- generated-card validation proving new cards do not emit the deprecated
  command;
- wrapper/conductor tests if issue-work commands are touched;
- focused closeout/readability proof for historical records;
- explicit non-claims for historical evidence that is not rewritten.

## Current v0.91.5 Disposition

No executable shim is deleted by #3628.

The safe v0.91.5 posture remains:

- `adl/tools/pr.sh` is canonical for issue work;
- `adl-csdlc` owns proven C-SDLC tooling command families;
- `adl-runtime` owns runtime command families where docs/cards migrate;
- `adl-review` owns review command families where docs/cards migrate;
- compatibility shims remain executable until a future issue passes this gate.

## Validation

This packet is validated by focused docs and command-string checks. Broad Rust
or runtime validation is not required because no executable behavior changed.

Recommended checks:

- `bash adl/tools/test_cli_owner_command_guidance.sh`
- `bash adl/tools/run_owner_validation_lane.sh csdlc`
- `rg -n "adl pr run .*\\.ya?ml|adl-csdlc issue run|adl-crypto|adl-godel|adl-identity" AGENTS.md docs/templates/prompts adl/tools/skills docs/milestones/v0.91.5`
- `git diff --check`

## Non-Claims

- This issue does not implement the scanner.
- This issue does not delete compatibility commands.
- This issue does not complete the v0.92 decruft cleanup.
- This issue does not rewrite active issue cards.
- This issue does not approve `adl-csdlc issue run <issue>` as the public
  issue-work entrypoint.
