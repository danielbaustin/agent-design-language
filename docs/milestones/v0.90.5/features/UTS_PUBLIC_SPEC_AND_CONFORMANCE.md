# UTS Public Spec And Conformance

## Purpose

Universal Tool Schema v1.0 is the portable, model-facing description layer for
tools. It should be JSON-compatible and useful outside ADL, but it must not
pretend to grant runtime authority.

This feature inherits the WP-02 proposal/action boundary from
`TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md`: UTS validity is schema compatibility,
not permission to execute.

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
- dangerous-category fixtures for process, network, destructive, and
  exfiltration proposals that classify risk without granting execution
- explicit rule that UTS validity is not permission to execute
- compatibility notes for existing JSON tool-call systems
- versioning and extension guidance

## Public Compatibility Plan

UTS v1.0 should be understandable to JSON-schema-style tool ecosystems without
claiming to be a public standard. Compatibility means:

- tool interfaces are expressible as JSON-compatible documents;
- input and output schemas use JSON Schema fragments rather than ADL runtime
  objects;
- unknown extensions are preserved when safe and rejected when marked required
  but unsupported;
- side-effect, determinism, replay, authentication, data sensitivity, and
  exfiltration metadata are explicit enough for downstream governance;
- compatibility never means a model, client, or adapter may execute the tool
  directly.

The public-facing language should describe UTS as a portable schema discipline
or compatibility profile. It must not call UTS a standard, certification
program, marketplace contract, sandbox, permission grant, or production
security boundary.

## Conformance Plan

The conformance suite should sort fixtures into four groups:

| Fixture group | Purpose | WP-05 fixture class ids |
| --- | --- | --- |
| Valid category fixtures | Prove UTS can describe expected tool categories without granting runtime authority. | `valid.safe_read`, `valid.local_write`, `valid.external_read`, `valid.external_write`, `valid.destructive`, `valid.process`, `valid.network`, `valid.exfiltration` |
| Invalid schema/semantics fixtures | Prove malformed or underspecified tool descriptions fail for intended reasons. | `invalid.missing_semantics`, `invalid.missing_security_metadata`, `invalid.malformed_schema`, `invalid.ambiguous_side_effects`, `invalid.unsafe_extension`, `invalid.incompatible_version` |
| Extension fixtures | Prove extension behavior is deterministic and publication-safe. | `extension.optional_vendor_metadata`, `extension.unsupported_required_extension`, `extension.reserved_authority_extension` |
| Dangerous-category fixtures | Prove dangerous categories are classified and denied by governance before action. | `dangerous.destructive_denied`, `dangerous.process_denied`, `dangerous.network_denied`, `dangerous.exfiltration_denied` |

Valid dangerous-category fixtures are valid UTS descriptions only. They do not
become executable actions. The expected proof is that the schema accepts the
description while later ACC, policy, and executor surfaces deny or refuse action
unless a deliberately scoped later fixture proves a safe dry-run path.

## Validation Expectations

WP-04 lands the first strongly typed UTS v1 artifact in
`adl/src/uts.rs`. The review-facing Rust surface is
`UniversalToolSchemaV1`, `validate_uts_v1`, and `uts_v1_schema_json`.

WP-04 should make the schema and strongly typed artifact enforce these
requirements:

- every fixture declares one side-effect class from the WP-02 taxonomy;
- every fixture declares determinism and replay safety;
- every fixture declares authentication posture and data sensitivity;
- every fixture declares exfiltration risk;
- every fixture declares execution-environment hints;
- every fixture declares extension behavior;
- no UTS field grants ADL actor authority, delegation, Freedom Gate approval,
  visibility rights, replay permission, or adapter execution permission.

WP-05 should provide a deterministic conformance command or harness that:

- accepts every `valid.*` fixture for schema compatibility;
- rejects every `invalid.*` fixture for the intended reason;
- records extension outcomes for every `extension.*` fixture;
- classifies every `dangerous.*` fixture without granting execution;
- emits repository-relative, portable output with no host paths or secrets.

WP-05 implements the review-facing conformance harness in
`adl/src/uts_conformance.rs`. The fixture packet is exposed through
`uts_conformance_fixtures`, and `run_uts_conformance_suite` evaluates the
packet without granting execution. The focused validation command is:

```sh
cargo test --manifest-path adl/Cargo.toml uts -- --nocapture
```

## Non-Goals

- UTS does not define ADL actor authority.
- UTS does not define Freedom Gate decisions.
- UTS does not define citizen standing or operator inspection rights.
- UTS does not certify public standard conformance.
- UTS does not grant execution permission for dangerous categories.
