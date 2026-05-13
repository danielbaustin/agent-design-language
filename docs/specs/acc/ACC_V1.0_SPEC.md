# ACC v1.0 Specification

## Status

Tracked formal baseline for the current implemented ADL Capability Contract
surface.

This document describes the canonical current `ACC v1.0` baseline as
implemented in:

- [`adl/src/acc.rs`](../../../adl/src/acc.rs)

Matching machine-readable schema:

- [`adl-spec/schemas/acc/v1.0/adl_capability_contract.v1.schema.json`](../../../adl-spec/schemas/acc/v1.0/adl_capability_contract.v1.schema.json)

## 1. Purpose

`ACC v1.0` captures the current implemented baseline without overclaiming the
additive `v1.1` direction that requires later repo/code adoption.

This spec/schema package is intentionally conservative:

- it matches the current Rust `AdlCapabilityContractV1` shape and validator
  shape
- it remains JSON-compatible and machine-validatable
- it keeps UTS/ACC separation explicit
- it does not treat structural validity as automatic execution authority

The machine-readable JSON Schema captures the current object model plus several
first-order coherence constraints. The runtime validator in
[`adl/src/acc.rs`](../../../adl/src/acc.rs) remains the normative source for
deeper semantic checks such as:

- actor / grant / capability identity equality
- known-evidence linkage for `policy_checks.evidence_ref`
- delegated-chain attribution against the grantor / grantee pair
- execution adapter equality against the declared tool adapter
- top-level / policy-check decision equality
- visibility-matrix audience coverage
- fail-closed public/projection visibility posture
- full redaction-surface coverage
- private-state leakage rejection in trace and redaction examples

## 2. Canonical Object Shape

A canonical `ACC v1.0` object contains:

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

## 3. Required Runtime Semantics

### Accountable actor

ACC requires an authenticated accountable actor.

- `actor.authenticated` must be `true`
- `actor.authority_evidence` must be present and non-empty
- authority must not be grounded in model self-assertion

### Authority grant

The authority grant must bind the actor to the same capability named by the
contract.

- `authority_grant.grantee_actor_id` must match `actor.actor_id`
- `authority_grant.capability_id` must match `capability.capability_id`

### Policy and decision coherence

`policy_checks` are required and must agree with the top-level `decision`.

Examples of invalid combinations:

- top-level `allowed` with a denied policy check
- top-level `revoked` with allowed policy checks

### Decision and execution coherence

Execution approval is not independent of the top-level decision.

- `allowed` requires an active grant and `approved_for_execution: true`
- `revoked` requires a revoked grant
- `denied`, `delegated`, and `revoked` must not approve execution

### Confirmation and Freedom Gate

When confirmation or Freedom Gate mediation is required, the contract must
carry that fact explicitly.

- required confirmation must name both confirming actor and confirmation id
- required Freedom Gate mediation must not leave the decision as
  `not_required`

### Replay and privacy

Replay metadata must remain evidence-backed and privacy-aware.

- `trace_replay.trace_id` is required
- `trace_replay.evidence_refs` must be non-empty
- visibility and redaction policy must be complete
- private-state leakage through trace metadata is invalid

## 4. Machine-Readable Boundary

The matching JSON Schema is intended to be truthful about the implemented
contract shape and about several high-value cross-field constraints:

- authenticated actor requirement
- non-model authority-evidence requirement
- active-grant / revoked-grant coherence
- execution-approval coherence
- confirmation and Freedom Gate requirement gating

It does not fully encode every semantic rule currently enforced by
`validate_acc_v1`. Where exact runtime linkage matters, the runtime validator
remains the stronger proving surface.

This includes several invariants that a second implementation SHOULD carry
forward explicitly:

- `authority_grant.grantee_actor_id == actor.actor_id`
- `authority_grant.capability_id == capability.capability_id`
- `execution.adapter_id == tool.adapter_id`
- every `policy_checks[].decision == decision`
- every `policy_checks[].evidence_ref` resolves to known evidence, grant, or
  delegation lineage
- visibility, redaction, and trace privacy checks fail closed

## 5. Canonical JSON Example

