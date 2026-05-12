# UTS v1.1 Schema

## Status

Draft additive schema evolution proposal for Universal Tool Schema.

Status note:

> This document describes the proposed `UTS v1.1` additive schema evolution.
> It is not the canonical current ADL `UTS v1` wire contract.

Matching machine-readable proposal artifacts:

- [`adl-spec/schemas/uts/v1.1/universal_tool_schema.v1_1.schema.json`](../../../adl-spec/schemas/uts/v1.1/universal_tool_schema.v1_1.schema.json)
- [`adl-spec/schemas/uts/v1.1/tool_invocation.v1_1.schema.json`](../../../adl-spec/schemas/uts/v1.1/tool_invocation.v1_1.schema.json)

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

This document defines the proposed machine-readable structure for:

- `UTS v1.1` tool definitions
- invocation metadata
- replayability metadata
- observability metadata
- planning-aware metadata

The goal is not to replace ACC or runtime governance. The goal is to make the
schema layer more explicit, transportable, and reviewable.

## 2. Design Principles

`UTS v1.1` SHOULD remain:

- transport-neutral
- provider-neutral
- runtime-neutral
- JSON-compatible
- human-reviewable
- machine-validatable

`UTS v1.1` intentionally avoids embedding:

- governance policy
- authorization semantics
- runtime approval semantics
- orchestration topology

For ADL, that higher-layer authority/governance companion is ACC:

- `UTS v1.1` defines schema and invocation semantics
- `ACC` defines approval, authority, and execution governance

UTS validity alone does not authorize execution or replay.

## 3. Schema Layers

### Core Tool Schema

Defines:

- identifier
- version
- compatibility metadata
- input schema
- output schema
- side effects

### Invocation Metadata

Defines:

- invocation semantics
- intent
- optional timeout/retry posture
- replay posture

### Replayability Metadata

Defines:

- deterministic replay posture
- observational replay posture
- non-replayable semantics

### Observability Metadata

Defines:

- runtime visibility posture
- audit expectations
- governance-sensitive visibility

Observability metadata describes visibility posture, not surveillance posture.
It does not require centralized monitoring, and it remains compatible with:

- local runtimes
- private runtimes
- operator-owned observability systems
- bounded audit surfaces

### Planning Metadata

Defines:

- high-risk indicators
- irreversible-action indicators
- review recommendations

## 4. Canonical Minimal Tool Object

Minimal proposal example:

```yaml
uts_version: "1.1"
compatible_versions:
  - "1.0"
  - "1.1"
id: filesystem.search
version: "1.1"
input_schema:
  type: object
output_schema:
  type: object
side_effects:
  - none
```

Equivalent JSON example:

```json
{
  "uts_version": "1.1",
  "compatible_versions": ["1.0", "1.1"],
  "id": "filesystem.search",
  "version": "1.1",
  "input_schema": {
    "type": "object"
  },
  "output_schema": {
    "type": "object"
  },
  "side_effects": ["none"]
}
```

These are proposal examples for `UTS v1.1`, not guaranteed `UTS v1` fixtures.

## 5. Extended Tool Object

Illustrative proposal example:

```yaml
uts_version: "1.1"
compatible_versions:
  - "1.0"
  - "1.1"
id: github.create_issue
version: "1.1"
category:
  - external_network
  - governance_sensitive
  - human_visible
input_schema:
  type: object
  required:
    - repository
    - title
output_schema:
  type: object
  required:
    - issue_url
side_effects:
  - external_state
  - human_visible
  - governance_relevant
replayability: observational
observability: governance
planning:
  high_risk: true
  irreversible: false
  review_required: true
```

## 6. Version Negotiation

`UTS v1.1` SHOULD make compatibility posture explicit.

Suggested fields:

- `uts_version`
- `compatible_versions`

Example:

```yaml
uts_version: "1.1"
compatible_versions:
  - "1.0"
  - "1.1"
```

This makes migration strategy concrete and lets runtimes distinguish:

- canonical current support
- additive forward support
- compatibility fallback posture

## 7. Replayability Taxonomy

Allowed proposal values:

- `deterministic`
- `observational`
- `non_replayable`

## 8. Observability Taxonomy

Allowed proposal values:

- `none`
- `basic`
- `full`
- `governance`

The observability taxonomy is metadata, not surveillance.

## 9. Invocation Schema

Canonical proposal invocation metadata SHOULD include:

```yaml
invocation_id: inv-1001
caller: runtime.review_agent
tool:
  id: github.create_issue
  version: "1.1"
intent: remediation
purpose: |
  Create remediation issue from failed review findings.
timeout_ms: 5000
retry:
  max_attempts: 1
  strategy: fixed_delay
side_effect_expectations:
  - external_state
  - human_visible
replayability: observational
observability: governance
```

## 10. Invocation Intent Taxonomy

Suggested invocation intents:

- `informational_query`
- `planning`
- `simulation`
- `review`
- `mutation`
- `governance_action`
- `observability_action`
- `remediation`

Runtimes MAY extend this taxonomy.

## 11. Threat Model

`UTS v1.1` is intended to reduce:

- hidden side effects
- replay ambiguity
- weak observability
- unsafe planning/execution coupling
- ambiguous invocation semantics

This is an operational goal, not a claim that schema alone eliminates runtime
risk.

## 12. Non-Claims

`UTS v1.1` does not by itself grant:

- authority
- execution permission
- replay permission

Those remain ACC and runtime concerns.

## 13. Summary

`UTS v1.1` is the additive standards-facing direction:

- cleaner invocation semantics
- explicit replayability
- explicit observability
- explicit version negotiation
- clean separation from ACC authority/governance

It is a proposal surface, not a claim that the current code already implements
the full model.
