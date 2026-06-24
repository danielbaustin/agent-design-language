# v0.91.6 ADR Mini-Sprint Packet

Issue: `#4324`
Status: `proposed_adr_packet`
Milestone: `v0.91.6`

## Purpose

This packet records the architecture decision work needed before the v0.91.6
release tail closes. It is a curation and review surface, not an acceptance
surface.

The entries below remain proposed ADR candidates until they are reviewed and
promoted into `docs/adr/`. This packet must not be read as silently accepting
new architecture decisions.

Accepted ADRs live in `docs/adr/`. Candidate draft copies and candidate
catalogs live in `docs/architecture/adr/`.

`#4416` adds a retained content-review packet for this candidate set at
`docs/milestones/v0.91.6/review/V0916_ADR_CANDIDATE_CONTENT_REVIEW_4416.md`.
That review repairs current-state sprint truth and records promotion gates, but
it still does not accept any ADR.

## Scope

This mini-sprint covers two tasks:

1. Route the existing v0.91.4 C-SDLC ADR candidates `0029` through `0034`.
2. Add the v0.91.6 decision candidates that emerged from the milestone's
   completed and retained review packets.

It does not implement runtime, provider, scheduler, validation, AWS, SSM,
public-record, or GitHub projection behavior.

## Required v0.91.6 ADR Candidates

| Candidate | Proposed status | Why it is needed before closeout | Primary evidence |
| --- | --- | --- | --- |
| [ADR 0035: Local Polis SSM Operations Boundary](../../architecture/adr/0035-local-polis-ssm-operations-boundary.md) | `candidate_required` | `#4109` and `#4113` establish AWS SSM as an operations and observability bridge, not a source of polis authority. This boundary should become durable before SSM expands to `nessus.local`, `opticon.local`, edge nodes, or non-AWS clouds. | `review/security/LOCAL_POLIS_SSM_OPERATIONS_BRIDGE_4109.md`; `review/security/LOCAL_POLIS_SSM_PROOF_4113.md` |
| [ADR 0036: Validation Lane Selector / PVF Test-Cost Policy](../../architecture/adr/0036-validation-lane-selector-pvf-test-cost-policy.md) | `candidate_required` | v0.91.6 made focused validation selection project-critical after the test surface expanded. The architecture decision should lock the rule that normal PR work runs the smallest deterministic proving lane, while ambiguous and release-gate surfaces fail closed or escalate. | `docs/architecture/VALIDATION_LANE_SELECTOR.md`; `review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md`; `review/PVF_LONG_VALIDATION_LANE_INDEX_4223.md` |
| [ADR 0037: GitHub/C-SDLC Projection Ownership](../../architecture/adr/0037-github-csdlc-projection-ownership.md) | `candidate_required` | `#4047` classified GitHub surfaces as managed projections, drift-checked projections, linked external state, card-local truth, or deferred surfaces. That distinction is now architecture, not just a report. | `review/github_projection/GITHUB_CSDLC_PROJECTION_MAP_4047.md`; `review/CSDLC_GITHUB_PROJECTION_CONVERGENCE_REVIEW_3935.md` |
| [ADR 0038: Runtime Integration Soak Boundary](../../architecture/adr/0038-runtime-integration-soak-boundary.md) | `candidate_required` | The milestone explicitly rejects the claim that separately implemented pieces equal an integrated runtime. Soak #1 and Soak #2 need a durable decision boundary before v0.92 runtime-coherence claims. | `RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md`; `RUNTIME_FIRE_UP_PLAN_v0.91.6.md`; `features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md` |
| [ADR 0039: Cognitive Scheduler v1 Authority Boundary](../../architecture/adr/0039-cognitive-scheduler-v1-authority-boundary.md) | `candidate_required` | `#4107` implemented deterministic lane-selection evidence, but did not implement timed execution, GitHub mutation, provider choice authority, or autonomous sprint conduction. That non-authority boundary should be durable before later scheduler expansion. | `review/scheduler/COGNITIVE_SCHEDULER_V1_4107.md`; `review/scheduler/SCHEDULER_ECONOMICS_INPUTS_4106.md`; `features/COGNITIVE_SCHEDULER_v0.91.6.md` |
| [ADR 0040: Workflow Lockfile Discipline](../../architecture/adr/0040-workflow-lockfile-discipline.md) | `candidate_required_needs_source_packet` | The `#4306` incident showed lifecycle tooling can create large, misleading validation cost if dependency resolution mutates `Cargo.lock` implicitly. The durable decision should require locked lifecycle fallback and explicit lockfile artifacts, but the release tail still needs a retained local source packet for the merged fix before promotion. | `review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md` |
| [ADR 0041: Provider/Model Suitability Boundary v2](../../architecture/adr/0041-provider-model-suitability-boundary-v2.md) | `candidate_required` | WP-05 separated provider availability, capability profiles, role suitability, model reliability, and advisory role-provider authority. Existing ADR 0004 covers provider profiles, but v0.91.6 adds model-role suitability and multi-agent readiness evidence boundaries. | `review/provider/WP05_PROVIDER_MINI_SPRINT_CLOSEOUT_3970.md`; `features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`; `review/provider/PROVIDER_RELIABILITY_CLOSEOUT_MATRIX_4012.md` |
| [ADR 0042: Public Prompt Records Publication Boundary](../../architecture/adr/0042-public-prompt-records-publication-boundary.md) | `candidate_required` | WP-04 established that `.adl` authoring records are not automatically public-safe. Public consumption requires export contract, redaction, validation, indexing, and security/CAV handoff rules. | `features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md`; `review/public_prompt_records/PUBLIC_PROMPT_RECORDS_DISTRIBUTION_CLOSEOUT_4006.md`; `review/public_prompt_records/PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md` |

