# v0.85 Milestone Issue Reconciliation

## Purpose

This document captures the revised v0.85 issue-planning model inside the canonical milestone doc set.

It exists so the milestone-facing documentation can carry the same issue-reconciliation and work-package intent that was originally developed in local planning notes, without depending on `.adl/` planning files as the only place where that structure exists.

This is now primarily a historical reconciliation record. For the live closeout queue and current milestone status, use `WBS_v0.85.md`, `SPRINT_v0.85.md`, and the GitHub issue tracker.

## Why This Exists

The earlier v0.85 issue set was directionally coherent but too permissive in several places:

- too many issues could be completed with docs alone
- demos were under-specified
- editor work did not guarantee a real editor
- Gödel issues existed but were not cleanly integrated into the milestone structure
- queueing/steering were split between placeholder and canonical issues
- endgame review/release work was too compressed

The revised milestone keeps the architectural ambition, but forces more concrete outputs and cleaner issue ownership.

## Canonical Tracker Rules

- `#886` is the umbrella milestone-reorganization issue until the issue graph is fully aligned.
- `#674` is the canonical queue/checkpoint/steering issue.
- `#867` is duplicate/placeholder queueing material to absorb, supersede, or close in favor of `#674`.
- Gödel issues `#748` through `#752` are first-class milestone work packages.
- The provisional generated issue set `#866` through `#882` is scaffolding to absorb, remap, split, merge, or close as needed.
- Every work package should map to one canonical issue.
- Every canonical issue should belong to one work package.

## Four-Sprint Structure

### Sprint 1 - Execution Substrate and Milestone Reorganization

Primary goal:
- stabilize milestone structure and execution substrate so later work lands on a strong base

Primary work:
- milestone reorganization under WP-01
- deterministic queueing, checkpointing, and steering
- cluster/distributed execution groundwork
- process updates required by the reorganization

Expected outputs:
- revised milestone docs
- issue graph aligned with the milestone
- queue/steering source of truth aligned around issue `#674`
- concrete runtime or artifact progress on queue/checkpoint/steering

### Sprint 2 - Authoring Surfaces and Review Tooling

Primary goal:
- make authoring and review materially easier with real tools

Primary work:
- Prompt Spec completeness updates that directly help authoring
- first working HTML editors for issue prompts and input cards
- working Card Editor, Issue Editor, and Card Reviewer GPTs or equivalent prompt assets/tooling
- workflow integration between editors and review surfaces

Expected outputs:
- real editor surfaces
- sharper Prompt Spec and card authoring flow
- more stable review outputs
- reusable editor/reviewer assets

### Sprint 3 - Gödel, Affect, Reasoning Graphs, and AEE Progress

Primary goal:
- deliver the milestone's major cognitive and runtime leap

Primary work:
- first hypothesis-engine work through issues `#748` through `#752`
- affect model / affective reasoning surfaces
- reasoning-graph schema and hypothesis-lineage surfaces
- stronger AEE/runtime progress
- verifiable inference and dependable execution tied to concrete artifacts

Expected outputs:
- first deterministic hypothesis-engine behavior
- reasoning-graph artifacts and schemas
- affective reasoning traces and specs backed by real implementation surfaces
- visible AEE/runtime progress with emitted artifacts, tests, or demos

### Sprint 4 - Demos, Quality Gate, Review, Release, and Next-Milestone Planning

Primary goal:
- prove the milestone, complete review and release work, and plan the next milestone cleanly

Primary work:
- multiple demos, including strong Gödel/affect/reasoning demos
- coverage/quality gate
- docs consistency pass
- internal review
- external review
- review-findings remediation
- release ceremony
- next-milestone planning as a required closing step

Expected outputs:
- multiple runnable demos
- release evidence
- aligned documentation
- explicit next-milestone planning materials before closure

## Revised Work Package Intent

The canonical twenty-five-work-package structure is defined in [WBS_v0.85.md](WBS_v0.85.md). The most important intent clarifications are:

