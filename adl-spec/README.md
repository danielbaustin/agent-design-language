# Agent Design Language (ADL)

Agent Design Language (ADL) is a declarative language for specifying
AI agents, tools, workflows, and execution contracts.

ADL separates **design-time intent** from **runtime execution**.
It enables deterministic prompt assembly, explicit output contracts,
and traceable agent runs across different model providers.

This repository contains the **language specification** for ADL 1.x.

## Specification

- [Specification index](spec/README.md)
- [Core concepts (ADL 1.0)](spec/1.0/core-concepts.md)
- [Normative language and invariants](spec/1.0/normative-language.md)

## Examples

The examples in this repository are intended to illustrate ADL concepts while ADL 1.0 is still in draft.
The exact YAML shape is therefore **illustrative**: runtimes may adopt a slightly different schema,
but the *concepts* should map 1:1.

- [Hello, contract (minimal ADL 1.0 example)](examples/hello-contract.yaml)

## Status

- Current version: ADL 1.0 (draft)
- Schema status: illustrative YAML while the ADL 1.0 schema is finalized
- Reference runtime: not yet published (planned)
- v0.x schema note: the `swarm/` runtime generates its authoritative schema from Rust structs; the committed JSON schema is a draft reference artifact and may lag.

## Design Principles

- Contract-driven execution
- Deterministic prompting
- Fail loudly, repair explicitly
- Traceable, debuggable runs

## Documents

- Product description (to be added under `docs/`)
- Software design document (to be added under `docs/`)
- v0.2 schema draft: `ADL_v0.2_Schema_Extensions.md`

## License

Apache 2.0