```json
{
  "schema_version": "acc.v1",
  "contract_id": "acc.review.github.create_issue.1001",
  "tool": {
    "tool_name": "github.create_issue",
    "tool_version": "1.0",
    "registry_tool_id": "github.create_issue",
    "adapter_id": "github.public"
  },
  "actor": {
    "actor_id": "runtime.review_agent",
    "actor_kind": "agent",
    "authenticated": true,
    "authority_evidence": [
      {
        "evidence_id": "credential.runtime.review_agent",
        "kind": "credential",
        "issuer": "adl.runtime"
      },
      {
        "evidence_id": "grant.github.issue.operator.primary",
        "kind": "operator_grant",
        "issuer": "operator.primary"
      }
    ]
  },
  "authority_grant": {
    "grant_id": "grant.github.issue.operator.primary",
    "grantor_actor_id": "operator.primary",
    "grantee_actor_id": "runtime.review_agent",
    "capability_id": "github.create_issue",
    "scope": "repo:issues.write",
    "status": "active",
    "revocation_reason": null
  },
  "role_standing": {
    "role": "review_agent",
    "standing": "service_actor"
  },
  "delegation_chain": [],
  "capability": {
    "capability_id": "github.create_issue",
    "side_effect_class": "external_write",
    "resource_type": "github_issue",
    "resource_scope": "repo"
  },
  "policy_checks": [
    {
      "policy_id": "policy.github.issue.creation",
      "decision": "allowed",
      "evidence_ref": "grant.github.issue.operator.primary"
    }
  ],
  "confirmation": {
    "required": false,
    "confirmed_by_actor_id": null,
    "confirmation_id": null
  },
  "freedom_gate": {
    "required": false,
    "decision": "not_required",
    "event_id": null
  },
  "execution": {
    "adapter_id": "github.public",
    "environment": "production",
    "dry_run": false,
    "approved_for_execution": true
  },
  "trace_replay": {
    "trace_id": "trace.github.issue.1001",
    "replay_allowed": false,
    "replay_posture": "governance_sensitive",
    "evidence_refs": [
      "credential.runtime.review_agent",
      "grant.github.issue.operator.primary"
    ]
  },
  "privacy_redaction": {
    "data_sensitivity": "internal",
    "visibility": {
      "actor_view": "full",
      "operator_view": "full",
      "reviewer_view": "redacted",
      "public_report_view": "aggregate",
      "observatory_projection": "aggregate"
    },
    "redaction_rules": [
      "redact_tokens",
      "redact_private_state",
      "aggregate_public_reporting"
    ],
    "visibility_matrix": [
      {
        "audience": "actor",
        "level": "full",
        "rationale": "actor requires complete local execution context"
      },
      {
        "audience": "operator",
        "level": "full",
        "rationale": "operator owns grant and remediation review"
      },
      {
        "audience": "reviewer",
        "level": "redacted",
        "rationale": "reviewers see enough for governance verification"
      },
      {
        "audience": "public_report",
        "level": "aggregate",
        "rationale": "public reporting must fail closed"
      },
      {
        "audience": "observatory_projection",
        "level": "aggregate",
        "rationale": "observatory projection must avoid full trace leakage"
      }
    ],
    "redaction_examples": [
      {
        "surface": "arguments",
        "source_shape": "title=Fix issue body=token:abc123",
        "redacted_shape": "title=Fix issue body=[REDACTED]"
      },
      {
        "surface": "results",
        "source_shape": "issue_url=https://github.com/org/repo/issues/10",
        "redacted_shape": "issue_url=https://github.com/org/repo/issues/[ID]"
      },
      {
        "surface": "errors",
        "source_shape": "403 invalid token abc123",
        "redacted_shape": "403 invalid token [REDACTED]"
      },
      {
        "surface": "traces",
        "source_shape": "trace github.create_issue operator.primary",
        "redacted_shape": "trace github.create_issue [REDACTED]"
      },
      {
        "surface": "projections",
        "source_shape": "created issue for citizen.private.ticket",
        "redacted_shape": "created issue for protected ticket"
      }
    ],
    "trace_privacy": {
      "exposes_citizen_private_state": false,
      "protected_state_refs": [
        "citizen.ticket",
        "operator.secret"
      ]
    }
  },
  "failure_policy": {
    "failure_code": "github_issue_denied",
    "message": "deny with recorded governance evidence when authority is insufficient",
    "retryable": false
  },
  "decision": "allowed"
}
```

## 6. Non-Claims

`ACC v1.0` does not claim:

- universal governance semantics across providers or ecosystems
- direct equivalence with UTS validity
- that runtime orchestration policy can be replaced by schema alone
- that replay permission can be inferred from replayability alone

## 7. Summary

`ACC v1.0` is the current implemented ADL governance baseline.

It exists to make authority, standing, decision, execution, replay, and privacy
posture explicit and reviewable around capability invocation.
