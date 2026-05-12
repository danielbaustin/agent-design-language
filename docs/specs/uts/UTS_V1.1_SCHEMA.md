# UTS v1.1 Schema

## Status

Tracked normative next-version specification for the Universal Tool Schema.

Status note:

> This document defines the `UTS v1.1` adoption target.
> The current implemented runtime baseline remains `UTS v1.0` until follow-on
> code adoption lands.

Matching machine-readable schema artifacts:

- [`adl-spec/schemas/uts/v1.1/universal_tool_schema.v1_1.schema.json`](../../../adl-spec/schemas/uts/v1.1/universal_tool_schema.v1_1.schema.json)
- [`adl-spec/schemas/uts/v1.1/tool_invocation.v1_1.schema.json`](../../../adl-spec/schemas/uts/v1.1/tool_invocation.v1_1.schema.json)

Implementation-facing baseline reference:

- [`adl/src/uts.rs`](../../../adl/src/uts.rs)
- [`UTS_V1.0_SCHEMA.md`](./UTS_V1.0_SCHEMA.md)

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

## 1. Purpose

This document defines the normative machine-readable structure for `UTS v1.1`
as an evolutionary successor to the current implemented `UTS v1.0` baseline.

`UTS v1.1` preserves the strongest parts of `v1.0` while adding explicit
compatibility, richer side-effect expression, and clearer invocation metadata.

## 2. Evolution From UTS v1.0

`UTS v1.1` is additive, not a rename-driven redesign.

It preserves the current field families from `UTS v1.0`:

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

It adds only bounded new metadata:

- `compatible_versions`
- `categories`
- `side_effects`
- `observability`
- `planning`

`v1.1` therefore remains consumable by a `v1.0`-aware implementation that
ignores unknown additive fields, while still letting newer runtimes act on the
new metadata.

## 3. Compatibility And Version Negotiation

`UTS v1.1` makes compatibility posture explicit.

Required fields:

- `schema_version: uts.v1.1`
- `compatible_versions`

`compatible_versions` semantics:

- a tool definition MUST be structurally valid under the `v1.1` schema
- `compatible_versions` declares the set of UTS schema versions, including the
  current `uts.v1.1` version, that the author expects a runtime to consume
  compatibly
- a runtime SHOULD validate against the highest version it supports
- a runtime that only supports an earlier compatible version MAY ignore unknown
  additive `v1.1` fields rather than rejecting the definition outright
- compatibility does not permit renaming or removing the preserved `v1.0`
  fields

Example:

```yaml
schema_version: uts.v1.1
compatible_versions:
  - uts.v1
  - uts.v1.1
```

## 4. Preserved Core Tool Schema

All `UTS v1.0` core schema semantics carry forward into `v1.1`.

### Required preserved fields

- `schema_version`
- `compatible_versions`
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

### Optional preserved field

- `extensions`

## 5. Additive v1.1 Fields

### `categories`

`categories` is an illustrative tagging/search taxonomy.

It is not the primary normative governance surface. Typed fields such as
`side_effect_class`, `data_sensitivity`, and `exfiltration_risk` remain the
stronger machine-readable signals.

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

### `side_effects`

`side_effects` is an additive richer representation of side-effect posture.

It complements rather than replaces `side_effect_class`.

Suggested values:

- `none`
- `local_state`
- `external_state`
- `irreversible`
- `human_visible`
- `governance_relevant`

This allows multi-dimensional expression that `side_effect_class` alone cannot
capture.

### `observability`

Suggested values:

- `none`
- `basic`
- `full`
- `governance`

Definitions:

- `none`: no specific runtime visibility expectations beyond local control
- `basic`: invocation recorded with timestamp and result status
- `full`: invocation metadata, arguments posture, and outputs posture are fully
  reviewable in the runtime's normal trace model
- `governance`: elevated audit/review posture is expected before or around
  execution

Observability metadata is metadata, not surveillance. It does not require
centralized monitoring or centralized orchestration.

### `planning`

Suggested fields:

- `high_risk`
- `irreversible`
- `expensive`
- `slow`
- `review_recommended`

Planning metadata is descriptive execution-planning metadata only.

It MUST NOT imply:

- authority
- approval
- execution permission
- replay permission

