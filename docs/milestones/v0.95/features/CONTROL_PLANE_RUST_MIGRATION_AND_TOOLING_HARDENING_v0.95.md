# v0.95 Feature: Control-Plane Rust Migration and Tooling Hardening

## Status

Forward-planning feature contract for `v0.95`.

## Purpose

Complete the highest-value Rust migration and tooling-hardening tranche so the
MVP control plane can be explained as a coherent, durable, low-friction
execution substrate rather than a mixed-language provisional layer.

## Source Inputs

- `docs/planning/PYTHON_ELIMINATION_STAGED_PLAN.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.85/features/ROAD_TO_v0.95.md`
- `.adl/reports/manual/rust_module_watch_list.md` generated 2026-06-16
- `docs/milestones/v0.91.5/CLI_REFACTOR_MINI_SPRINT_REVIEW_3600.md`
- `docs/milestones/v0.91.5/review/REFACTOR_MINI_SPRINT_CODE_REVIEW_2026-06-04.md`

## Scope

This feature should establish:

- Rust migration of the highest-risk remaining control-plane/tooling surfaces
- hardening of workflow, validation, review, and publication paths
- explicit residual-language boundary if any non-Rust tooling still remains
- final convergence between lifecycle tooling and MVP reviewability
- a late pre-Sprint-4 Rust refactoring mini-sprint that reduces
  change-specific test burden by improving ownership boundaries, fixture
  locality, and validation routing

## Rust Refactoring Mini-Sprint Placement

The current operator-directed placement is after the tooling, logging, and
toolkit-simplification sequence settles, and before Sprint 4, unless the
operator explicitly changes the sequence. This placement comes from the
2026-06-16 planning thread for issue `#3861`; the watch list itself is sizing
evidence, not sequencing evidence. The mini-sprint is not a generic
beautification pass. Its purpose is to make ordinary changes cheaper to test,
review, and reason about before `v0.95` convergence.

The mini-sprint should use the current watch list as evidence, but should not
track or move the generated report. The current source snapshot is
`.adl/reports/manual/rust_module_watch_list.md`, generated on 2026-06-16 from
`adl/tools/report_large_rust_modules.sh --format tsv`.

Success means:

- smaller issue-local ownership surfaces
- clearer characterization tests at the boundary being changed
- fewer unrelated tests required for a routine change
- lower reviewer context load
- no behavior changes without characterization proof

Success does not mean every large file is below an arbitrary line-count target
after one sprint.

## Current Watch-List Priorities

The 2026-06-16 watch list classifies modules at three levels:

- `RATIONALE`: >= 1500 LoC; any further growth needs explicit rationale and
  the module should be considered for near-term decomposition.
- `REVIEW`: >= 1000 LoC; review for ownership, test locality, and whether
  future changes can be routed through a smaller boundary.
- `WATCH`: >= 800 LoC; monitor and avoid adding broad responsibilities.

Current top `RATIONALE` targets:

| Priority | Module | LoC | Refactoring intent |
| --- | --- | ---: | --- |
| 1 | `adl/src/cli/pr_cmd/github.rs` | 4534 | Split by GitHub operation families and transport contracts so issue, PR, checks, reviews, and closeout callers can validate narrower behavior. This is now the dominant hotspot and should be treated as control-plane reliability and cycle-time work, not cosmetic decomposition. |
| 2 | `adl/src/csdlc_prompt_editor.rs` | 2468 | Continue extracting editor responsibilities into template/value/schema/import/render boundaries, with tests owned by each editor concern. |
| 3 | `adl/src/cli/pr_cmd/finish_support.rs` | 1985 | Separate finish validation planning, changed-path policy, SOR/card checks, and publication preparation so docs-only/tooling-only changes do not pay unrelated proof costs. |
| 4 | `adl/src/cli/tests/pr_cmd_inline/basics.rs` | 1752 | Split fixture setup, happy-path assertions, and compatibility/edge-case coverage so basic PR command changes can run a smaller focused fixture set. |
| 5 | `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs` | 1684 | Split finish argument-rendering fixtures by command shape, path policy, and publication mode so finish-policy edits do not require reading one oversized render matrix. |
| 6 | `adl/src/cli/run_artifacts_types.rs` | 1550 | Split stable artifact data contracts by artifact family and preserve serde/schema characterization tests near each family. |
| 7 | `adl/src/cli/tests/pr_cmd_inline/lifecycle/start_ready.rs` | 1500 | Reorganize test fixtures by lifecycle state and setup helper ownership so one start/run behavior change does not require reading the whole readiness fixture file. |

The first sprint should not attempt all `REVIEW` and `WATCH` targets. It should
use them as route planning input and pick slices that reduce the next real
change's test burden.

## Refactoring Strategy

