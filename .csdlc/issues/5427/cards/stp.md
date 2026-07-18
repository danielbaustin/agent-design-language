# Structured Task Prompt

Template: 1.0.0

Issue: 5427

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement only the typed identity repair and its focused proof; do not change runtime or product behavior.

## Deliverables

- Typed version identity operation
- Focused regression tests
- Validated #5353 repair
- Updated design and review evidence

## Acceptance

1. Valid version updates canonical identity and all six cards atomically
2. Malformed versions are rejected
3. Non-identity content is preserved
4. Round-trip and atomicity tests pass
5. #5353 is repaired through the supported typed route

## Dependencies

- csdlc-v2 semantic edit API
- #5353 retained cards

## Inputs

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests
- .csdlc/issues/5353

## Non Goals

- Manual card editing
- Issue title/label/milestone changes
- Runtime or product changes
