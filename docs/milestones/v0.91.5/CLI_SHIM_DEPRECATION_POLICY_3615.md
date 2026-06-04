# CLI Shim Deprecation And Compatibility Sunset Policy (#3615)

Issue: #3615
Status: implemented as policy and routing

## Purpose

This packet defines how ADL should deprecate CLI compatibility shims after the
first v0.91.5 owner-binary split, without creating a second workflow truth or
breaking historical C-SDLC records.

The policy is conservative:

- keep executable compatibility shims during v0.91.5 Sprint 1;
- make generated-card and live-guidance validation stricter than interactive
  terminal warnings;
- require an active-bundle scan before any deletion issue starts;
- preserve historical records as readable audit artifacts even when old
  executable fallbacks are later removed.

This issue does not delete commands, rewrite cards, or change runtime behavior.

## Source Inputs

- `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md`
- `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md`
- `docs/milestones/v0.91.5/CLI_REFACTOR_MINI_SPRINT_REVIEW_3600.md`
- `docs/milestones/v0.91.5/CLI_OWNER_COMMAND_GUIDANCE_AUDIT_3611.md`
- `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md`
- `docs/planning/PR_CONTROL_PLANE_DECRUFT_COMPATIBILITY_CUT_PLAN.md`
- `AGENTS.md`
- `docs/templates/prompts/current.json`

## Compatibility Phases

| Phase | Timing | Terminal shim behavior | Generated-card and live-guidance behavior | Deletion behavior |
| --- | --- | --- | --- | --- |
| Phase 0: proven owner binaries | v0.91.5 current state | Old `adl ...` and wrapper forms continue to run where already supported. | New guidance prefers proven owner commands where #3611 moved them, while issue work still teaches `adl/tools/pr.sh`. | No deletion. |
| Phase 1: opt-in warnings | v0.91.5 follow-on window | Warnings may be hidden unless an explicit environment flag or debug mode requests them. | Validators and review checks may fail or flag new durable cards that teach deprecated command strings. | No deletion. |
| Phase 2: default warnings | v0.92 or later, after active-bundle scan | Deprecated executable forms print visible warnings with owner-command replacements. | Generated cards must already avoid deprecated forms unless a compatibility issue explicitly permits them. | No deletion unless scan proves no active dependency. |
| Phase 3: compatibility cut | v0.92 cleanup tranche or later | Selected deprecated forms may fail closed with migration guidance. | Historical records remain readable; current cards use supported commands only. | Deletion only through bounded issues with scan evidence and regression fixtures. |

## Command-Family Sunset Table

| Command family | Current owner truth | Preferred owner command | v0.91.5 shim posture | Generated-card rule | Sunset route |
| --- | --- | --- | --- | --- | --- |
| `adl/tools/pr.sh create/init/doctor/run/finish/closeout` | Canonical agent-facing issue-work wrapper. | No public replacement yet. `adl-csdlc` may own internals only after wrapper equivalence is proven. | Keep canonical. Do not warn. | Keep teaching `adl/tools/pr.sh run <issue>` for issue binding. | No sunset before a tracked wrapper migration updates `AGENTS.md`, templates, skills, portable adapters, and conductor routing together. |
| direct `adl pr ...` issue-mode commands | Legacy Rust compatibility surface. | Future `adl-csdlc issue ...` only after wrapper migration gate. | Keep executable as compatibility. Optional warning later. | Do not teach direct `adl pr ...` in new cards or live skills. | Active-bundle scan plus wrapper migration issue before cut. |
| `adl pr run <adl.yaml>` runtime-through-PR | Deprecated ambiguity path. | `adl-runtime run <adl.yaml> ...` | Keep only as compatibility/fail-closed path defined by run-ambiguity policy. | Forbid in new generated cards. Runtime cards should use `adl-runtime run`. | Cut after scan proves no active cards or workflow docs depend on it. |
| `adl tooling prompt-template ...` | Legacy umbrella tooling form. | `adl-csdlc tooling prompt-template ...` | Keep shim. Warning may become opt-in, then default. | Use `adl-csdlc tooling prompt-template ...` in live docs and new cards where prompt-template commands are needed. | Cut only after skills, templates, cards, and portable adapter docs are migrated. |
| `adl tooling csdlc-prompt-editor` and C-SDLC card tooling | Legacy umbrella tooling form. | `adl-csdlc ...` owner family, command names case-by-case. | Keep shim. | Do not teach new direct editor commands unless the issue explicitly requires editor operation. | Defer until field-level editor and template-schema work stabilize. |
| review tooling under `adl tooling ...` | Legacy umbrella review form. | `adl-review ...` | Keep shim. | Use `adl-review ...` in live review guidance after #3611. | Cut after review skills and output contracts use `adl-review` only. |
| `adl <adl.yaml>`, `adl resume`, `adl demo`, `adl runtime-v2`, `adl provider`, `adl agent`, `adl instrument`, `adl learn`, `adl keygen/sign/verify`, `adl godel`, `adl identity`, `adl artifact` | Legacy umbrella runtime forms. | `adl-runtime ...` | Keep shims. Warnings only after runtime docs/cards have migrated. | New runtime proof cards should prefer `adl-runtime ...` when the command family is in scope. | Cut only after runtime docs, v0.92 activation packets, and active cards are scanned. |
| `adl/tools/codex_pr.sh ...` | Legacy compatibility wrapper. | `adl/tools/pr.sh ...` now; later `adl-csdlc` internals if proven. | Keep for historical/compatibility paths. | Do not prefer in new cards unless testing the legacy wrapper itself. | Cut after active-bundle scan and PR-control-plane decruft issue. |
| hypothetical `adl-crypto`, `adl-godel`, `adl-identity` | Not implemented and not approved by #3614. | Keep under `adl-runtime` for v0.91.5. | Not applicable. | Forbid these helper binaries in generated cards until a future issue implements them. | Future helper-binary mini-sprint only after three-owner split review. |

