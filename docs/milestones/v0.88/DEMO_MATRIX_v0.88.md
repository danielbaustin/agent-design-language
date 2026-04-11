# Demo Matrix - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Date: `2026-04-11`
- Owner: `Daniel Austin`

## Current State

`v0.88` is still in planning / issue-seeding state, so this matrix records the concrete proof rows we expect to implement.

Each row must eventually resolve to:
- one obvious runner or command
- one obvious artifact directory or manifest
- one crisp reviewer claim the demo proves

## Planned Proof Rows

| Demo ID | Focus | Concrete output expectation | Planned artifact | Status |
|---|---|---|---|---|
| D1 | temporal schema and anchors | schema-aware trace output plus at least one fixture-backed assertion over anchors / clocks / execution posture | schema review artifact | planned |
| D2 | continuity and identity | explicit continuity / interruption / resumption artifact with reviewer-legible state transitions | continuity proof artifact | planned |
| D3 | temporal retrieval and commitments | queryable retrieval result plus commitment/deadline artifact showing open and resolved states | query / commitment artifact | planned |
| D4 | execution policy and cost | run output that shows requested posture, realized cost, and their relationship in one review surface | cost / policy artifact | planned |
| D5 | PHI-style integration metrics | bounded comparison across low / medium / high integration profiles with stable reported dimensions | PHI review artifact | planned |
| D6 | instinct declaration and influence | explicit instinct configuration plus trace/artifact evidence that the setting materially influenced behavior | instinct runtime artifact | planned |
| D7 | bounded agency under instinct | one deterministic proof case where instinct changes routing or prioritization without violating policy bounds | bounded-agency demo artifact | planned |
| D8 | Paper Sonata flagship demo | bounded manuscript runner with stable artifact tree, intermediate role outputs, final draft package, and truthful runtime bundle | paper-sonata manifest, manuscript package, and trace bundle | planned |

## Reviewer Rule

No row is complete until a reviewer can answer all three:
- what command do I run?
- what artifact do I inspect?
- what claim does this row prove?
