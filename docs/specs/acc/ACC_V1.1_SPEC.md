# ACC v1.1 Specification

## Status

Tracked normative next-version specification for the ADL Capability Contract.

Status note:

> This document defines the `ACC v1.1` adoption target.
> The current implemented runtime baseline remains `ACC v1.0` until follow-on
> code adoption lands.

Matching machine-readable schema:

- [`adl-spec/schemas/acc/v1.1/adl_capability_contract.v1_1.schema.json`](../../../adl-spec/schemas/acc/v1.1/adl_capability_contract.v1_1.schema.json)

Implementation-facing baseline reference:

- [`adl/src/acc.rs`](../../../adl/src/acc.rs)
- [`ACC_V1.0_SPEC.md`](./ACC_V1.0_SPEC.md)

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

This document defines the normative machine-readable structure for `ACC v1.1`
as an evolutionary successor to the current implemented `ACC v1.0` baseline.

`ACC v1.1` preserves the current governance model while tightening explicit
versioning and delegation semantics.

## 2. Evolution From ACC v1.0

`ACC v1.1` is evolutionary, not a redesign.

It preserves the current field families:

- `schema_version`
- `contract_id`
- `tool`
- `actor`
- `authority_grant`
- `role_standing`
- `delegation_chain`
- `capability`
- `policy_checks`
- `confirmation`
- `freedom_gate`
- `execution`
- `trace_replay`
- `privacy_redaction`
- `failure_policy`
- `decision`

It adds only bounded new metadata:

- `compatible_versions`
- `governance_profile`
- `delegation_constraints`

`ACC v1.1` does not replace the anti-self-assertion model, the active-grant
requirement, or the execution / replay / privacy separation already present in
`ACC v1.0`.

## 3. Core Design Rules

ACC remains the ADL-specific runtime governance layer.

ACC metadata MUST NOT be treated as equivalent to:

- UTS validity
- model preference
- prompt content
- self-asserted authority

Authority MUST originate from reviewable runtime evidence such as:

- credentials
- operator grants
- registry grants
- policy records
- delegation records

`model_claim` MUST NOT establish authority in `ACC v1.1`.

Standing semantics remain runtime-defined.

`ACC v1.1` preserves standing-bearing fields and governance consequences
without claiming one universal cross-runtime standing taxonomy.

## 4. Additive Fields

### `compatible_versions`

Optional list of ACC versions that a runtime may parse as compatible.

Example:

```yaml
compatible_versions:
  - acc.v1
  - acc.v1.1
```

This is version-negotiation metadata. It does not weaken runtime validation.

### `governance_profile`

Optional token naming the runtime governance profile expected by the contract.

Examples:

- `standard_reviewed_runtime`
- `high_observability_runtime`
- `restricted_external_mutation`

This field is descriptive and routing-oriented. It does not replace the
contract's explicit decision, execution, replay, or privacy fields.

### `delegation_constraints`

Optional additive delegation guardrails.

Fields:

- `max_depth`
- `allow_redelegation`
- `scope_ceiling`

These constraints narrow delegated authority; they do not expand it.

## 5. ACC-Lite Compatibility

Some runtimes may adopt `ACC v1.1` incrementally.

An `ACC-Lite` integration may focus operationally on a subset such as:

- accountable actor identity
- authority grant
- top-level decision
- execution approval posture
- replay posture
- observability / privacy posture

This does not define a separate wire contract.

It means a runtime may begin by enforcing the most important governance fields
first while still targeting the canonical `ACC v1.1` schema and semantics.

## 6. Canonical Minimal ACC Object

Minimal `ACC v1.1` adoption-target example:

This is the canonical next-version fixture for schema and docs review.
It is not a current runtime fixture until follow-on code adoption lands.