- WP-01 is the milestone reorganization and issue-graph alignment pass under `#886`.
- WP-02 is the deterministic queue/checkpoint/steering substrate under `#674`.
- WP-05 must produce real editor surfaces.
- WP-10 through WP-14 are the Gödel issue set `#748` through `#752`.
- WP-15 through WP-17 must deliver a minimal working affect engine, reasoning-graph integration, and a runnable affect-plus-Gödel vertical slice.
- WP-18 is the demo program and should explicitly inherit the bounded-demo rule from `#743`.
- WP-20 through WP-25 separate docs consistency, internal review, external review, remediation, release, and next-milestone planning into distinct closeout stages.

## Current Reconciliation Map

This table is the current working rewrite plan for the live GitHub issue list. It is intended to drive issue-title/body/label cleanup under `#886` without forcing the tracker to remain in the earlier provisional seventeen-WP shape.

| WP | Intended package | Canonical issue | Current state | Reconciliation action |
|---|---|---|---|---|
| WP-01 | Milestone reorganization and docs alignment | `#886` | Open | Keep as umbrella issue; continue using it to drive tracker and doc reconciliation. |
| WP-02 | Deterministic queue, checkpoint, and steering substrate | `#674` | Open | Keep as canonical; `#867` stays closed as duplicate/placeholder material. |
| WP-03 | Cluster / distributed execution groundwork | `#868` | Closed | Keep as canonical completed WP issue; treat the work as landed rather than reopening placeholder `#868`-replacement scaffolding. |
| WP-04 | Prompt Spec completeness for editors | `#716` | Open/active canonical issue | Keep as canonical; `#869` stays closed as duplicate placeholder material. |
| WP-05 | First authoring/editor surfaces | `#870` | Open | Keep, but retitle/edit body so it guarantees real editor surfaces rather than editor direction only. |
| WP-06 | Editing and review GPT/tooling surfaces | `#871` | Open | Keep, but retitle/edit body so it covers reusable editing/review tooling rather than reviewer-only stabilization. |
| WP-07 | Dependable execution runtime surfaces | `#872` | Open | Keep, tighten title/body around concrete runtime and artifact outputs. |
| WP-08 | Verifiable inference runtime surfaces | `#873` | Open | Keep, tighten title/body around evidence-linked artifacts, provenance, and replay/report surfaces. |
| WP-09 | Adaptive Execution Engine bounded progress | `#874` | Open | Keep, retitle/edit body around bounded AEE progress and artifact-bearing runtime behavior. |
| WP-10 | Deterministic hypothesis generation engine | `#748` | Open | Keep as-is; absorb into canonical WP numbering and sprint structure. |
| WP-11 | Policy-learning and adaptive Gödel loop | `#749` | Open | Keep as-is; absorb into canonical WP numbering and sprint structure. |
| WP-12 | Experiment prioritization and strategy confidence | `#750` | Open | Keep as-is; absorb into canonical WP numbering and sprint structure. |
| WP-13 | Cross-workflow learning and recursive improvement | `#751` | Open | Keep as-is; absorb into canonical WP numbering and sprint structure. |
| WP-14 | Promotion and eval-report artifact loop | `#752` | Open | Keep as-is; absorb into canonical WP numbering and sprint structure. |
| WP-15 | Affect engine core | `#875` | Open | Keep, but retitle/edit body so it requires a minimal working affect engine, not design-only output. |
| WP-16 | Reasoning graph and affect integration | `#876` | Open | Keep, but retitle/edit body so it requires affect-integrated reasoning-graph surfaces rather than schema direction only. |
| WP-17 | Affect-plus-Gödel vertical slice | `#877` | Open | Keep, but retitle/edit body so it requires a runnable bounded demo slice rather than planning linkage only. |
| WP-18 | Demo program for v0.85 features | `#878` | Open | Keep, retitle/edit body so it requires runnable milestone demos and explicitly inherits the bounded-demo rule from `#743`. |
| WP-19 | Coverage / quality gate | `#879` | Open | Keep and relabel/rebody to match the canonical quality-gate scope. |
| WP-20 | Documentation consistency pass | `#880` | Open | Keep, retitle/edit from “docs + review pass” to docs consistency and issue/body alignment. |
| WP-21 | Internal review | new issue | Missing | Create as a distinct review gate under the `#886` umbrella. |
| WP-22 | External review | new issue | Missing | Create as a distinct external review gate under the `#886` umbrella. |
| WP-23 | Review findings remediation | new issue | Missing | Create as a distinct remediation issue under the `#886` umbrella. |
| WP-24 | Release ceremony | `#881` | Open | Retitle/remap existing release-ceremony issue from old WP-16 numbering to canonical WP-24. |
| WP-25 | Next milestone planning | `#882` | Open | Retitle/remap existing closeout issue from old WP-17 numbering to canonical WP-25. |

