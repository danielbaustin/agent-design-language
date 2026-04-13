# Test Generation Playbook

Use this file after the main skill triggers and the target context is already concrete.

## Priorities

Prefer this order:
1. understand the changed or target implementation
2. inspect nearby existing tests
3. identify the smallest missing regression or edge-case surface
4. add or update the minimal test surface
5. run the smallest truthful validation command

## High-Value Questions

Ask:
- What exact behavior is missing coverage?
- Is this a regression, failure path, or edge case?
- Does an existing nearby test file already express the right style?
- Can one focused test cover the issue instead of a broad suite expansion?
- Is a fixture or snapshot really needed, or would explicit assertions be clearer?

## Selection Guidance

Prefer:
- extending an existing nearby test file
- one focused new test file only when separation is clearly better
- one targeted command over a whole-repo test sweep

Avoid:
- broad speculative case generation
- large snapshot-only updates with weak assertions
- changing unrelated production code
- adding multiple test frameworks or harness patterns in one issue

## Validation Guidance

When feasible, run:
- a focused Rust test target
- one shell regression test
- one package-local test pattern

If no narrow command exists, say so explicitly instead of pretending the tests were validated.