```yaml
schema_version: acc.v1.1
contract_id: acc.basic.1001
compatible_versions:
  - acc.v1
  - acc.v1.1
governance_profile: standard_reviewed_runtime

tool:
  tool_name: filesystem.search
  tool_version: "1.0"
  registry_tool_id: filesystem.search
  adapter_id: local.filesystem

actor:
  actor_id: runtime.review_agent
  actor_kind: agent
  authenticated: true
  authority_evidence:
    - evidence_id: credential.runtime.review_agent
      kind: credential
      issuer: adl.runtime

authority_grant:
  grant_id: grant.readonly.1001
  grantor_actor_id: operator.primary
  grantee_actor_id: runtime.review_agent
  capability_id: filesystem.search.read
  scope: local-readonly
  status: active

role_standing:
  role: review_agent
  standing: service_actor

delegation_chain: []

capability:
  capability_id: filesystem.search.read
  side_effect_class: read
  resource_type: filesystem
  resource_scope: local

policy_checks:
  - policy_id: policy.local.readonly
    decision: allowed
    evidence_ref: grant.readonly.1001

confirmation:
  required: false
  confirmed_by_actor_id: null
  confirmation_id: null

freedom_gate:
  required: false
  decision: not_required
  event_id: null

execution:
  adapter_id: local.filesystem
  environment: local
  dry_run: false
  approved_for_execution: true

trace_replay:
  trace_id: trace.local.readonly.1001
  replay_allowed: true
  replay_posture: reviewable
  evidence_refs:
    - credential.runtime.review_agent
    - grant.readonly.1001

privacy_redaction:
  data_sensitivity: internal
  visibility:
    actor_view: full
    operator_view: full
    reviewer_view: redacted
    public_report_view: aggregate
    observatory_projection: aggregate
  redaction_rules:
    - redact_private_state
  visibility_matrix:
    - audience: actor
      level: full
      rationale: actor requires full local context
    - audience: operator
      level: full
      rationale: operator owns authority review
    - audience: reviewer
      level: redacted
      rationale: reviewer sees review-safe trace surfaces
    - audience: public_report
      level: aggregate
      rationale: public report must fail closed
    - audience: observatory_projection
      level: aggregate
      rationale: projection must avoid full trace disclosure
  redaction_examples:
    - surface: arguments
      source_shape: path=/srv/secret.txt
      redacted_shape: path=/srv/[REDACTED]
    - surface: results
      source_shape: content=secret
      redacted_shape: content=[REDACTED]
    - surface: errors
      source_shape: permission denied for secret path
      redacted_shape: permission denied for protected path
    - surface: traces
      source_shape: trace local secret path
      redacted_shape: trace local protected path
    - surface: projections
      source_shape: projected citizen.private.record
      redacted_shape: projected protected record
  trace_privacy:
    exposes_citizen_private_state: false
    protected_state_refs:
      - protected.record

failure_policy:
  failure_code: readonly_access_denied
  message: deny execution when authority or standing is insufficient
  retryable: false

decision: allowed
```

## 7. Authority Scope Semantics

The contract continues to express effective authority through the combination
of:

- `authority_grant.scope`
- `capability.capability_id`
- `capability.resource_type`
- `capability.resource_scope`
- `decision`
- optional `delegation_constraints`

`ACC v1.1` does not introduce a separate authority-shape object because the
current model already makes authority reviewable through these fields.

A runtime SHOULD interpret authority scope conservatively:

- grants bound the ceiling of what may happen
- capability metadata narrows what the invocation is actually attempting
- delegation constraints can narrow the ceiling further
- execution approval must not widen any of the above

## 8. Governance Consistency Requirements

Governance state MUST remain internally coherent.

An ACC object MUST NOT simultaneously represent:

- revoked authority and approved execution
- denied governance review and approved execution
- missing confirmation and confirmed execution when confirmation is required
- required Freedom Gate mediation with a `not_required` decision

Runtime validation SHOULD reject contradictory governance states.

## 9. Delegation Constraints

When present, `delegation_constraints` further narrow delegated execution.

### `max_depth`

- positive integer
- MUST NOT exceed the runtime's supported delegation ceiling
- SHOULD be greater than or equal to the maximum observed depth in
  `delegation_chain`

### `allow_redelegation`

- boolean
- `false` means the delegated actor may not create a further delegated contract

### `scope_ceiling`

- string token naming the maximum delegated scope that remains valid

Delegated authority MUST NOT exceed:

- the grant expressed in `authority_grant`
- the resource boundaries in `capability`
- standing restrictions
- replay restrictions
- explicit `delegation_constraints`

## 10. Approval, Replay, And Freedom Gate Semantics

`ACC v1.1` keeps these decisions inside the contract rather than pushing them
into undefined external records.

The contract itself carries:

- execution approval posture in `execution.approved_for_execution`
- replay posture in `trace_replay`
- Freedom Gate mediation in `freedom_gate`
- confirmation requirements in `confirmation`

Runtimes MAY still emit companion trace or audit records, but those records are
supplemental evidence rather than the primary contract surface.

## 11. Machine-Readable Boundary

The matching `ACC v1.1` JSON Schema is intended to remain evolutionary over the
current `ACC v1.0` runtime contract.

It captures:

- the preserved field families from `ACC v1.0`
- additive `compatible_versions`, `governance_profile`, and
  `delegation_constraints`
- key first-order coherence rules for decision, grant status, execution,
  confirmation, and Freedom Gate semantics

It does not yet encode every deeper runtime semantic already enforced by the
current validator, such as exact evidence-reference linkage or full privacy
coverage analysis. Those remain runtime-validator responsibilities unless and
until a later schema/tooling pass encodes them more completely.

Observability metadata in ACC describes governance visibility posture. It does
not imply centralized logging, centralized monitoring, or centralized
orchestration.

## 12. Summary

`ACC v1.1` is the next ADL governance target, built directly on the existing
`ACC v1.0` model.

It stays evolutionary by preserving the current contract shape and validator
logic while adding explicit compatibility, governance-profile, and delegation
constraint metadata.
