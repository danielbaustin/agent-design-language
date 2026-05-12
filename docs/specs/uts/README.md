# Universal Tool Schema (UTS)

## Status

Tracked canonical UTS spec entrypoint.

This directory now uses the following framing:

- `UTS v1`
  - the currently implemented schema surface
- `UTS v1.1`
  - an additive evolution path and future standardization direction

Unless a section explicitly says otherwise, statements in this note should be
read as describing the current `UTS v1` posture. The guaranteed formal current
baseline for `UTS v1` is the narrower
[`UTS_V1.0_SCHEMA.md`](./UTS_V1.0_SCHEMA.md) document plus its matching
machine-readable JSON Schema. Proposal material for `UTS v1.1` is called out
separately and must not be treated as guaranteed current runtime behavior.

Machine-readable companions:

- [`adl-spec/schemas/uts/v1.0/universal_tool_schema.v1.schema.json`](../../../adl-spec/schemas/uts/v1.0/universal_tool_schema.v1.schema.json)
- [`adl-spec/schemas/uts/v1.1/universal_tool_schema.v1_1.schema.json`](../../../adl-spec/schemas/uts/v1.1/universal_tool_schema.v1_1.schema.json)
- [`adl-spec/schemas/uts/v1.1/tool_invocation.v1_1.schema.json`](../../../adl-spec/schemas/uts/v1.1/tool_invocation.v1_1.schema.json)

Authority/governance companion:

- `UTS` defines schema semantics
- `ACC` defines runtime authority, approval, and execution governance

The two are intentionally related but not interchangeable.

## 1. Purpose

The Universal Tool Schema defines a transport-independent, provider-neutral
model for describing tools, capabilities, and invocation semantics in agent
systems.

UTS exists to improve:

- interoperability
- inspectability
- replayability
- observability
- structured invocation

The goal is not merely to standardize function calling. The goal is to define a
safer and more reviewable model for capability invocation across heterogeneous
agent ecosystems.

## 2. Scope

UTS standardizes:

- tool description
- machine-readable input/output shape
- side-effect semantics
- replayability posture
- observability posture
- transport-independent capability structure

UTS does not standardize:

- runtime governance policy
- provider-specific safety systems
- authorization frameworks
- orchestration topology
- agent cognition
- identity systems

These concerns may be layered above UTS by runtimes or governance frameworks.

UTS validity is therefore narrower than runtime authority:

- `UTS validity != authority`
- `UTS validity != execution permission`
- `UTS validity != replay permission`

A tool or invocation may be structurally valid under UTS while still requiring
runtime review, authorization, or governance before it may execute or replay.
In ADL terms, that higher-layer decision surface belongs in ACC rather than in
UTS itself.

## 3. Current Implemented Baseline (`UTS v1`)

The current implemented baseline lives in Rust today at:

- [`adl/src/uts.rs`](../../../adl/src/uts.rs)

That baseline is expressed as `UniversalToolSchemaV1` and validated by
`validate_uts_v1`.

Its shape is intentionally richer than a minimal function-call schema. The
current implemented surface already includes:

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

The authoritative tracked baseline for that shape is:

- [`UTS_V1.0_SCHEMA.md`](./UTS_V1.0_SCHEMA.md)
- [`universal_tool_schema.v1.schema.json`](../../../adl-spec/schemas/uts/v1.0/universal_tool_schema.v1.schema.json)

## 4. Additive Proposal Direction (`UTS v1.1`)

`UTS v1.1` is an additive proposal. It is not the current wire contract.

The proposal direction does three things:

- keeps schema semantics cleanly separated from runtime authority
- makes invocation metadata first-class and machine-validatable
- improves standardization posture through explicit replayability,
  observability, and version-negotiation semantics

The tracked proposal documents are:

- [`UTS_V1.1_SCHEMA.md`](./UTS_V1.1_SCHEMA.md)
- [`universal_tool_schema.v1_1.schema.json`](../../../adl-spec/schemas/uts/v1.1/universal_tool_schema.v1_1.schema.json)
- [`tool_invocation.v1_1.schema.json`](../../../adl-spec/schemas/uts/v1.1/tool_invocation.v1_1.schema.json)

Proposal examples in the `v1.1` surfaces are illustrative. They are not
guaranteed `v1` fixtures.

## 5. UTS And ACC

UTS answers:

> What is this tool?

ACC answers:

> Who may use it, under what authority, with what visibility, and with what
> evidence?

The split is intentional. A model can propose a tool call, and a tool can have
a valid portable schema, but neither fact grants execution authority.

See also:

- [`docs/explainers/UTS_AND_ACC.md`](../../explainers/UTS_AND_ACC.md)

## 6. Invocation Semantics Versus Approval

Invocation contracts may describe structure and expected execution posture
without deciding whether execution is allowed.

Invocation contracts therefore remain responsible for expressing:

- invocation intent
- replayability classification
- observability posture

Authority, authorization, and governance remain layered runtime concerns.

## 7. Non-Goals

UTS does not attempt to:

- solve alignment
- define universal governance policy
- replace authorization systems
- replace orchestration frameworks
- define agent cognition
- define identity continuity
- guarantee correctness of model outputs
- guarantee tool safety
- eliminate the need for runtime governance

This scoped approach is intentional. It supports interoperability and
incremental adoption without smuggling runtime approval into the schema layer.

## 8. Summary

The key distinction is simple:

- `UTS v1` is the current implemented schema surface
- `UTS v1.1` is the additive future direction
- `UTS` defines schema semantics
- `ACC` defines runtime authority and governance

That separation keeps ADL's governed-tools model reviewable, portable, and
honest about what schema validity does and does not mean.
