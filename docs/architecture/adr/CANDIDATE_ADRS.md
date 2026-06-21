# Candidate ADRs

## Status

The unpromoted entries in this file are proposed decisions from the source
packets named in each section. They are not accepted ADRs until reviewed and
promoted into `docs/adr/`.

## Candidate 0001: Trace And Artifacts As Runtime Truth

- Proposed status: candidate
- Context: ADL runtime behavior can be misunderstood if final prose summaries
  are treated as proof.
- Proposed decision: runtime truth should be reconstructed from trace events
  and deterministic run artifacts, with prose reports treated as interpretation.
- Consequences: validators and reviewers should prefer trace/artifact evidence
  over ungrounded claims.
- Evidence: `adl/src/trace.rs`, `adl/src/artifacts.rs`,
  `docs/architecture/TRACE_SYSTEM_ARCHITECTURE.md`.

## Candidate 0002: Worktree-First Issue Execution

- Proposed status: candidate
- Context: implementation on the root checkout creates drift, conflicts, and
  closeout ambiguity.
- Proposed decision: tracked issue implementation should run in issue-specific
  worktrees with SIP/STP/SPP/SRP/SOR truth recorded before finish and closeout.
- Consequences: conductor and lifecycle skills must route to worktree-bound
  execution, and closeout must reconcile GitHub and local truth.
- Evidence: `docs/default_workflow.md`, `adl/src/control_plane.rs`, `pr-*`
  skills.

## Candidate 0003: Modular Skill Composition

- Proposed status: candidate
- Context: conductor-style prompts can accidentally absorb downstream skills and
  hide lifecycle responsibility.
- Proposed decision: the conductor coordinates and routes, while lifecycle and
  specialist skills own their bounded tasks.
- Consequences: multi-skill workflows remain auditable and easier to debug.
- Evidence: `docs/milestones/v0.89.1/features/SKILL_COMPOSITION_MODEL.md`,
  `docs/milestones/v0.89.1/features/OPERATIONAL_SKILLS_SUBSTRATE.md`.

## Candidate 0004: Long-Lived Agents Are Cycle-Bounded

- Proposed status: promoted as `docs/adr/0011-long-lived-agent-runtime.md`
- Context: continuous agents can obscure state, authority, and failure history
  unless each cycle is bounded.
- Proposed decision: long-lived agents should emit cycle-scoped leases, status,
  manifests, observations, decisions, run refs, guardrails, continuity records,
  and inspection packets.
- Consequences: long-lived demos can show continuity without bypassing operator
  control or artifact truth.
- Evidence: `adl/src/long_lived_agent.rs`,
  `docs/milestones/v0.90/DESIGN_v0.90.md`.

## v0.91.2 Candidate ADR Packet

These candidate records were created for the v0.91.2 ADR authoring pass and
promoted to accepted ADRs during the v0.91.3 review tail. The accepted records
live in `docs/adr/`; the files in this directory remain the source candidate
copies for provenance.

| Candidate | Accepted record | Summary |
| --- | --- | --- |
| ADR 0020 | `../../adr/0020-universal-tool-schema-portable-tool-description-standard.md` | UTS is portable tool description, not execution authority. |
| ADR 0021 | `../../adr/0021-adl-capability-contract-runtime-authority-boundary.md` | ACC is ADL-native governed runtime authority. |
| ADR 0022 | `../../adr/0022-speculative-decoding-deterministic-commit-boundary.md` | Speculative decoding may accelerate proposals, not commits. |
| ADR 0023 | `../../adr/0023-google-workspace-cms-bridge-canonical-promotion-boundary.md` | GWS is collaboration infrastructure, not canonical repo truth. |
| ADR 0024 | `../../adr/0024-workflow-guardrails-issue-lifecycle-control-plane.md` | ADL issue lifecycle discipline is architecture policy. |
| ADR 0025 | `../../adr/0025-codefriend-review-packet-product-boundary.md` | CodeFriend is evidence-bound review/report workflow. |
| ADR 0026 | `../../adr/0026-repo-visibility-manifest-linkage-layer.md` | Repo visibility is manifest/linkage support, not full repo cognition. |
| ADR 0027 | `../../adr/0027-governed-code-modernization-moderne-openrewrite-lst.md` | Modernization remains dry-run/review/approval bounded. |
| ADR 0028 | `../../adr/0028-c-sdlc-tracked-workflow-state-and-signed-trace.md` | C-SDLC durable workflow truth becomes tracked and signed-trace-backed. |

## v0.91.4 Candidate ADR Packet

These candidate records were created for the v0.91.4 C-SDLC completion pass.
They are proposed decisions only. They should not be treated as accepted until
reviewed and promoted into `docs/adr/`.

