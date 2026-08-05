# Structured Intent Prompt

Template: 1.0.0

Issue: 5795

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Provide a governed Runtime v3 and Observatory path for real local model-backed Shepherd dialogue.

## Required Outcome

A bounded operator message travels through authenticated Runtime v3 control to a configured local MLX/Gemma provider and returns truthful response evidence, with unavailable and unauthorized cases distinguished.

## Scope

- adl-runtime/src/runtime_api.rs
- adl-runtime-kernel Shepherd adapter/control paths
- local MLX/Gemma provider binding
- demos/html-observatory Shepherd controls and status
- .csdlc/issues/5795
- .csdlc/evidence/5795

## Authority

- Issue 5795 owns the local-only Shepherd MVP
- WP-03 owns Runtime launch stability and WP-18A owns final consumer integration
- The v0.95 full Shepherd, training, and evaluator program remains separate
- No cloud or global default model change is authorized

## Assumptions

- none

## Operator Constraints

- Use the governed Runtime command path, not a UI shortcut
- Require a real configured local model for production-path success
- Use deterministic fakes only for adapter regression tests
- Never edit tracked work on main
- Use one bounded pre-PR review
