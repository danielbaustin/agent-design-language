# Structured Intent Prompt

Template: 1.0.0

Issue: 5715

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Wire the current Agent Logic podcast page to the operator-provided studio page design without editing the exported HTML content and without regressing audio or RSS.

## Required Outcome

The generated podcast landing page links to the studio route, the studio route serves the exact exported HTML text/images under a clean filename, and the existing audio/RSS launch behavior remains validated.

## Scope

- podcast launch generator and generated demo surfaces
- copied podcast studio reference assets
- podcast RSS and audio validation
- issue-local C-SDLC lifecycle evidence

## Authority

- GitHub issue #5715
- operator-provided design zip as immutable source evidence
- #5711 audio/RSS launch foundation

## Assumptions

- none

## Operator Constraints

- Use the exported HTML text and images exactly.
- Make only the changes needed to wire the export into the route.
- Use a clean filename without .dc and without spaces.
- Do not write tracked changes on main.
- Use FastWork for issue work.
- Preserve audio and RSS; do not replace them with a visual mock.
- Do not claim production deployment without live deployment proof.
