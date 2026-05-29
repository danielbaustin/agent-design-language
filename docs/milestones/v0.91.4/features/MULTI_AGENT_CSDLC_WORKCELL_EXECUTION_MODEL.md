# Multi-Agent C-SDLC Workcell Execution Model

## Status

Planned `v0.91.4` feature and design source for the bounded multi-agent
C-SDLC workcell mini-sprint.

## Purpose

Define how ADL can coordinate a small number of parallel issue lanes without
weakening card truth, worktree isolation, review truth, PR gates, validation
boundaries, or closeout discipline.

This is a bounded execution model for the first proof sprint. It is not a claim
that ADL now supports unbounded autonomous multi-agent software delivery.

## Core Roles

### Conductor

The conductor owns sprint-level control-plane truth.

Responsibilities:
- admit or reject candidate shards before work starts
- confirm child issue order, dependency truth, and stop conditions
- assign non-overlapping shard ownership
- keep sprint state, child state, and closeout truth aligned
- stop the workcell when a blocking dependency, review failure, merge conflict,
  or truth drift appears

The conductor does not implement shard work itself and does not silently merge,
close, or skip review.

### Worker

A worker owns one bounded shard with one explicit write set.

Responsibilities:
- execute only the assigned issue/shard scope
- work only in its bound branch and worktree
- run the smallest proving validation for the touched surface
- update issue-local cards truthfully when execution diverges from plan
- stop and escalate on dependency drift, overlapping writes, or scope pressure

Workers do not self-assign, merge, or absorb adjacent shards.

### Reviewer

A reviewer owns bounded pre-PR or proof-packet review over shard output.

Responsibilities:
- inspect the changed work product without widening scope
- record findings, non-findings, and residual risks clearly
- verify that claimed parallelism matches the actual shard boundaries and
  validation evidence

The reviewer is independent from the workers whose output is under review.

### Janitor

The janitor owns bounded post-publication repair when a shard PR hits a clear
mechanical blocker.

Responsibilities:
- diagnose failed checks, merge conflicts, linkage errors, or review nits
- apply narrow fixes inside the shard's declared scope
- preserve the original issue/PR topology and truth surfaces

The janitor does not convert a blocked shard into fresh feature work.

### Closeout

The closeout lane owns issue and sprint terminal truth.

Responsibilities:
- confirm GitHub issue/PR state, retained records, and closeout artifacts agree
- record what landed, what was pruned, what remains local-only, and what follow-on
  work exists
- classify the proof outcome as proving, non-proving, blocked, or failed

## Admission Rules

A shard may be assigned to a worker only when all of the following are true:
- the issue has a complete five-card bundle from the active template registry
- `SIP`, `STP`, and `SPP` are issue-specific and design-time ready
- `SRP` is present in pre-review state and `SOR` is present in scaffold state
- the shard has a declared write set and repo surface
- the conductor has checked dependency readiness for upstream shards
- no open or planned shard has an overlapping write set that would create
  ambiguous ownership
- the validation lane for the shard is known and proportionate

If any one of these is false, the shard remains unassigned.

## Write-Set And Dependency Rules

### Write-set rules

- One worker owns one shard write set at a time.
- Overlapping tracked-file ownership is not allowed for the first proof sprint.
- Shared planning or retained state surfaces must remain conductor-owned unless
  a single explicit shard is assigned to update them.
- Reviewers and janitors may comment on or repair a shard, but they do so inside
  that shard's ownership boundary.

### Dependency rules

- A shard may start only after its declared prerequisite shards are either:
  - complete and reviewable, or
  - explicitly modeled as independent by the conductor
- A shard that depends on another shard's design output must consume that output
  as a source input rather than recreating it ad hoc.
- Downstream proof/demo shards may not start while upstream model/planner/state
  shards are still speculative.

## Parallelism Rules

The first proof sprint allows bounded parallelism only where ownership and proof
stay legible.

### May run in parallel

- independent worker shards with disjoint write sets
- independent reviewer work over already-published shard outputs
- janitor work on one shard while another independent shard remains in local
  implementation state, provided ownership does not overlap
- focused shard-local validation when the validation surface is shard-bounded

### Must remain serialized

- sprint admission and initial shard assignment
- any transition that mutates shared sprint-state truth
- merge order when downstream shards depend on upstream landed output
- sprint closeout and proof classification
- any work involving overlapping write surfaces or unresolved dependency drift

## Lifecycle And Truth Rules

Parallel work does not change the underlying card lifecycle.

Each shard still uses:
- `SIP -> STP -> SPP -> SRP -> SOR`

Parallel execution must preserve:
- issue-local `SPP` as the operative shard plan
- `SRP` as the review-result surface for the shard
- `SOR` as the shard outcome and integration truth surface
- normal PR review before merge
- normal issue closeout after merge or intentional stop

Sprint state remains conductor-owned. `SPP` is never redefined as a sprint-level
control artifact.

## Evidence And Handoff Model

### SRP and SOR treatment

For the bounded proof sprint, each shard should record:
- assigned role and owned surface
- declared dependency assumptions
- focused validation actually run
- review findings and dispositions
- whether the shard finished, blocked, failed, or deferred

The sprint-level proof packet should then synthesize:
- which shards ran in parallel
- which transitions remained serialized
- coordination latency and validation latency where visible
- residual risks and follow-on issues

### Signed trace expectation

This model does not require a production-grade distributed trace system for the
first proof, but it does require a minimal reviewer-facing trace of:
- admission decision
- shard assignment
- branch/worktree binding
- validation claims
- review decisions
- closeout outcome

### ObsMem handoff boundary

ObsMem or later memory ingestion should consume tracked evidence and explicit
proof packets, not private subagent chat or untracked operator notes.

## Stop Conditions

The conductor must stop or route for operator judgment when:
- two shards need the same tracked file set
- a dependency shard is incomplete or unreviewed
- validation evidence is missing or contradictory
- a reviewer finds a blocking issue in the shard model or execution record
- a janitor repair would widen scope beyond the shard
- sprint state and live GitHub truth drift apart

## Proof Slice Recommendation For `#3419`

The later bounded proof sprint should use the smallest credible slice that can
show real parallelism without write-set ambiguity.

Recommended proof shape:
- two worker shards on disjoint docs/policy surfaces
- one independent reviewer lane over both shard outputs
- one conductor-owned sprint-state/control-plane lane
- optional bounded janitor lane only if a real PR/check blocker appears

Recommended proof constraints:
- avoid risky overlapping implementation code as the first proof
- prefer docs/tools surfaces with clear ownership and focused validation
- require branch/worktree evidence for every shard
- require one synthesized proof packet that distinguishes:
  - parallel worker time
  - serialized review/merge gates
  - blocked or deferred follow-on work

This keeps `#3419` honest: it should prove bounded multi-agent coordination, not
claim general production autonomy.

## Relationship To Later Children

- `#3417` should turn the admission and assignment rules here into a concrete
  shard planner surface.
- `#3418` should define the state artifacts and conductor hook points needed to
  record this model operationally.
- `#3419` should execute only a bounded proof slice that conforms to this model.

## Non-Claims

This model does not claim:
- unrestricted parallel execution for all ADL work
- autonomous merge authority
- hidden local-only state as canonical truth
- replacement of human review or protected branch policy
