# v0.91.4 Sprint Plan

## Status

Planned sprint map. Sprint issue numbers are `pending`.

## Sprint Overview

| Sprint | Title | Ordered Children | Goal |
| --- | --- | --- | --- |
| Sprint 1 | Lifecycle And Routing Hardening | WP-01, WP-02, WP-03, WP-04 | Make validators, doctor, conductor, and editor skills agree on C-SDLC state. |
| Sprint 2 | Transition Operation | WP-05, WP-06, WP-07, WP-08 | Make actor standing, shards, evidence, merge gates, and memory handoff repeatable. |
| Sprint 3 | Sprint Default And Metrics | WP-09, WP-10, WP-11, WP-12 | Make sprint execution default-safe and measure repeatability. |
| Sprint 4 | Review, Adoption, And Release | WP-13, WP-14, WP-15, WP-16 | Review, remediate, document, and close the completion milestone. |

## Sprint Goals

The sprint overview table above is the generator-facing sprint map. The goals
below explain the intended execution posture for each sprint without replacing
that canonical table.

## Execution Policy

Every sprint must preserve:

- conductor routing for every lifecycle stage
- editor-only card edits
- bound worktree execution
- pre-PR review before publication
- closeout after issue closure
- combined-lane validation where integration risk exists

## Closeout Bar

The milestone is not complete merely because the process is documented.

It must show that the C-SDLC lane can run repeatedly with:

- correct card creation
- correct routing
- correct actor standing and shard ownership
- correct review recording
- correct SOR closeout
- correct sprint state
- correct memory handoff boundary
- measured coordination behavior
