# Finish Validation Path Ownership Registry

`pr finish` still fails closed for unknown changed paths.

That behavior is intentional. What changed in `#4418` is where known path
ownership is declared.

## Purpose

The finish-validation selector needs one bounded place to answer four questions
for a changed path:

- which owner binary or owner lane owns the surface
- which focused validation lane applies
- which proof role the path triggers
- whether the known classification is publication-sufficient

## Current Registry Surface

The registry currently lives in
`adl/src/cli/pr_cmd/finish_support.rs` as `FINISH_PATH_OWNERSHIP_RULES`.

Each rule declares:

- exact paths and optional path prefixes
- `owner_binary`
- `validation_lane`
- `proof_role`
- `publication_sufficient`

Matching is additive for registry queries. If more than one
`FINISH_PATH_OWNERSHIP_RULES` entry matches the same path, every matching
publication-sufficient rule contributes its owner/proof-role classification.
Shared paths must therefore either agree on the validation lane or carry an
explicit regression test proving the intended combined behavior.

## How To Declare A New Command Surface

When a new command or control-plane surface should be recognized by
`pr finish`:

1. Add the new exact path or prefix to `FINISH_PATH_OWNERSHIP_RULES`.
2. Set the owner binary or owner lane that is responsible for the surface.
3. Set the focused validation lane that should run when the path changes.
4. Set the proof role that should be triggered by the path.
5. Add or update a regression test in
   `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs`.

If a path does not belong to any known rule, leave it unclassified and let
finish fail closed.

## Non-goals

- This registry does not replace the future validation manager.
- This registry does not auto-classify unknown paths.
- This registry does not make docs-only or wider owner-lane policy optional.
