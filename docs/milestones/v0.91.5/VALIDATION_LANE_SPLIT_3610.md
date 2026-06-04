# Owner Validation Lane Split (#3610)

Issue: #3610
Status: implementation slice and split plan

## Purpose

The first CLI refactor mini-sprint proved owner binaries, but it did not make
ordinary validation appreciably faster by itself. This issue adds a concrete
owner-lane runner and records the next split boundary so C-SDLC, runtime, and
review changes can use focused proof before broad integration proof.

## Immediate Implementation

The new focused runner is:

```bash
adl/tools/run_owner_validation_lane.sh <csdlc|runtime|review|all> [--build] [--print-plan]
```

Owner lanes:

| Lane | Commands |
| --- | --- |
| `csdlc` | `test_cli_wrapper_migration_contract.sh`, `test_pr_run_ambiguity_policy.sh`, `test_control_plane_observability.sh` |
| `runtime` | `test_adl_runtime_compatibility.sh` |
| `review` | `test_adl_review_compatibility.sh` |
| `all` | `csdlc`, then `runtime`, then `review` |

`--build` builds `adl`, `adl-csdlc`, `adl-runtime`, and `adl-review` once and
then runs compatibility scripts through prebuilt binary overrides. This avoids
repeated `cargo run` startup costs inside the owner compatibility scripts.

`--print-plan` prints the lane commands without executing them. It is intended
for issue cards, docs review, and external adapter diagnostics.

## Timing Evidence

Measured from the #3610 worktree on 2026-06-03.

| Command | Context | Result | Wall time |
| --- | --- | --- | --- |
| `bash adl/tools/test_cli_wrapper_migration_contract.sh` | shell-only focused C-SDLC check | PASS | `0.08s` |
| `bash adl/tools/test_pr_run_ambiguity_policy.sh` | shell-only focused C-SDLC check | PASS | `0.14s` |
| `bash adl/tools/test_adl_review_compatibility.sh` | fresh worktree before target warmup | PASS | `43.87s` |
| `bash adl/tools/test_adl_runtime_compatibility.sh` | fresh worktree before target warmup | PASS | `44.44s` |
| `bash adl/tools/test_adl_runtime_compatibility.sh` | warm target cache | PASS | `1.51s` |
| `bash adl/tools/test_adl_review_compatibility.sh` | warm target cache | PASS | `0.87s` |
| `bash adl/tools/run_owner_validation_lane.sh runtime --build` | warm target cache with prebuilt binary overrides | PASS | `3.44s` |
| `bash adl/tools/run_owner_validation_lane.sh review --build` | warm target cache with prebuilt binary overrides | PASS | `3.28s` |
| `bash adl/tools/run_owner_validation_lane.sh all --build` | warmed owner binaries after prior build | PASS | `0.68s` |

Interpretation:

- C-SDLC shell policy checks are already sub-second.
- Runtime/review compatibility checks are fast once built, but fresh worktrees
  still pay a large compilation/startup tax.
- The owner-lane runner gives operators one deterministic command for focused
  local proof now.
- Real first-run speedup still requires a later Cargo/workspace or target-cache
  split; this issue does not claim that larger extraction is complete.

## Validation Policy

Use the smallest proving lane for local work:

| Touched surface | Local proof before PR | Broad proof |
| --- | --- | --- |
| C-SDLC wrapper, prompt-card policy, control-plane docs | `bash adl/tools/run_owner_validation_lane.sh csdlc` | CI integration lane |
| Runtime command routing/provider/demo/agent behavior | `bash adl/tools/run_owner_validation_lane.sh runtime --build` plus focused Rust selector for touched module | CI integration lane |
| Review command routing/review packets/contracts | `bash adl/tools/run_owner_validation_lane.sh review --build` plus focused Rust selector for touched module | CI integration lane |
| Cross-owner command routing | `bash adl/tools/run_owner_validation_lane.sh all --build` | CI integration lane |
| Cargo/workspace/package changes | owner lane plus `cargo check`/focused Rust tests for affected packages | CI integration lane |

## Next Split Boundary

The safe next decomposition should not be a blind file shuffle. It should start
from these owner boundaries:

1. Extract a C-SDLC/tooling library boundary around prompt-template,
   structured-prompt, PR lifecycle, and issue-card validation internals.
2. Keep the runtime centerpiece conservative: runtime workflow, providers,
   demos, agents, signing, learning, identity, and Gödel remain together until
   their own module-size/navigability review is complete.
3. Keep review tooling as a thin owner boundary first, then extract review
   packet/contract helpers only after docs and skills migrate.
4. Preserve `adl/tools/pr.sh` as the canonical workflow spine until generated
   cards, skills, portable adapter docs, and compatibility sunset policy move
   together.

## Routed Follow-Ons

- #3607 owns local-preflight vs CI-integration proof policy.
- #3611 owns docs/skills/generated-card migration to proven owner commands.
- #3612 owns module navigability and consolidation review.
- #3614 owns deferred helper-binary candidate review.
- #3615 owns shim deprecation and compatibility sunset policy.

If a true workspace/package extraction is still required after #3607 and #3612,
open a dedicated extraction issue with a source-grounded module map, rollback
plan, and timing target.

## Non-Claims

- This does not split the Rust workspace into multiple crates.
- This does not remove the legacy `adl` binary.
- This does not weaken runtime validation for runtime-owned changes.
- This does not replace CI integration proof.