Do not repeat the old pattern of splitting large files into generic `parts`
modules. Each extraction must name the responsibility it owns and the proof it
keeps local.

Test placement should follow the same rule. Rust `#[cfg(test)]` modules do not
ship in normal production binaries, so the sprint must not claim production
binary-size savings from moving inline tests. The real concern is production
source maintainability: large inline tests make production modules harder to
read, widen reviewer context, and couple narrow behavior changes to broad
fixture surfaces. Tiny private-helper invariant tests may remain inline when
they document the code beside them; fixture-heavy suites, CLI lifecycle
scenarios, GitHub/mock transport tests, argument-rendering matrices, and broad
behavioral tests should move into focused test modules or files when that
creates a smaller proof lane.

Preferred slice shapes:

- **Operation family modules**: for example GitHub issue operations, PR body
  operations, check-run/status operations, review/comment operations, and
  post-merge closeout operations.
- **Policy modules**: validation selection, changed-path classification,
  branch/worktree safety, publication readiness, and card lifecycle checks.
- **Contract modules**: durable data shapes, schema/serde adapters, and
  conversion helpers with focused round-trip tests.
- **Fixture helper modules**: reusable test setup and assertion helpers that
  reduce duplication without hiding behavior.
- **Boundary tests**: characterization tests that prove the extracted unit
  preserves the old behavior and can be run without the full workflow.
- **Test-surface extraction**: move large inline `#[cfg(test)]` modules or
  oversized scenario matrices into focused test modules/files only when the
  moved tests retain characterization coverage and enable a narrower command
  for the next expected change.

Rejected slice shapes:

- `foo_parts.rs`, `foo_parts2.rs`, or similar generic buckets
- extraction based only on line count
- moving tests away from the behavior they characterize
- broad reformatting mixed with behavior-preserving moves
- workspace splits without a concrete validation-speed hypothesis
- claiming production binary shrinkage from moving `#[cfg(test)]` tests
- moving tiny local invariant tests out of a source module when doing so would
  make the invariant harder to understand

## Candidate Work Packages

| Work package | Primary target | Boundary | Local proof goal |
| --- | --- | --- | --- |
| WP-R1 | `adl/src/cli/pr_cmd/github.rs` | GitHub operation families and transport helpers | Focused mock-octocrab tests for the touched operation family without exercising every PR command path; large inline transport tests should move beside the operation family they characterize, while tiny helper tests may remain inline. |
| WP-R2 | `adl/src/cli/pr_cmd/finish_support.rs` | Finish validation policy and publication prep | Unit tests for changed-path/validation selection that can run without full finish publication; policy and argument tests should live in focused test modules instead of bloating the production support module. |
| WP-R3 | `adl/src/csdlc_prompt_editor.rs` | Prompt editor import/render/value/schema responsibilities | Prompt-template/editor tests grouped by responsibility rather than one broad editor surface; fixture-heavy editor tests should be separated by concern. |
| WP-R4 | `adl/src/cli/tests/pr_cmd_inline/basics.rs` and `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs` | PR command inline fixture families | Smaller setup/assertion helpers and argument-rendering characterization tests that can run without scanning one broad fixture matrix. |
| WP-R5 | `adl/src/cli/tests/pr_cmd_inline/lifecycle/start_ready.rs` | Start/run readiness fixtures and helpers | Smaller lifecycle fixture tests with clear setup helpers and no hidden branch/worktree side effects. |
| WP-R6 | `adl/src/cli/run_artifacts_types.rs` | Run-artifact contract families | Serde and schema characterization per artifact family. |
| WP-R7 | `REVIEW` tier triage | Provider/runtime/test-heavy modules over 1000 LoC | Route only the modules whose next expected changes currently require unrelated validation. |
| WP-R8 | `WATCH` tier guardrail | Modules over 800 LoC | Add planning guardrails and avoid new responsibilities unless a follow-on issue owns the boundary. |

## Execution-Ready Backlog

The mini-sprint should be opened as a bounded docs-and-code sprint at the
operator-directed point in the current roadmap sequence. The sprint umbrella
should be created from this backlog, with child issues rendered through the
active prompt templates rather than copied from this table.

