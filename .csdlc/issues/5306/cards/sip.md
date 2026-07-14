# Structured Intent Prompt

Template: 1.0.0

Issue: 5306

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Remove only approved obsolete incumbent C-SDLC surfaces in bounded reviewed slices.

## Required Outcome

A measured deletion wave reaches the reviewed target without removing useful code or protected rollback/importer surfaces.

## Scope

- Exact D1 manifest paths approved for deletion
- Retained-surface ownership and LoC/test accounting

## Authority

- No mutation until D1 reports eligible=true for current evidence
- Explicit operator deletion approval is independently required
- Rollback and importer paths are excluded

## Assumptions

- none

## Operator Constraints

- none