## Candidate Summaries

The release-tail candidate ADR drafts now exist as proposed records:

- `docs/architecture/adr/0035-local-polis-ssm-operations-boundary.md`
- `docs/architecture/adr/0036-validation-lane-selector-pvf-test-cost-policy.md`
- `docs/architecture/adr/0037-github-csdlc-projection-ownership.md`
- `docs/architecture/adr/0038-runtime-integration-soak-boundary.md`
- `docs/architecture/adr/0039-cognitive-scheduler-v1-authority-boundary.md`
- `docs/architecture/adr/0040-workflow-lockfile-discipline.md`
- `docs/architecture/adr/0041-provider-model-suitability-boundary-v2.md`
- `docs/architecture/adr/0042-public-prompt-records-publication-boundary.md`

## Child Review Issue Wave

The proposed ADR drafts are tracked for review/promotion routing by this child
issue wave:

| Issue | Candidate ADR | Review purpose |
| --- | --- | --- |
| `#4369` | ADR 0035: Local Polis SSM Operations Boundary | Confirm SSM remains operations-plane only and does not own polis authority. |
| `#4370` | ADR 0036: Validation Lane Selector / PVF Test-Cost Policy | Confirm deterministic focused validation and fail-closed escalation boundaries. |
| `#4371` | ADR 0037: GitHub/C-SDLC Projection Ownership | Confirm projection ownership classes and legacy-linkage caveat. |
| `#4372` | ADR 0038: Runtime Integration Soak Boundary | Confirm Soak #1/#2/#3 routing without v0.92 readiness overclaim. |
| `#4373` | ADR 0039: Cognitive Scheduler v1 Authority Boundary | Confirm Scheduler v1 remains planning/evidence only. |
| `#4374` | ADR 0040: Workflow Lockfile Discipline | Confirm the evidence-capture gap remains visible before acceptance. |
| `#4375` | ADR 0041: Provider/Model Suitability Boundary v2 | Confirm provider availability, capability, suitability, reliability, and advisory authority remain distinct. |
| `#4376` | ADR 0042: Public Prompt Records Publication Boundary | Confirm public prompt records are reviewed projections, not raw `.adl` publication. |

These child issues are review and promotion-routing surfaces. They do not mean
the candidate ADRs are accepted.

## Post-Review Candidate Disposition Route

Issue `#4383` repaired the release-tail ADR routing truth after the retained
candidate review packets landed. The follow-on acceptance/disposition owner is
`#4476`. That issue must either promote, refresh-then-promote, defer, or
evidence-gate each candidate through the normal ADR acceptance path.

