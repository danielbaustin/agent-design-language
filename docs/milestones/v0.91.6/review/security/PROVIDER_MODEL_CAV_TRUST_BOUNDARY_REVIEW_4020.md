# Provider, Model, And CAV Trust-Boundary Review for #4020

## Scope

This packet records the bounded WP-07 security review for provider/model/CAV
trust boundaries consumed on the `v0.91.6` activation path.

It is not a full-repository threat model, not proof of completed Continuous
Adversarial Verification operations, and not an approval to widen provider
authority beyond the documented workflow boundaries.

## Source evidence

- `docs/milestones/v0.91.6/features/SECURITY_BRIDGE_AND_CAV_v0.91.6.md`
- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_ROLE_SUITABILITY_MATRIX_4008.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_FAILURE_MODE_INTEGRATION_4010.md`
- `docs/milestones/v0.91.6/review/provider/PRIVATE_ENDPOINT_FIXTURE_SANITATION_4011.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_RELIABILITY_CLOSEOUT_MATRIX_4012.md`
- `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_SECURITY_CAV_HANDOFF_4005.md`
- `docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md`
- `docs/security/THREAT_MODEL_v0.7.md`
- `adl/src/provider_communication.rs`
- `adl/src/resilience.rs`

## Review goal

Determine whether the currently documented provider/model reliability surfaces
are safe to consume under explicit trust boundaries, and record which residuals
must stay routed to later WP-07 lanes instead of being upgraded into implicit
security closure.

## Trust-boundary rules established here

1. Provider or model output is governed input, not execution authority.
2. Provider route identity, model identity, and failure classification must stay
   explicit when a lane is consumed.
3. Credential, token, and private-endpoint surfaces must fail closed and must
   not be published into durable reviewer-facing packets.
4. Empty, malformed, or semantically untrustworthy model output must remain a
   first-class degraded or failed result rather than being silently promoted to
   success.
5. Prompt-injection and semantic-manipulation residuals are reviewed here but
   are not claimed closed without the later CAV taxonomy/corpus route.

## Boundary matrix

| Boundary | Why it matters | Current evidence | Current disposition |
| --- | --- | --- | --- |
| Provider route and model identity | Downstream consumers must know which provider lane produced the result and must not confuse route support with authority. | `#4008`, `#4010`, and `#4012` preserve explicit provider/model/role limits and advisory-only orchestration posture. | `reviewed_and_currently_covered` |
| Credential and token boundary | Missing or invalid credentials must fail closed instead of silently falling back or leaking into durable artifacts. | `#4010` names `provider_auth_missing` and `provider_auth_error` as operator-gated failures; WP-03 logging proof is consumed as the diagnostic floor via `#4012`. | `reviewed_and_currently_covered` |
| Private endpoint and durable packet hygiene | Security review packets must not expose private LAN coordinates, brittle local host data, or machine-local fixtures. | `#4011` scanned the bounded provider proof roots and found them clean of private-LAN literals and host-local portability residue. | `reviewed_and_currently_covered` |
| Provider failure and degraded output classification | Timeouts, rate limits, empty output, and provider faults must remain explicit so later security lanes can reason about abuse and failure honestly. | `#4010` maps provider failures into the shared resilience vocabulary and names empty output as an explicit failure class. | `reviewed_and_currently_covered` |
| Malformed-output / semantic confusion boundary | A provider can return syntactically valid but misleading or low-integrity output that should not be treated as trustworthy. | `#4010` establishes the routing contract for malformed or partial output, but it does not provide a full adversarial harness for semantic-confusion classes. | `reviewed_and_routed` |
| Prompt-injection / adversarial content boundary | Provider responses or adjacent packet content can manipulate later reviewers or consumers even when the transport succeeds. | The v0.91.5 CAV source packet and the WP-04 security handoff both keep prompt-injection-style abuse on the activation path, but they do not claim repository-wide closure. | `reviewed_and_routed` |
| Cross-surface protocol / message trust | Provider trust assumptions can bleed into ACIP, A2A, or other communication layers if message authority is left implicit. | The shared security bridge feature doc keeps ACIP/A2A/provider communications open and explicitly routes that protocol security work to WP-06/WP-07 follow-on lanes. | `reviewed_and_routed` |

