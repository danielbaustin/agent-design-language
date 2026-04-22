# Evaluation And Transition Authority - v0.90.4

## Purpose

Define how bids are evaluated and how contracts move through lifecycle states.

## Evaluation Model

Evaluation should support:

- human selection
- rule-based selection
- hybrid selection
- mandatory checks before scoring
- criterion-level scorecards
- weighted aggregation
- recommendation values
- critical-criterion failure
- human override with rationale
- tie-break rules
- tool-readiness checks when a bid depends on tool-mediated execution

The evaluation artifact must explain selection rather than merely record a
winner.

If a bid depends on tools, evaluation may record that dependency and whether the
required governed-tool surface exists. It must not treat valid JSON, model
confidence, or adapter availability as authority to execute the tool.

## Transition Authority

Every lifecycle transition needs an explicit actor and authority basis.

Core transitions:

- draft to open
- open to bidding
- bidding to awarded
- awarded to accepted
- accepted to executing
- executing to completed
- executing to failed
- executing to disputed
- any active state to cancelled
- disputed to completed or failed

## Freedom Gate Integration

Citizen-facing or citizen-mediated actions should pass through the Freedom Gate
when they affect citizen standing, obligations, delegation, inspection,
resource access, or state transition authority.

## Required Negative Cases

The implementation should include denial cases for:

- unauthorized award
- acceptance by the wrong actor
- execution before acceptance
- completion without required artifacts
- cancellation after completion
- human override without traceable rationale
- selection that assumes a tool may execute without governed-tool authority