## 6. Side-Effect Migration Note

`UTS v1.0` used a single `side_effect_class` enum.

`UTS v1.1` keeps that field and adds `side_effects` for richer expression.

Suggested mapping examples:

- `read` -> `side_effects: [none]`
- `local_write` -> `side_effects: [local_state]`
- `external_write` -> `side_effects: [external_state]`
- `destructive` -> `side_effects: [external_state, irreversible]`
- `exfiltration` -> `side_effects: [external_state, governance_relevant]`

This is a design improvement, not a replacement of the scalar field.

## 7. Canonical Extended Tool Object

Illustrative complete `UTS v1.1` example:

```yaml
schema_version: uts.v1.1
compatible_versions:
  - uts.v1
  - uts.v1.1
name: github.create_issue
version: 1.1.0
description: Create a GitHub issue in a bounded repository under governed runtime control.
categories:
  - external_network
  - human_visible
  - governance_sensitive
input_schema:
  type: object
  required:
    - repository
    - title
  properties:
    repository:
      type: string
    title:
      type: string
    body:
      type: string
output_schema:
  type: object
  required:
    - issue_url
  properties:
    issue_url:
      type: string
      format: uri
side_effect_class: external_write
side_effects:
  - external_state
  - human_visible
  - governance_relevant
determinism: bounded_nondeterministic
replay_safety: replay_requires_approval
idempotence: conditionally_idempotent
resources:
  - resource_type: github_issue
    scope: repository
authentication:
  mode: user_delegated
  required: true
data_sensitivity: internal
exfiltration_risk: medium
execution_environment:
  kind: external_service
  isolation: provider_controlled
errors:
  - code: permission_denied
    message: Caller lacks permission to create issues in the target repository.
    retryable: false
observability: governance
planning:
  high_risk: true
  irreversible: false
  expensive: false
  slow: false
  review_recommended: true
extensions:
  x-adl-human-impact: issue_creation
```

## 8. Invocation Schema Relationship

The invocation schema is a companion metadata contract, not a replacement for
actual tool arguments.

It is intended to capture:

- invocation metadata
- intent
- reviewable execution posture
- correspondence to the tool definition

It is not, by itself, a full execution payload unless a runtime chooses to pair
it with argument/result bodies elsewhere.

Invocation correspondence rules:

- `tool.name` and `tool.version` MUST refer to a tool definition
- `side_effect_expectations` SHOULD be consistent with the tool definition and
  MAY express a subset of the declared `side_effects` when a particular call is
  narrower than the tool's full capability surface
- `replay_safety` and `observability` SHOULD match or refine the tool
  definition's posture; they MUST NOT silently weaken it

## 9. Canonical Invocation Example

```yaml
invocation_id: inv-1842
caller: runtime.review_agent

tool:
  name: github.create_issue
  version: 1.1.0

intent: remediation
purpose: |
  Create remediation issue from failed review findings.

side_effect_expectations:
  - external_state
  - human_visible

replay_safety: replay_requires_approval
observability: governance

timeout_ms: 30000
retry:
  max_attempts: 1
  strategy: fixed_delay
```

## 10. Conformance Expectations

A conformant `UTS v1.1` implementation SHOULD:

- validate preserved `v1.0` fields using the same core baseline rules
- validate additive `v1.1` fields when present
- ignore unknown additive higher-version fields only when a compatible earlier
  version is explicitly declared and the runtime supports that compatibility
- reject malformed structural definitions

The repository already contains implementation-facing conformance work in the
Rust codebase. That test posture should be treated as the current reference
implementation of UTS conformance until a standalone portable conformance suite
is published.

## 11. Transport Binding Note

`UTS v1.1` remains transport-neutral.

A future non-normative appendix may show example bindings for:

- MCP-style tool registries
- HTTP/REST tool catalogs
- OpenAI-compatible tool/function wrappers
- local runtime registries

Those bindings should preserve UTS semantics rather than redefine them.

## 12. Summary

`UTS v1.1` is the next UTS target, built directly on the existing `UTS v1.0`
model.

It stays additive by preserving the current core field vocabulary and semantics
while adding explicit compatibility, richer side-effect expression,
observability metadata, planning metadata, and invocation-companion structure.
