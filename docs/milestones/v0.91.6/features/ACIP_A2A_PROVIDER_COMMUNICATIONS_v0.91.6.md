# ACIP, A2A, And Provider Communications

## Metadata

- Feature Name: ACIP, A2A, And Provider Communications
- Milestone Target: `v0.91.6`
- Status: complete
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
  is no longer the authoritative source for this issue's provider/profile
  boundary truth. `v0.91.6` now depends on the merged WP-05 provider artifacts
  and the reconciliation packet for `#4111` rather than on that earlier authored
  input path.

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

## External-Agent / Citizen / Guild Routing Posture

`v0.91.6` records the routing posture for external agents, citizens, guilds,
providers, and capability-market concepts without claiming the full social or
economic runtime already exists.

| Surface | `v0.91.6` posture | Must remain distinct from |
| --- | --- | --- |
| External agent | a potential participant class that may later originate or receive allowed ACIP/A2A routes | provider profile, model identity, or institutional authority |
| Citizen | a higher-layer identity/governance record that may later constrain what routes are legitimate | provider substrate, capability record, or guild policy by itself |
| Guild | an MVP-scope policy/grouping input that may influence route classes, capability discovery, and membership-based restrictions | transport substrate, provider identity, or direct runtime execution authority |
| Capability market | a planned routing/economic concept above capability and role-provider policy | current `v0.91.6` runtime behavior or low-level provider transport |
| Provider communication | infrastructure-level invocation and substrate response path | citizen identity, guild membership, or capability-market semantics |

## MVP Versus Post-MVP Boundary

The MVP boundary for this lane is:

- guilds may be represented as an input to capability/routing policy;
- citizens and institutions remain distinct higher-layer identity surfaces;
- capability-market concepts may be planned and named;
- provider communication remains low-level substrate infrastructure;
- no claim is made that market negotiation, citizen governance, or guild-state
  runtime exists yet.

Post-MVP or later-lane work includes:

- first-class capability-market runtime behavior;
- citizen/institution/guild persistent state and governance flows;
- external-agent onboarding or trust protocols;
- economic selection or bidding logic above capability routing;
- autonomous multi-party route negotiation.

## Routing Layer Dependencies

The routing dependency order after this issue is:

1. capability semantics remain primary
2. access and authority boundaries remain fail-closed
3. guild or citizen policy may narrow the route class
4. role-provider policy may narrow acceptable provider/model families
5. provider substrate selection happens only after the higher-layer checks pass

This preserves the non-collapse rule: neither provider infrastructure nor
market-style planning should become a disguised identity or authority layer.

## Capability-Market Non-Claims

- `v0.91.6` does not implement a live capability market.
- `v0.91.6` does not claim pricing, bidding, auction, or settlement semantics.
- `v0.91.6` does not claim guild membership is already a runtime-enforced
  first-class object.
- `v0.91.6` does not claim external agents are trusted by default.

## `v0.91.7` / `v0.92` Handoff Notes

- `v0.91.7` may refine residual routing boundaries where protobuf, WebSocket,
  or security ownership affects external-agent posture.
- `v0.92` may consume this posture as a planning and activation-bridge input,
  but must not treat capability-market or citizen/guild runtime as already
  implemented.
- Any future identity/capability selector work must preserve the separation
  between:
  - citizen/institution identity;
  - guild or role policy;
  - capability need;
  - provider/model substrate choice.

## Agent Comms 1.0 Message Substrate Boundary

The first-tranche Agent Comms boundary in `v0.91.6` is message-first.

Messages are the communication primitive.
Prompts, cards, STP/SIP/SPP/SRP/SOR artifacts, contracts, evidence packets,
and review records are payloads or references carried by messages, not the
communication substrate itself.

Invocation is a specialized message pattern, not a separate first-class
protocol universe.

## Core Message Substrate Fields

An Agent Comms 1.0 message record should preserve these conceptual fields:

- message id
- conversation id
- causal parent or source message id when applicable
- sender class
- recipient class
- time/order metadata
- intent
- visibility
- content body
- payload refs
- artifact refs
- memory refs
- authority scope
- trace posture
- attachments

This issue records the boundary and vocabulary only. It does not claim the full
typed runtime schema already exists.

## Interaction Mode Vocabulary

Agent Comms 1.0 recognizes these interaction modes as the bounded first-tranche
vocabulary:

- conversation
- consultation
- invocation
- review
- delegation
- negotiation
- handoff
- broadcast

These modes describe message purpose and expected handling posture. They do not
grant authority by themselves.

## Invocation Contract Boundary

Invocation is the special-case message form used when one party asks another to
perform a bounded task.

An invocation-shaped message should preserve the following contract elements:

