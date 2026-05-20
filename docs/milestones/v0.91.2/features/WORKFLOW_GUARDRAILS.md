# Workflow Guardrails

## Metadata

- Feature Name: Workflow Guardrails Hardening
- Milestone Target: `v0.91.2`
- Status: in_flight
- Planned WP Home: WP-16
- Source Docs: `.adl/docs/TBD/workflow_tooling/`
- Proof Modes: scripts, fixtures, runbooks, review packet

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

## Delivered Surface

- `adl/tools/workflow_guardrails.sh`
- `adl/tools/test_workflow_guardrails.sh`
- `docs/milestones/v0.91.2/review/workflow_guardrails/`

## Validation

- `bash adl/tools/test_workflow_guardrails.sh`

## Operator Notes

- `main-write` blocks dirty tracked execution from `main`/`master`.
- `closeout-watch` surfaces closed/completed issues that still need local
  closeout.
- `safe-report-command` blocks shell command strings that rely on backticks or
  `$(...)` substitution when generating Markdown reports.
- `card-drift` routes issue drift checks through `pr doctor` instead of ad hoc
  local guesses.