| Candidate | Source focus | Summary |
| --- | --- | --- |
| [ADR 0029](0029-c-sdlc-default-software-development-lane.md) | C-SDLC default operation | C-SDLC becomes the default ADL software-development lane once v0.91.4 completion evidence supports the claim. |
| [ADR 0030](0030-software-development-polis-actor-standing-and-shard-ownership.md) | Actor standing and shard ownership | C-SDLC work is governed as a software-development polis with bounded actor standing, shard ownership, and interface-freeze rules. |
| [ADR 0031](0031-c-sdlc-multi-agent-parallel-execution-boundary.md) | Multi-agent parallel execution | Multi-agent C-SDLC execution is allowed only behind conductor-owned shard, review, merge, and closeout boundaries. |
| [ADR 0032](0032-parallel-validation-fabric.md) | Parallel Validation Fabric | Validation decomposes into lane-scoped proof without hiding failed, pending, deferred, or blocked evidence. |
| [ADR 0033](0033-merge-readiness-and-pr-gate-truth-boundary.md) | Merge readiness and PR gates | Merge readiness is a convergence boundary across issue, branch, PR, CI, review, evidence, trace, and closeout truth. |
| [ADR 0034](0034-c-sdlc-evidence-convergence-signed-trace-and-obsmem-handoff.md) | Evidence convergence and memory handoff | C-SDLC durable proof links SRP/SOR evidence, signed trace verification, review synthesis, and ObsMem handoff. |

Deferred or folded candidates:

- Active issue migration remains a v0.91.4 operational policy surface unless a
  later review decides it needs a standalone accepted architecture decision.
- CodeFriend pre-alpha publication remains covered by ADR 0025 and current
  sidecar milestone planning unless a future product-boundary change requires a
  separate ADR.

## v0.91.6 ADR Mini-Sprint Candidate Packet

Issue `#4324` records the v0.91.6 release-tail ADR mini-sprint packet at
`../../milestones/v0.91.6/ADR_MINI_SPRINT_PACKET_v0.91.6.md`.

The packet is a proposed candidate catalog and routing surface. It does not
accept new ADRs by itself.

Required v0.91.6 candidates:

| Candidate | Proposed status | Summary |
| --- | --- | --- |
| [ADR 0035: Local Polis SSM Operations Boundary](0035-local-polis-ssm-operations-boundary.md) | candidate_required | AWS SSM may act as an operations/observability bridge for approved local polis hosts, but not as authority for polis state, governance, memory, identity, provider selection, or model content. |
| [ADR 0036: Validation Lane Selector / PVF Test-Cost Policy](0036-validation-lane-selector-pvf-test-cost-policy.md) | candidate_required | Normal PR work should run deterministic focused validation lanes selected from tracked metadata, while ambiguous and release-gate surfaces escalate or fail closed. |
| [ADR 0037: GitHub/C-SDLC Projection Ownership](0037-github-csdlc-projection-ownership.md) | candidate_required | C-SDLC cards remain authority; GitHub surfaces are managed projections, drift-checked projections, linked external state, card-local-only surfaces, or deferred. |
| [ADR 0038: Runtime Integration Soak Boundary](0038-runtime-integration-soak-boundary.md) | candidate_required | Runtime coherence requires integrated soak evidence; Soak #1 is a walking skeleton and Soak #2 gates full feature-list readiness before v0.92. |
| [ADR 0039: Cognitive Scheduler v1 Authority Boundary](0039-cognitive-scheduler-v1-authority-boundary.md) | candidate_required | Scheduler v1 produces deterministic lane-selection evidence and does not execute tasks, mutate GitHub, select live providers, or own timed automation. |
| [ADR 0040: Workflow Lockfile Discipline](0040-workflow-lockfile-discipline.md) | candidate_required_needs_source_packet | Lifecycle tooling must use locked dependency resolution and treat lockfile changes as explicit issue-scoped artifacts; promotion still needs a retained local source packet for the `#4306` fix. |
| [ADR 0041: Provider/Model Suitability Boundary v2](0041-provider-model-suitability-boundary-v2.md) | candidate_required | Provider availability, capability profiles, model-role suitability, reliability evidence, and role-provider advisory authority are distinct surfaces. |
| [ADR 0042: Public Prompt Records Publication Boundary](0042-public-prompt-records-publication-boundary.md) | candidate_required | `.adl` authoring records become public only through reviewed export, redaction, validation, indexing, and security/CAV handoff paths. |

Existing candidate routing from the same packet:

| Candidate | v0.91.6 recommendation |
| --- | --- |
| ADR 0029: C-SDLC Default Software-Development Lane | promote or refresh then promote |
| ADR 0030: Software Development Polis Actor Standing And Shard Ownership | defer with active route |
| ADR 0031: C-SDLC Multi-Agent Parallel Execution Boundary | defer with active route |
| ADR 0032: Parallel Validation Fabric | promote or refresh then promote |
| ADR 0033: Merge Readiness And PR Gate Truth Boundary | promote or refresh then promote |
| ADR 0034: C-SDLC Evidence Convergence, Signed Trace, And ObsMem Handoff | defer with active route |
