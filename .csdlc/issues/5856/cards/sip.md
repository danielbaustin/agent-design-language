# Structured Intent Prompt

Template: 1.0.0

Issue: 5856

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Coordinate cleanup, refactoring, quality gates, reviews, remediation, planning, and release ceremony in strict dependency order.

## Required Outcome

A reviewable sprint coordination lane can route #5786, #5841, #5842, #5843, #5846, #5847, #5848, #5849, #5850, #5851, #5852 through their own typed lifecycles without scope collision or false completion.

## Scope

- .csdlc/issues/5856
- .csdlc/prepared/issues/5856
- .csdlc/evidence/5856

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
