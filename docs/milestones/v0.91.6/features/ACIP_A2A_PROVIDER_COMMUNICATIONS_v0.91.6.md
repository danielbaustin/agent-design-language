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

## Security Handoff To The Security Bridge Lane

This issue defines the delegation/provider boundary, but it does not close the
access-control or message-security questions.

The security bridge and CAV lane in `v0.91.6` must consume this contract for
the following security decisions:

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

## ACIP/A2A Access-Rule Table

`v0.91.6` defines the first explicit access posture for ACIP/A2A message
families.

| Message family | Default access posture | Required check before acceptance | Not allowed to decide alone |
| --- | --- | --- | --- |
| ACIP envelope metadata | metadata-only, not self-authorizing | schema family, version, sender class, and route class must be valid | sender authority, citizen identity, provider trust, or institutional authority |
| Capability delegation request | deny by default until route class is known | caller class must be allowed to request the named capability and constraints | provider selection, citizen impersonation, or institution-wide authority |
| Capability delegation result | deny by default for cross-boundary replay or upgrade | result must match an allowed originating request or tracked route | creation of new authority, identity assertion, or provider trust elevation |
| A2A route request | deny by default across agent or polis boundaries | route target, sender class, and allowed boundary crossing must be explicit | provider execution authority or institutional authorization |
| A2A route response | accept only as a response to an allowed route | response must remain inside the approved route and result class | minting new access or identity privileges |
| Provider execution request | internal low-level substrate message only | selected provider/model route must already be authorized by higher layers | user-facing identity, guild membership, or citizen authority |
| Provider execution response | internal low-level substrate response only | output must be interpreted by higher-layer ACIP/A2A rules before any boundary crossing | direct policy authority or identity proof |

The default rule is fail-closed: if a message family cannot prove its allowed
route class and caller class, it is not accepted as an authoritative ACIP/A2A
message.

## Authority Boundary Table

| Surface | What it may authorize | What it must never authorize by itself |
| --- | --- | --- |
| Schema family and message version | structural parsing and compatibility | actor trust, citizen identity, or access approval |
| Capability request | what kind of work is being requested | provider trust, institution membership, or identity proof |
| Role-provider policy | which provider/model families are acceptable candidates | final authority to act across identity or polis boundaries |
| Provider profile and model selection | low-level substrate route choice after approval | citizen, guild, or institutional legitimacy |
| Guild/citizen/member policy | whether a caller class is allowed to request or receive a route class | low-level provider execution details |
| Institution or polis policy | broader organizational authority and boundary-crossing approval | provider execution semantics or model trust by itself |
| Security/CAV review | whether unresolved abuse or malformed-output risk blocks activation | protocol meaning, capability semantics, or provider identity |

This boundary is the key non-collapse rule for `v0.91.6`: provider
infrastructure may satisfy a route, but it must not become the authoritative
identity or authority layer by schema accident.

## Abuse-Case And Security Notes

The minimum abuse cases made explicit by this issue are:

- provider impersonation of citizen or institutional identity through reused
  schema fields;
- capability escalation by naming a privileged provider/model route directly
  instead of requesting an allowed capability;
- cross-boundary replay of delegation results as if they were fresh authority;
- A2A route injection where metadata is treated as self-authorizing;
- malformed or hostile provider output being mistaken for a valid authority or
  identity assertion;
- route resolution that silently upgrades advisory role-provider policy into
  direct security authority.

The required posture for these cases is explicit deny, fail-closed, or routed
to the security bridge lane for stronger message-security treatment. `v0.92`
must not consume this surface as if those abuse classes were already fully
solved.

## Routed Residuals

The remaining open security and authority residuals from this issue are:

- final message-signing or integrity-marker decisions;
- stronger provenance requirements for cross-boundary A2A traffic;
- precise malformed-output classification and test coverage for authority-shaped
  failures;
- provider/model trust findings that should block activation rather than merely
  warn;
- any institutional or citizen identity surface that needs a first-class typed
  contract beyond the bounded policy tables recorded here.

