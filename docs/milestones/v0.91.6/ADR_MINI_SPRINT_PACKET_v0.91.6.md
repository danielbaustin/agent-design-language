# v0.91.6 ADR Mini-Sprint Packet

Issue: `#4324`, dispositioned by `#4476`
Status: `candidate_packet_dispositioned`
Milestone: `v0.91.6`

## Purpose

This packet records the architecture decision work needed before the v0.91.6
release tail closes. It began as a curation and review surface. Issue `#4476`
then dispositioned the candidate set and promoted the acceptance-ready records
into `docs/adr/`.

The entries below are not silently accepted by this packet. Accepted records
are explicitly listed in `docs/adr/`; deferred candidates remain visible with
their active routes.

Accepted ADRs live in `docs/adr/`. Candidate draft copies and candidate
catalogs live in `docs/architecture/adr/`.

`#4416` adds a retained content-review packet for this candidate set at
`docs/milestones/v0.91.6/review/V0916_ADR_CANDIDATE_CONTENT_REVIEW_4416.md`.
That review repairs current-state sprint truth and records promotion gates.
Issue `#4476` records the final v0.91.6 disposition.

## Scope

This mini-sprint covers two tasks:

1. Route the existing v0.91.4 C-SDLC ADR candidates `0029` through `0034`.
2. Add the v0.91.6 decision candidates that emerged from the milestone's
   completed and retained review packets.

It does not implement runtime, provider, scheduler, validation, AWS, SSM,
public-record, or GitHub projection behavior.

## Required v0.91.6 ADR Candidates

| Candidate | Disposition | Why it is needed before closeout | Primary evidence |
| --- | --- | --- | --- |
| [ADR 0035: Local Polis SSM Operations Boundary](../../architecture/adr/0035-local-polis-ssm-operations-boundary.md) | accepted as `docs/adr/0035-local-polis-ssm-operations-boundary.md` | `#4109` and `#4113` establish AWS SSM as an operations and observability bridge, not a source of polis authority. This boundary should become durable before SSM expands to `nessus.local`, `opticon.local`, edge nodes, or non-AWS clouds. | `review/security/LOCAL_POLIS_SSM_OPERATIONS_BRIDGE_4109.md`; `review/security/LOCAL_POLIS_SSM_PROOF_4113.md` |
| [ADR 0036: Validation Lane Selector / PVF Test-Cost Policy](../../architecture/adr/0036-validation-lane-selector-pvf-test-cost-policy.md) | accepted as `docs/adr/0036-validation-lane-selector-pvf-test-cost-policy.md` | v0.91.6 made focused validation selection project-critical after the test surface expanded. The architecture decision should lock the rule that normal PR work runs the smallest deterministic proving lane, while ambiguous and release-gate surfaces fail closed or escalate. | `docs/architecture/VALIDATION_LANE_SELECTOR.md`; `review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md`; `review/PVF_LONG_VALIDATION_LANE_INDEX_4223.md` |
| [ADR 0037: GitHub/C-SDLC Projection Ownership](../../architecture/adr/0037-github-csdlc-projection-ownership.md) | accepted as `docs/adr/0037-github-csdlc-projection-ownership.md` | `#4047` classified GitHub surfaces as managed projections, drift-checked projections, linked external state, card-local truth, or deferred surfaces. That distinction is now architecture, not just a report. | `review/github_projection/GITHUB_CSDLC_PROJECTION_MAP_4047.md`; `review/CSDLC_GITHUB_PROJECTION_CONVERGENCE_REVIEW_3935.md` |
| [ADR 0038: Runtime Integration Soak Boundary](../../architecture/adr/0038-runtime-integration-soak-boundary.md) | accepted as `docs/adr/0038-runtime-integration-soak-boundary.md` | The milestone explicitly rejects the claim that separately implemented pieces equal an integrated runtime. Soak #1 and Soak #2 need a durable decision boundary before v0.92 runtime-coherence claims. | `RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md`; `RUNTIME_FIRE_UP_PLAN_v0.91.6.md`; `features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md` |
| [ADR 0039: Cognitive Scheduler v1 Authority Boundary](../../architecture/adr/0039-cognitive-scheduler-v1-authority-boundary.md) | accepted as `docs/adr/0039-cognitive-scheduler-v1-authority-boundary.md` | `#4107` implemented deterministic lane-selection evidence, but did not implement timed execution, GitHub mutation, provider choice authority, or autonomous sprint conduction. That non-authority boundary should be durable before later scheduler expansion. | `review/scheduler/COGNITIVE_SCHEDULER_V1_4107.md`; `review/scheduler/SCHEDULER_ECONOMICS_INPUTS_4106.md`; `features/COGNITIVE_SCHEDULER_v0.91.6.md` |
| [ADR 0040: Workflow Lockfile Discipline](../../architecture/adr/0040-workflow-lockfile-discipline.md) | deferred until evidence packet exists | The `#4306` incident showed lifecycle tooling can create large, misleading validation cost if dependency resolution mutates `Cargo.lock` implicitly. The durable decision should require locked lifecycle fallback and explicit lockfile artifacts, but the release tail still needs a retained local source packet for the merged fix before promotion. | `review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md` |
| [ADR 0041: Provider/Model Suitability Boundary v2](../../architecture/adr/0041-provider-model-suitability-boundary-v2.md) | accepted as `docs/adr/0041-provider-model-suitability-boundary-v2.md` | WP-05 separated provider availability, capability profiles, role suitability, model reliability, and advisory role-provider authority. Existing ADR 0004 covers provider profiles, but v0.91.6 adds model-role suitability and multi-agent readiness evidence boundaries. | `review/provider/WP05_PROVIDER_MINI_SPRINT_CLOSEOUT_3970.md`; `features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`; `review/provider/PROVIDER_RELIABILITY_CLOSEOUT_MATRIX_4012.md` |
| [ADR 0042: Public Prompt Records Publication Boundary](../../architecture/adr/0042-public-prompt-records-publication-boundary.md) | accepted as `docs/adr/0042-public-prompt-records-publication-boundary.md` | WP-04 established that `.adl` authoring records are not automatically public-safe. Public consumption requires export contract, redaction, validation, indexing, and security/CAV handoff rules. | `features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md`; `review/public_prompt_records/PUBLIC_PROMPT_RECORDS_DISTRIBUTION_CLOSEOUT_4006.md`; `review/public_prompt_records/PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md` |

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

