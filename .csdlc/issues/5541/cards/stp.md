# Structured Task Prompt

Template: 1.0.0

Issue: 5541

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Fix only the two review findings and focused guardrail needed for #5306 closeout.

## Deliverables

- Corrected init skill
- Typed-v2 default workflow document
- Gate 10A stale-guidance regression

## Acceptance

1. AC-1: Retained typed skills contain no v1-default instruction
2. AC-2: Current default workflow uses only installed typed v2 binaries and skills
3. AC-3: Gate 10A rejects stale operational v1 guidance without scanning historical evidence
4. AC-4: Exact-revision review dispositions both #5306 P1 findings

## Dependencies

- #5306
- PR #5331

## Inputs

- csdlc-v2/operator/generation-selector.json
- csdlc-v2/operator/coexistence.json
- AGENTS.md
- PR #5331
- issue #5306

## Non Goals

- No historical-record rewriting
- No runtime changes
- No lifecycle JSON hand edits