## Provider log, token, and endpoint checks

Current bounded security posture for the provider lane:

- provider route/model identity is expected to remain visible in bounded
  observability surfaces rather than being hidden behind generic "model ran"
  narration
- credential absence and auth failure are explicit operator-gated failures, not
  silent fallback paths
- raw token values are not a durable proof surface and must not be promoted
  into review packets
- private endpoints are acceptable only as runtime/operator configuration; they
  are not acceptable as durable public or reviewer-facing packet literals
- bounded provider packet roots consumed by WP-05 have already been scanned for
  private-LAN and host-local residue

## Malformed-output and prompt-injection notes

### Malformed-output

Current reviewed truth:

- empty text output is already a named fault class
- malformed or partial output must remain a contract-level degradation or
  failure
- no packet in this issue claims that every provider lane already has a full
  malformed-output regression harness

### Prompt injection and semantic manipulation

Current reviewed truth:

- prompt-injection-style abuse remains a real trust-boundary concern even when
  provider transport and packet structure are nominally valid
- the current provider packets are good enough to keep this risk visible and
  bounded, but they are not sufficient to claim dynamic adversarial closure
- later security lanes must keep semantic-confusion, malicious-content, and
  manipulation classes explicit instead of collapsing them into generic
  "provider quality" concerns

## Findings and dispositions

1. Provider/model trust boundaries are explicit enough for bounded `v0.91.6`
   consumption, but they must remain advisory-only and must not be mistaken for
   execution authority.  
   Disposition: fixed by this packet and the already-merged WP-05 packet set.

2. Malformed-output and prompt-injection classes are reviewed here, but the
   repository still lacks a full CAV taxonomy/corpus closeout for those abuse
   classes.  
   Disposition: routed to `#4064` and retained under WP-07 closeout.

3. Cross-surface protocol/message trust remains broader than the provider-only
   review lane and cannot be closed from WP-05 proof alone.  
   Disposition: routed to the open ACIP/A2A/provider communications security
   lanes, especially `#4021`, instead of being hidden here.

4. Historical authoring drift still exists around the old local/TBD provider
   source path mentioned by upstream provider packets.  
   Disposition: already documented in `#4111`; keep as remediation residue
   rather than reopening provider implementation scope inside `#4020`.

## Consumption rule for v0.91.6 and v0.92

Current decision:

- `provider_model_trust_boundary_reviewed_with_explicit_residual_routes`

That means later milestone work may consume:

- explicit provider/model trust boundaries
- fail-closed credential/auth posture
- bounded private-endpoint hygiene
- bounded malformed-output routing expectations

It may not consume this issue as proof of:

- completed CAV operations
- full prompt-injection closure
- universal provider trustworthiness
- authority to let provider/model lanes bypass workflow, merge, or closeout
  governance

## Residual routing

- CAV threat taxonomy, exploit-corpus routing, and unclosed abuse classes route
  to `#4064`.
- Cross-surface ACIP/A2A/provider message-trust residuals route to `#4021` and
  the still-open WP-06 tranche.
- Any future privacy/publication implications from provider-derived content
  route to WP-07 `#4022` as the bounded publication/privacy security lane; if
  WP-10 memory/privacy closure is still incomplete at milestone closeout, the
  remaining residue must stay owned by the open WP-10 issue set in `v0.91.6`
  or be carried into the `v0.91.7` residual guard instead of being silently
  closed here.

## Reviewer takeaway

`#4020` is ready when reviewers can confirm that:

- provider/model trust boundaries are explicit and bounded
- log/token/private-endpoint posture is honest and fail-closed
- malformed-output and prompt-injection classes are named rather than implied
- unresolved security work is routed to named open issues instead of being
  collapsed into a broad "provider security is done" claim
