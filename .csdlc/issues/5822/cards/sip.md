# Structured Intent Prompt

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete WP-05: C-SDLC estimation and cycle-time reduction.

## Required Outcome

measured estimation, reconnection, and simplified lifecycle path

## Scope

- csdlc-v2 typed observation, forecast, outcome, schema, card-reference, and closeout comparison surfaces
- csdlc-v2/src/cards.rs static PlanningProfile estimates retained as explicit fallback
- Historical session telemetry and deterministic predictor scripts under adl/tools/skills/sprint-conductor/scripts as migration inputs only
- Issue-local joined fixture corpus, backtests, cycle-time baseline, candidate comparison, and operator workflow proof
- .csdlc/issues/5822, .csdlc/prepared/issues/5822, and .csdlc/evidence/5822

## Authority

- Independent csdlc-v2 binaries and schemas own current estimation integration
- Historical v1 sprint-conductor scripts are source evidence only and are not restored as lifecycle commands
- Estimates are advisory and cannot stop work or change lifecycle truth
- WP-02A owns the stable CI substrate and WP-07 consumes the resulting typed-card boundary

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
