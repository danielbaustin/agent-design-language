# Structured Intent Prompt

Template: 1.0.0

Issue: 5789

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the HTML Observatory work as a real Runtime v3 observatory and operator communication surface.

## Required Outcome

Default and explicit Observatory routes render live Runtime v3 truth, all controls have tested behavior, WebSocket/read fallback is truthful, and operator-to-agent communication works end to end or fails closed with precise diagnostics.

## Scope

- HTML Observatory UI and configuration
- Runtime v3 observatory read and stream behavior
- Operator-to-agent write/control channel
- Focused browser and CLI validation
- Issue-local lifecycle and evidence

## Authority

- GitHub issue #5789
- Live local Runtime v3 endpoints on https://localhost:20997
- Checked-in HTML Observatory route under demos/html-observatory
- Typed C-SDLC v2 lifecycle

## Assumptions

- none

## Operator Constraints

- Do not use AWS.
- Do not write tracked changes on main.
- Use /Volumes/FastWork for worktree and build output.
- All Observatory features need to work.
- The Observatory must allow the operator to communicate with the agents.
- Do not convert the Observatory into a mock/demo-only surface.
