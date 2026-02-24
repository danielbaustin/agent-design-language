# ADR 0004: Provider Profiles (v0.6)

## Status
Accepted (v0.6)

## Context
v0.6 introduces provider profiles to reduce repeated provider configuration
boilerplate while preserving deterministic runtime behavior. The project needs a
small, explicit profile surface that is easy to validate and does not introduce
runtime heuristics or dynamic selection.

This ADR captures the v0.6 boundary and records what is intentionally deferred
to later milestones.

## Decision
Provider profiles are a deterministic, configuration-time indirection:
- a provider may declare `profile: <id>`
- profile expansion happens during resolve/validation before execution
- execution uses the expanded concrete provider spec
- unknown profiles and invalid profile endpoints fail fast with actionable errors

Provider execution semantics, scheduler ownership, retry behavior, and
deterministic ordering are unchanged by profile indirection.

## Design: Profile Model
A provider profile is an in-code preset keyed by profile id, with stable fields:
- `kind` (e.g., `ollama`, `mock`, `http`)
- optional `default_model`
- optional endpoint placeholder for HTTP profiles

In v0.6, profile registry data is local/static and deterministic. No dynamic
discovery or remote profile fetching is performed.

## Selection And Precedence Rules
Profile resolution rules are strict:
1. If `providers.<id>.profile` is absent, the provider spec is used as-is.
2. If `profile` is present, explicit provider fields must not also be set
   (`type`, `base_url`, `default_model`, `config`) for that provider.
3. `profile` must match a known registry id; otherwise resolution fails with a
   deterministic error including sorted available profile names.
4. Profile expansion produces a concrete provider spec used by runtime.

No nondeterministic fallback exists (for example, no "best match" or dynamic
auto-selection).

## Determinism And Safety Considerations
- Registry is deterministic (stable ordering for profile names/errors).
- Profile expansion is byte-stable across runs for identical inputs.
- Expansion occurs before execution; runtime path does not branch on profile id.
- Provider profile indirection does not alter step ordering, retries, or
  scheduler behavior.

## Error Handling Semantics
v0.6 fails fast during resolve/validation for profile-related errors:
- unknown profile id
- mixed profile + explicit provider fields
- placeholder/invalid HTTP endpoint profiles (issue #452)

Placeholder guard is string-based in v0.6 and rejects:
- empty endpoint
- known placeholder endpoint
- endpoints containing `example.invalid`

## Security And Privacy Considerations
- No secrets are embedded in committed profile presets.
- Profiles do not persist credentials in run artifacts.
- Credentials remain environment/operator-managed (for example auth env vars).
- Placeholder HTTP endpoints prevent accidental outbound calls from unresolved
  profile scaffolding.

## CLI / Config Surface (v0.6 vs Planned)
v0.6 provides:
- ADL config field `providers.<id>.profile`
- deterministic resolve-time profile expansion and validation

v0.6 does not provide:
- profile marketplace/catalog sync
- dynamic capability negotiation
- remote profile discovery
- profile signing/attestation policy

## Alternatives Considered
- Full explicit provider config only (no profiles)
  - Rejected: high duplication and inconsistent examples.
- Dynamic provider auto-selection at runtime
  - Rejected: introduces nondeterministic behavior and hidden policy.
- Remote profile registry in v0.6
  - Rejected: increases trust/supply-chain complexity too early.

## Consequences
- Configuration becomes simpler for common provider setups.
- Validation becomes stricter and earlier (fail-fast behavior).
- Runtime behavior remains deterministic and unchanged by profile indirection.
- Some endpoints require explicit user configuration before runtime use.

## In Scope (v0.6)
- Minimal profile placeholder scaffolding.
- Deterministic profile registry and resolve-time expansion.
- Fail-fast validation for invalid/placeholder profile endpoints.

## Out Of Scope (v0.6)
- Profile marketplace/discovery.
- Dynamic provider feature negotiation.
- Remote/signed profile distribution and policy enforcement.

## Future Work (v0.7+)
- Richer profile metadata and compatibility constraints.
- Signed/attested profile bundles.
- Optional remote profile sources under explicit trust policy.
- Stronger profile policy controls (for example org allowlists).
