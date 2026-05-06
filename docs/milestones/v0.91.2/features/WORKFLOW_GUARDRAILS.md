# Workflow Guardrails

## Metadata

- Feature Name: Workflow Guardrails Hardening
- Milestone Target: `v0.91.2`
- Status: planned
- Planned WP Home: WP-14
- Source Docs: `.adl/docs/TBD/workflow_tooling/`
- Proof Modes: scripts, fixtures, runbooks, review

## Purpose

Harden ADL workflow behavior around the operational failures that slowed recent
milestones: main-branch writes, hung closeout watchers, unsafe Markdown report
generation, stale cards, and ambiguous lifecycle state.

## Scope

In scope:

- Main-write guardrails.
- Closeout watcher cleanup or bounded monitor behavior.
- Safe Markdown/report generation rules.
- Card drift checks and runbooks.

Out of scope:

- Eliminating all operator error.
- Destructive cleanup of user work.
- Silent merge or closeout automation.

## Acceptance Criteria

- Guardrails fail closed without clobbering user changes.
- Unsafe shell/report patterns are prevented or documented with safe
  alternatives.
- Operators can diagnose stuck workflow state quickly.
