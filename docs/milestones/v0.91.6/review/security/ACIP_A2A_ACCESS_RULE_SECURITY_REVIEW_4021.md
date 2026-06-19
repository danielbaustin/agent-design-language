# ACIP, A2A, And Access-Rule Security Review for #4021

## Scope

This packet records the bounded WP-07 security review for the ACIP/A2A access
rules, authority boundaries, and transport-decision posture consumed on the
`v0.91.6` activation path.

It is not a full protocol implementation, not a completed threat model for all
external-agent communications, and not approval to let provider or transport
surfaces mint authority on their own.

## Source evidence

- `docs/milestones/v0.91.6/features/ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md`
- `docs/milestones/v0.91.6/features/SECURITY_BRIDGE_AND_CAV_v0.91.6.md`
- `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_ACTIVE_SPRINT_EXECUTION_PACKETS_2026-06-18.md`
- `docs/milestones/v0.91.5/review/upstream_delegation/UPSTREAM_DELEGATION_CONTRACT_3689.md`
- `docs/milestones/v0.91.5/review/reasoning_graph_upstream_delegation/REASONING_GRAPH_UPSTREAM_DELEGATION_PROOF_3691.md`

## Review goal

Determine whether the current ACIP/A2A contract is safe to consume under
explicit access and authority boundaries, and record which unresolved protocol
or security decisions must remain routed instead of being upgraded into implied
message trust.

The WP-06 child-issue intent consumed here is the currently tracked tranche:

- `#4013` communication schema catalog/profile boundaries
- `#4014` capability-based delegation/provider-message boundary
- `#4015` ACIP/A2A access rules and authority boundaries
- `#4016` JSON/protobuf/WebSocket projection boundaries
- `#4017` external-agent/citizen/guild/capability-market routing
- `#4018` protocol decision closeout proof

Those child owners are used as routing truth in this packet even though this
security review is anchored to the live feature and sprint surfaces rather than
to each issue-body path directly.

## Access and authority rules established here

1. ACIP/A2A message structure is not self-authorizing; schema and version
   validity do not prove caller authority.
2. Capability requests, provider profiles, role-provider policy, and civil
   identity surfaces must remain distinct layers.
3. Provider infrastructure may satisfy a route, but it must not impersonate an
   identity-bearing participant by schema accident.
4. Cross-boundary delegation results, A2A route messages, and transport
   projections must fail closed when caller class, route class, or boundary
   crossing is not explicit.
5. JSON projection remains the only deterministic canonical projection consumed
   here; unresolved protobuf, WebSocket, signing, and provenance questions stay
   routed rather than implied complete.

## Boundary matrix

| Boundary | Why it matters | Current evidence | Current disposition |
| --- | --- | --- | --- |
| Schema family vs authority boundary | A valid message shape can be mistaken for permission if version/schema checks are allowed to carry authority. | The ACIP feature doc makes envelope metadata structural and explicitly non-self-authorizing; `#4013` and `#4015` prompts require schema and authority separation. | `reviewed_and_currently_covered` |
| Capability request vs provider route boundary | Capability requests must not collapse into raw provider or model route choices. | The feature doc and `#4014` keep delegation capability-first and treat provider/model selection as a lower layer. | `reviewed_and_currently_covered` |
| Provider substrate vs identity-bearing participant boundary | Provider execution messages are infrastructure records, not citizen, guild, institutional, or polis identity records. | The feature doc's schema catalog and authority tables explicitly forbid provider execution surfaces from acting as civil identity or policy authority. | `reviewed_and_currently_covered` |
| Cross-boundary replay / result reuse boundary | Delegation results or A2A responses can be replayed as fresh authority unless tied to an approved route. | The feature doc's access-rule table denies default acceptance and requires route/caller checks before responses cross boundaries. | `reviewed_and_currently_covered` |
| A2A route injection / metadata-as-authority boundary | Metadata-only route messages can be used as self-authorizing tickets if route class and sender class are left implicit. | The feature doc names A2A route injection as a minimum abuse case and requires explicit deny/fail-closed posture. | `reviewed_and_currently_covered` |
| Malformed-output / hostile-authority boundary | Hostile or malformed provider output can be mistaken for a valid authority or identity assertion if higher layers treat it as protocol truth. | The feature doc names malformed or hostile provider output as an abuse case, but detailed authority-shaped malformed-output classification remains open. | `reviewed_and_routed` |
| JSON/protobuf/WebSocket transport-security boundary | Wire format or transport decisions can imply trust, compatibility, or integrity guarantees that do not yet exist. | The feature doc accepts deterministic JSON only, while protobuf and WebSocket remain decision surfaces with explicit deferral/routing language. | `reviewed_and_routed` |
| External-agent / citizen / guild routing boundary | Planning toward external agents, citizens, guilds, and capability markets can accidentally overgrant runtime or policy authority. | The `#4017` prompt and feature doc keep those concepts as routed posture and MVP-vs-post-MVP planning, not v0.91.6 authority-bearing runtime truth. | `reviewed_and_routed` |