## Warning Policy

Terminal warnings and durable workflow validation intentionally have different
strictness.

- Interactive terminal shims may start with opt-in warnings so ongoing work is
  not interrupted by noise.
- Generated cards, prompt templates, skills, and live public guidance are
  durable process state. They should be strict earlier so deprecated commands do
  not fossilize into new records.
- Historical evidence packets may preserve old command strings as history.
  They should not be rewritten just to satisfy live guidance scans.
- Any warning text should include the owner command and the reason, for example:
  `deprecated compatibility command: use adl-runtime run <adl.yaml>; issue work
  still uses adl/tools/pr.sh run <issue>`.

## Active-Bundle Scan Gate

No compatibility shim deletion may begin until a bounded issue produces an
active-bundle scan report.

The scan must classify command references as:

- `active`: current issue cards, open work packages, templates, live skills,
  root guidance, portable adapter contracts, and release-tail instructions;
- `historical`: closed milestone evidence, archived review packets, old PR
  bodies, and records that must remain readable but not executable;
- `unknown`: references that need human routing before deletion.

Minimum scan inputs:

- `.adl/v0.91.5/tasks/`
- `.adl/cards/`
- `AGENTS.md`
- `docs/templates/prompts/`
- `docs/templates/`
- `adl/tools/skills/`
- `docs/milestones/v0.91.5/`
- `docs/planning/`
- portable ADL adapter docs and templates when they exist

Deletion is blocked while any active or unknown reference depends on the old
executable path.

## Alignment With #3611

#3611 moved live owner-command guidance without deleting compatibility shims.
This policy preserves that split:

- `adl/tools/pr.sh` remains the issue-work command taught to agents.
- `adl-csdlc tooling prompt-template ...` is the preferred prompt-template
  tooling command in live guidance.
- `adl-runtime ...` is the preferred runtime command family where runtime
  execution is being documented.
- `adl-review ...` is the preferred review command family where review tooling
  is being documented.
- Historical docs and closed review evidence are not automatically rewritten.

## Alignment With v0.92 Decruft Planning

`docs/planning/PR_CONTROL_PLANE_DECRUFT_COMPATIBILITY_CUT_PLAN.md` remains the
cleanup-tranche planning surface for future executable cut work.

#3615 adds the missing bridge between the v0.91.5 owner-binary split and that
future cleanup:

- v0.91.5 proves boundaries and records command policy;
- v0.91.5 does not delete shims;
- v0.92 or later may run the active-bundle scan and cut selected shims through
  bounded issues.

## Follow-On Routes

| Route | Purpose | Required before execution |
| --- | --- | --- |
| #3582 | Rewrite downstream issue cards after prompt templates v1.1. | Keep issue-work command truth as `adl/tools/pr.sh`; reject runtime-through-PR strings in new generated cards. |
| #3621 | Add field-level prompt-card editor backed by markdown-rs schemas. | Use this policy when deciding whether command-string fields are editable values or locked template structure. |
| #3622 | Split prompt-template editor internals after card rewrite. | Keep split behavior-preserving; do not move compatibility enforcement into unreviewed helper binaries. |
| #3623 | Create runtime-v2 feature navigation registry before v0.92 activation. | Prefer `adl-runtime` command names for new runtime-facing v0.92 activation packets. |
| #3628 | Produce the required active-bundle scan gate before compatibility deletion. | Must be closed before any executable shim removal issue. |
| `v0.92 PR-control-plane decruft issue wave` | Execute the compatibility cut line from the existing decruft plan. | Requires active-bundle scan, focused fixtures, and review packet. |

## Validation

This policy packet is validated by focused docs and command-guidance checks
only. It does not require Rust behavior validation because no executable path is
changed.

Recommended checks:

- `bash adl/tools/test_cli_owner_command_guidance.sh`
- `bash adl/tools/run_owner_validation_lane.sh csdlc`
- `git diff --check`
- structured prompt validation for touched lifecycle cards

## Non-Claims

- This issue does not remove or disable any compatibility shim.
- This issue does not complete the wrapper migration.
- This issue does not claim `adl-csdlc issue run <issue>` is the public
  issue-work entrypoint.
- This issue does not approve `adl-crypto`, `adl-godel`, or `adl-identity` as
  separate helper binaries.
- This issue does not rewrite historical records.
