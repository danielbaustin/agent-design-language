# Structured Intent Prompt

Template: 1.0.0

Issue: 5778

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Replace the post-merge closeout transaction with derived terminal truth and one retry-safe typed finish command.

## Required Outcome

An exact reviewed and green issue converges through merge and terminal confirmation without tracked post-merge mutation, a terminal receipt projection, or a second PR.

## Scope

- C-SDLC v2 terminal resolution and finish command
- Exact-head review and merge convergence
- Logical terminal claim release
- Legacy terminal read compatibility
- Focused lifecycle tests and operator contracts

## Authority

- Typed C-SDLC v2 Rust binaries only
- GitHub mutation limited to expected-head merge and governed terminal observation
- No raw lifecycle state or card edits
- No cleanup or deletion authority

## Assumptions

- none

## Operator Constraints

- Do not use AWS or Spot
- Do not write tracked files on main
- Use the dedicated #5778 worktree
- Keep implementation within the declared protected paths
- Run a bounded subagent review before publication
