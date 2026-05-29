# v0.91.5 Design

## Metadata
- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Status: `draft_pre_open`

## Purpose

Define the planned design surfaces for the v0.91.5 bridge milestone so issue
execution can proceed without reconstructing scope from chat.

## Problem Statement

v0.91.4 landed substantial C-SDLC hardening, PVF work, and proof infrastructure.
Additional work remains before v0.92 can safely open: multi-agent execution,
provider/model breadth, public prompt records, demo readiness, and activation
testing. If this work stays scattered in side issues, v0.92 will inherit
unclear dependencies.

## Goals

- Route all open pre-v0.92 bridge issues into v0.91.5.
- Make multi-agent execution testable and evidence-bound.
- Make provider/model identity and role aptitude visible.
- Make public prompt packet export/redaction/review safe.
- Prepare demo and Unity Observatory readiness.
- Produce a v0.92 activation test map and `#3377` readiness packet.

## Non-Goals

- No first-birthday implementation.
- No unbounded multi-agent autonomy claim.
- No unreviewed deletion of `.adl` data.
- No production CodeFriend or OpenRouter product claim.
- No v0.93 constitutional governance work.

## Scope

Scope is limited to bridge planning, issue routing, implementation/readiness
work for the named bridge issues, review, remediation, v0.92 preflight, and
release closeout.

## Requirements

- Every v0.91.5 issue uses all five prompt cards from the active registry.
- Multi-agent roles must record provider/model identity and shard boundaries.
- Provider/model testing must distinguish hosted, local, remote, and OpenRouter
  substrates.
- Public prompt records must be redaction-safe before publication.
- `.adl` cleanup must be review-before-delete.
- v0.92 activation testing must include all known feature surfaces.

## Proposed Design

The milestone uses four execution sprints:

- Sprint 1: bridge package and public prompt record transition.
- Sprint 2: provider/model matrix and multi-agent stabilization.
- Sprint 3: demo readiness and v0.92 activation preflight.
- Sprint 4: docs, review, remediation, v0.92 final preflight, and release.

## Interfaces And Contracts

- Issue routing: `version:v0.91.5` label and issue titles.
- Planning contract: `docs/templates/planning/current.json`.
- Prompt contract: `docs/templates/prompts/current.json`.
- Work-package contract: [WP_ISSUE_WAVE_v0.91.5.yaml](WP_ISSUE_WAVE_v0.91.5.yaml).
- Activation contract: [V092_ACTIVATION_TEST_MAP_v0.91.5.md](V092_ACTIVATION_TEST_MAP_v0.91.5.md).

## Validation Plan

- Validate planning docs against `docs/templates/planning/current.json`.
- Parse v0.91.5 YAML.
- Check links in changed milestone docs.
- Verify moved GitHub issues carry `version:v0.91.5`.
- Run implementation-specific tests only for issues that change runtime/tooling.

## Exit Criteria

- Design, WBS, sprint, issue wave, demo matrix, checklist, release plan, and
  release notes agree on scope.
- v0.92 docs consume v0.91.5 closeout and `#3377`.
- Multi-agent and activation readiness are not left as untracked chat context.
