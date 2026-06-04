# v0.91.5 Module Navigability Review

Issue: #3612
Umbrella: #3592
Captured: 2026-06-04
Status: ready_for_follow_on_routing

## Purpose

This packet reviews ADL Rust module navigability after the first CLI ownership
split mini-sprint. The goal is to identify safe consolidation and decomposition
candidates without making silent behavior changes while the project is still
closing v0.91.5 Sprint 1.

This is an architecture/refactoring review packet, not an implementation PR for
deep module surgery.

## Scope

Included:

- Rust source shape under `adl/src`.
- The large-module manual watch-list when available.
- The first-level module clusters that affect review cost and test-lane scope.
- Safe follow-on slices for C-SDLC tooling, runtime-v2, CLI tests, and
  long-lived agent surfaces.

Excluded:

- Workspace crate split implementation.
- Runtime behavior changes.
- Feature activation changes for v0.92.
- OpenTelemetry or full structured runtime observability implementation.
- Broad Rust test execution.

## Deterministic Metrics

Command:

```bash
bash adl/tools/report_module_navigability.sh --top 12
```

Result summary:

| Signal | Value |
| --- | ---: |
| Rust files under `adl/src` | 496 |
| Rust LoC under `adl/src` | 206,460 |
| Largest single source file | `adl/src/csdlc_prompt_editor.rs` at 2,055 LoC |
| Largest first-level source cluster | `adl/src/runtime_v2` at 163 files / 69,098 LoC |
| Second-largest first-level source cluster | `adl/src/cli` at 141 files / 55,779 LoC |

Top file hotspots from the deterministic report:

| File | LoC | Initial disposition |
| --- | ---: | --- |
| `adl/src/csdlc_prompt_editor.rs` | 2,055 | Split after Sprint 1 card rewrite stabilizes. |
| `adl/src/runtime_v2/contract_market_demo.rs` | 1,382 | Keep behavior stable; route through runtime-v2 feature organization. |
| `adl/src/cli/tests/run_state/persistence.rs` | 1,381 | Review with CLI test navigation follow-on. |
| `adl/src/long_lived_agent.rs` | 1,377 | Extract only after run-loop characterization tests. |
| `adl/src/cli/tests/pr_cmd_inline/lifecycle/start_ready.rs` | 1,375 | Review with CLI test navigation follow-on. |
| `adl/src/runtime_v2/humor_and_absurdity.rs` | 1,370 | Keep feature module intact until v0.92 activation map confirms ownership. |
| `adl/src/runtime_v2/kindness_model.rs` | 1,315 | Keep feature module intact until v0.92 activation map confirms ownership. |
| `adl/src/runtime_v2/moral_resources.rs` | 1,314 | Keep feature module intact until v0.92 activation map confirms ownership. |

Manual tracker note:

The local manual tracker at
`.adl/reports/manual/rust_module_watch_list.md` was available in the primary
checkout and reviewed as operator-local evidence. It is not treated as tracked
release evidence in this PR. Its earlier top-file ordering is now stale for
`csdlc_prompt_editor.rs`, which has grown to 2,055 LoC after Sprint 1 prompt
template work. Future quality gates should regenerate the tracker rather than
copy old values.

## Findings

### P2: C-SDLC prompt editor now mixes too many ownership layers

Evidence:

- `adl/src/csdlc_prompt_editor.rs` is the largest Rust file in the tree at
  2,055 LoC.
- The file currently carries editor data models, template values rendering,
  locked-field policy, Markdown/structure validation, and schema-facing helper
  logic.
- Sprint 1 is actively using this surface for prompt-template v1.1 and card
  rewrite work.

Impact:

This file is now central enough that every prompt-template change risks dragging
renderer, editor, validator, and schema concerns into the same review. That is
exactly the review-cost pattern the mini-sprint is trying to reduce.

Recommended follow-on:

Create a behavior-preserving C-SDLC prompt tooling split after `#3582` or its
card-rewrite equivalent lands. The safe shape is likely:

- model/types;
- values validation;
- rendering;
- structure-schema validation;
- Markdown AST / locked-prose validation;
- editor export/import surface.

Required proof:

- Existing prompt-template validator tests must pass before and after the split.
- Generated-card samples must remain byte-stable unless an explicit template
  version change is approved.
- The split must not change the active `SIP -> STP -> SPP -> SRP -> SOR`
  lifecycle semantics.

### P2: Runtime-v2 is the dominant cluster and needs a navigation contract, not immediate churn

Evidence:

- `adl/src/runtime_v2` is the largest first-level cluster at 163 files and
  69,098 LoC.
- `adl/src/runtime_v2/mod.rs` is a broad public re-export hub for many feature
  surfaces.
- Several runtime-v2 files in the top hotspot list are v0.92 activation
  features: humor, kindness, moral resources, private-state observatory, and
  contract-market demo surfaces.

Impact:

Runtime-v2 is the centerpiece surface for v0.92. Blindly splitting or moving it
now would reduce visible file size while increasing activation risk. The current
problem is primarily navigability and ownership indexing, not a proven behavior
bug.

Recommended follow-on:

Create a runtime-v2 feature registry/navigation issue before deeper code moves.
The first safe deliverable should classify runtime-v2 modules by owner family,
activation milestone, proof lane, and public re-export posture.

Required proof:

- No runtime behavior movement in the registry issue.
- v0.92 activation map links to the runtime-v2 ownership index.
- Any later code movement gets characterization tests before edits.

### P3: CLI test forests are large enough to obscure command ownership proof

Evidence:

- `adl/src/cli` is the second-largest first-level cluster at 141 files and
  55,779 LoC.
- Several large hotspot files are test files under `adl/src/cli/tests`, notably
  run-state persistence and PR lifecycle readiness tests.
- The CLI ownership split succeeded in creating `adl-csdlc`, `adl-runtime`, and
  `adl-review`, but the test layout still makes it expensive to answer "which
  command family does this prove?"

Impact:

The refactor mini-sprint reduced command ownership ambiguity, but future PRs
can still pay broad test-review cost if tests are not indexed by owner lane and
proof role.

Recommended follow-on:

Create a CLI test navigation issue that adds a test ownership index and trims
duplicated helper setup where safe. Avoid broad test rewrites until each helper
extraction has characterization proof.

Required proof:

- Owner lanes still pass after any test helper extraction.
- Test names or docs make command-family ownership visible.
- Generated cards and skills do not resurrect deprecated command strings.

### P3: Long-lived agent root file needs a conservative run-loop extraction plan

Evidence:

- `adl/src/long_lived_agent.rs` remains a top hotspot at 1,377 LoC.
- The adjacent `adl/src/long_lived_agent/` directory already contains
  `inspection`, `schema`, `storage`, `tests`, and `types` surfaces.

Impact:

The long-lived agent is an operational surface that will interact with
observability, runtime control, and multi-agent execution. It should not be
split casually, but leaving the root file as the main orchestration sink will
make runtime/agent reliability review harder.

Recommended follow-on:

Create a long-lived agent run-loop extraction issue that starts with
characterization tests and only then extracts a small orchestration helper
module if behavior can be preserved.

Required proof:

- Tick/lease/status tests remain stable.
- No persistence layout or artifact path changes without explicit approval.
- Logging/observability work remains a separate follow-on unless the issue is
  explicitly widened.

## Safe First Slice

This issue intentionally avoids moving Rust behavior. The safe first slice is
the new deterministic report helper:

```bash
bash adl/tools/report_module_navigability.sh --top 25
```

That helper gives future quality gates and refactor reviews a reproducible
source-shape baseline without deciding whether any module should be changed.

## Follow-On Issue Set

The following work should not be lost after this mini-sprint:

| Issue | Suggested milestone | Why it exists |
| --- | --- | --- |
| `#3622` `[v0.91.5][refactor][csdlc] Split prompt-template editor internals after card rewrite` | v0.91.5 | `csdlc_prompt_editor.rs` is now the largest source file and mixes renderer/editor/schema concerns. |
| `#3623` `[v0.91.5][refactor][runtime] Create runtime-v2 feature navigation registry before v0.92 activation` | v0.91.5 | Runtime-v2 is the largest cluster and needs an ownership map before behavior movement. |
| `#3624` `[v0.91.5][refactor][tests] Add CLI test ownership index and consolidate helpers safely` | v0.91.5 | CLI tests remain broad and hard to map to command-family proof. |
| `#3625` `[v0.91.5][refactor][agent] Plan long-lived agent run-loop extraction with characterization proof` | v0.91.5 | Long-lived agent orchestration needs a bounded split before observability/runtime work leans on it. |

## Refactor Sequencing Recommendation

1. Finish Sprint 1 prompt-template/card rewrite work before splitting
   `csdlc_prompt_editor.rs`.
2. Create the runtime-v2 navigation registry before any runtime-v2 code move.
3. Apply test ownership indexing before trying to reduce CLI test file count.
4. Add long-lived agent characterization tests before extracting run-loop code.
5. Keep workspace-crate splits and OpenTelemetry implementation out of this
   mini-sprint unless a later issue explicitly scopes them.

## Validation

Validation for this issue:

| Command | Purpose |
| --- | --- |
| `bash adl/tools/test_module_navigability_report.sh` | Proves the deterministic report helper emits schema, totals, file hotspots, and cluster rows. |
| `bash adl/tools/report_module_navigability.sh --top 12` | Produces the review evidence summarized in this packet. |
| `bash adl/tools/run_owner_validation_lane.sh csdlc` | Focused C-SDLC/tooling lane because this issue adds a repo tool and planning packet. |
| `git diff --check` | Patch hygiene. |
| `bash adl/tools/validate_structured_prompt.sh --type srp --input <srp>` | SRP lifecycle-record proof. |
| `bash adl/tools/validate_structured_prompt.sh --type sor --input <sor>` | SOR lifecycle-record proof. |

## Non-Claims

- This packet does not claim ADL module complexity is solved.
- This packet does not claim runtime-v2 is ready for v0.92 activation by itself.
- This packet does not claim the full Rust test suite was run.
- This packet does not move runtime, C-SDLC, review, or long-lived agent
  behavior.
- This packet does not implement OpenTelemetry or long-running runtime
  observability.

## Residual Risk

The largest remaining risk is treating navigability metrics as a refactor
mandate. Large files and clusters identify where review cost accumulates; they
do not prove that splitting the code is safe. Each follow-on must preserve
behavior through characterization tests and owner-lane proof before claiming
quality or speed improvement.
