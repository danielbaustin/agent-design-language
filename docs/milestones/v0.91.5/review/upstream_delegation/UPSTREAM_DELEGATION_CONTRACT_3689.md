# Upstream Delegation Contract

## Status

`contract_defined_for_follow_on_implementation`

## Purpose

This packet defines `UpstreamDelegationV1` as the reviewable contract for governed cognitive escalation from one accountable ADL actor to another actor, service, provider, or external boundary.

The contract is intentionally narrow. It does not implement full IAM, provider execution, standing transitions, appeal governance, or autonomous model authority. It defines the minimum evidence shape that follow-on trace and proof work must preserve before upstream delegation can be treated as a safe C-SDLC runtime surface.

## Source Evidence

| Source | Current meaning |
| --- | --- |
| `docs/milestones/v0.93/features/DELEGATION_IAM_STANDING_AND_APPEAL_GOVERNANCE_v0.93.md` | Forward-planning feature contract for delegation, upstream delegation, IAM, standing transition, challenge, and appeal governance. |
| `docs/milestones/v0.7/DELEGATION_TRACE_v1.md` | Historical deterministic delegation trace lifecycle: requested, policy evaluated, approved/denied, dispatched, result received, completed. |
| `docs/milestones/v0.7/DELEGATION_POLICY_v1.md` | Historical deterministic delegation policy surface with first-match allow/deny/needs-approval semantics. |
| `adl/src/runtime_v2/delegation_subcontract.rs` | Current Runtime v2 subcontract model preserving delegated scope, parent responsibility, review requirements, trace requirements, and non-inheritance of authority. |
| `adl/src/runtime_v2/tests/delegation_subcontract.rs` | Focused tests prove parent responsibility, bounded subcontractor selection, non-authoritative tool constraints, and negative cases. |
| `adl/src/acc/types.rs` | Current ACC authority/delegation types: actor identity, authority evidence, grant status, delegation steps, role standing, capability requirement, and decisions. |
| `adl/src/acc/validation.rs` | Current ACC validation rejects model-self-reported authority, hidden delegation, misattributed delegation chains, excessive delegation depth, and disallowed redelegation. |
| `adl/src/agent_comms.rs` | Current ACIP message surface includes `delegation` intent and optional `authority_scope` with `delegation_permitted`. |
| `docs/milestones/v0.91.4/review/software_development_polis/SOFTWARE_DEVELOPMENT_POLIS_PROOF_PACKET_v0.91.4.md` | Actor standing and non-delegable authority boundaries are already tracked as bounded proof concepts. |
| `docs/milestones/v0.91.5/review/reasoning_graph/REASONING_GRAPH_CURRENT_CONTRACT_3688.md` | Reasoning graphs may reference delegation, but delegation authority requires this separate contract and cannot be invented by graph edges. |

## Contract Boundary

`UpstreamDelegationV1` is a public, reviewable delegation record. It binds an accountable delegating actor to a bounded upstream target, requested capability, authority basis, trace requirements, and parent responsibility.

It is not:

- an execution permit by itself
- a full IAM system
- a provider credential
- a model self-claim of competence or authority
- an implicit transfer of parent authority
- a replacement for ACC validation
- a replacement for human/operator review or merge authority
- an appeal or standing-transition implementation

## Target Classes

`target_class` must be one of:

- `local_agent`
- `local_service`
- `trusted_external_polis`
- `hosted_provider`
- `remote_runtime`
- `human_operator`

Every target class remains subordinate to the delegating actor's authority, policy, trace, and review boundary. A hosted frontier model or remote runtime is not a sovereign actor outside the delegating contract.

## Required Contract Shape

A public upstream delegation record that claims `UpstreamDelegationV1` must preserve these fields or validated equivalents:

```yaml
schema_version: upstream_delegation.v1
delegation_id: stable deterministic id
parent_run_ref: run-relative or repo-relative ref
delegating_actor:
  actor_id: stable accountable actor id
  actor_kind: human | agent | service | operator
  role_ref: role or standing ref
  authority_evidence_refs: non-model evidence refs
upstream_target:
  target_id: stable target id
  target_class: local_agent | local_service | trusted_external_polis | hosted_provider | remote_runtime | human_operator
  provider_or_runtime_ref: optional provider/runtime/model identity ref
request:
  capability_id: requested capability
  scope: bounded scope string
  deliverables: list of bounded deliverables
  forbidden_actions: list of actions that remain forbidden
  inherited_constraints: list of parent constraints
  trace_requirements: list of trace requirements
authority:
  acc_ref: ACC contract or decision ref
  grant_ref: authority grant or delegation record ref
  delegation_chain_refs: explicit chain refs
  redelegation_allowed: boolean
  max_depth: integer
  parent_responsibility_retained: true
  parent_review_required: true
lifecycle:
  state: requested | policy_evaluated | approved | denied | dispatched | result_received | completed | failed | revoked | blocked
  policy_decision: allowed | denied | needs_approval
  acc_decision: allowed | denied | delegated | revoked
  grant_status: active | denied | delegated | revoked
  decision_source_refs: list of policy, ACC, or grant refs
  failure_code: optional fail-closed code
output:
  delegated_output_ref: optional run/repo-relative ref
  parent_integration_ref: optional run/repo-relative ref
  reasoning_graph_ref: optional reasoning graph ref
redaction:
  private_reasoning_exposed: false
  secrets_exposed: false
```

## Required Invariants

