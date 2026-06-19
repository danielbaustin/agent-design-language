# Provider Profiles V2 Reconciliation for #4111

## Scope

This packet reconciles the intended provider/profile V2 shape for `v0.91.6`
against the live tracked repository state after WP-05.

It turns the lingering V2 concept into one tracked contract and implementation
matrix. It does not implement new runtime schema enforcement by itself, and it
does not treat non-provider identity concepts as part of the provider lane.

## Source evidence

Tracked local sources used here:

- `docs/adr/0004-provider-profiles.md`
- `adl/src/provider/profiles.rs`
- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_CAPABILITY_PROFILE_CATALOG_4007.md`
- `docs/milestones/v0.91.6/review/provider/ROLE_PROVIDER_PROFILES_4053.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_RELIABILITY_CLOSEOUT_MATRIX_4012.md`
- `docs/milestones/v0.91.6/review/provider/WP05_PROVIDER_MINI_SPRINT_CLOSEOUT_3970.md`

Authoring/input drift retained for traceability:

- the issue was authored against `.adl/docs/TBD/ADL_PROFILES_PROVIDERS_V2.MD`
- that path is not present in the current tracked repository state
- the live tracked sources above are therefore the only authoritative basis for
  this reconciliation

## Reconciled V2 boundary

The truthful V2 split after WP-05 is:

1. provider profiles remain low-level infrastructure and transport records
2. capability profiles remain provider-independent behavioral descriptors
3. role-provider profiles remain higher-level C-SDLC policy/routing records
4. model identity remains a separate descriptive surface and is not yet a fully
   implemented first-class schema/runtime layer
5. citizen, institution, guild, continuity, and broader identity surfaces are
   not provider-profile work and must remain routed out of this lane

This means provider/profile V2 is not one monolithic object.

## Implementation matrix

| Surface | Current truth | Evidence | Next executable step | Out-of-scope boundary |
| --- | --- | --- | --- | --- |
| Provider profiles | `partially_implemented_runtime_plus_documented_contract` | `adl/src/provider/profiles.rs`; ADR 0004; `#4007`; `#4012` | add a typed tracked provider-profile schema that matches the documented boundary beyond `kind`, `default_model`, and endpoint validation | do not collapse provider profiles into actor identity or role authority |
| Capability profiles | `documented_only` | `PROVIDER_CAPABILITY_PROFILE_CATALOG_4007.md`; feature doc | add a first tracked schema/record format for capability profiles and bind it to deterministic validation | do not bind vendor, endpoint, or credential state into capability profiles |
| Role-provider profiles | `documented_only` | `ROLE_PROVIDER_PROFILES_4053.md`; `#4012` | add a typed role-provider policy format and deterministic resolver input surface that references low-level provider profiles without replacing them | do not grant autonomous lifecycle authority to external providers |
| Model identity surfaces | `partially_documented_only` | feature doc; `ROLE_PROVIDER_PROFILES_4053.md`; `profiles.rs` model strings/profile families | define whether model identity becomes a first-class typed record or remains provider/capability metadata, then track that decision explicitly | do not treat model identity as citizen/institution/continuity identity |
| Provider profile runtime expansion/validation | `implemented_v1_but_narrow` | `expand_provider_profiles`; profile registry; endpoint validators in `profiles.rs` | widen the runtime contract only if it is backed by a tracked schema and deterministic config migration path | do not silently widen config semantics from docs alone |
| Qualitative provider routing metadata (`provider_family`, `economics_class`, `latency_class`, `tool_support_class`, etc.) | `documented_only` | feature doc WP-05 boundary sections | decide whether these fields live in code, tracked schema artifacts, or documentation-only policy tables, then implement that one path consistently | do not pretend documented qualitative fields are already enforced in runtime |
| Non-provider identity / citizen / institution / guild / continuity concepts | `explicitly_deferred_out_of_lane` | issue body non-goals; feature doc out-of-scope; `#4053` non-claims | route to later identity/governance/continuity work instead of widening provider V2 | not provider/profile implementation work for `v0.91.6` |

## What WP-05 actually finished

WP-05 completed the provider sprint truth needed for this reconciliation:

- provider versus capability profile boundary
- provider/model role suitability proof posture
- role-provider contract and advisory-authority limits
- provider closeout matrix and `v0.92` consumption limits

WP-05 did not finish:

- a first-class capability-profile runtime/schema layer
- a first-class role-provider runtime/schema layer
- a typed model-identity contract
- non-provider identity/citizen/institution/guild/continuity implementation

## Current runtime truth from `profiles.rs`

The current runtime substrate proves:

- deterministic profile registries for local Ollama, mock/test, bounded HTTP,
  ChatGPT-facing, and Claude-facing profile families
- profile expansion from compact `profile` references into explicit provider
  specs
- endpoint validation and placeholder rejection for bounded HTTP profiles

The current runtime substrate does not yet prove:

- typed capability-profile storage
- typed role-provider-profile storage
- typed model-identity storage
- runtime enforcement for the broader qualitative metadata described in WP-05

## Recommended follow-on implementation slices

1. `provider profile schema parity`
   - create one tracked schema artifact for provider profiles that matches the
     low-level runtime contract and intentionally names any still-doc-only
     fields
2. `capability profile typed contract`
   - add a deterministic capability-profile record format plus validation path
3. `role-provider resolver contract`
   - add a typed role-provider policy input surface and deterministic
     fail-closed resolver boundary above provider profiles
4. `model identity boundary decision`
   - decide whether model identity is its own typed surface or remains attached
     to provider/capability/role policy records
5. `non-provider identity routing`
   - open or route later work for citizen/institution/guild/continuity concepts
     explicitly outside the provider sprint lane

## Consumption guidance

- `v0.92` may consume WP-05 as the bounded provider-proof baseline.
- `v0.92` may not assume capability profiles, role-provider runtime routing, or
  model-identity schema are already implemented just because the contracts are
  documented.
- Future work should cite this reconciliation packet instead of relying on the
  missing `.adl/docs/TBD/ADL_PROFILES_PROVIDERS_V2.MD` path.

## Non-Claims

- This packet does not claim provider/profile V2 is fully implemented.
- This packet does not claim capability profiles or role-provider profiles are
  already runtime-enforced.
- This packet does not claim model identity has a settled final schema.
- This packet does not pull citizen, institution, guild, or continuity work
  back into the provider lane.
