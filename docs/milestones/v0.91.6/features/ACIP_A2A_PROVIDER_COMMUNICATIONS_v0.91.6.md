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

## Capability-Based Delegation Contract

Delegation requests in `v0.91.6` are capability-first records.

The canonical delegation request must be expressible in terms of:

- requested capability or capability family;
- optional role posture such as guild, citizen, member, or workflow role;
- proof, safety, latency, cost, or tool-use constraints;
- allowed candidate classes;
- explicit refusal or escalation posture when no safe candidate exists.

The canonical delegation request must not require:

- a provider name as the primary routing key;
- a model identifier as the primary routing key;
- an endpoint or credential surface;
- a collapsed identity record that mixes capability needs with provider or
  citizen identity.

Provider names, model ids, and endpoint families remain implementation
candidates selected by lower layers after the capability request has been
interpreted.

## Provider-Message Boundary Rules

Provider-message records stay below the capability delegation contract.

| Surface | Allowed to express | Must not express |
| --- | --- | --- |
| Capability delegation request | capability need, constraints, allowed candidate classes, refusal posture | raw provider endpoint contract, credential material, or provider-specific transport framing as the canonical request |
| Role-provider policy | which provider or model families are suitable candidates for a role/capability posture | direct replacement for capability semantics or citizen/guild identity |
| Provider execution request | selected provider profile, model/profile expansion, low-level invocation payload, transport-specific request details | the authoritative meaning of the user-facing capability request |
| Provider execution response | low-level completion/result/error payload from the selected substrate | the final policy meaning of a delegation result without ACIP-layer interpretation |

This means a provider or model can satisfy a delegation request, but it is not
the meaning of the request.

## Candidate Discovery Ordering

When resolving a delegation request, `v0.91.6` uses this discovery posture:

1. interpret the requested capability and constraints
2. determine the allowed candidate classes for that request
3. evaluate whether guild, citizen, member, or workflow-role policy narrows the
   candidate set
4. consult capability profiles and role-provider policy before selecting any
   concrete provider or model candidate
5. resolve provider-profile and model candidates only after the higher-layer
   capability and policy checks pass
6. fail closed when no candidate satisfies the capability and security posture

This ordering keeps capability semantics stable even if the available provider
or model catalog changes later.

## Delegation Candidate Classes

`v0.91.6` recognizes distinct candidate classes for delegation routing:

- capability-policy candidates: provider-independent descriptions of what kind
  of behavior is needed
- role-provider candidates: higher-level routing policies that can narrow which
  provider families or model families are acceptable
- provider-profile candidates: infrastructure-backed service families and
  transport descriptors
- model candidates: descriptive model identities under a selected provider or
  profile family
- guild/citizen/member or institutional candidates: higher-layer identity or
  governance concepts that may authorize or forbid a route, but do not replace
  provider or capability records

These classes must remain distinguishable in tracked docs and later runtime
artifacts. `v0.92` must not flatten them into one “best provider” field.

## Security Handoff To WP-07

This issue defines the delegation/provider boundary, but it does not close the
access-control or message-security questions.

WP-07 must consume this contract for the following security decisions:

- which delegation or A2A messages require access checks before route
  resolution;
- whether any message families require signing, provenance markers, or stronger
  integrity proof;
- which provider/model failures are security-relevant rather than ordinary
  routing failures;
- how malformed-output, prompt-injection, or cross-boundary message abuse is
  classified for ACIP/A2A traffic;
- which unresolved security residuals block `v0.92`.

Until WP-07 resolves those questions, `v0.92` may use capability-first
delegation vocabulary but may not assume final access or message-security
closure.

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
