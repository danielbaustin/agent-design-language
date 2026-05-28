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
| `0029` | C-SDLC Default Software-Development Lane | `DESIGN_v0.91.4.md`, `DECISIONS_v0.91.4.md`, `features/COGNITIVE_SDLC_DEFAULT_OPERATION.md` |
| `0030` | Software Development Polis Actor Standing And Shard Ownership | `features/SOFTWARE_DEVELOPMENT_POLIS_AND_ACTOR_STANDING.md`, `features/SHARD_OWNERSHIP_AND_INTERFACE_FREEZE.md` |
| `0031` | C-SDLC Multi-Agent Parallel Execution Boundary | `DESIGN_v0.91.4.md`, shard ownership and actor-standing feature docs |
| `0032` | Parallel Validation Fabric | `features/PARALLEL_VALIDATION_FABRIC.md`, PVF taxonomy and manifest surfaces |
| `0033` | Merge-Readiness And PR Gate Truth Boundary | `features/MERGE_READINESS_AND_PR_GATE_HARDENING.md`, merge-readiness gate policy and proof packet |
| `0034` | C-SDLC Evidence Convergence, Signed Trace, And ObsMem Handoff | `features/EVIDENCE_CONVERGENCE_REVIEW_SYNTHESIS_AND_SIGNED_TRACE.md`, `features/OBSMEM_TRANSITION_MEMORY_INTEGRATION.md` |

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
