# Structured Intent Prompt

Template: 1.0.0

Issue: 5855

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Coordinate one resilient Runtime and Observatory path with distributed polis, protocol, provider, and consumer integration.

## Required Outcome

A reviewable sprint coordination lane can route #5800, #5820, #5795, #5821, #5832, #5837 through their own typed lifecycles without scope collision or false completion.

## Scope

- .csdlc/issues/5855
- .csdlc/prepared/issues/5855
- .csdlc/evidence/5855

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
