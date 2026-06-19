# Identity, Continuity, And Capability Selector Bridge

## Metadata

- Feature Name: Identity, Continuity, And Capability Selector Bridge
- Milestone Target: `v0.91.6`
- Status: in_progress
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy
- Proof Modes: review, replay

## Purpose

Connect capability evidence, identity continuity, resilience, and negative
cases before `v0.92` consumes birthday or activation identity claims.

## Scope

In scope:

- capability evidence consumption;
- identity continuity boundaries;
- negative cases and invalid continuity claims;
- resilience and persistence dependencies;
- Aptitude Atlas boundary.

Out of scope:

- Aptitude Atlas productization;
- full identity runtime implementation;
- Memory Palace implementation.

## Required Decisions

- Which capability evidence may feed `v0.92`?
- Which identity continuity claims require replay or witness proof?
- Which negative cases invalidate continuity?
- Which surfaces route to Memory Palace or `v0.91.7`?

## Dependencies

- Resilience, persistence, and sleep/wake feature doc.
- Provider/model reliability feature doc.
- `v0.92` identity and birthday docs.

## Source Boundary Inputs

This bridge consumes the currently tracked `v0.91.6` boundary inputs rather
than inventing a new provider or identity catalog:

- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_CAPABILITY_PROFILE_CATALOG_4007.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_PROFILES_V2_RECONCILIATION_4111.md`
- `docs/milestones/v0.91.6/features/ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md`
- `docs/milestones/v0.91.6/features/RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md`

The authored issue input named `ADL_PROFILES_PROVIDERS_V2.MD`, but that path is
not the live tracked source of truth in the current repository. WP-08 consumes
the tracked provider/profile and ACIP feature surfaces above instead.

## I-00 Boundary Decision

`#4025` establishes the first WP-08 boundary vocabulary that downstream
identity, capability-evidence, continuity, and selector work must preserve.

| Surface | Owns | Must remain distinct from |
| --- | --- | --- |
| Provider profile | infrastructure/service identity, locality, endpoint class, substrate defaults, operator-managed auth expectations | capability need, civil identity, citizen record, institution record, guild policy |
| Capability profile | provider-independent behavioral capability descriptors, interaction modes, determinism/safety posture, tool-orchestration posture | vendor identity, endpoint configuration, civil identity, institution record |
| Identity profile vocabulary | planning-only higher-layer identity attributes that may later help describe continuity or selector-relevant identity posture | provider substrate, model identity, role authority, or a settled first-class runtime/schema contract in `v0.91.6` |
| Citizen record | governance/continuity-facing record that may later constrain what routes or actions are legitimate | provider profile, raw capability evidence, guild membership alone |
| Guild | MVP-scope policy/grouping input that may influence capability discovery or route narrowing | runtime execution authority, transport substrate, provider identity |
| Institution | higher-layer organizational identity/governance surface | provider substrate, capability profile, guild policy by itself |

The non-collapse rule for WP-08 is:

- provider profiles remain substrate descriptors;
- capability profiles remain behavioral descriptors;
- identity/citizen/institution surfaces remain governance/continuity surfaces;
- guilds remain MVP policy/grouping inputs;
- no single profile object becomes a disguised union of provider, capability,
  identity, citizen, and institution state.

## Selector Inputs And Non-Inputs

The capability selector MVP may consume only bounded higher-layer inputs.

Allowed selector inputs:

- requested capability need;
- capability evidence and freshness posture;
- provider/model suitability evidence from WP-05;
- guild-policy inputs where the route policy explicitly allows them;
- identity/citizen continuity posture only when a later issue proves the
  relevant continuity constraint and non-claim boundary.

Selector non-inputs for `v0.91.6`:

- raw provider identity as the primary routing primitive;
- civil personhood or unproved continuity assertions;
- Aptitude Atlas productized scoring/badging;
- institutional authority inferred from provider or capability metadata alone;
- guild presence treated as runtime execution authority by itself.

## Citizen And Guild Planning Hooks

WP-08 keeps citizen and guild concepts visible without pretending the full
runtime or governance layer already exists.

Citizen planning hooks:

- continuity-sensitive selector narrowing may later consume citizen-record
  posture;
- continuity claims remain evidence-bound and must not be inferred from labels
  alone;
- citizen state depends on later resilience and persistence proof, and any
  privacy-sensitive publication or display claims must remain separately
  reviewed rather than inferred here.

Guild planning hooks:

- guilds may later act as policy/grouping inputs for capability discovery or
  route narrowing;
- guilds are not yet first-class runtime authority objects;
- guild membership must not bypass higher-layer authority or security review.

## Aptitude Atlas Boundary

WP-08 may consume bounded capability-testing evidence, but `v0.91.6` still does
not claim:

- an Aptitude Atlas product baseline;
- a shipped capability-market runtime;
- selector decisions driven by productized aptitude scores;
- continuity proof derived from capability-testing output.

## Validation And Review

- Review identity claims against evidence and non-goals.
- Require negative-case language for continuity boundaries.
- Ensure capability evidence is consumed without Aptitude Atlas product claims.

## v0.92 Consumption

`v0.92` may consume the WP-08 boundary vocabulary and capability-evidence
posture established here. It must not:

- collapse provider, capability, identity, citizen, guild, and institution
  layers into one selector object;
- treat capability testing as Aptitude Atlas productization;
- treat continuity-sensitive identity claims as already proved;
- treat guild or citizen inputs as shipped runtime authority objects.

## Non-Goals

- No productized Aptitude Atlas baseline.
- No unproved personhood, continuity, or memory claims.
- No hidden Memory Palace implementation.
