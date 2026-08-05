# Structured Intent Prompt

Template: 1.0.0

Issue: 5857

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Coordinate the identity, continuity, memory, capability, learning, witness, and review contracts for the first true Godel-agent birthday.

## Required Outcome

A reviewable sprint coordination lane can route #5825, #5826, #5827, #5828, #5829, #5830, #5831, #5833, #5834 through their own typed lifecycles without scope collision or false completion.

## Scope

- .csdlc/issues/5857
- .csdlc/prepared/issues/5857
- .csdlc/evidence/5857

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