- Delegating actor identity must be explicit and authenticated by non-model evidence.
- Model claims cannot establish authority.
- `parent_responsibility_retained` must remain true for delegated work.
- `parent_review_required` must remain true before parent integration.
- Delegated targets cannot silently inherit parent authority.
- Delegated tool or provider capability does not imply execution authority.
- Delegation chains must bind grantor, delegate, grant, and depth.
- Delegation depth must obey ACC bounds and contract-local `max_depth`.
- Redelegation must fail closed unless explicitly allowed.
- Policy decisions, ACC decisions, and grant statuses must preserve source provenance and must not be collapsed into an untraceable normalized status.
- Missing authority, ambiguous identity, unsupported target class, hidden delegation, and policy conflict must produce blocked/denied/failed states, not implicit execution.
- All public refs must be repo-relative, run-relative, or explicit external issue/PR URLs.
- No raw prompts, raw tool arguments, credentials, secrets, or absolute host paths may be emitted through this contract.

## Lifecycle Semantics

| State | Meaning | May execute? |
| --- | --- | --- |
| `requested` | Delegation intent exists, but policy has not approved it. | no |
| `policy_evaluated` | Policy, ACC, or grant status has been evaluated, but the action may still need approval. | no unless source decisions explicitly allow dispatch and the dispatch gate is satisfied |
| `approved` | Delegation is approved for dispatch under the bounded scope. | yes, only within scope |
| `denied` | Policy/ACC denied the delegation. | no |
| `dispatched` | The bounded request was sent to the upstream target. | already in progress |
| `result_received` | Upstream result exists but is not parent-integrated. | no additional authority granted |
| `completed` | Parent review/integration accepted the delegated result. | terminal |
| `failed` | Provider/runtime/tool failure prevented completion. | no |
| `revoked` | Authority was revoked before completion. | no |
| `blocked` | Missing or ambiguous authority prevented safe progress. | no |

Denied, blocked, failed, and revoked states are first-class outcomes. They must never be reported as successful delegated execution.

## Relationship To Current ADL Surfaces

| Surface | Relationship |
| --- | --- |
| ACC | ACC remains the authority boundary. `UpstreamDelegationV1` must cite ACC authority or delegation evidence and inherit ACC fail-closed constraints. |
| Runtime v2 subcontracting | Runtime v2 already models bounded subcontracting with parent responsibility retained, parent review required, non-inherited authority, delegated output, and parent integration. `UpstreamDelegationV1` generalizes that shape beyond one contract-market demo. |
| Delegation trace v1 | Historical delegation trace states provide the lifecycle vocabulary. The refreshed contract keeps deterministic, audit-friendly state progression. |
| Delegation policy v1 | First-match deterministic policy remains a valid precedent, but this contract does not claim a full policy DSL. |
| ACIP | ACIP can carry delegation intent and authority scope, but ACIP messages do not grant authority unless backed by this contract and ACC evidence. |
| Software Development Polis | Actor standing and non-delegable powers constrain which actors can delegate, review, merge, or close work. |
| Reasoning graphs | Reasoning graphs may cite `delegation_id` or delegation refs as provenance, but graph edges cannot create delegation authority. |
| Provider/review-provider lanes | Provider outputs are advisory unless mediated by this contract, ACC, and parent review. Hosted provider availability is not authority. |

## Fail-Closed Error Codes

Follow-on implementation should preserve these minimum failure classes. This list is not a replacement for ACC-native validation errors; when ACC validation runs, its source error codes must remain visible by reference or direct inclusion.

- `missing_accountable_actor_identity`
- `missing_non_model_authority_evidence`
- `model_self_reported_authority`
- `invalid_authority_evidence`
- `invalid_authority_grant`
- `allowed_requires_active_grant`
- `revoked_requires_revoked_grant`
- `unsupported_upstream_target_class`
- `hidden_delegation`
- `misattributed_delegation_chain`
- `delegation_constraints_exceeded`
- `redelegation_not_allowed`
- `policy_decision_mismatch`
- `unknown_policy_evidence_ref`
- `missing_confirmation`
- `parent_responsibility_not_retained`
- `parent_review_not_required`
- `implicit_parent_authority_inheritance`
- `policy_conflict`
- `trace_requirement_missing`
- `private_reasoning_or_secret_leakage`
- `private_state_trace_exposure`

## Follow-On Contract Requirements

`#3690` should wire trace records that can carry or reference this contract without claiming full v0.94 signed/queryable trace completion.

`#3691` should prove one small end-to-end flow where:

- a delegation request is created
- policy/authority evidence is recorded
- unsafe implicit authority paths fail closed
- a reasoning graph or proof packet cites the delegation ref as evidence
- parent responsibility and review remain explicit

## Non-Claims

This issue does not claim:

- full IAM is implemented
- v0.93 delegation governance is complete
- hosted providers can act outside the delegating actor's authority
- upstream targets can silently inherit parent authority
- reasoning graphs, provider output, or ACIP messages can grant authority by themselves
- signed/queryable trace is complete
- appeal or standing transition governance is implemented

## Validation Expectations

A reviewer should be able to verify this packet with focused checks:

- the v0.93 delegation/IAM plan exists
- v0.7 delegation trace and policy docs exist
- Runtime v2 delegation subcontract types and tests exist
- ACC delegation types and fail-closed validation paths exist
- ACIP includes delegation intent and authority scope fields
- the reasoning graph contract routes delegation authority to this separate contract
- this packet avoids implementation and authority overclaims

## Disposition

`#3689` should close only when this upstream delegation contract is reviewed and accepted as design input for:

- `#3690` trace-record implementation
- `#3691` proof/demo flow