These residuals route to the `v0.91.6` security bridge lane and, where
transport or projection shape is still unsettled, to the later WP-06
transport/closeout issues rather than being silently absorbed into `v0.92`.

## JSON / Protobuf / WebSocket Decision Table

`v0.91.6` records the first projection/transport posture and the required
downstream ownership as follows:

| Surface | `v0.91.6` decision | Status | Owner / next lane | Rationale |
| --- | --- | --- | --- | --- |
| Deterministic JSON projection | current documented baseline for message shape; final downstream consumption stays explicitly owned later | documented_baseline_only | `v0.91.7` residual clarification and `v0.92` projection/carrier work | JSON is the only projection this milestone can describe without inventing unproven transport, signing, fixture, or runtime claims, but downstream implementation/proof ownership remains tracked later. |
| Protobuf wire format | not required for `v0.91.6`; explicit residual for later decision | deferred | `v0.91.7` ACIP/A2A protobuf residuals | Protobuf must not ship without schema-version, compatibility, and security ownership that this milestone does not yet prove. |
| WebSocket transport | not part of the reviewed `v0.91.6` activation baseline | deferred | `v0.91.7` residual clarification and `v0.92` transport work | WebSocket remains transport productization until access, integrity, carrier, and projection rules are explicit enough to consume safely. |
| Mock or internal carrier posture | may exist only as bounded internal/testing carriage; not a settled protocol decision here | documented_non_claim | `v0.92` carrier/fixture work | Internal carriage may exist, but `v0.91.6` does not claim that mock or loopback transport has already been accepted as the protocol route. |
| Cross-boundary external-agent transport | blocked from implied approval in `v0.91.6` | rejected_for_now | later ACIP/A2A residual and security lanes | Activation cannot infer external-agent trust or transport closure from schema-only decisions. |

## Projection Boundary Decision Notes

- Deterministic JSON is the only projection this milestone can describe as the
  current documented message-shape baseline.
- That baseline does not take away later tracked ownership for projection,
  carrier, fixture, or runtime proof work in `v0.91.7` and `v0.92`.
- Protobuf is neither silently approved nor vaguely “future”; it is explicitly
  routed to the tracked `v0.91.7` residual surface.
- WebSocket is not rejected forever, but it is outside the `v0.91.6` reviewed
  communication baseline and must remain a residual until transport and
  security ownership are stronger.

## Versioning And Compatibility Posture

- Every projection must preserve the schema family, message kind, and version
  identifier already required by the earlier schema-catalog decision.
- JSON projection is required to remain deterministic for identical message
  content and ordering assumptions.
- Protobuf cannot be accepted until field numbering, compatibility posture, and
  downgrade/upgrade rules are explicitly tracked.
- WebSocket cannot be accepted until it is clear whether it carries JSON,
  protobuf, or another bounded framed payload, and what integrity/access rules
  apply at that boundary.

## Validation And Projection Expectations

- `v0.91.6` may claim schema/documentation truth for the current JSON-shaped
  message baseline only.
- No claim in this milestone should imply that protobuf serialization,
  WebSocket carriage, or external-agent interop has already been runtime-proven.
- No claim in this milestone should imply that later tracked `v0.91.7` or
  `v0.92` ownership for projection/carrier proof has been silently collapsed
  into `v0.91.6`.
- Any later acceptance of protobuf or WebSocket must also consume the access
  and authority boundaries from `#4015` and the security-bridge residuals
  already routed above.

## ADR Candidate Routing

No transport ADR is accepted in `v0.91.6` from this issue alone.

If a later tranche accepts protobuf, WebSocket, or a concrete carrier contract,
that acceptance should be captured as an ADR candidate then. The current
decision is intentionally a bounded “JSON-shaped baseline now, protobuf and
WebSocket deferred, downstream ownership preserved” record rather than a final
transport architecture claim.

## Dependencies

- Security bridge and CAV feature doc.
- Constructability Gate residual in `v0.91.7`.
- Existing ACIP and WebSocket planning notes.
- WP-07 access-rule security review packet:
  `docs/milestones/v0.91.6/review/security/ACIP_A2A_ACCESS_RULE_SECURITY_REVIEW_4021.md`

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
