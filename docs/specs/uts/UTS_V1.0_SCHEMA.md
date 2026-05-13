# UTS v1.0 Schema

## Status

Tracked formal baseline for the current implemented Universal Tool Schema
surface.

This document describes the canonical current `UTS v1` baseline as implemented
in:

- [`adl/src/uts.rs`](../../../adl/src/uts.rs)

Matching machine-readable schema:

- [`adl-spec/schemas/uts/v1.0/universal_tool_schema.v1.schema.json`](../../../adl-spec/schemas/uts/v1.0/universal_tool_schema.v1.schema.json)

## 1. Purpose

`UTS v1.0` captures the current implemented baseline without overclaiming the
additive `v1.1` direction that requires later repo/code adoption.

This schema is intentionally conservative:

- it matches the current Rust `UniversalToolSchemaV1` shape and key validator
  rules
- it remains JSON-compatible and machine-validatable
- it does not imply runtime authority, execution permission, or replay
  permission

## 2. Canonical Object Shape

A canonical `UTS v1.0` tool definition contains:

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
- optional `extensions`

## 3. Required Fields

### `schema_version`

- type: string
- required value: `uts.v1`

### `name`

- type: string
- required pattern: lowercase token-like identifier
- semantic rule: 3-80 characters, starts lowercase, then lowercase ASCII,
  digits, `-`, `_`, or `.`

### `version`

- type: string
- semantic rule: numeric `major.minor.patch`

### `description`

- type: string
- semantic rule: specific enough for reviewers

### `input_schema`

- type: JSON Schema fragment object
- semantic rule: must include a non-empty `type`

### `output_schema`

- type: JSON Schema fragment object
- semantic rule: must include a non-empty `type`

### `side_effect_class`

Allowed values:

- `read`
- `local_write`
- `external_read`
- `external_write`
- `process`
- `network`
- `destructive`
- `exfiltration`

### `determinism`

Allowed values:

- `deterministic`
- `bounded_nondeterministic`
- `nondeterministic`

### `replay_safety`

Allowed values:

- `replay_safe`
- `replay_requires_approval`
- `not_replay_safe`

### `idempotence`

Allowed values:

- `idempotent`
- `conditionally_idempotent`
- `not_idempotent`

### `resources`

- type: array
- semantic rule: at least one resource boundary is required

Each entry contains:

- `resource_type`
- `scope`

### `authentication`

Required fields:

- `mode`
- `required`

Allowed `mode` values:

- `none`
- `api_key`
- `oauth`
- `user_delegated`
- `service_account`

### `data_sensitivity`

Allowed values:

- `public`
- `internal`
- `confidential`
- `secret`
- `protected_prompt`
- `private_state`

### `exfiltration_risk`

Allowed values:

- `none`
- `low`
- `medium`
- `high`

### `execution_environment`

Required fields:

- `kind`
- `isolation`

Allowed `kind` values:

- `fixture`
- `dry_run`
- `local`
- `external_service`
- `process`
- `network`

### `errors`

- type: array
- semantic rule: at least one error-model entry is required

Each entry contains:

- `code`
- `message`
- `retryable`

## 4. Optional Fields

### `extensions`

- type: object
- optional

Extension keys remain constrained in the validator:

- keys must be lower-case token-like strings
- keys must start with `x-`
- keys must not smuggle authority or approval semantics into the schema layer
- required extension behavior is not allowed in the baseline

## 5. Canonical JSON Example

```json
{
  "schema_version": "uts.v1",
  "name": "fixture.safe_read",
  "version": "1.0.0",
  "description": "Read a bounded local fixture for reviewer-visible conformance.",
  "input_schema": {
    "type": "object",
    "properties": {
      "fixture_id": {
        "type": "string"
      }
    },
    "required": ["fixture_id"],
    "additionalProperties": false
  },
  "output_schema": {
    "type": "object",
    "properties": {
      "content": {
        "type": "string"
      }
    },
    "required": ["content"],
    "additionalProperties": false
  },
  "side_effect_class": "read",
  "determinism": "deterministic",
  "replay_safety": "replay_safe",
  "idempotence": "idempotent",
  "resources": [
    {
      "resource_type": "fixture",
      "scope": "local-readonly"
    }
  ],
  "authentication": {
    "mode": "none",
    "required": false
  },
  "data_sensitivity": "internal",
  "exfiltration_risk": "none",
  "execution_environment": {
    "kind": "fixture",
    "isolation": "local deterministic fixture only"
  },
  "errors": [
    {
      "code": "fixture_not_found",
      "message": "The requested fixture is not available.",
      "retryable": false
    }
  ],
  "extensions": {
    "x-adl-review-note": "UTS compatibility only; no execution authority."
  }
}
```

## 6. Explicit Non-Claims

`UTS v1.0` does not by itself define:

- runtime approval semantics
- authorization semantics
- replay permission
- orchestration topology
- actor authority
- ACC governance rules

Schema compatibility is not authority.

## 7. Summary

`UTS v1.0` is the truthful compatibility baseline for current ADL code. It is
already machine-readable, but it remains scoped to schema semantics rather than
runtime permission or governance.
