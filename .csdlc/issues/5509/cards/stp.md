# Structured Task Prompt

Template: 1.0.0

Issue: 5509

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add one bounded mixed Runtime v3 and CSM validation route without weakening other fail-closed classifications.

## Deliverables

- Mixed Runtime v3 and CSM source classifier
- Independent focused test execution for both crates
- Composed focused coverage execution
- Regression contracts preserving unrelated mixed-crate fail-closed behavior

## Acceptance

1. AC1: The #5504 source shape selects bounded PR-fast validation
2. AC2: Focused tests execute in both owning crates
3. AC3: Focused coverage composes both crate summaries
4. AC4: Runtime v2 tests are not selected
5. AC5: Unrelated or unmapped mixed-crate changes still fail closed

## Dependencies

- Existing PR-fast nextest runner
- Existing PR-fast coverage summary composition
- Existing validation manager and path policy

## Inputs

- PR #5504 CI run 29642366877
- Issue #5509
- Issue #5494

## Non Goals

- Runtime v2 source changes
- AWS execution
- General mixed-crate validation relaxation
