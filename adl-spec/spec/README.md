# ADL Specification

This directory contains the **normative specification** for Agent Design Language (ADL).

The specification defines the meaning of ADL language constructs, the invariants that affect
observable behavior, and the minimum expectations for interoperable runtimes.

ADL is a **design-time language**. Runtime artifacts (such as sessions, traces, retries, and
execution backends) are out of scope for the language definition, except where they affect
observable behavior required for interoperability.

## Versions

- **ADL 1.0 (draft)**: `spec/1.0/`

Each versioned directory is intended to be self-contained and stable once finalized.

## ADL 1.0 documents

The ADL 1.0 draft specification currently consists of the following documents:

- **Core concepts** — [`spec/1.0/core-concepts.md`](1.0/core-concepts.md)
  - Defines design-time vs runtime separation
  - Introduces the six primary ADL abstractions (Provider, Tool, Agent, Task, Workflow, Run)
  - Describes the state model and execution pipeline at a conceptual level

- **Normative language and invariants** — [`spec/1.0/normative-language.md`](1.0/normative-language.md)
  - Defines MUST / SHOULD / MAY terminology
  - Specifies determinism and output pipeline ordering
  - Establishes required runtime invariants

- **Contracts and guard semantics** — [`spec/1.0/contracts.md`](1.0/contracts.md)
  - Defines contracts as first-class language concepts
  - Specifies guard evaluation, repair, and failure semantics
  - Introduces contract profiles (json, schema, protobuf, text)

Additional documents (schemas, contract profiles, and wire formats) may be added as ADL 1.0 evolves.

## Non-normative material

Examples and explanatory material live outside the specification tree:

- See [`examples/`](../examples/) for illustrative ADL documents

These materials are informative and do not override the normative requirements defined here.