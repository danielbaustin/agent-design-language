# ADL Architecture

## Scope And Evidence

This document describes the tracked ADL runtime, control-plane, workflow-skill,
and review architecture as of v0.90. It is source-grounded in public repository
files only. It does not claim behavior from private worktrees, local traces, or
untracked planning notes.

Primary evidence:

- `adl/src/adl/types.rs` defines providers, tools, agents, tasks, runs,
  workflows, steps, delegation metadata, and conversation metadata.
- `adl/src/execution_plan.rs` compiles workflows and reusable patterns into
  acyclic execution plans.
- `adl/src/execute/mod.rs` runs sequential and concurrent workflows, records
  lifecycle phases, enforces scheduler and delegation policy, and validates
  write paths.
- `adl/src/trace.rs` records lifecycle, scheduling, step, prompt, delegation,
  call, and completion events.
- `adl/src/artifacts.rs` defines deterministic run artifact paths.
- `adl/src/provider/mod.rs` and `docs/architecture/PROVIDER_CAPABILITY_AND_TRANSPORT_ARCHITECTURE.md`
  define provider construction, retry classification, and provider identity
  boundaries.
- `adl/src/signing.rs`, `adl/src/sandbox.rs`, and
  `adl/src/remote_exec/security.rs` define signing, sandbox path validation,
  and remote-execution envelope validation.
- `adl/src/control_plane.rs`, `docs/default_workflow.md`, and the `pr-*`
  skills define the issue, task-bundle, branch, worktree, PR, and closeout
  lifecycle.
- `docs/milestones/v0.89.1/features/OPERATIONAL_SKILLS_SUBSTRATE.md` and
  `docs/milestones/v0.89.1/features/SKILL_COMPOSITION_MODEL.md` define the
  skill composition model used by the ADL workflow skills.
- `docs/milestones/v0.90/DESIGN_v0.90.md` and `adl/src/long_lived_agent.rs`
  define the v0.90 long-lived-agent substrate.

## System Context

ADL is a repository-first agent orchestration system. It represents work as
versioned documents, compiles those documents into bounded execution plans,
runs those plans through explicit provider and tool boundaries, records
truth-bearing traces and artifacts, and uses a PR/worktree control plane for
human-reviewable repository changes.

The architecture has two closely related layers:

- The runtime layer executes ADL documents and emits reviewable artifacts.
- The authoring/control-plane layer turns issues into task bundles, binds
  isolated worktrees, publishes PRs, monitors review state, and closes issues
  truthfully.

The workflow-skill layer sits above both. Skills are not hidden magic prompts.
They are bounded procedures that route to specific lifecycle actions, edit
specific card types, run specialist review lanes, or generate artifacts with
explicit evidence and validation.

## Runtime Model

An ADL run starts with a typed document. Providers, tools, agents, tasks, and
runs are parsed into strongly typed Rust structures with unknown fields denied
on the core specs. A run resolves either a referenced workflow or an inline
workflow. A workflow contains steps with optional state outputs, write targets,
call semantics, delegation metadata, and conversation metadata.

The planner builds an `ExecutionPlan` from resolved steps. Saved state
references create dependencies. Duplicate step ids, duplicate saved-state keys,
unknown saved-state references, self-dependencies, and cycles are rejected
before execution. Concurrent workflows add fork/join structure while preserving
deterministic ordering.

Execution proceeds through explicit lifecycle phases. The runtime records
scheduler policy, step starts, prompt assembly hashes, output chunks,
delegation decisions, call entry/exit, failures, and run completion. The trace
is therefore the audit surface for what the runtime actually attempted, rather
than a retroactive prose summary.

## Authoring And Control Plane

The ADL control plane uses GitHub issues as work intents and repository-local
cards as the canonical execution packet:

- STP: the bounded task prompt and acceptance contract.
- SIP: the input state and worktree binding.
- SOR: the output record, validation results, PR state, and closeout truth.

`adl/src/control_plane.rs` defines deterministic issue prompt paths, task bundle
paths, branch names, and default worktree paths. The default ADL lifecycle is:

1. `pr init` creates or normalizes the issue prompt and task bundle.
2. `pr ready` verifies that the issue and cards are structurally ready.
3. `pr run` binds an execution worktree and performs bounded implementation.
4. `pr finish` stages intended paths, runs validation, and publishes or updates
   a draft PR.
5. `pr janitor` monitors PR checks, conflicts, and review state.
6. `pr closeout` verifies integration truth, normalizes final cards, and prunes
   the local execution surface.

The root checkout remains the stable coordination checkout. Tracked
implementation work belongs in issue-specific worktrees, not on the root branch.

## Task Bundle Lifecycle

Task bundles are intentionally separate from PR state. This prevents a merged PR
from being mistaken for a closed issue or a local worktree from being mistaken
for integrated repository truth.

The task bundle lifecycle is:

1. Issue intent is captured in a tracked or generated issue prompt.
2. STP/SIP/SOR cards are created in the versioned task area.
3. `pr run` copies or binds the executable packet into the worktree.
4. Implementation and validation happen inside the worktree.
5. `pr finish` records the publication state in SOR.
6. `pr closeout` reconciles GitHub closure, PR merge state, final cards, and
   local worktree cleanup.

This split is important for ADL because many failures in prior milestones were
not code failures. They were truth-model failures: closed GitHub issues with
stale cards, merged PRs with uncleared worktrees, or local residue that made
future operators think work was still active.

## Provider And Tool Boundaries

