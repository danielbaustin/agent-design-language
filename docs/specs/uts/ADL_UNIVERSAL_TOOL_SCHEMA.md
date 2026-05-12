# Universal Tool Schema (UTS)

## Status

Tracked canonical narrative specification for the Universal Tool Schema.

This document describes:

- the current implemented `UTS v1.0` baseline
- the additive `UTS v1.1` target
- the architectural separation between UTS schema semantics and ACC runtime
  governance

## Normative Language

The key words:

- MUST
- MUST NOT
- REQUIRED
- SHALL
- SHALL NOT
- SHOULD
- SHOULD NOT
- RECOMMENDED
- MAY
- OPTIONAL

are to be interpreted as described in RFC 2119.

## 1. Introduction

The Universal Tool Schema (UTS) defines a transport-independent,
provider-neutral standard for describing tools, capabilities, and invocation
semantics in agent systems.

UTS exists to improve:

- interoperability
- safety posture
- inspectability
- replayability
- observability
- structured invocation

UTS is intentionally narrower than runtime governance.

It answers:

> What is this tool, what does it touch, and what structural execution posture
> does it declare?

UTS does not answer:

> Who may use it, under what authority, and with what approval?

Those higher-layer questions belong to ACC and runtime governance.

## 2. Scope

UTS standardizes:

- tool description
- machine-readable input/output shape
- side-effect semantics
- replay posture metadata
- observability posture metadata
- transport-independent capability structure
- invocation-companion metadata

UTS does not standardize:

- runtime governance policy
- authorization frameworks
- provider-specific safety systems
- orchestration topology
- agent cognition
- identity systems

UTS validity therefore does not imply:

- authority
- execution permission
- replay permission
- governance approval

## 3. Core Design Principles

### Transport Independence

UTS defines semantics and structure without mandating a specific transport such
as HTTP, gRPC, WebSocket, message bus, or local in-process invocation.

### Provider Neutrality

UTS is not tied to one model provider, one runtime, or one orchestration
framework.

### Human And Machine Readability

UTS is designed to be:

- machine-validatable
- human-reviewable
- diff-friendly
- documentation-friendly

Human readability is a core safety feature.

### Incremental Adoption

UTS is designed for gradual adoption.

A runtime may begin by using only:

- schema normalization
- side-effect classification
- replay metadata
- observability metadata

without replacing its existing transport or orchestration system.

## 4. Relationship To ACC

UTS and ACC are intentionally separate.

UTS defines:

- schema semantics
- invocation-companion semantics
- side effects
- replay posture
- observability posture

ACC defines:

- authority
- approval
- standing
- delegation
- runtime governance

The split is load-bearing.

A tool definition may be perfectly valid under UTS while still being denied by
ACC or by runtime policy.

## 5. UTS v1.0 Baseline

The current implemented baseline lives in:

- [`adl/src/uts.rs`](../../../adl/src/uts.rs)

Its current field families are:

- `schema_version`
- `name`
- `version`
- `description`
- `input_schema`
- `output_schema`
- `side_effect_class`
- `determinism`
- `replay_safety`
- `idempotence`
- `resources`
- `authentication`
- `data_sensitivity`
- `exfiltration_risk`
- `execution_environment`
- `errors`
- `extensions`

The formal baseline documents for that shape are:

- [`UTS_V1.0_SCHEMA.md`](./UTS_V1.0_SCHEMA.md)
- [`universal_tool_schema.v1.schema.json`](../../../adl-spec/schemas/uts/v1.0/universal_tool_schema.v1.schema.json)

## 6. UTS v1.1 Additive Evolution

`UTS v1.1` is additive over `UTS v1.0`.

It preserves every core `v1.0` field family and adds new metadata alongside
those fields rather than renaming or removing them.

The additive `v1.1` fields are:

- `compatible_versions`
- `categories`
- `side_effects`
- `observability`
- `planning`

This means `v1.1` is not a fresh object model.

It is a richer layer over the existing `v1.0` shape.

The formal `v1.1` documents are:

- [`UTS_V1.1_SCHEMA.md`](./UTS_V1.1_SCHEMA.md)
- [`universal_tool_schema.v1_1.schema.json`](../../../adl-spec/schemas/uts/v1.1/universal_tool_schema.v1_1.schema.json)
- [`tool_invocation.v1_1.schema.json`](../../../adl-spec/schemas/uts/v1.1/tool_invocation.v1_1.schema.json)

## 7. Compatibility And Version Negotiation

`UTS v1.1` makes compatibility posture explicit.

A `v1.1` definition MUST include:

- `schema_version: uts.v1.1`
- `compatible_versions`

Example:

```yaml
schema_version: uts.v1.1
compatible_versions:
  - uts.v1
  - uts.v1.1
```

Semantics:

- a tool definition MUST be structurally valid under the `v1.1` schema
- `compatible_versions` declares the set of UTS schema versions, including the
  current `uts.v1.1` version, that the author expects a runtime to consume
  compatibly
