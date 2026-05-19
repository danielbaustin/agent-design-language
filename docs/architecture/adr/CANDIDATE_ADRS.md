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

## v0.91.2 Candidate ADR Packet

These candidate records were created for the v0.91.2 ADR authoring pass. They
are not accepted decisions until reviewed and promoted.

| Candidate | File | Summary |
| --- | --- | --- |
| ADR 0020 | `0020-universal-tool-schema-portable-tool-description-standard.md` | UTS is portable tool description, not execution authority. |
| ADR 0021 | `0021-adl-capability-contract-runtime-authority-boundary.md` | ACC is ADL-native governed runtime authority. |
| ADR 0022 | `0022-speculative-decoding-deterministic-commit-boundary.md` | Speculative decoding may accelerate proposals, not commits. |
| ADR 0023 | `0023-google-workspace-cms-bridge-canonical-promotion-boundary.md` | GWS is collaboration infrastructure, not canonical repo truth. |
| ADR 0024 | `0024-workflow-guardrails-issue-lifecycle-control-plane.md` | ADL issue lifecycle discipline is architecture policy. |
| ADR 0025 | `0025-codefriend-review-packet-product-boundary.md` | CodeFriend is evidence-bound review/report workflow. |
| ADR 0026 | `0026-repo-visibility-manifest-linkage-layer.md` | Repo visibility is manifest/linkage support, not full repo cognition. |
| ADR 0027 | `0027-governed-code-modernization-moderne-openrewrite-lst.md` | Modernization remains dry-run/review/approval bounded. |
| ADR 0028 | `0028-c-sdlc-tracked-workflow-state-and-signed-trace.md` | C-SDLC durable workflow truth becomes tracked and signed-trace-backed. |
