# v0.91.6 Decisions

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Owner: ADL maintainers

## Purpose

Capture significant first-tranche bridge decisions and open questions.

## Decision Log

| ID | Decision | Status | Rationale | Impact | Link |
| --- | --- | --- | --- | --- | --- |
| D-01 | Treat `v0.91.6` as first bridge tranche, not cleanup-only. | accepted | `v0.92` activation depends on bridge surfaces that need tracked docs. | Creates first-tranche planning and feature docs before activation. | `#3824` |
| D-02 | Keep runtime implementation out of `v0.91.6` docs package. | accepted | Planning truth must not masquerade as shipped behavior. | Feature docs define routes and proof expectations, not runtime completion. | `DESIGN_v0.91.6.md` |
| D-03 | Preserve `v0.91.7` for residual conceptual surfaces. | accepted | Curiosity, Constructability, reasoning graphs, residual security, ACIP residuals, affect, Godel mechanics, and economics need their own tranche. | Avoids vague spillover and protects `v0.92` from rediscovery. | `FEATURE_DOCS_v0.91.6.md` |
| D-04 | Public prompt records require redaction, validation, indexing, and security review before public consumption. | accepted | Local editable records are not automatically public-safe. | Export remains blocked until the feature doc proof path is met. | `features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md` |
| D-05 | Provider/model reliability must include Gemma and multi-agent suitability limits. | accepted | Multi-agent reliability depends on role-appropriate model behavior, not only provider availability. | Reliability proof remains separate from product/training claims. | `features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md` |
| D-06 | ACIP/A2A protobuf can be a decision point rather than forced implementation in `v0.91.6`. | accepted | Schema/access/security posture must come first. | Residual wire-format work may route to `v0.91.7`. | `features/ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md` |
| D-07 | C-SDLC cards should remain authoritative while owned GitHub issue/PR surfaces converge into deterministic managed projections. | proposed | Partial projection without explicit ownership allows PR/issue drift to become workflow truth by accident. | `#3935` should define which surfaces are managed projections, drift-checked mirrors, linked surfaces, or card-local only. | `review/CSDLC_GITHUB_PROJECTION_CONVERGENCE_REVIEW_3935.md` |
| D-08 | v0.91.6 release-tail architecture decisions should be captured as proposed ADR candidates before closeout. | proposed | The milestone established durable boundaries for SSM, validation selection, GitHub projection ownership, runtime soak, scheduler authority, lockfile discipline, provider/model suitability, and public prompt records. | Creates a reviewable ADR packet and candidate draft set without silently accepting new ADRs. | `ADR_MINI_SPRINT_PACKET_v0.91.6.md`; `../../architecture/adr/0035-local-polis-ssm-operations-boundary.md`; `../../architecture/adr/0042-public-prompt-records-publication-boundary.md` |

## Open Questions

- Which `v0.91.6` feature docs become implementation issues versus bridge-only
  records?
- Which tooling reliability fixes from `#3802`-`#3805` are required before
  `v0.92` refresh versus acceptable as routed residuals?
- Which GitHub-facing surfaces should be automatically repaired from card truth
  versus only classified as drift?
- Which public prompt-record export checks must run locally versus in CI?
- Which Observatory/Unity surfaces count as proof versus rehearsal?
- Which v0.91.6 ADR candidates should be promoted immediately versus carried
  as explicitly deferred v0.91.7/v0.92 architecture work?

## Exit Criteria

- Milestone-critical decisions are logged with rationale.
- Deferred choices have an owner route.
- Open questions are closed, blocked, deferred, or routed before activation.