| Order | Issue candidate | Required output | Acceptance signal |
| --- | --- | --- | --- |
| 1 | Rust refactoring mini-sprint setup and routing | Sprint umbrella plus child issue cards for the selected first wave. | Each child names one responsibility boundary, source LoC evidence, local characterization proof, and focused validation command. |
| 2 | Split `github.rs` by PR/issue/check/review operation families | Operation-family modules with shared transport helpers kept explicit. | A change to one GitHub operation family can run a focused mock/live-boundary test without reading or exercising the whole PR command surface. |
| 3 | Split `finish_support.rs` validation and publication policy | Finish validation planner, changed-path policy, card/SOR checks, and publication-prep boundaries. | Docs-only and tooling-only finish changes have a narrower validation path than full publication rehearsal. |
| 4 | Continue `csdlc_prompt_editor.rs` responsibility extraction | Editor import, values, render, schema, and structure-validation responsibilities separated by owned tests. | Prompt-template/editor changes can validate the touched responsibility without broad editor fixture coupling. |
| 5 | Split PR command inline fixture hotspots | `basics.rs`, `finish/arg_render.rs`, and `lifecycle/start_ready.rs` fixture setup and assertions organized by behavior and command shape. | A PR command behavior change can point to a smaller fixture helper and test subset rather than a broad inline fixture megafile. |
| 6 | Split `run_artifacts_types.rs` by artifact contract family | Stable artifact contract modules with serde/schema characterization nearby. | Artifact-family changes run focused round-trip/schema tests before broader owner validation. |
| 7 | Review-tier routing pass | Updated route table for `REVIEW` modules, including provider/runtime/test-heavy families. | Remaining large modules are explicitly deferred, routed, or converted into follow-on issue candidates with no silent backlog loss. |

The first executable wave should normally include candidates 1 through 3.
Candidate 5 may move earlier if the immediate bottleneck is PR command test
fixture coupling rather than production code ownership. Candidates 4 through 7
can be included only if the sprint capacity remains truthful and each issue
keeps its own focused proof surface.

## Ready-To-Start Gate

Before the first implementation child starts, the sprint should have:

- a current regenerated `rust_module_watch_list.md` snapshot recorded as source
  evidence
- one umbrella issue and child issues for the selected wave
- card-rendered `SIP`, `STP`, and `SPP` surfaces for every child issue
- a baseline validation-burden note for each target, naming the current broad
  command and the intended narrower command
- a test-placement note for each target, naming which tests stay inline, which
  fixture-heavy tests move out, and why that improves review or validation
  locality
- a no-behavior-change invariant unless a child issue explicitly includes
  characterization proof and behavior acceptance
- a review route for any extraction that touches GitHub, card, publication, or
  validation behavior

## Review-Tier Routing

The first execution issue should build a short routing table from the full
watch list rather than turning every large module into immediate scope.

Recommended families:

| Family | Examples from current tracker | Routing posture |
| --- | --- | --- |
| Control-plane PR lifecycle | `github.rs`, `finish_support.rs`, `pr_cmd.rs`, `cards.rs`, `lifecycle/tests.rs`, `pr_cmd_inline/basics.rs`, `pr_cmd_inline/finish/arg_render.rs` | Highest priority because these files directly affect issue/PR cycle time and validation breadth. |
| Prompt/template/editor | `csdlc_prompt_editor.rs`, `tooling_cmd/structured_prompt.rs`, prompt-template tests | High priority where extraction reduces card/template validation blast radius. |
| Run artifacts and trace contracts | `run_artifacts_types.rs`, runtime trace envelope/validation modules | Refactor by data-contract family with serde/schema proof. |
| Provider and communication | `provider_communication.rs`, `provider_adapter.rs`, `provider/http_family.rs`, HTTP tests | Defer unless the provider/model reliability sprint needs narrower tests. |
| Runtime v2 feature packets | `contract_market_demo.rs`, kindness/humor/moral/private-state modules | Generally defer until Runtime v2 feature work needs a local slice. |
| Test fixture megafiles | `start_ready.rs`, `basics.rs`, `arg_render.rs`, demo and run-flow tests | Split helpers and fixtures only when it reduces setup duplication, shortens the fixture surface for a specific command family, and clarifies expected behavior. |

## Validation And Evidence Expectations

Every refactoring implementation issue spawned from this plan should state:

- the current large-module evidence used
- the named responsibility boundary being extracted
- the test-placement decision: inline invariant tests retained, fixture-heavy
  suites moved, or no test movement needed
- the behavior-preserving characterization test before or with the move
- the smaller validation command expected after the extraction
- what broad validation remains CI/release-only

The sprint-level closeout should compare before/after validation burden for at
least the touched surface. A useful result could be a narrower focused test
command, fewer fixture dependencies, or a clearer owner-binary validation lane.
Line-count reduction alone is not sufficient.

## Non-goals

- rewriting every script regardless of value
- destabilizing MVP delivery for migration purity alone
- treating migration-only work as a user-visible feature demo obligation
- generic file splitting without a named responsibility boundary
- mechanical separation of all inline tests regardless of locality value
- production-binary-size claims for moving Rust `#[cfg(test)]` code
- claiming validation-speed improvement without measured or reviewable proof
- refactoring provider/runtime feature code merely because it is large when the
  next planned change is elsewhere

## Completion Target

`v0.95`
