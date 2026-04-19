# Candidate ADRs

## Status

These are proposed decisions extracted from the v0.90 architecture packet. They
are not accepted ADRs until reviewed and promoted.

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
  worktrees with STP/SIP/SOR truth recorded before finish and closeout.
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