## Supporting Issue Notes

- `#743` is not itself a top-level work package in the twenty-five-WP structure. It should remain the canonical bounded-demo rule that WP-18 explicitly inherits and references.
- `#898` and `#899` are authoring/tooling follow-on issues that support WP-04 and the structured-prompt workflow. They should be treated as follow-on implementation issues, not as extra top-level work packages.
- Closed migration issues such as `#887`, `#889`, and `#891` are milestone-support work already completed and should not be treated as active replacement work packages.
- Older umbrella/epic issues such as `#559` remain important historical context, but they should not compete with the canonical WP-to-issue mapping for v0.85.

## Rewrite Sequence

Recommended order for tracker cleanup under `#886`:

1. Keep `#886`, `#674`, `#716`, and `#748` through `#752` as canonical anchors.
2. Retitle/rebody `#870` through `#880`, `#881`, and `#882` to match the canonical WP-05 through WP-25 structure.
3. Create the three missing closeout issues for WP-21 through WP-23.
4. Ensure WP-18 explicitly references `#743` as the bounded-demo rule.
5. Keep `#898` and `#899` as follow-on authoring/tooling issues under the relevant work packages rather than forcing them into the top-level WP numbering.

## Prompt Bootstrap Sequence

The intended structured-prompt workflow for a milestone now follows this order:

1. milestone planning
2. work-package reconciliation and canonical WP list creation
3. State-A Structured Task Prompt (STP) stub creation for each work package
4. editor pass to promote selected STPs from State A into authored State-B prompts
5. GitHub issue creation or reconciliation from the authored STPs
6. `pr start` generation of Structured Implementation Prompts (SIPs)
7. implementation, validation, and Structured Output Record (SOR) capture

This sequence matters because it makes the milestone editor-ready before implementation starts. The milestone should not depend on ad hoc issue-body writing or guessed work-package structure once the canonical WP list is established.

Practical rule:

- once milestone planning and WP reconciliation are complete, batch State-A STP stub creation is expected and should be treated as normal milestone setup work
- State-B authored STPs can then be produced incrementally by the editor agent in execution order or dependency order
- issue creation/rewrite should follow the authored STPs rather than preceding them by guesswork unless early bootstrap numbering is explicitly needed

## Required Demo Proof Surfaces

The milestone should explicitly prove:

- steering/queueing/checkpoint behavior
- HITL/editor/review workflow behavior
- Gödel hypothesis-engine behavior
- affect-engine behavior
- affect-plus-Gödel/reasoning behavior

These are milestone evidence, not optional extras.

## Sanity-Check Rule

After issue reconciliation is complete, the milestone should read conceptually like this:

- first, milestone alignment work
- then runtime foundation work such as queueing, steering, distributed execution, and execution reliability
- then authoring and tooling work such as prompt-spec improvements, editors, and review tooling
- then the cognitive layer including the Gödel hypothesis engine, affect engine, and reasoning graphs
- then demos that prove the system works
- finally quality gates, review, release, and next-milestone planning

If the docs and issue tracker reflect that structure, the milestone is correctly organized.