| Candidate | Current disposition | Owner issue | Target milestone | Required evidence before acceptance |
| --- | --- | --- | --- | --- |
| ADR 0035: Local Polis SSM Operations Boundary | `ready_for_acceptance_review` | `#4476` | `v0.91.6` release tail | Confirm `#4109`, `#4113`, `#4318`, `#4319`, and `#4343` preserve SSM as operations-plane authority only. |
| ADR 0036: Validation Lane Selector / PVF Test-Cost Policy | `ready_for_acceptance_review` | `#4476` | `v0.91.6` release tail | Confirm validation selector docs, PVF lane index, and retained review evidence agree on deterministic lane selection and fail-closed escalation. |
| ADR 0037: GitHub/C-SDLC Projection Ownership | `refresh_then_acceptance_review` | `#4476` | `v0.91.6` release tail | Refresh against current typed closing-linkage and GitHub convergence truth, including `#4286` disposition or an explicit deferred implementation note. |
| ADR 0038: Runtime Integration Soak Boundary | `ready_for_acceptance_review` | `#4476` | `v0.91.6` release tail | Confirm Soak #1 remains the v0.91.6 walking-skeleton proof and Soak #2 remains the v0.91.7 feature-integration gate before v0.92. |
| ADR 0039: Cognitive Scheduler v1 Authority Boundary | `ready_for_acceptance_review` | `#4476` | `v0.91.6` release tail | Confirm scheduler v1 evidence remains planning/evidence only and does not grant timed execution, GitHub mutation, or provider-selection authority. |
| ADR 0040: Workflow Lockfile Discipline | `defer_until_evidence` | `#4476` | `v0.91.6` release tail if evidence is captured; otherwise `v0.91.7` | Capture or cite the durable source packet for the lockfile-discipline fix, including exact tracked files and validation proof, before promotion. |
| ADR 0041: Provider/Model Suitability Boundary v2 | `ready_for_acceptance_review` | `#4476` | `v0.91.6` release tail | Confirm WP-05/provider suitability evidence distinguishes provider availability, capability, role suitability, reliability, failure modes, and advisory authority. |
| ADR 0042: Public Prompt Records Publication Boundary | `ready_for_acceptance_review` | `#4476` | `v0.91.6` release tail | Confirm public prompt records remain reviewed/redacted/indexed projections and do not claim raw `.adl` publication or unrestricted historical export. |

`#4476` is an ADR disposition issue, not an implementation issue. It may accept
only the candidates whose evidence gates are satisfied and must leave deferred
candidate routes visible.

### Local Polis SSM Operations Boundary

Proposed decision: AWS Systems Manager may manage approved local polis hosts as
an operations-plane bridge for bounded host status, logging, and evidence
export. It must not become authority for local polis state, governance, memory,
identity, provider selection, model content, or runtime semantics.

Approval boundary: promote only after reviewers confirm the `#4109` boundary
and `#4113` proof packet are sufficient, and that later managed-node work can
reuse the boundary without broadening it silently.

### Validation Lane Selector / PVF Test-Cost Policy

Proposed decision: ADL keeps the full validation surface available, but ordinary
PR work should run a deterministic validation profile selected from tracked
surface metadata. Ambiguous, shared Rust, release-gate, live-provider, and
credentialed surfaces must escalate or fail closed instead of returning a false
green.

Approval boundary: promote only after reviewers confirm the selector manifest,
profile runner, CI policy, and retained review packet agree on authority,
escalation, and non-claims.

### GitHub/C-SDLC Projection Ownership

Proposed decision: C-SDLC cards and tracked lifecycle records remain the
authoritative workflow source. GitHub issue/PR surfaces are typed projections,
drift-checked mirrors, linked external state, card-local-only surfaces, or
explicitly deferred surfaces.

Approval boundary: promote only after `#4286` or an equivalent follow-on
disposes of the remaining legacy closing-linkage guard, or the ADR explicitly
records that residue as a deferred implementation note.

### Runtime Integration Soak Boundary

Proposed decision: runtime coherence requires integrated soak evidence. Soak #1
in v0.91.6 proves a walking skeleton; Soak #2 in v0.91.7 proves full
feature-list integration before v0.92. Soak #3 exists only if Soak #2 exposes
blockers that need another pass.

Approval boundary: promote as soon as the release tail accepts the soak model
as the v0.92 runtime-coherence gate, even if Soak #1 execution is still tracked
separately.

### Cognitive Scheduler v1 Authority Boundary

Proposed decision: Scheduler v1 is deterministic planning and evidence
generation for work-lane selection. It does not execute tasks, mutate GitHub,
select live providers, run timed jobs, or absorb sprint-conductor authority.

Approval boundary: promote after reviewers confirm the implementation packet
and economics input packet preserve that non-authority boundary.

### Workflow Lockfile Discipline

Proposed decision: lifecycle tooling must use locked Cargo dependency
resolution when falling back to Rust delegates, and any lockfile update must be
an explicit issue-scoped artifact. Tooling must not silently modify
`Cargo.lock` during readiness, doctor, run, finish, or closeout checks.

Approval boundary: promote only after the `#4306` fix has a durable tracked
source packet, or after the ADR promotion issue explicitly cites the exact
tracked files and validation proof that landed the locked fallback behavior.
Until then, this remains a required candidate with an evidence-capture gap, not
an acceptance-ready ADR.