- causal message or triggering request
- caller
- target
- purpose
- constraints
- expected outputs
- stop policy
- authority scope
- policy refs
- response channel
- trace requirement

Messages alone do not create execution legitimacy. Invocation authority must be
backed by policy, standing, contract, or Freedom Gate approval.

## Artifact-As-Payload Rule

Structured C-SDLC artifacts remain governed payloads or references within the
message substrate:

- SIP
- STP
- SPP
- SRP
- SOR
- proof packets
- contracts
- review artifacts
- evidence bundles

This means ADL must not mistake “the card exists” for “the communication layer
is the card.” The substrate stays message-first even when the payload is a
governed record.

## Evidence / Trace / Redaction Posture

Agent Comms 1.0 messages may carry or reference evidence, traces, and governed
artifacts, but the communication layer must preserve:

- visibility boundaries
- authority boundaries
- trace posture declarations
- payload-vs-reference distinction
- redaction posture when sensitive evidence is referenced instead of copied

The substrate therefore feeds the existing evidence and observability model
without collapsing into raw trace dumps or uncontrolled prompt payloads.

## Specialization Map

The general message substrate supports specialized lanes without turning each
lane into a distinct protocol:

| Lane | Specialization posture on top of Agent Comms 1.0 |
| --- | --- |
| Review agent | review-mode messages and evidence-bound findings/dispositions |
| Coding agent | invocation plus bounded artifact/output expectations |
| Delegation / handoff | authority-scoped reassignment or continuation messages |
| Demo / operator | operator-visible messages with proof, control, or replay posture |
| Multi-agent negotiation | negotiation/broadcast/conversation patterns above the same substrate |

## Substrate Non-Claims

- `v0.91.6` does not implement live provider transport through Agent Comms 1.0.
- `v0.91.6` does not define encrypted transport as a v1 requirement.
- `v0.91.6` does not define cross-polis federation.
- `v0.91.6` does not define reputation or karma systems.
- `v0.91.6` does not grant autonomous authority through messages alone.
- `v0.91.6` does not replace governed tools, UTS, ACC, Freedom Gate, or the
  tracked lifecycle records.

## Agent Comms Downstream Fit

This substrate boundary feeds the rest of WP-06 cleanly:

- schema catalog: messages become the common carrier shape
- A2A posture: interaction modes and invocation fit above transport
- provider-message boundary: provider invocations remain low-level substrate
  messages under higher-layer policy
- WebSocket/protobuf decisions: transport and encoding remain downstream of the
  message substrate, not the other way around
- closeout: `#4018` can verify whether Agent Comms 1.0 is accepted as the
  first-tranche communication boundary or explicitly routed onward

## Dependencies

- Security bridge and CAV feature doc.
- Constructability Gate residual in `v0.91.7`.
- Existing ACIP and WebSocket planning notes.
- The `v0.91.6` security bridge lane and its future access-rule security review
  outputs.

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

## WP-06 Protocol Decision Closeout Package

This section records the final WP-06 closeout package for umbrella `#3971`.

The completed child surfaces for this mini-sprint are:

| Issue | Scope closed in `v0.91.6` | Final status |
| --- | --- | --- |
| `#4013` | schema catalog and profile/layer boundaries | closed |
| `#4014` | capability-first delegation and provider-message boundary | closed |
| `#4015` | access rules and authority boundaries | closed |
| `#4016` | JSON/protobuf/WebSocket projection posture | closed |
| `#4017` | external-agent, citizen, guild, and capability-market routing posture | closed |
| `#4055` | Agent Comms 1.0 message substrate boundary | closed |

## Final `v0.91.6` Decision Summary

The first-tranche ACIP/A2A/provider communication decisions accepted by
`v0.91.6` are:

- communication is message-first; invocation is a specialized message pattern;
- capability semantics remain above provider/model substrate selection;
- provider, capability, role-policy, and civil identity layers remain distinct;
- access and authority posture is fail-closed until allowed route and caller
  class are explicit;
- deterministic JSON is the only documented message-shape baseline this
  milestone accepts for downstream consumption;
- Agent Comms 1.0 is accepted as the first-tranche substrate boundary for
  messages, payload refs, invocation vocabulary, and specialization posture.

## What `v0.91.6` Does Not Claim

This closeout must remain explicit about the non-claims preserved by the child
issues:

- agents are not yet proven to communicate over a live production protocol;
- no end-to-end inter-agent runtime or transport stack is claimed here;
- protobuf is not accepted in this milestone;
- WebSocket is not accepted in this milestone;
- cross-boundary external-agent transport is not approved in this milestone;
- identity/governance runtime is not implemented by these communication docs.

