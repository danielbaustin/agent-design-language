# Five-Command Regression Suite

This is the canonical regression entrypoint for the implemented v0.85 editing lifecycle.

Run:

- `bash adl/tools/test_five_command_regression_suite.sh`

## What It Covers

The suite protects the shipped five-command surface and its truthful editor claims:

1. `pr init`
2. `pr create`
3. `pr start`
4. `pr run`
5. `pr finish`

It also verifies:

- the installed `adl_pr_cycle` skill still matches the tracked contract and preserves the real five-command state machine
- the browser/editor adapter remains bounded to `pr start`
- the editor docs do not overclaim direct browser execution for the other commands
- the bounded five-command demo still emits the expected lifecycle artifacts

## Suite Components

- `adl/tools/test_pr_init.sh`
- `adl/tools/test_pr_create.sh`
- `adl/tools/test_pr_start_template_validation.sh`
- `adl/tools/test_install_adl_pr_cycle_skill.sh`
- `adl/tools/test_pr_run.sh`
- `adl/tools/test_pr_finish_default_stage_root.sh`
- `adl/tools/test_pr_finish_relative_card_paths.sh`
- `adl/tools/test_editor_action.sh`
- `adl/tools/test_demo_five_command_editing.sh`
- `adl/tools/test_five_command_editor_truth.sh`

## Why This Is The Proof Surface

This suite is the deterministic guardrail for the full editing story now implemented in the repo.

It does not invent a second lifecycle. Instead, it reuses the real command tests, the bounded demo, and the editor truth contract checks so drift in commands, artifacts, or browser claims fails in one place.
