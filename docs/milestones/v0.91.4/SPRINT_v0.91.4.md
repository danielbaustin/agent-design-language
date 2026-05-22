# v0.91.4 Sprint Plan

## Status

Planned sprint map. Sprint issue numbers are `pending`.

## Sprint Overview

| Sprint | Title | Ordered Children | Goal |
| --- | --- | --- | --- |
| Sprint 1 | Lifecycle And Routing Hardening | WP-01, WP-02, WP-03, WP-04 | Make validators, doctor, conductor, and editor skills agree on C-SDLC state. |
| Sprint 2 | Transition Operation | WP-05, WP-06, WP-07, WP-08 | Make actor standing, shards, evidence, merge gates, and memory handoff repeatable. |
| Sprint 3 | Sprint Default And Metrics | WP-09, WP-10, WP-11, WP-12 | Make sprint execution default-safe and measure repeatability. |
| Sprint 4 | Review, Remediation, Planning, And Release | WP-13, WP-14, WP-15, WP-16, WP-17, WP-18, WP-19, WP-20, WP-21 | Prove, gate, review, remediate, plan the next milestone, re-review the handoff, and close the completion milestone. |

## Sidecar Mini-Sprint

The CodeFriend pre-alpha repo/S3 welcome-page setup runs as a bounded sidecar
mini-sprint in v0.91.4:

| Sidecar | Title | Ordered Children | Goal |
| --- | --- | --- | --- |
| CodeFriend Pre-Alpha Setup | CodeFriend pre-alpha site setup | CF-PRE-01, CF-PRE-02, CF-PRE-03, CF-PRE-04 | Establish the private CodeFriend repo and a verified S3/CloudFront/HTTPS welcome page without making CodeFriend part of C-SDLC core proof. |

The sidecar may run after WP-01 has opened the v0.91.4 issue wave. It must not
interrupt the required C-SDLC closeout tail or add extra release-tail gates.

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
- proof coverage, quality gate, docs/adoption review, internal review,
  external review, remediation, next-milestone planning, next-milestone review
  pass, and release ceremony remain separate ordered tail issues
- sidecar product work remains separate from C-SDLC default-operation proof

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

The CodeFriend sidecar is complete only when it has either a verified HTTPS
welcome page and handoff record or a truthful blocked handoff with AWS/DNS
approval blockers recorded.
