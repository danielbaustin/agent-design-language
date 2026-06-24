# ADR 0041: Provider/Model Suitability Boundary v2

- Status: Accepted
- Date: 2026-06-23
- Accepted in: v0.91.6
- Candidate source: docs/architecture/adr/0041-provider-model-suitability-boundary-v2.md
- Target milestone: v0.91.6
- Related issues: #3970, #4007, #4008, #4010, #4012, #4020, #4053, #4111, #4154, #4158
- Related ADRs: ADR 0004, ADR 0024, ADR 0028
- Source evidence:
  - `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
  - `docs/milestones/v0.91.6/review/provider/WP05_PROVIDER_MINI_SPRINT_CLOSEOUT_3970.md`
  - `docs/milestones/v0.91.6/review/provider/PROVIDER_CAPABILITY_PROFILE_CATALOG_4007.md`
  - `docs/milestones/v0.91.6/review/provider/PROVIDER_ROLE_SUITABILITY_MATRIX_4008.md`
  - `docs/milestones/v0.91.6/review/provider/PROVIDER_FAILURE_MODE_INTEGRATION_4010.md`
  - `docs/milestones/v0.91.6/review/provider/PROVIDER_RELIABILITY_CLOSEOUT_MATRIX_4012.md`
  - `docs/milestones/v0.91.6/review/provider/ROLE_PROVIDER_PROFILES_4053.md`
  - `docs/milestones/v0.91.6/review/provider/PROVIDER_PROFILES_V2_RECONCILIATION_4111.md`
  - `docs/milestones/v0.91.6/review/security/PROVIDER_MODEL_CAV_TRUST_BOUNDARY_REVIEW_4020.md`

## Context

Existing provider architecture distinguishes provider profiles from runtime
authority. v0.91.6 added a more precise reliability problem: multi-agent work
depends on whether specific models are suitable for specific roles, not merely
whether a provider is available.

The milestone reviewed provider capability profiles, role suitability,
failure-mode integration, current model suitability, and CAV trust boundaries.
Those surfaces need a v2 boundary that preserves earlier provider-profile
policy while adding model-role evidence and reliability limits.

## Decision

ADL should treat provider availability, provider capability profiles,
model-role suitability, reliability evidence, failure-mode behavior, and
role-provider advisory authority as distinct architecture surfaces.

Provider availability proves only that a provider path can be reached under
approved credentials and setup.

Capability profiles describe what a provider/model lane claims to support.

Role suitability evidence describes whether a model behaves reliably enough
for a specific ADL role, such as review, planning, coding, synthesis,
validation, or local multi-agent work.

Advisory provider-role logic may recommend a lane, but it must not overrule
credential policy, cost policy, CAV/security boundaries, or issue-specific
validation requirements.

## Consequences

### Positive

- Prevents provider reachability from being mistaken for multi-agent readiness.
- Gives model evaluation work a durable place in architecture.
- Supports local and remote model comparison without turning experiments into
  product claims.
- Preserves separation between provider setup, capability, suitability, and
  authority.

### Negative

- Provider matrix maintenance becomes an ongoing release obligation.
- Models may be usable with limits rather than simply approved or rejected.
- Suitability claims require retained evidence and may expire as models change.

## Alternatives Considered

### Treat all current frontier models as interchangeable

This is rejected. v0.91.6 evidence shows role behavior and reliability vary by
model and provider lane.

### Treat provider profile success as readiness

Provider profile success is necessary infrastructure evidence, but it does not
prove role suitability or multi-agent reliability.

## Validation Notes

Promotion should review WP-05 closeout, provider profile catalogs, role
suitability matrices, reliability closeout, failure-mode integration, and CAV
trust-boundary review. If promoted as a v2 ADR, it should explicitly preserve
the earlier provider-profile authority boundary instead of replacing it with a
broader claim.

## Non-Claims

- This ADR does not certify every provider/model lane as production-ready.
- This ADR does not disclose or require committing provider credentials.
- This ADR does not make advisory role-provider selection autonomous.
- This ADR does not claim aptitude-atlas work is part of the v0.95 MVP.
