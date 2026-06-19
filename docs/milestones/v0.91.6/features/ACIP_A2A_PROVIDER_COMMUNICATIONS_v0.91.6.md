# ACIP, A2A, And Provider Communications

## Metadata

- Feature Name: ACIP, A2A, And Provider Communications
- Milestone Target: `v0.91.6`
- Status: in_progress
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy, schema
- Proof Modes: schema, review

## Purpose

Define first-tranche communication boundaries for ACIP, A2A, and provider
messages before `v0.92` consumes agent-communication surfaces.

## Scope

In scope:

- schema catalog and message families;
- access rules and authority boundaries;
- external-agent posture;
- provider-message boundary;
- WebSocket support posture;
- deterministic JSON projection;
- protobuf decision point.

Out of scope:

- full protocol implementation;
- broad transport productization;
- residual protobuf/wire-format closure owned by `v0.91.7`.

## Required Decisions

- Which schemas are canonical in `v0.91.6`?
- Which messages can cross provider or polis boundaries?
- Is protobuf required before `v0.92`, or is JSON projection sufficient?
- Which access-control failures are terminal?

## Canonical Schema Catalog For `v0.91.6`

The first ACIP boundary decision in `v0.91.6` is a schema-catalog decision,
not a transport implementation claim.

| Schema family | Role in `v0.91.6` | Canonical owner | May cross provider boundary? | May cross polis or external-agent boundary? | Notes |
| --- | --- | --- | --- | --- | --- |
| ACIP envelope metadata | Top-level message framing, schema family, version, sender class, and routing posture | ACIP contract layer | No | Yes, only after later access-rule review | This is the minimal catalog entry every later message family must inherit. |
| ACIP capability delegation request | Requests work by named capability, constraints, and required proof posture | ACIP delegation layer | No | Not yet approved by this issue | Capability routing must stay distinct from provider/model selection. |
| ACIP capability delegation result | Returns outcome, refusal, failure class, and proof handles | ACIP delegation layer | No | Not yet approved by this issue | Output posture is capability-shaped, not provider-shaped. |
| A2A inter-agent route message | Agent-to-agent request/response contract above provider transport | A2A route layer | No | Deferred to the explicit A2A/access-rule issues | A2A remains a policy/documentation boundary in `v0.91.6`. |
| Provider execution request | Low-level provider invocation payload sent to model/runtime substrates | Provider substrate layer | Yes, internally | No | Provider messages are infrastructure records, not citizen or institution records. |
| Provider execution response | Low-level provider completion/result payload | Provider substrate layer | Yes, internally | No | Provider output stays behind the provider boundary unless explicitly projected upward. |
| Capability profile record | Provider-independent description of what a lane or role needs | Capability profile layer | No | Yes, as reviewed policy metadata | Capability profiles may inform delegation and routing without carrying vendor/endpoint state. |
| Provider profile record | Deterministic infrastructure descriptor for service family, endpoint, and model defaults | Provider profile layer | No | No | Provider profiles remain low-level substrate descriptors. |
| Role-provider policy record | Higher-level C-SDLC routing policy that references provider and capability layers | C-SDLC policy layer | No | No | This is advisory policy, not direct transport authority. |
| Identity, citizen, institution, or guild records | Civil/governance identity surfaces | Identity/governance layers | No | Not part of this lane | These remain explicitly out of provider/profile transport scope. |

## Layer Boundary Table

| Layer | Canonical responsibility | Must not be collapsed into |
| --- | --- | --- |
| Provider profile | Infrastructure-backed service identity, endpoint family, model default, and substrate validation | capability semantics, actor identity, or role authority |
| Capability profile | Provider-independent statement of required behavior or ability | vendor, endpoint, credential, or transport identity |
| Role-provider policy | C-SDLC routing preference or suitability policy referencing provider/capability layers | autonomous provider authority or civil identity |
| ACIP message layer | Capability-shaped communication semantics and versioned message families | raw provider transport payloads |
| A2A message layer | Agent-to-agent protocol posture and cross-agent route semantics | provider execution metadata or identity registries |
| Identity/citizen/institution/guild layer | Civil, institutional, continuity, or governance records | provider profiles, capability policy, or runtime routing metadata |

## Message Ownership And Versioning Notes

- `v0.91.6` adopts deterministic JSON as the only canonical projection proved by
  this issue.
- Each message family must declare a schema family name, message kind, and
  version identifier before later transport or wire-format work is accepted.
- Provider execution payloads may reference provider-profile identifiers and
  model names, but they must not become the canonical identity of a capability
  request.
- Capability-facing ACIP or A2A records may reference provider suitability or
  role-provider policy, but they must not inline provider endpoint contracts as
  if those were the user-facing protocol.
- The historical authored input path `.adl/docs/TBD/ADL_PROFILES_PROVIDERS_V2.MD`
  is not present in the tracked repository state. For provider/profile boundary
  truth, `v0.91.6` now depends on the merged WP-05 provider artifacts and the
  reconciliation packet for `#4111`, not on the missing TBD file.

## `v0.92` Consumption Rules

- `v0.92` may consume the schema catalog and boundary vocabulary from this file
  as the required baseline for later ACIP/A2A decisions.
- `v0.92` may not assume protobuf, WebSocket transport, or cross-boundary A2A
  execution authority is settled by this issue.
- `v0.92` must continue to route delegation requests by capability semantics
  first and only resolve provider/model choices through bounded lower layers.
- `v0.92` must preserve the separation between provider profiles, capability
  profiles, role-provider policy, and civil identity surfaces.

## Dependencies

- Security bridge and CAV feature doc.
- Constructability Gate residual in `v0.91.7`.
- Existing ACIP and WebSocket planning notes.

## Validation And Review

- Review schema catalog for access and privacy boundaries.
- Validate JSON projection determinism before implementation claims.
- Route protobuf residuals explicitly to `v0.91.7` if unresolved.

## v0.92 Consumption

`v0.92` may consume only the reviewed communication posture. If protobuf,
WebSocket, or access rules remain unresolved, they must be blocked, deferred,
or routed before activation.

## Non-Goals

- No protocol completion claim.
- No implicit external-agent trust.
- No unreviewed provider-message exposure.