## ADR Candidate Routing

No ADR is accepted directly by WP-06 in `v0.91.6`.

The current decisions are sufficient as a reviewed boundary package, but later
acceptance of:

- protobuf wire format,
- WebSocket carriage,
- concrete carrier contracts,
- live cross-agent runtime transport,

should be captured as later ADR candidates when those choices are actually
implemented and proved.

## Validation And Review Transcript Links

The closeout evidence for this mini-sprint is carried by the child issue
threads plus their issue-local review/output records:

| Issue | GitHub issue thread | Review transcript | Output / validation transcript |
| --- | --- | --- | --- |
| `#4013` | `https://github.com/danielbaustin/agent-design-language/issues/4013` | `.adl/v0.91.6/tasks/issue-4013__v0-91-6-wp-06-acip-c-00-define-communication-schema-catalog-and-profile-boundaries/srp.md` | `.adl/v0.91.6/tasks/issue-4013__v0-91-6-wp-06-acip-c-00-define-communication-schema-catalog-and-profile-boundaries/sor.md` |
| `#4014` | `https://github.com/danielbaustin/agent-design-language/issues/4014` | `.adl/v0.91.6/tasks/issue-4014__v0-91-6-wp-06-acip-c-01-define-capability-based-delegation-and-provider-message-boundary/srp.md` | `.adl/v0.91.6/tasks/issue-4014__v0-91-6-wp-06-acip-c-01-define-capability-based-delegation-and-provider-message-boundary/sor.md` |
| `#4015` | `https://github.com/danielbaustin/agent-design-language/issues/4015` | `.adl/v0.91.6/tasks/issue-4015__v0-91-6-wp-06-acip-c-02-define-acip-a2a-access-rules-and-authority-boundaries/srp.md` | `.adl/v0.91.6/tasks/issue-4015__v0-91-6-wp-06-acip-c-02-define-acip-a2a-access-rules-and-authority-boundaries/sor.md` |
| `#4016` | `https://github.com/danielbaustin/agent-design-language/issues/4016` | `.adl/v0.91.6/tasks/issue-4016__v0-91-6-wp-06-acip-c-03-decide-json-protobuf-and-websocket-projection-boundaries/srp.md` | `.adl/v0.91.6/tasks/issue-4016__v0-91-6-wp-06-acip-c-03-decide-json-protobuf-and-websocket-projection-boundaries/sor.md` |
| `#4017` | `https://github.com/danielbaustin/agent-design-language/issues/4017` | `.adl/v0.91.6/tasks/issue-4017__v0-91-6-wp-06-acip-c-04-define-external-agent-citizen-guild-and-capability-market-routing/srp.md` | `.adl/v0.91.6/tasks/issue-4017__v0-91-6-wp-06-acip-c-04-define-external-agent-citizen-guild-and-capability-market-routing/sor.md` |
| `#4055` | `https://github.com/danielbaustin/agent-design-language/issues/4055` | `.adl/v0.91.6/tasks/issue-4055__wp06-agent-comms-1-0-message-substrate/srp.md` | `.adl/v0.91.6/tasks/issue-4055__wp06-agent-comms-1-0-message-substrate/sor.md` |

WP-06 consumes those child records rather than inventing a second umbrella-only
validation transcript.

The final `#4018` proof surface used to complete this closeout was:

- issue thread: `https://github.com/danielbaustin/agent-design-language/issues/4018`
- review transcript: `.adl/v0.91.6/tasks/issue-4018__v0-91-6-wp-06-acip-c-05-complete-protocol-decision-closeout-proof/srp.md`
- output/validation transcript: `.adl/v0.91.6/tasks/issue-4018__v0-91-6-wp-06-acip-c-05-complete-protocol-decision-closeout-proof/sor.md`

## Residuals And Owners

The remaining protocol residuals after WP-06 are:

| Residual | Owner |
| --- | --- |
| protobuf schema/version/security acceptance | `v0.91.7` ACIP/A2A residual lane |
| WebSocket carriage and integrity/access closure | `v0.91.7` residual clarification and `v0.92` transport work |
| stronger message integrity/provenance and malformed-output security closure | `v0.91.6` security bridge lane and later activation proof |
| live agent-to-agent runtime transport proof | later `v0.92` implementation/proof work |
| external-agent trust/onboarding/runtime posture | later ACIP/A2A residual and security lanes |

## `v0.92` Consumption Gate

`v0.92` may consume WP-06 as a complete communication-semantics and boundary
package.

`v0.92` must not consume WP-06 as proof that:

- live inter-agent transport is already working;
- protobuf or WebSocket is accepted;
- external agents are trusted by default;
- authority can be inferred from messages alone.
