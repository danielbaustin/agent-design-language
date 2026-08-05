# Structured Intent Prompt

Template: 1.0.0

Issue: 5858

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Coordinate current documentation, repository migration, CI repair, build acceleration, workflow efficiency, remote validation, and prompt typing without overlapping child ownership.

## Required Outcome

A reviewable sprint coordination lane can route #5818, #5819, #5812, #5801, #5853, #5822, #5823, #5824 through their own typed lifecycles without scope collision or false completion.

## Scope

- .csdlc/issues/5858
- .csdlc/prepared/issues/5858
- .csdlc/evidence/5858

## Authority

- Sprint coordination records only; child issues own implementation and evidence.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Never write tracked changes on main
- Never use /private/tmp
- Use repo-native GitHub tools only
- The umbrella coordinates child sessions and never implements child code
- Every child session reads AGENTS.md, binds its own worktree, and creates its own goal before implementation
