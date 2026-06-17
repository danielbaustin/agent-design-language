# Rust Module Consolidation Decisions

Issue: `#3750`  
Umbrella: `#3745`  
Captured: 2026-06-17  
Status: ready_for_follow_on_routing

## Purpose

This packet reviews current `parts`-style Rust module shapes and records where
ADL should consolidate, rename, leave alone, or defer work. The goal is to
reduce review cost and validation blast radius without repeating earlier
generic-split churn.

## Inputs

- `.adl/reports/manual/rust_module_watch_list.md` regenerated locally on
  2026-06-17 from `bash adl/tools/report_large_rust_modules.sh --root adl/src --format tsv`
- `bash adl/tools/report_module_navigability.sh --top 12 --format tsv`
- `docs/milestones/v0.91.5/MODULE_NAVIGABILITY_REVIEW_3612.md`
- `docs/milestones/v0.95/features/CONTROL_PLANE_RUST_MIGRATION_AND_TOOLING_HARDENING_v0.95.md`
- current source slices under `adl/src/`

## Snapshot

- Current dominant hotspots still include control-plane files already
  identified and routed in the sprint planning material: `cli/pr_cmd/github.rs`,
  `csdlc_prompt_editor.rs`, `cli/pr_cmd/finish_support.rs`, and
  `cli/run_artifacts_types.rs`.
- First-level cluster weight is still concentrated in `adl/src/cli` and
  `adl/src/runtime_v2`, each at about 69k lines from the navigability report.
- The reviewed `parts` families are not all equal: some reduce cognitive load,
  while others add scatter through tiny generic helper buckets.

## Decision Table

| Surface | Current shape | Disposition | Why |
| --- | --- | --- | --- |
| `adl/src/runtime_v2/governed_tools_flagship_demo.rs` + `governed_tools_flagship_demo_parts/` | 1,451 lines spread across `core`, `constants`, `helpers`, `models`, `reports`, `trace_support`, `cases` | Consolidate or rename on next touch | This is the clearest example of generic `parts` churn. The root file delegates into several tiny files, including `core.rs` at 19 lines and `constants.rs` at 29 lines, which increases lookup hops without buying an obvious proof boundary. |
| `adl/src/runtime_v2/cultivating_intelligence.rs` + `cultivating_intelligence_parts/` | 1,548 lines across `models`, `builder`, `validation` | Defer, keep current split | The container suffix is generic, but the internal split is semantic and reviewable. `models`, `builder`, and `validation` are the right proof-owning boundaries; the main remaining concern is naming polish, not structural urgency. |
| `adl/src/runtime_v2/wellbeing_metrics.rs` + `wellbeing_metrics_parts/` | 1,619 lines across `models`, `builder`, `validation` | Defer, keep current split | Same posture as cultivating intelligence. The file family is large enough to watch, but the current boundaries already line up with responsibility and validation role. |
| `adl/src/governed_executor.rs` + `governed_executor_parts/` | 1,591 lines split between `logic.rs` and test-only `tests.rs` | No-op | This is not a generic production split problem. The production boundary is a single `logic.rs` file, and the other file is test-only characterization kept out of the shipped binary. |
| `adl/src/provider/http_family.rs` + `provider/http_family/config.rs` | 811-line transport file plus focused config helper | No-op | The file is near the watch threshold, but the split is semantically strong and already names the policy/config boundary clearly. This should not be churned just because it is large. |

## Findings

### One real consolidation candidate exists now

`governed_tools_flagship_demo_parts` is the strongest candidate for future
cleanup because it hides one conceptual surface behind several tiny generic
files. A later issue should either:

- flatten the tiny files back into one semantically named module, or
- rename the bundle into explicit submodules that match the proof surfaces
  actually owned there.

Required proof for that follow-on:

- characterization tests for the flagship proof bundle stay green
- public/operator report rendering stays byte-stable unless explicitly approved
- no new generic `*_parts` buckets are introduced in the replacement shape

### Several reviewed `parts` families should be left alone for now

The `cultivating_intelligence`, `wellbeing_metrics`, and `governed_executor`
surfaces do not currently show the same smell. They either:

- already split along semantic boundaries that reduce review cost, or
- isolate test-only code from production code without widening runtime review
  burden

Changing those now would mostly create churn without reducing the next issue's
validation cost.

## Guidance For Future Refactors

1. Do not create `*_parts` containers unless the submodules are semantically
   named and each owns a local proof boundary.
2. Prefer top-level semantic names like `models`, `builder`, `validation`,
   `transport`, `reports`, or `checks` over `core`, `helpers`, or `parts`
   when the latter hide the real responsibility.
3. Do not claim success from line-count reduction alone; a refactor only helps
   when the next change can validate a narrower boundary or the reviewer can
   read less unrelated code.
4. Treat local watch-list snapshots as operator evidence, then carry the real
   decisions into tracked review packets like this one.

## Outcome

This issue intentionally performs no code movement. Its useful result is the
routing decision:

- keep the semantic builder/models/validation families as-is for now
- keep `provider/http_family` and `governed_executor` as-is
- route `governed_tools_flagship_demo_parts` as the next real consolidation
  candidate when runtime-v2 demo work touches that area again

## Validation

Commands run:

```bash
bash adl/tools/report_large_rust_modules.sh --root adl/src --format tsv
bash adl/tools/report_module_navigability.sh --top 12 --format tsv
```

Result: passed.

These commands provided the current sizing and cluster evidence used for the
decisions above. No broad Rust test suite was run because this issue changes
review artifacts only and intentionally makes no behavior changes.
