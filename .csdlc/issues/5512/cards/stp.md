# Structured Task Prompt

Template: 1.0.0

Issue: 5512

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair the focused coverage filter without changing Runtime v2, production runtime behavior, or general coverage policy.

## Deliverables

- Owning-crate coverage expression split
- Exact failed-expression regression fixture

## Acceptance

1. AC-1: The exact run 29644007246 expression produces a valid ADL-only coverage invocation
2. AC-2: Runtime v3 selectors execute only through adl-runtime/Cargo.toml
3. AC-3: Both crate summaries remain composed
4. AC-4: Generic non-bridge coverage behavior remains unchanged
5. AC-5: No Runtime v2 source changes and no AWS

## Dependencies

- Issue #5509 merged focused route
- Existing PR-fast coverage composition

## Inputs

- GitHub run 29644007246 job 88079048688
- Issue #5512
- Issue #5494

## Non Goals

- Runtime v2 source changes
- Runtime v3 production changes
- AWS execution
- General nextest binary-ID cleanup
