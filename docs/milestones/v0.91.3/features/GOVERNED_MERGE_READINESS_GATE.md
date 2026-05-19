# Governed Merge-Readiness Gate

## Status

Planned `v0.91.3` feature.

## Purpose

Define the first C-SDLC merge-readiness gate while preserving ADL's existing
GitHub issue, PR, CI, branch, review, and closeout discipline.

The gate is not an automatic merge button. It is a reviewable decision surface
that says whether a transition is ready to request human merge review.

## Scope

The first slice must define a gate record that captures:

- issue identity and lifecycle state
- branch and worktree identity
- PR identity and publication state
- validation commands and results
- CI/check status when available
- review findings and dispositions
- evidence bundle link
- residual risks and blocked conditions

## Acceptance Criteria

- A transition cannot be considered merge-ready without linked `SRP` and `SOR`
  truth.
- The gate distinguishes local validation from remote CI/check truth.
- The gate records human review requirements instead of pretending to replace
  them.
- The gate fails closed when evidence is missing, stale, or local-only.
- The first proof includes a gate fixture or output record.

## Non-Goals

- This feature does not merge PRs automatically.
- This feature does not bypass protected branch rules.
- This feature does not treat speed metrics as sufficient merge evidence.
