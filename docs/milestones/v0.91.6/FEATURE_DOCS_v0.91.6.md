# v0.91.6 Feature-Doc Index

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Setup issue: `#3800`
- Documentation completion issue: `#3824`

## Status

Feature-doc package created for the first pre-`v0.92` bridge tranche. These
docs define planning, decisions, validation expectations, and `v0.92`
consumption limits; they do not implement runtime behavior.

## Required Feature Docs And Bridge Records

| Feature doc | Surface | Required questions | Exit state before v0.92 |
| --- | --- | --- | --- |
| [`RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md`](features/RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md) | Resilience, citizen persistence, sleep/wake | What retry/fault classes, persistence states, migration/replay paths, and continuity proofs are required? | Phase 1 resilience substrate/pattern/integration proof landed; checkpoint/restore, sleep/wake, hibernation, migration, replay, and durable continuity remain explicit residual work. |
| [`TOOLING_PROOF_LOOP_RELIABILITY_v0.91.6.md`](features/TOOLING_PROOF_LOOP_RELIABILITY_v0.91.6.md) | Logging/tooling proof-loop fixes | How do validation split, CI budget observability, OTel/logging consumption, and issues `#3802`-`#3805` improve bounded PR reliability? | Bridge doc and WP-03 execution packet exist; `#4001` remains a separate routed lane. |
| [`PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md`](features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md) | Public prompt records | How do local editable records export publicly with redaction, validation, indexing, and security review? | Bridge doc plus WP-04 proof/closeout packet landed through `#4002`-`#4006`; broader security activation residuals remain explicitly routed. |
| [`PROVIDER_MODEL_RELIABILITY_v0.91.6.md`](features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md) | Provider/model reliability | Which models are suitable for which roles, including hosted, local, remote, OpenRouter, and Gemma lanes? | Bridge doc exists; proof gaps must be named by implementation issues. |
| [`ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md`](features/ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md) | ACIP/A2A/provider communications | What schema catalog, access rules, JSON projection, protobuf decision, WebSocket boundary, and provider-message posture are required? | Bridge doc exists; residual protocol closure may route to `v0.91.7`. |
| [`SECURITY_BRIDGE_AND_CAV_v0.91.6.md`](features/SECURITY_BRIDGE_AND_CAV_v0.91.6.md) | Security bridge and CAV | What threat-model refresh, adversarial checks, malformed-output checks, provider trust, prompt-record security, and ACIP security are required? | Bridge doc exists; security remains activation-path work. |
| [`IDENTITY_CONTINUITY_CAPABILITY_SELECTOR_BRIDGE_v0.91.6.md`](features/IDENTITY_CONTINUITY_CAPABILITY_SELECTOR_BRIDGE_v0.91.6.md) | Identity/continuity and capability selector | How does capability evidence feed v0.92 without starting Aptitude Atlas? How do continuity and resilience connect? | Bridge doc exists; v0.92 consumes evidence boundaries only. |
| [`OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md`](features/OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md) | Observatory/Unity readiness | Which surfaces are proof, rehearsal, substrate, blocked, or deferred? | Bridge doc exists; classification is required before demos rely on surfaces. |
| [`AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md`](features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md) | AEE completion, Memory/ObsMem handoff, ACP/cognitive profiles | What AEE boundaries are complete, what residual runtime/provider action work remains, what Memory/ObsMem can hand off, and what ACP/profile privacy scope may v0.92 consume? | Bridge doc exists; activation consumption remains bounded. |

## Cross-Doc Requirements

- Every doc must name non-goals and unsupported claims.
- Every doc must include validation and review expectations.
- Every doc must say what `v0.92` may consume.
- Every doc must preserve `v0.91.7` residuals where the first tranche cannot
  truthfully finish the surface.
- Security, ACIP/A2A, resilience, and tooling reliability must not be collapsed
  into generic backlog text.
- AEE completion, Memory/ObsMem handoff, and ACP/cognitive profiles must not be
  dropped just because most implementation work belongs to `v0.92`.

## v0.91.7 Residual Guard

The following belong to `#3825` for docs-package completion, building on the
`#3801` planning package, unless a v0.91.6 issue explicitly pulls them forward
with reviewed scope:

- Curiosity Engine / Discovery Substrate
- Constructability Gate
- reasoning graph, loop runtime, and `adl.skill.v1`
- residual security readiness
- residual ACIP/A2A/protobuf/JSON projection decisions
- affect/happiness, Godel mechanics, and economics-context accounting

## Validation

When this index is consumed:

- verify each planned implementation/proof route has an owning issue or
  explicit blocked/deferred route
- scan for `v0.92` readiness overclaims
- scan for local authoring-workspace links or host-local paths
- verify `#3825` / `v0.91.7` residuals remain visible
