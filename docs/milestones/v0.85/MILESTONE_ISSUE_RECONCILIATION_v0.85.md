# v0.85 Milestone Issue Reconciliation

## Purpose

This document captures the revised v0.85 issue-planning model inside the canonical milestone doc set.

It exists so the milestone-facing documentation can carry the same issue-reconciliation and work-package intent that was originally developed in local planning notes, without depending on `.adl/` planning files as the only place where that structure exists.

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

The canonical twenty-five-work-package structure is defined in [WBS_v0.85.md](/Users/daniel/git/adl-wp-886/docs/milestones/v0.85/WBS_v0.85.md). The most important intent clarifications are:

- WP-01 is the milestone reorganization and issue-graph alignment pass under `#886`.
- WP-02 is the deterministic queue/checkpoint/steering substrate under `#674`.
- WP-05 must produce real editor surfaces.
- WP-10 through WP-14 are the Gödel issue set `#748` through `#752`.
- WP-15 through WP-17 must deliver a minimal working affect engine, reasoning-graph integration, and a runnable affect-plus-Gödel vertical slice.
- WP-18 is the demo program and should explicitly inherit the bounded-demo rule from `#743`.
- WP-20 through WP-25 separate docs consistency, internal review, external review, remediation, release, and next-milestone planning into distinct closeout stages.

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
