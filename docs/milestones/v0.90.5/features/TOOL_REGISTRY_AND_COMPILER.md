# Tool Registry And Compiler

## Purpose

The tool registry and UTS-to-ACC compiler prevent loose model output from
binding directly to execution.

## Registry Requirements

The registry must:

- list known tools
- bind UTS definitions to approved adapters
- reject unknown tools
- reject unregistered tools
- version tool definitions
- record adapter capabilities and dry-run posture

## Compiler Requirements

The compiler must:

- validate UTS
- normalize untrusted arguments
- reject ambiguous or malformed proposals
- map UTS semantics into ACC execution semantics
- inject policy context
- construct visibility and redaction rules
- reject unsatisfiable authority, resource, privacy, or execution constraints
- emit trace evidence for validation, normalization, policy, and rejection

## Determinism

Identical UTS, proposal, registry state, and policy context should produce an
identical ACC or identical rejection.

