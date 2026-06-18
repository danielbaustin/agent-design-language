# Provider And Model Reliability

## Metadata

- Feature Name: Provider And Model Reliability
- Milestone Target: `v0.91.6`
- Status: `m_00_provider_capability_catalog_defined`
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture
- Proof Modes: tests, review, replay
- Related issues: `#3970`, `#4007`
- Catalog proof note: [PROVIDER_CAPABILITY_PROFILE_CATALOG_4007.md](../review/provider/PROVIDER_CAPABILITY_PROFILE_CATALOG_4007.md)

## Purpose

Define model/provider suitability for reliable multi-agent operation before
`v0.92` consumes provider or birthday-demo claims.

## Scope

In scope:

- hosted, local, remote, OpenRouter, and Gemma lanes;
- provider-profile versus capability-profile catalog boundaries;
- role suitability for planning, review, execution, and synthesis;
- failure modes, timeouts, malformed outputs, and retry expectations;
- multi-agent readiness and proof limits.

Out of scope:

- model training;
- Aptitude Atlas productization;
- broad benchmark product claims.

## Required Decisions

- Which models may be used for which C-SDLC roles?
- Which models are useful with limits versus blocked?
- What evidence makes Gemma reliable enough for multi-agent work?
- How are provider failures surfaced and routed?
- Which fields belong to provider infrastructure profiles versus capability profiles?

## Dependencies

- Sprint 2 remediation proof packets.
- Remote Gemma proof and multi-agent comparison remediation.
- Tooling proof-loop reliability feature doc.

## Provider/Capability Catalog Boundary

WP-05 begins with one explicit split:

- provider profiles answer what infrastructure-backed services are available
- capability profiles answer what an entity or lane can do with those services

This milestone must not collapse providers, models, capabilities, identities,
citizens, or institutions into one profile object just because later routing
consumes all of them.

### Provider profile contract

Provider profiles are infrastructure/service descriptors. They are not actor,
identity, or authority records.

Required provider-profile fields for WP-05:

| Field | Meaning |
| --- | --- |
| `provider_family` | Stable provider family or substrate such as `openai`, `anthropic`, `google`, `ollama`, `openrouter`, `mock`, or bounded `http` profile families. |
| `profile_id` | Deterministic profile identifier used by config and routing. |
| `service_kind` | Runtime service substrate such as `http`, `ollama`, or `mock`. |
| `default_model` | Default model or model family when the profile owns one. |
| `endpoint_class` | Hosted HTTPS, local loopback, remote HTTPS, or placeholder/invalid until configured. |
| `economics_class` | Qualitative economics bucket for later routing, such as premium hosted, commodity hosted, local compute, or aggregator lane. |
| `latency_class` | Qualitative latency posture for later role/routing decisions. |
| `cost_class` | Relative cost posture for bounded routing decisions. |
| `tool_support_class` | Whether the provider surface is text-only, bounded tool-capable, or intentionally limited. |
| `lane_class` | Reliability lane such as hosted first-party, local model, remote open-weight, aggregator, or test/mock lane. |
| `locality_class` | `hosted`, `local`, or `remote` execution locality. |
| `auth_surface` | Operator-managed credential or local endpoint expectation; not embedded secrets. |

Provider profiles answer:

- what services exist
- how they are reached
- what default model and endpoint expectations they carry
- what qualitative latency/cost/tool/routing lane they belong to

Provider profiles do not answer:

- what one agent, citizen, or institution is allowed to do
- what role authority a C-SDLC lane has
- what identity continuity or civil record a system owns

### Capability profile contract

Capability profiles are provider-independent behavioral descriptors consumed by
role routing and later provider/model matrices.

Required capability-profile fields for WP-05:

| Field | Meaning |
| --- | --- |
| `capability_id` | Stable capability identifier. |
| `interaction_modes` | Chat, completion, tool use, review/synthesis, batch, or replay modes. |
| `structured_output_posture` | Expected reliability for machine-readable output. |
| `tool_orchestration_posture` | Whether the capability can safely act in tool-using or review-only lanes. |
| `context_class` | Qualitative context-window class, independent of one provider. |
| `reasoning_posture` | Qualitative reasoning/depth posture for later role suitability. |
| `determinism_posture` | How safely the capability supports repeatable review or routing surfaces. |
| `safety_limit_notes` | Named limitations or blocked cases that later matrices must preserve. |

Capability profiles answer:

- what a role or lane needs from a model surface
- how later matrices can compare providers without confusing provider identity
  with role authority

Capability profiles do not answer:

- which vendor hosts the model
- where credentials live
- which citizen/institution/identity record owns the action

### Identity and authority non-claims

The provider/capability catalog is intentionally not:

- a civil identity registry
- a citizen profile system
- an institution directory
- an authority or approval ledger
- the final C-SDLC role-provider matrix

Those layers may consume provider/capability catalogs later, but they are not
represented as provider config in WP-05.

## Current provider mapping notes

WP-05 uses the current deterministic provider-profile sources in the repository
to define the first catalog split.

| Provider family / lane | Current profile shape | Catalog notes |
| --- | --- | --- |
| OpenAI | bounded `http` preset plus ChatGPT-facing profile family | Hosted HTTPS lane; premium-first provider family with distinct ChatGPT-facing profile names. |
| Anthropic | bounded `http` preset plus Claude-facing profile family | Hosted HTTPS lane; provider family distinct from role authority. |
| Google | bounded `http` preset for Gemini | Hosted HTTPS lane; profile catalog owns provider/service identity, not capability claims. |
| Ollama | explicit local `ollama:*` presets | Local loopback or configured HTTP lane; locality matters independently of capability posture. |
| OpenRouter | bounded `http` preset | Aggregator lane; provider family is not equivalent to the downstream model capability. |
| DeepSeek remote | bounded `http` preset | Remote hosted lane; later reliability proof owns quality/resilience claims. |
| Mock/test | `mock:echo-v1` | Test-only substrate; useful for deterministic harnesses, not general role authority. |
| Local/remote open-weight lanes | represented through locality + endpoint class rather than identity objects | Locality and transport belong to provider profile; model-role suitability remains later matrix work. |

## Current tracked source truth

This issue was authored against `.adl/docs/TBD/ADL_PROFILES_PROVIDERS_V2.MD`,
but that path is not present in the current tracked repository state.

The live tracked sources consumed here are:

- [ADR 0004 provider profiles](../../../adr/0004-provider-profiles.md)
- `adl/src/provider/profiles.rs`

Remediation note:

- the stale issue-input path should be corrected or retired in future
  authoring/remediation work rather than silently treated as canonical

## Downstream consumption

- WP-05 later issues consume this split for role suitability, Gemma/OpenRouter
  proof, failure-mode/resilience integration, and closeout.
- WP-06 should treat provider communication and access/catalog decisions as
  consuming provider-profile and capability-profile boundaries rather than
  collapsing them into one routing object.
- WP-08 may consume capability evidence later, but identity/continuity remains
  separate from provider config.

## Validation And Review

- Require self-validating proof bundles for provider claims.
- Run role-specific smoke/deep checks where needed.
- Review model-output quality and reproducibility separately.
- Record unsupported models as blocked or limited.

## v0.92 Consumption

`v0.92` may consume provider/model readiness only as a role-scoped matrix with
named limits. It must not infer general intelligence, training readiness, or
product benchmark status from this tranche.

## Non-Goals

- No training claims.
- No Aptitude Atlas baseline.
- No unqualified "all models work" claim.
- No provider catalog that doubles as identity, citizen, or institution state.
