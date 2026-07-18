# Structured Intent Prompt

Template: 1.0.0

Issue: 5426

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Allow supported terminal closeout when a later validation observation supersedes an earlier state for the same logical validation.

## Required Outcome

Readiness and terminal validation use one shared latest-observation rule while retaining append-only evidence.

## Scope

- C-SDLC v2 terminal validation semantics
- focused readiness and card-validation regression tests
- retained design and diagram

## Authority

- Do not hand-edit lifecycle cards
- Do not weaken required local or remote gates
- Do not touch runtime or product code

## Assumptions

- none

## Operator Constraints

- none
