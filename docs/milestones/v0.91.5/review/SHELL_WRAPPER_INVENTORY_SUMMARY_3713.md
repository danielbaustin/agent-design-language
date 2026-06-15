# Shell Wrapper Inventory Summary for #3713

Date: 2026-06-14
Milestone: v0.91.5
Issue: #3713
Authoritative machine inventory: [SHELL_WRAPPER_INVENTORY_3713.tsv](SHELL_WRAPPER_INVENTORY_3713.tsv)
Focused validation: `bash adl/tools/test_shell_wrapper_inventory.sh`

## Purpose

This packet records the current status of every surviving root-level shell
wrapper under `adl/tools/*.sh` so the refactored `adl-csdlc` command surface can
advance without hiding legacy wrapper obligations.

The inventory does not claim that all wrappers have been migrated to Rust. It
classifies each wrapper by its current role so follow-on work can separate
workflow transport gaps from ordinary local utilities and validation fixtures.

## Status Classes

| Status | Count | Meaning |
| --- | ---: | --- |
| `delegated_to_adl_csdlc` | 3 | Compatibility wrappers that delegate to the Rust `adl-csdlc` control plane for covered workflow behavior. |
| `explicit_fail_closed_gap` | 1 | A wrapper with a known GitHub/workflow gap that must fail closed or route to explicit follow-up instead of silently using legacy transport. |
| `local_only_utility` | 357 | Local validation, demo, setup, reporting, or helper scripts that are not the source of issue/PR workflow truth. |
| `scheduled_for_removal` | 2 | Legacy wrappers retained temporarily but marked for removal rather than migration. |

Total wrappers inventoried: 363.

## Key Findings

- The issue/PR workflow surface is small enough to police: only `pr.sh`,
  `card_prompt.sh`, and `review_card_surface.sh` are classified as delegated
  `adl-csdlc` compatibility wrappers.
- `attach_post_merge_closeout.sh` is the only current explicit fail-closed gap in
  this inventory and should stay visible until routed or replaced.
- The large majority of shell scripts are local utilities, validation lanes,
  demos, historical proof helpers, or setup/reporting scripts. They should not be
  mistaken for remaining GitHub transport authority.
- `codex_pr.sh` and `codexw.sh` are marked `scheduled_for_removal`; they should
  not regain authority by accident.

## Validation Contract

The focused validation script checks that:

- the inventory has the expected tab-separated schema,
- every row has a known status value and non-empty rationale,
- every listed wrapper still exists,
- no wrapper path is duplicated,
- the inventory exactly matches the current sorted set of `adl/tools/*.sh` files.

This is intentionally narrow. It proves inventory freshness and shape, not the
runtime behavior of every wrapper.

## Non-claims

- This does not remove or rewrite the wrapper fleet.
- This does not prove every local utility is still useful.
- This does not claim release/watcher GitHub gaps are implemented; those are
  routed separately by #3712 and #3718.
- This does not complete prompt-template or Markdown AST integration work; those
  remain separate checklist tracks.

## Checklist Impact

This satisfies the v0.91.5 checklist item:

> Every surviving shell wrapper has one of these statuses: delegated to
> `adl-csdlc`, local-only utility, explicit fail-closed gap, or scheduled for
> removal.