Providers are configured separately from agents and tasks. Agents reference a
provider and model; tasks define prompts and tool allowlists; runs bind workflow
and default behavior. Provider construction classifies schema errors, provider
runtime errors, timeouts, panics, and retryability. Provider identity is also
separated in the provider-capability architecture so human-readable model refs,
transport-specific ids, and policy decisions do not collapse into one string.

Tools are intentionally not equivalent to arbitrary filesystem or network
access. Tool specs are typed, task tool allowlists are explicit, and delegation
policy can evaluate tool invocation, provider calls, remote execution, and
filesystem read/write requests.

## Trace And Artifact Truth

ADL treats traces and artifacts as the truth model for runtime behavior. The
artifact layer builds deterministic paths under a run-scoped root, including
run metadata, step records, pause state, status, manifests, outputs, logs,
trace files, learning artifacts, and control-path proof artifacts.

The trace architecture provides event-level evidence for:

- lifecycle phase transitions
- scheduler and concurrency policy
- prompt assembly
- step output streaming
- delegation requests and policy decisions
- nested workflow calls
- failures and completion

This allows reviewers to distinguish what was executed from what was merely
planned or described.

## Security And Trust Boundaries

ADL has several explicit trust boundaries:

- Repository input boundary: ADL documents are parsed and validated before
  execution.
- Provider boundary: model calls cross from local runtime into local or remote
  providers.
- Tool boundary: task-level tool use is governed by allowlists and delegation
  policy.
- Filesystem boundary: write paths are resolved under sandbox constraints and
  host-absolute path leaks are avoided in stable errors.
- Remote execution boundary: execute requests can require signed envelopes,
  allowed algorithms, key sources, key ids, and requested-path validation.
- Publication boundary: generated review reports and architecture packets must
  pass redaction and evidence gates before being treated as customer-facing.

The architecture does not claim that all future workflows are safe by default.
It claims that safety-sensitive crossings are represented explicitly enough to
be validated, reviewed, and strengthened.

## Long-Lived Agent Layer

v0.90 adds a long-lived-agent substrate for bounded recurring work. The current
runtime state model includes specs, leases, status records, stop records, cycle
manifests, observations, decision requests/results, run refs, memory writes,
guardrail reports, continuity records, cycle ledger entries, provider bindings,
memory indexes, operator events, and inspection packets.

The key architectural constraint is that long-lived agents still run through
bounded cycles. A heartbeat may schedule repeated cycles, but each cycle must
have a lease, status, artifacts, and operator controls. The long-lived layer is
therefore an extension of the trace/artifact truth model, not a replacement for
bounded execution.

## Operational Skills

ADL skills implement the operational procedure around the runtime. The skill
composition model is a deterministic graph over stochastic nodes. A conductor
skill may route to a lifecycle skill, but it must not absorb the routed skill's
responsibility. Specialist skills can produce review packets, diagrams,
threat-boundary notes, test plans, issue candidates, ADR candidates, fitness
function plans, and product reports.

The important architectural invariant is modularity: the conductor coordinates,
the lifecycle skill executes the lifecycle step, and specialist skills produce
bounded artifacts. If a session bypasses this separation, it becomes difficult
to audit what happened and why.

## Review And Release Surfaces

ADL releases are backed by milestone docs, demos, validation scripts, review
packets, and GitHub issue/PR state. The v0.90 review stack adds CodeBuddy-style
skills for repo-packet building, code review, security review, test review,
docs review, architecture review, dependency review, diagram planning, diagram
authoring, redaction, issue planning, ADR curation, fitness-function planning,
product reporting, and review-quality evaluation.

Architecture review uses the same packet-first discipline:

- collect bounded source evidence
- assign specialist lanes
- synthesize findings without hiding disagreement
- generate diagrams only from evidence
- separate machine-checkable invariants from human judgment
- record residual risk and missing coverage

## Architecture Invariants

These invariants are intended to become automated checks where practical:

- Implementation work for tracked issues occurs in issue worktrees.
- STP/SIP/SOR cards exist before issue execution begins.
- SOR does not claim merge, closure, or validation that did not happen.
- Public architecture docs do not contain host-absolute private paths or secret
  markers.
- Diagrams include evidence, assumptions, and render or validation commands.
- Provider selection remains separate from provider transport ids and policy.
- Runtime traces distinguish execution from planning.
- Long-lived agents emit cycle-scoped truth instead of relying on memory alone.
- Closeout verifies GitHub state, local cards, and worktree cleanup.

## Known Gaps

- Documentation specialist automation is tracked separately and should help
  maintain this packet once available.
- Gap-analysis automation is tracked separately and should compare architecture
  claims against code, demos, and release docs.
- Closeout automation still needs permanent guardrails so merged issues cannot
  leave stale local truth behind.
- Architecture review quality should continue converging toward the third-party
  review bar: evidence-rich, severity-accurate, actionable, and candid about
  residual risk.

## Diagram Index

See `diagrams/DIAGRAM_PACKET.md` for diagram evidence and assumptions.

- `diagrams/system_context.mmd`
- `diagrams/runtime_lifecycle.mmd`
- `diagrams/control_plane_lifecycle.mmd`
- `diagrams/task_bundle_state.mmd`
- `diagrams/task_bundle_and_pr_lifecycle.mmd`
- `diagrams/skill_orchestration.mmd`
- `diagrams/artifact_data_flow.mmd`
- `diagrams/trust_boundaries.mmd`
- `diagrams/runtime_v2_subsystem_structure.mmd`
- `diagrams/codebuddy_review_flow.mmd`

## ADR Candidates

See `adr/CANDIDATE_ADRS.md`. The ADRs are candidates, not accepted decisions,
until a human reviewer explicitly promotes them.