- a runtime SHOULD validate against the highest version it supports
- a runtime that only supports an earlier compatible version MAY ignore unknown
  additive `v1.1` fields
- compatibility does not permit renaming or removing preserved `v1.0` fields

## 8. Side-Effect Semantics

UTS requires explicit declaration of side effects.

### v1.0 scalar field

`side_effect_class` remains the preserved baseline field.

Allowed `v1.0`-style values include:

- `read`
- `local_write`
- `external_read`
- `external_write`
- `process`
- `network`
- `destructive`
- `exfiltration`

### v1.1 additive array field

`side_effects` is the richer additive representation.

Suggested values:

- `none`
- `local_state`
- `external_state`
- `irreversible`
- `human_visible`
- `governance_relevant`

The array field does not replace the scalar field.

It complements it so a tool can express multiple side-effect dimensions.

## 9. Replayability, Determinism, And Idempotence

UTS treats these as separate concepts.

Preserved baseline fields:

- `determinism`
- `replay_safety`
- `idempotence`

This separation is intentional because these properties are not the same.

A tool may be:

- deterministic but not idempotent
- idempotent but not replay-safe
- replay-safe only under approval conditions

This is one of the most important parts of the schema.

## 10. Data Sensitivity And Exfiltration Risk

UTS keeps the following strong fields from `v1.0`:

- `data_sensitivity`
- `exfiltration_risk`

These fields are among the most valuable machine-readable signals in UTS.

They allow runtimes to distinguish ordinary tool use from higher-risk data
movement or disclosure surfaces before execution.

## 11. Categories

`categories` is an illustrative tagging/search taxonomy.

It is not the primary normative governance surface.

Suggested values:

- `read_only`
- `computational`
- `state_mutating`
- `external_network`
- `human_visible`
- `governance_sensitive`
- `identity_sensitive`
- `continuity_sensitive`
- `observability_sensitive`

These tags support:

- planning
- review
- search/indexing
- runtime routing

Typed fields such as `side_effect_class`, `data_sensitivity`, and
`exfiltration_risk` remain the stronger machine-readable semantics.

## 12. Observability

`UTS v1.1` adds explicit observability posture.

Suggested levels:

- `none`
- `basic`
- `full`
- `governance`

Interpretation:

- `none`: no specific runtime visibility expectations beyond local control
- `basic`: invocation recorded with timestamp and result status
- `full`: invocation metadata and result posture are fully reviewable
- `governance`: elevated audit/review posture is expected

Observability metadata is metadata, not surveillance.

It does not require:

- centralized logging
- centralized monitoring
- centralized orchestration
- centralized storage

## 13. Planning Metadata

`UTS v1.1` adds optional planning metadata.

Suggested fields:

- `high_risk`
- `irreversible`
- `expensive`
- `slow`
- `review_recommended`

Planning metadata is descriptive only.

It MUST NOT imply:

- authority
- approval
- execution permission
- replay authorization

That boundary keeps UTS out of ACC territory.

## 14. Invocation Companion Schema

The `v1.1` invocation schema is a companion metadata contract.

It is intended to capture:

- invocation identity
- caller identity
- intent
- purpose
- side-effect expectations
- replay-safety posture
- observability posture
- timeout/retry posture

It is not, by itself, a full argument/result payload unless a runtime chooses
that representation.

Invocation companion rules:

- `tool.name` and `tool.version` MUST refer to a tool definition
- `side_effect_expectations` SHOULD be consistent with the tool definition and
  MAY be a subset of the tool's full declared `side_effects`
- `replay_safety` and `observability` MUST NOT silently weaken the tool
  definition's declared posture

## 15. Conformance

A conformant `UTS` implementation SHOULD be able to validate:

- required fields
- schema structure
- side-effect declarations
- replay declarations
- observability declarations
- invocation-companion structure

The repository already contains implementation-facing conformance work in Rust.
That code should be treated as the current reference implementation of UTS
conformance until a standalone portable conformance suite is published.

## 16. Transport Binding Note

UTS is transport-neutral, but not transport-agnostic in the sense of ignoring
real deployment concerns.

Future non-normative bindings may show how UTS maps onto:

- MCP-style tool systems
- HTTP/REST tool catalogs
- OpenAI-compatible tool/function wrappers
- local runtime registries

Those bindings should preserve UTS semantics rather than redefine them.

## 17. Summary

UTS attempts to move tool ecosystems from:

> ad hoc provider-specific function calling

Toward:

> structured, replayable, observable, transport-neutral capability invocation.

The important structural truths are:

- `UTS v1.0` is the current implemented baseline
- `UTS v1.1` is additive over that baseline
- UTS keeps schema semantics separate from runtime authority
- replayability, idempotence, determinism, data sensitivity, and exfiltration
  risk remain first-class fields

That combination is what makes UTS stronger than ordinary tool-definition
schemas.
