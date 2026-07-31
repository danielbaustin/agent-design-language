# Structured Planning Prompt

Template: 1.0.0

Issue: 5340

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the engine state machine, Runtime boundary, typed ports, COTS, protected paths, hard source/test/time budgets, PVF classes, no-deferral matrix, rollback, and exact #5338 gate during preparation; run bounded review and bind ownership; watch until #5338 is merged plus retained typed closed_out with merged-SHA ancestry; then align to the landed plan contract, implement, prove, review, publish, shepherd, merge, post-merge validate, close out, retain the receipt, and prune safely. Generated large-profile SPP/VPP totals are lifecycle and CI reservations; per-lane 120/120/300/600-second ceilings are authoritative acceptance limits.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Complete and review all six issue-specific cards, clean-room design and diagram, protected paths, COTS, LoC/test/time budgets, PVF classification, no-deferral matrix, rollback, and executable preparation validation; bind the dedicated owner claim without product implementation",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Maintain the permanent read-only dependency watch and only after #5338 GitHub merged plus retained typed closed_out receipt verify terminal disposition, observed merge SHA, sole-writer authority, and current-HEAD ancestry before implementation",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-9",
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Reconcile the landed #5338 API, then implement the isolated deterministic engine state machine, typed port protocol, bounds, joins, retry/failure/cancellation, checkpoint, and strict resume contracts",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Add complete #5338 fixture mapping, golden and negative cases, completion permutations, saturation boundaries, fresh-process resume equivalence, exact dependency/COTS/scope/LoC/latency gates, and FastWork proof",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run exact-revision validation and mandatory subagent code review, fix every actionable finding, publish through typed v2, shepherd green required CI and authorized merge, capture post-merge proof, complete typed closeout, retain the terminal receipt, and guarded-prune the worktree",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- The engine is a deterministic bounded state machine over inert plan data and typed inputs/effects
- Runtime v3 retains operational runtime authority; Runtime v2 and Runtime v3 source remain unchanged
- No product implementation before #5338 GitHub merged plus retained typed closed_out merged-SHA ancestry proof
- Every queue, attempt, payload, output, event, checkpoint, and logical turn has a non-zero explicit finite bound
- Checkpoint is quiescent-only and resume never guesses about an in-flight side effect
- Port requests/completions are typed and identity-bound; adapters and actual IO remain downstream
- No acceptance criterion or required release lane is deferred after the dependency gate opens
- The crate never depends on incumbent ADL, Runtime v2/v3, C-SDLC, async runtime, network/cloud/provider/database, graph/workflow/scheduler/retry/persistence, or RNG families
- All Cargo output remains under /Volumes/FastWork
- Primary main remains clean and all canonical #5340 artifacts remain in the dedicated worktree

## Risks

- The landed #5338 ExecutionPlan node, edge, port, digest, diagnostic, or fixture contract may differ materially from this provisional consumer boundary
- Milestone wording can blur ADL plan-level scheduling with Runtime v3 operational scheduling unless authority stays explicit
- Completion arrival order, unordered metadata, or host timing can leak nondeterminism into snapshots and effects
- Engine-owned retry, join, resume, cancellation, or duplicate-completion paths can accidentally reset attempts or reproduce a side effect
- Checkpoint compatibility can be too weak and accept a changed plan, policy, limits, or request sequence
- A hard source/test budget can pressure proof coverage or move logic into unmeasured helper surfaces
- A COTS convenience library can silently introduce async, timer, network, graph, scheduler, or RNG authority
- The permanent watch can become stale unless the typed claim is heartbeated and source-task state changes are reported

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5340/retained/design.md

Digest: 11d1545c9eb05a81f550d896c310688c192206d312f391d922d64310c2d6723e

## Diagram

.csdlc/issues/5340/retained/diagram.mmd

Digest: 618c0b1aae37521feccebd83320800419b86b3ad9c419ca5fac96406f1bdd8e7

## Stop Conditions

- #5338 is not GitHub merged, lacks a retained typed closed_out receipt, has non-merged terminal disposition, lacks observed merge SHA, or that SHA is not an ancestor of #5340 HEAD when product implementation would begin
- The landed #5338 ExecutionPlan or fixtures are missing, stale, unreviewed, ambiguous, or materially incompatible without typed replanning
- Required behavior would copy, adapt, import, link, or change incumbent ADL, Runtime v2, or Runtime v3 source
- Plan scheduling cannot remain distinct from Runtime v3 operational scheduling and supervision
- Any queue, attempt, join, payload, output, event, checkpoint, or logical-turn behavior would be unbounded or nondeterministic
- A proposed dependency is outside the exact reviewed adl-compiler, serde, serde_json, sha2, and hex set
- Implementation requires a shared workspace, Runtime, adapter, CLI, selector, or other path outside the protected #5340 scope
- Measured implementation LoC, test/fixture LoC, focused, quality, deterministic, or complete validation time exceeds the hard budget without typed exact-revision replanning and review
- Any validation would use a Cargo path outside /Volumes/FastWork, raw gh, AWS, live providers, or credentials
- Required review, CI, merge authorization, post-merge evidence, terminal receipt, or safe-prune truth is incomplete

## Handoff

Proceed only after doctor readiness.