## Protocol abuse-case checklist

The minimum security-significant abuse cases for this lane are:

- provider impersonation of citizen, guild, institutional, or polis identity
  through reused schema fields
- capability escalation by naming a privileged provider/model route directly
  instead of requesting an allowed capability
- cross-boundary replay of delegation results as if they were fresh authority
- A2A route injection where structural metadata is treated as self-authorizing
- malformed or hostile provider output being mistaken for authority-bearing
  protocol truth
- advisory role-provider policy being upgraded into direct security authority
- transport or projection choices implying signing, integrity, or compatibility
  guarantees that have not actually been reviewed

## JSON / protobuf / WebSocket security notes

### JSON

Current reviewed truth:

- deterministic JSON is the only canonical projection explicitly consumed by
  the current ACIP feature contract
- JSON projection may be consumed only with the access and authority rules
  above; deterministic shape alone is not a trust mechanism

### Protobuf

Current reviewed truth:

- protobuf is not implicitly approved by the current communication feature doc
- protobuf must remain decided, explicitly deferred, or rejected with owner and
  rationale before later protocol consumers can treat it as settled
- shipping protobuf without security/version decisions remains an explicit
  non-goal in the WP-06 source prompts

### WebSocket

Current reviewed truth:

- WebSocket posture is a decision boundary, not an approved trust surface
- WebSocket transport may not be treated as cross-boundary authority or
  integrity proof by default
- any eventual WebSocket activation must consume the same access, provenance,
  and residual-routing rules recorded here

## Findings and dispositions

1. The current ACIP/A2A contract is explicit enough to keep message structure,
   provider substrate, and advisory policy from being documented as the
   authority layer by default, but this packet does not prove runtime
   enforcement of that boundary.  
   Disposition: fixed as reviewed contract posture by the existing ACIP feature
   contract and this security review packet; enforcement-sensitive follow-ons
   remain routed.

2. Authority-shaped malformed-output handling is identified, but the contract
   does not yet provide detailed classification or test coverage for every
   hostile-output case.  
   Disposition: routed to WP-07 `#4064` and retained under final security
   closeout.

3. Cross-boundary signing, integrity markers, and stronger provenance rules for
   A2A traffic remain open security decisions.  
   Disposition: routed first to WP-06 `#4016` as the substantive
   transport/projection decision owner, then to WP-06 closeout `#4018` and
   WP-07 closeout `#4024` as the consuming closeout surfaces rather than
   implied complete here.

4. Protobuf and WebSocket remain bounded decision surfaces rather than approved
   trust substrates.  
   Disposition: routed to `#4016` and, if still unresolved at milestone
   closeout, to the `v0.91.7` residual guard.

5. Historical source-input drift remains visible because several WP-06 issue
   prompts still cite the missing local path
   `.adl/docs/TBD/ADL_PROFILES_PROVIDERS_V2.MD`.  
   Disposition: do not trust that path as source authority; consume the live
   ACIP feature doc and the reconciled WP-05 provider packet set instead, and
   keep the stale-input problem as remediation residue.

## Consumption rule for v0.91.6 and v0.92

Current decision:

- `acip_a2a_access_rule_security_reviewed_with_explicit_residual_routes`

That means later milestone work may consume:

- explicit access-rule and authority-boundary tables
- capability-first delegation and provider-substrate separation
- fail-closed A2A and delegation posture when route/caller class is not proven
- deterministic JSON as the only currently consumed canonical projection

It may not consume this issue as proof of:

- final message-signing or provenance-marker decisions
- approved protobuf or WebSocket adoption
- external-agent, guild, citizen, or institution runtime authority
- provider or transport layers gaining identity-bearing authority by default
- enforced runtime rejection for every authority-boundary violation discussed in
  this review packet

## Residual routing

- Message-signing, integrity-marker, and stronger provenance decisions route to
  WP-06 `#4016` as the transport/projection decision owner; they must then stay
  visible in WP-06 closeout `#4018` and WP-07 closeout `#4024` until resolved
  or explicitly deferred.
- Authority-shaped malformed-output and threat-taxonomy residuals route to
  WP-07 `#4064`.
- Protobuf/WebSocket adoption or deferral truth remains owned by `#4016`; if
  unresolved at milestone closeout, it must route to the `v0.91.7` residual
  guard.
- External-agent, citizen, guild, and capability-market authority residuals
  remain owned by `#4017` and may not be silently promoted from planning
  posture into runtime trust.

## Reviewer takeaway

`#4021` is ready when reviewers can confirm that:

- ACIP/A2A access rules are explicit and deny by default
- provider infrastructure cannot impersonate identity-bearing participants by
  schema accident
- transport-decision surfaces do not overclaim trust or compatibility
- unresolved signing, malformed-output, projection, and external-agent residuals
  are routed to named open owners instead of being implied complete
