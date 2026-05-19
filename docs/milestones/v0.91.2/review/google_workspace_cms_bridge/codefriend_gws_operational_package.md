# CodeFriend GWS CMS Operational Package

## Purpose

Turn the bounded `v0.91.2` Google Workspace CMS bridge into a reusable
CodeFriend/ADL project surface instead of a milestone-only proof packet.

This package is the adoption layer on top of the landed bridge work from:

- `WP-08` / `#3007`
- `WP-09` / `#3008`
- `#3091`
- `#3092`
- `#3093`

It tells future projects:

- how to set the bridge up safely
- which auth/scope defaults are acceptable
- which workflows Workspace may support
- which operations must still go through GitHub issue/PR controls
- which proof surfaces to preserve for reviewability

## Package Contents

- `gws_project_setup_and_onboarding.md`
- `gws_safe_defaults_and_scope_checklist.md`
- `gws_project_workflow_template.md`
- `codefriend_gws_git_workspace_boundary_runbook.md`
- `gws_reusable_proof_packet_template.md`

## What This Package Enables

A future CodeFriend/ADL project can:

- inventory one bounded Drive folder
- read one bounded doc snapshot
- read one bounded content-card sheet range
- prepare promotion-packet context from Workspace state
- run bounded preview/apply contract flows for content-card management
- produce reusable review artifacts without rediscovering the safety rules

## Required Preconditions

Before using the package on a project:

1. the project has one bounded Workspace folder, doc, and sheet scope
2. the operator can authenticate `gws` explicitly
3. the project accepts that GitHub remains canonical repo truth
4. the project can tolerate truthful skipped live behavior when auth, scope,
   or tooling is unavailable

## Operational Guarantees

This package preserves the already-landed bridge guarantees:

- no Workspace surface becomes canonical repo truth
- no silent tracked repo mutation from Workspace state
- no ambient broad Workspace authority
- no hidden live-write posture
- no broad enterprise Workspace administration claims

## Non-Claims

This package does not claim:

- autonomous publication from Workspace
- bidirectional Git/Workspace sync
- live writes are always available
- any project may widen scopes without issue/PR review
- Google Workspace is required for normal ADL issue work

## Adoption Rule

Projects should adopt this package only when they actually benefit from
Workspace-backed draft/content-card management. If a project can stay entirely
inside GitHub and tracked docs, that remains the lower-risk default.
