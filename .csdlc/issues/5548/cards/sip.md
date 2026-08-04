# Structured Intent Prompt

Template: 1.0.0

Issue: 5548

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Repair the Gate 2 C-SDLC v2 test fixtures so temporary roots reach their intended assertions.

## Required Outcome

Choose and implement the architecture-correct fix for Gate 2 temporary roots while preserving fail-closed terminal receipt/common-directory invariants for real repositories.

## Scope

- csdlc-v2 Gate 2 test fixture setup
- terminal receipt/common-directory discovery behavior needed by those fixtures
- focused regression proof for non-Git fixture behavior

## Authority

- issue #5548 is the implementation authority
- typed C-SDLC v2 lifecycle state is authoritative
- ADL owner binaries provide live issue truth

## Assumptions

- none

## Operator Constraints

- Typed v2 only
- No implementation during preparation
- No raw gh
- No AWS
- Do not touch occupied issue #5558 worktree
- One review before PR, not during preparation
