# Multi-Agent Repo Code Review Demo Contract

## Purpose

Define the bounded packet and reviewer-role contract for the `v0.89`
multi-agent repo code review demo.

## Packet Contract

The demo packet must include:

- one packet manifest
- one repo inventory summary
- one explicit selected-path list
- one bounded explanation of what is in and out of scope

The packet should stay:

- deterministic
- reviewable
- repo-relative
- small enough for a human to inspect

## Reviewer Roles

### Code Reviewer

Focus:

- correctness
- design fit
- maintainability
- behavioral regression risk

### Security Reviewer

Focus:

- trust boundaries
- dangerous defaults
- data handling
- abuse paths

### Test Reviewer

Focus:

- regression coverage
- failure-path proof
- brittle or misleading tests
- validation gaps

### Docs Reviewer

Focus:

- README
- operator guidance
- docs-to-code coherence
- reviewer onboarding clarity

### Synthesis Reviewer

Focus:

- deduplication
- severity ordering
- blocking vs lower-priority classification
- final action plan

## Output Expectations

Each specialist reviewer artifact should:

- follow the repo review surface format
- stay findings-first
- state what was and was not reviewed
- avoid secrets, absolute host paths, and raw prompts

The final synthesis artifact must:

- distinguish blocking findings from lower-priority observations
- attribute findings to the specialist review pass
- remain explicit that the demo is not autonomous merge authority
