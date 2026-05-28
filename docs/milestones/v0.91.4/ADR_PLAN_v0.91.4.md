# v0.91.4 ADR Plan

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Status: candidate packet authored
- Issue: `#3444`

## Purpose

Record the Architecture Decision Record candidates needed for the v0.91.4
C-SDLC completion milestone.

This file is a planning and review index. It does not accept the ADRs. Accepted
ADRs live under `docs/adr/` only after explicit review and promotion.

## Candidate ADRs To Review

| Candidate | Title | Primary Source Surfaces |
| --- | --- | --- |
| `0029` | C-SDLC Default Software-Development Lane | `docs/milestones/v0.91.4/DESIGN_v0.91.4.md`, `docs/milestones/v0.91.4/DECISIONS_v0.91.4.md`, `docs/milestones/v0.91.4/features/COGNITIVE_SDLC_DEFAULT_OPERATION.md`, `docs/milestones/v0.91.4/features/SPRINT_CONDUCTOR_DEFAULT_CSDL_LANE.md` |
| `0030` | Software Development Polis Actor Standing And Shard Ownership | `docs/milestones/v0.91.4/features/SOFTWARE_DEVELOPMENT_POLIS_AND_ACTOR_STANDING.md`, `docs/milestones/v0.91.4/features/SHARD_OWNERSHIP_AND_INTERFACE_FREEZE.md`, `docs/milestones/v0.91.4/DESIGN_v0.91.4.md`, `docs/milestones/v0.91.4/DECISIONS_v0.91.4.md` |
| `0031` | C-SDLC Multi-Agent Parallel Execution Boundary | `docs/milestones/v0.91.4/DESIGN_v0.91.4.md`, `docs/milestones/v0.91.4/features/SHARD_OWNERSHIP_AND_INTERFACE_FREEZE.md`, `docs/milestones/v0.91.4/features/SOFTWARE_DEVELOPMENT_POLIS_AND_ACTOR_STANDING.md`, `docs/milestones/v0.91.4/features/PROCESS_DRIFT_REGRESSION_FIXTURES.md` |
| `0032` | Parallel Validation Fabric | `docs/milestones/v0.91.4/features/PARALLEL_VALIDATION_FABRIC.md`, `docs/milestones/v0.91.4/features/PVF_VALIDATION_LANE_TAXONOMY_v0.91.4.md`, `docs/milestones/v0.91.4/features/PVF_VALIDATION_LANE_MANIFEST_SCHEMA_v0.91.4.json`, `docs/milestones/v0.91.4/FIVE_MINUTE_SPRINT_REPEATABILITY_REPORT_2026-05-27.md` |
| `0033` | Merge-Readiness And PR Gate Truth Boundary | `docs/milestones/v0.91.4/features/MERGE_READINESS_AND_PR_GATE_HARDENING.md`, `docs/tooling/merge_readiness_gate_policy_v0.91.4.md`, `docs/milestones/v0.91.4/review/merge_readiness/MERGE_READINESS_GATE_PACKET_v0.91.4.md` |
| `0034` | C-SDLC Evidence Convergence, Signed Trace, And ObsMem Handoff | `docs/milestones/v0.91.4/features/EVIDENCE_CONVERGENCE_REVIEW_SYNTHESIS_AND_SIGNED_TRACE.md`, `docs/milestones/v0.91.4/features/OBSMEM_TRANSITION_MEMORY_INTEGRATION.md`, `docs/milestones/v0.91.4/DESIGN_v0.91.4.md`, `docs/milestones/v0.91.4/DECISIONS_v0.91.4.md` |

## Existing ADR Relationships

- ADR `0018` establishes durable planning and review artifacts.
- ADR `0024` establishes workflow guardrails as architecture policy.
- ADR `0028` establishes tracked C-SDLC workflow state and signed trace
  direction.

The v0.91.4 candidates refine these accepted decisions for default operation,
multi-agent execution, validation sharding, merge gates, and evidence/memory
handoff.

## Deferred Or Folded Decisions

- Active issue migration is currently an operational migration policy, not a
  standalone ADR candidate. Revisit only if review concludes the policy should
  become durable architecture doctrine.
- CodeFriend pre-alpha publication remains covered by ADR `0025` and v0.91.4
  sidecar planning. Revisit only if repo/S3/CloudFront publication changes
  product authority or release policy.

## Review Checklist

- Each candidate remains visibly candidate/proposed, not accepted.
- Each candidate cites tracked v0.91.4 source evidence.
- No candidate claims v0.91.4 release completion before release evidence exists.
- No candidate grants unbounded multi-agent autonomy.
- No candidate weakens human review, GitHub PRs, CI, branch protection, or
  closeout.
- Signed trace claims remain scoped to the v0.91.4 minimal proof surface, not
  full trace-query completion.

## Promotion Boundary

Promotion to accepted ADRs should happen only after review confirms that the
candidate decision is source-grounded, internally consistent, and still aligned
with the final v0.91.4 evidence packet.
