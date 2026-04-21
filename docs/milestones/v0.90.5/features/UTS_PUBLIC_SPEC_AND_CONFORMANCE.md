# UTS Public Spec And Conformance

## Purpose

Universal Tool Schema v1.0 is the portable, model-facing description layer for
tools. It should be JSON-compatible and useful outside ADL, but it must not
pretend to grant runtime authority.

## Required Contract

UTS v1.0 must define:

- tool name and version
- human-readable description
- input and output JSON Schema fragments
- side-effect class
- determinism and replay safety
- idempotence
- required resources
- authentication requirements
- data sensitivity
- exfiltration risk
- execution environment hints
- error model
- extension rules

## Conformance Requirements

Conformance must include:

- valid fixtures for safe read, local write, external read, external write, and
  destructive categories
- invalid fixtures for missing semantics, missing security, malformed schema,
  invalid enum values, unsafe extension shape, and ambiguous side effects
- explicit rule that UTS validity is not permission to execute
- compatibility notes for existing JSON tool-call systems
- versioning and extension guidance

## Non-Goals

- UTS does not define ADL actor authority.
- UTS does not define Freedom Gate decisions.
- UTS does not define citizen standing or operator inspection rights.