The proposed ADR drafts were tracked for review/promotion routing by this child
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

These child issues were review and promotion-routing surfaces. Final
acceptance/defer truth is recorded by `#4476`.

## Post-Review Candidate Disposition Route

Issue `#4383` repaired the release-tail ADR routing truth after the retained
candidate review packets landed. Issue `#4476` completed the
acceptance/defer disposition pass.

| Candidate | Final disposition | Target route or accepted record | Evidence note |
| --- | --- | --- | --- |
| ADR 0035: Local Polis SSM Operations Boundary | accepted | `docs/adr/0035-local-polis-ssm-operations-boundary.md` | SSM remains operations-plane authority only. |
| ADR 0036: Validation Lane Selector / PVF Test-Cost Policy | accepted | `docs/adr/0036-validation-lane-selector-pvf-test-cost-policy.md` | Validation selector/PVF evidence supports deterministic focused validation and fail-closed escalation. |
| ADR 0037: GitHub/C-SDLC Projection Ownership | accepted | `docs/adr/0037-github-csdlc-projection-ownership.md` | Current GitHub projection/convergence truth supports the ownership classes while leaving deferred automation visible. |
| ADR 0038: Runtime Integration Soak Boundary | accepted | `docs/adr/0038-runtime-integration-soak-boundary.md` | Soak #1/#2 boundaries are accepted without claiming v0.92 runtime readiness. |
| ADR 0039: Cognitive Scheduler v1 Authority Boundary | accepted | `docs/adr/0039-cognitive-scheduler-v1-authority-boundary.md` | Scheduler v1 remains planning/evidence only. |
| ADR 0040: Workflow Lockfile Discipline | deferred until evidence | `v0.91.7` evidence packet route | Requires durable lockfile-discipline source packet or exact tracked files and validation proof before acceptance. |
| ADR 0041: Provider/Model Suitability Boundary v2 | accepted | `docs/adr/0041-provider-model-suitability-boundary-v2.md` | Provider availability, capability, role suitability, reliability, failure modes, and advisory authority remain distinct. |
| ADR 0042: Public Prompt Records Publication Boundary | accepted | `docs/adr/0042-public-prompt-records-publication-boundary.md` | Public prompt records remain reviewed/redacted/indexed projections, not raw `.adl` publication. |

`#4476` is the ADR disposition issue. It accepted ADR 0035, ADR 0036, ADR
0037, ADR 0038, ADR 0039, ADR 0041, and ADR 0042. It deferred ADR 0040 until
the lockfile-discipline evidence packet is captured.

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

These older C-SDLC candidates are dispositioned by `#4476` alongside the
v0.91.6 release-tail candidates.

| Candidate | Final disposition | Target route or accepted record | Rationale |
| --- | --- | --- | --- |
| ADR 0029: C-SDLC Default Software-Development Lane | accepted | `docs/adr/0029-c-sdlc-default-software-development-lane.md` | Root `AGENTS.md`, workflow-conductor usage, and v0.91.6 issue execution show this has become active architecture policy. |
| ADR 0030: Software Development Polis Actor Standing And Shard Ownership | deferred | `v0.91.7`/`v0.93` governance | Actor standing and shard ownership are not fully enforced enough to accept yet. |
| ADR 0031: C-SDLC Multi-Agent Parallel Execution Boundary | deferred | `v0.91.7` C-SDLC hardening | Durable multi-agent shard enforcement and integration review need stronger evidence before acceptance. |
| ADR 0032: Parallel Validation Fabric | accepted | `docs/adr/0032-parallel-validation-fabric.md` | v0.91.6 validation-manager/test-tax work made PVF-style lane selection active project architecture. |
| ADR 0033: Merge Readiness And PR Gate Truth Boundary | accepted | `docs/adr/0033-merge-readiness-and-pr-gate-truth-boundary.md` | v0.91.6 closeout, review, projection, and lockfile incidents reinforce this as active architecture policy. |
| ADR 0034: C-SDLC Evidence Convergence, Signed Trace, And ObsMem Handoff | deferred | `v0.91.7`/`v0.92` bridge | Evidence convergence is real, but signed trace and ObsMem handoff are not complete enough for acceptance. |

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

- Accepted entries are explicitly promoted into `docs/adr/`; deferred entries remain visible.
- Every candidate cites tracked evidence.
- No ADR claims v0.92 activation readiness.
- No runtime, provider, SSM, GitHub, scheduler, or validation implementation is
  implied by this docs packet.
- Existing accepted ADRs before ADR 0029 are not rewritten by this issue.
- Candidate ADRs 0029-0034 are dispositioned by `#4476`.

## Closeout Position

This packet is dispositioned for `#4324`/`#4476`. Acceptance-ready ADRs are in
`docs/adr/`; deferred candidates remain in `docs/architecture/adr/` with active
routes.