### Provider/Model Suitability Boundary v2

Proposed decision: provider profiles, capability profiles, model-role
suitability, model reliability evidence, and role-provider advisory authority
are distinct surfaces. Multi-agent readiness depends on role-appropriate model
behavior, not provider availability alone.

Approval boundary: promote as a v2/supersession note to ADR 0004 only if the
ADR explicitly preserves ADR 0004's provider-profile boundary and adds the
newer suitability/reliability layer without claiming every provider lane is
ready.

### Public Prompt Records Publication Boundary

Proposed decision: `.adl` remains the private/local authoring surface. Public
prompt records are review-safe projections only after export contract,
redaction/publication safety, validation/indexing, and security/CAV handoff
requirements are satisfied.

Approval boundary: promote after reviewers confirm the ADR does not claim
unrestricted release approval, completed WP-07 CAV closure, or automatic export
of every historical `.adl` bundle.

## Existing Candidate ADR 0029-0034 Routing

These older C-SDLC candidates are routed to the same explicit
acceptance/disposition owner as the v0.91.6 release-tail candidates so #4383
does not leave its original scope as recommendation-only prose.

| Candidate | Current state | v0.91.6 recommendation | Owner issue | Target route | Rationale |
| --- | --- | --- | --- | --- | --- |
| ADR 0029: C-SDLC Default Software-Development Lane | proposed in `docs/architecture/adr/` | `promote_or_refresh_then_promote` | `#4476` | `v0.91.6` release tail | Root `AGENTS.md`, workflow-conductor usage, and v0.91.6 issue execution show this has become active architecture policy. Refresh with current v0.91.6 lifecycle/tooling evidence before acceptance. |
| ADR 0030: Software Development Polis Actor Standing And Shard Ownership | proposed in `docs/architecture/adr/` | `defer_with_active_route` | `#4476` | `v0.91.7`/`v0.93` governance | Still important, but actor standing and shard ownership are not fully enforced enough to accept without a focused review. Keep as v0.91.7/v0.93 governance route. |
| ADR 0031: C-SDLC Multi-Agent Parallel Execution Boundary | proposed in `docs/architecture/adr/` | `defer_with_active_route` | `#4476` | `v0.91.7` C-SDLC hardening | v0.91.6 used parallel sessions operationally, but durable multi-agent shard enforcement and integration review need more explicit evidence before acceptance. |
| ADR 0032: Parallel Validation Fabric | proposed in `docs/architecture/adr/` | `promote_or_refresh_then_promote` | `#4476` | `v0.91.6` release tail | v0.91.6 validation-manager/test-tax work made PVF-style lane selection active project architecture. Refresh with `VALIDATION_LANE_SELECTOR.md` and `#4212` evidence before acceptance. |
| ADR 0033: Merge Readiness And PR Gate Truth Boundary | proposed in `docs/architecture/adr/` | `promote_or_refresh_then_promote` | `#4476` | `v0.91.6` release tail | v0.91.6 closeout, review, projection, and lockfile incidents reinforce this as active architecture policy. Refresh with current issue lifecycle and validation-manager evidence. |
| ADR 0034: C-SDLC Evidence Convergence, Signed Trace, And ObsMem Handoff | proposed in `docs/architecture/adr/` | `defer_with_active_route` | `#4476` | `v0.91.7`/`v0.92` bridge | Evidence convergence is real, but signed trace and ObsMem handoff are not yet complete enough for acceptance. Keep routed to v0.91.7/v0.92 bridge work. |

## Deferred ADR Candidates

These are important, but this issue should not draft or promote them unless the
operator explicitly widens scope:

- ACIP/A2A protobuf wire-format decision.
- Memory Palace / long-running context architecture.
- Curiosity engine architecture.
- Observatory/Unity inhabitant readiness.
- AWS account/Terraform bootstrap architecture.
- Strategic Cognitive Reserve / Opticon platform architecture.

## Review Checklist

- Proposed entries are not labeled as accepted.
- Every candidate cites tracked evidence.
- No ADR claims v0.92 activation readiness.
- No runtime, provider, SSM, GitHub, scheduler, or validation implementation is
  implied by this docs packet.
- Existing accepted ADRs are not rewritten by this issue.
- Candidate ADRs 0029-0034 are either promoted through a later explicit ADR
  issue or left with visible deferred routing.

## Closeout Position

This packet is sufficient for `#4324` when it is linked from the v0.91.6
decision log and from the candidate ADR catalog, and when focused docs
validation plus bounded review confirm the packet is evidence-bound.
