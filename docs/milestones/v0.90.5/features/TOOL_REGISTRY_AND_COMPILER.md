# Tool Registry And Compiler

## Purpose

The tool registry and UTS-to-ACC compiler prevent loose model output from
binding directly to execution.

This feature inherits the WP-02 proposal/action boundary from
`TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md`: model output can only become an ACC
or a deterministic rejection after registry binding, normalization, and policy
context are explicit.

## Registry Requirements

The registry must:

- list known tools
- bind UTS definitions to approved adapters
- reject unknown tools
- reject unregistered tools
- version tool definitions
- record adapter capabilities and dry-run posture

WP-08 implements the review-facing registry and binding surface in
`adl/src/tool_registry.rs`:

- `ToolRegistryV1`
- `RegisteredToolV1`
- `ToolAdapterCapabilityV1`
- `ToolBindingRequestV1`
- `ToolBindingOutcomeV1`
- `bind_tool_registry_v1`
- `validate_tool_registry_v1`
- `registry_state_fingerprint_v1`
- `wp08_tool_registry_v1_fixture`
- `wp08_registry_rejection_fixtures`

The WP-08 fixture registry is deterministic and explicit: each tool carries a
validated UTS definition, each adapter carries its approved capability,
side-effect class, execution environment, dry-run support, and binding approval,
and the registry fingerprint is stable even when tool or adapter vectors are
reordered.

Approved binding requires all of the following:

- the request comes from the registry/compiler path, not direct model output
- the tool name is known
- the exact tool version is registered and active
- the requested adapter is registered
- the adapter is approved for the tool
- the adapter tool name, version, side-effect class, and execution environment
  match the UTS entry
- dry-run execution is explicitly requested and supported by the adapter

The WP-08 rejection fixtures cover:

- direct model-output binding
- unknown tools
- inactive or unregistered tools
- incompatible tool versions
- mismatched adapter capabilities
- unsafe dry-run posture

## Compiler Requirements

The compiler must:

- validate UTS
- normalize untrusted arguments
- reject ambiguous or malformed proposals
- reject proposals whose side effects are unknown or higher-risk than the
  registered tool and policy context allow
- map UTS semantics into ACC execution semantics
- inject policy context
- construct visibility and redaction rules
- reject unsatisfiable authority, resource, privacy, or execution constraints
- emit trace evidence for validation, normalization, policy, and rejection

WP-09 implements the review-facing deterministic compiler surface in
`adl/src/uts_acc_compiler.rs`:

- `ToolProposalV1`
- `UtsAccPolicyContextV1`
- `UtsAccCompilerInputV1`
- `UtsAccCompilerOutcomeV1`
- `UtsAccRejectionRecordV1`
- `compile_uts_to_acc_v1`
- `wp09_compiler_registry_fixture`
- `wp09_compiler_input_fixture`

The compiler consumes a normalized tool proposal, explicit WP-08 registry
state, and a bounded policy context. It emits either an `AdlCapabilityContractV1`
or a deterministic rejection record with evidence for validation,
normalization, registry binding, policy, and rejection. Review-facing evidence
uses redacted argument summaries and opaque registry digests rather than raw
proposal arguments or registry payloads.

The WP-09 mapping tests cover:

- safe read to allowed ACC
- delegated local write to delegated ACC
- denied destructive action
- denied exfiltration
- ambiguous proposal rejection

The WP-09 rejection tests cover unsatisfiable authority, resource, privacy,
visibility, replay, and execution constraints.

WP-09 remains fixture-backed and dry-run only. WP-10 owns broader argument
normalization and injection/path-traversal validation. WP-11 owns the fuller
policy evaluator beyond this bounded policy-context slice. Because the WP-09
ACC fixture emits one representable resource scope, multi-resource UTS inputs
are rejected instead of being truncated into a partial ACC.

WP-10 implements the bounded argument-normalization surface in the compiler
module:

- `UtsArgumentNormalizationErrorCodeV1`
- `UtsArgumentNormalizationErrorV1`
- `UtsArgumentNormalizationReportV1`
- `normalize_tool_proposal_arguments_v1`

The normalizer treats model-produced proposal arguments as untrusted input. It
checks UTS-declared required fields, additional-field posture, top-level JSON
types, bounded payload size, injection markers, path-traversal syntax, and
ambiguous schema defaults before policy evaluation or ACC construction. Error
evidence records only rejection classes and redacted digests; it does not echo
protected prompt text, secret-like values, or raw tool arguments.

## Determinism

Identical UTS, proposal, registry state, and policy context should produce an
identical ACC or identical rejection.

For WP-08, determinism is proven at the registry-binding layer. WP-09 extends
that proof to deterministic UTS/proposal/registry/policy compilation into ACC
or rejection records for the fixture-backed governed-tools slice.
