# CLI Owner Command Guidance Audit (#3611)

Issue: #3611
Status: implemented

## Purpose

Keep live ADL guidance aligned with the first CLI ownership split without
creating a second workflow truth.

The migration rule remains conservative:

- `adl/tools/pr.sh` is still the canonical agent-facing issue-work wrapper.
- `adl-csdlc` owns C-SDLC tooling and future wrapper internals where proven.
- `adl-runtime` owns runtime workflow execution and runtime-facing command
  families where proven.
- `adl-review` owns review tooling where proven.

## Surfaces Audited

- Root agent guidance: `AGENTS.md`
- Prompt-template docs: `docs/templates/prompts/README.md`
- Invocation template docs: `docs/templates/PR_INIT_INVOCATION_TEMPLATE.md`
- Live operational skills: `adl/tools/skills/`
- Editor adapter docs: `docs/tooling/editor/`
- Refactor truth docs:
  - `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md`
  - `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md`
  - `docs/milestones/v0.91.5/CLI_RUNTIME_COMPATIBILITY_3598.md`
  - `docs/milestones/v0.91.5/CLI_REVIEW_COMPATIBILITY_3599.md`

Historical review packets and archived milestone evidence were intentionally
not rewritten. Those files may preserve old commands as historical evidence.

## Updates Made

- Removed direct `adl pr ...` commands from live `pr-init`, `pr-ready`,
  `pr-run`, and `pr-finish` skill guidance and machine-readable skill metadata.
- Kept `adl/tools/pr.sh run <issue>` as the taught issue-binding command.
- Updated prompt-template guidance to use
  `adl-csdlc tooling prompt-template ...` where the owner binary is now proven.
- Updated the repo-code-review output contract to use
  `adl-review verify-repo-contract --review <review.md>`.
- Added `adl/tools/test_cli_owner_command_guidance.sh` as a fast guardrail for
  live guidance drift.

## Command Policy

| Surface | Taught command |
| --- | --- |
| Issue creation/bootstrap | `adl/tools/pr.sh create`, `adl/tools/pr.sh init` |
| Issue readiness | `adl/tools/pr.sh doctor --json` |
| Issue execution binding | `adl/tools/pr.sh run <issue>` |
| Issue finish/publication | `adl/tools/pr.sh finish` |
| Prompt-template rendering/validation | `adl-csdlc tooling prompt-template ...` |
| Runtime workflow execution | `adl-runtime run <adl.yaml> ...` |
| Review contract validation | `adl-review verify-repo-contract --review <review.md>` |

## Deferred Routes

- Do not teach `adl-csdlc issue run <issue>` as the primary agent-facing issue
  command until the wrapper migration gate explicitly changes.
- Do not rewrite downstream issue cards in this issue; that remains #3582.
- Do not rewrite historical review evidence or old milestone packets.
- Do not remove legacy `adl ...` compatibility shims in this issue; deprecation
  and sunset policy remains #3615.

## Validation

- `bash adl/tools/test_cli_owner_command_guidance.sh`
- `bash adl/tools/run_owner_validation_lane.sh csdlc`
- `git diff --check`

## Non-Claims

- This does not complete the wrapper migration.
- This does not remove compatibility shims.
- This does not claim every historical command string in the repository was
  rewritten.
- This does not perform the downstream card rewrite owned by #3582.
