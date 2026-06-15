# Prompt Template Workflow Integration Proof for #3714

Date: 2026-06-14
Milestone: v0.91.5
Issue: #3714
Focused proof: `bash adl/tools/test_prompt_template_workflow_integration.sh`

## Purpose

This packet records the v0.91.5 proof boundary for C-SDLC prompt-template card
handling. It separates the supported Rust-owned workflow path from legacy shell
compatibility paths so the checklist can advance without pretending every old
wrapper has already been deleted.

## Supported Path

The supported card-producing workflow path is the Rust `adl-csdlc` / `adl pr`
control plane invoked through `adl/tools/pr.sh run`, `finish`, and `closeout`.
For that path, the repository already carries Rust tests proving that issue
bootstrap resolves the active registry and renders versioned prompt templates.
This issue adds a focused operator proof lane that exercises the public
prompt-template commands directly.

## Proof Coverage

`adl/tools/test_prompt_template_workflow_integration.sh` proves:

- `docs/templates/prompts/current.json` is resolvable through `--repo-root`.
- Sample values can be written for all five lifecycle cards.
- `render-all` renders `sip`, `stp`, `spp`, `srp`, and `sor` from values.
- `validate-values` passes for each values file.
- `validate-structure` passes for each rendered card.
- `validate-structured-prompt` accepts each rendered card, with `SOR` checked in
  bootstrap phase.
- `edit-values` can update a declared editable field without patching rendered
  Markdown.
- The edited values can be rendered and structure-validated.
- `import-values` can recover values from the rendered edited STP card.
- `validate-schemas` verifies tracked structure schemas against the active
  templates.
- `python3 adl/tools/test_prompt_template_structure_schemas.py` verifies the
  Python-readable schema smoke path.

## Existing Rust Coverage This Relies On

The existing Rust suite already covers the workflow bootstrap integration path.
The relevant focused tests are:

- `bootstrap_cards_use_versioned_prompt_templates_when_available`;
- `prompt_template_registry_redirects_rust_template_loading`;
- `versioned_bootstrap_refreshes_existing_template_placeholder_cards`;
- `pre_run_bootstrap_cards_preserve_reviewed_design_time_ready_spp`;
- `prompt_template_cli_renders_and_validates_all_five_cards_from_values`;
- `prompt_template_cli_edits_declared_values_and_fails_closed`;
- `prompt_template_cli_imports_values_and_round_trips_rendered_card`;
- `prompt_template_cli_rejects_markdown_structure_drift`.

This issue's new script is the operator-facing proof lane for those public
commands. It does not itself execute a full `pr run`/`finish`/`closeout` cycle.

This issue does not duplicate all of those unit tests. It adds the operator
proof lane and the review packet needed by the checklist.

## Legacy Compatibility Boundary

`adl/tools/pr.sh` still contains legacy compatibility functions for direct card
commands. Those functions are not the taught workflow path for issue execution.
The taught issue workflow routes through the Rust control plane first.

Remaining legacy card command cleanup should be handled as a separate removal or
compatibility issue rather than silently widened into this proof issue. Until
that cleanup lands, the legacy shell functions are a compatibility surface, not
the authority for new card-generation workflow claims.

## Checklist Impact

This proof supports checking the prompt-template checklist items for:

- renderer/editor-skill boundary documentation;
- focused prompt-template validation lane;
- active registry resolution;
- values editing/import proof;
- render plus structure validation for all five cards;
- schema validation and Python schema smoke proof.

It does not claim Markdown AST editing is implemented, and it does not retire the
legacy shell compatibility card functions.

## Non-claims

- This does not change canonical prompt-template semantics.
- This does not create a new prompt-template version.
- This does not claim every legacy shell card command has been removed.
- This does not replace editor skills for lifecycle-truth repairs.
