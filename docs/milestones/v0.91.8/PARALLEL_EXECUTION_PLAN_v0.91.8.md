# v0.91.8 Parallel Execution Plan

Status: planned. This document is a scheduling and readiness plan only. It
does not execute v0.91.8 work, prove release readiness, or approve v0.92
activation.

Routing authority: GitHub milestone configuration is not currently configured
as the operative gate for this package. The `version:v0.91.8` label plus the
checked-in [WP_ISSUE_WAVE_v0.91.8.yaml](WP_ISSUE_WAVE_v0.91.8.yaml) are the
current routing authority until live issue truth and operator direction replace
them.

## Goals

- Maximize safe parallelism without breaking dependency order, interface
  freezes, review authority, or closeout truth.
- Keep implementation issue-bound: no tracked issue work on `main`.
- Prepare cards one dependency wave ahead so executors can move quickly when a
  predecessor closes, without mutating every future issue at once.
- Preserve a single conductor, a global writable-actor WIP cap, and one
  integration/merge queue so parallel execution does not become unreviewable.

## Current Readiness Constraint

WP-01 `#5594` is the active readiness authority and milestone sprint `#5595`
is the single sprint umbrella. Closed `#5335` and `#5383` are historical
planning inputs, not active WP-01 owners.

The checked core, acceptance, WP-10A, and provider issues do not currently have
tracked `.csdlc/issues/<issue>/index.json` projections in the local planning
surface for:

`#5336`, `#5337`, `#5338`, `#5339`, `#5340`, `#5341`, `#5342`, `#5343`,
`#5344`, `#5345`, `#5346`, `#5347`, `#5349`, `#5350`, `#5358`, `#5361`,
`#5384`, `#5497`, `#5498`, `#5499`, `#5500`, `#5501`, `#5502`, `#5526`,
`#5548`, `#5558`, `#5589`, `#5590`, `#5591`, and `#5592`.

Therefore Wave 0 is not optional clerical setup. It is a real parallel
card/readiness factory using active prompt templates and typed C-SDLC v2
`init`/`validate` flows. Implementation is forbidden for each issue until its
issue-specific SIP, STP, SPP, and VPP are ready and validated.

WP-21A fails closed before publication if the canonical-document inventory
detects a missing or contradictory v0.91.8 planning, architecture, review,
release, handoff, routing, validation, or feature-doc surface.

## Roles

| Role | Authority | Limits |
| --- | --- | --- |
| Conductor | Owns dependency order, WIP cap, integration queue, merge queue, and readiness handoffs. | Does not implement, review its own work, or bypass typed C-SDLC state. |
| Issue executor | Implements one bound issue in its issue worktree. | Cannot widen scope, merge, close, or write outside protected paths. |
| Required internal reviewer | Reviews design-ready and final exact-revision surfaces. | Cannot merge or close the issue under review. |
| External-agent shadow reviewer | Produces evidence for the synthesizer. | Read-only only; never a lifecycle actor; cannot merge, close, create scope, or mutate repo state. |
| CI watcher / janitor | Watches checks, conflicts, and stale-base state; applies bounded fixes when authorized. | Does not absorb implementation scope or let failed runs continue unattended. |
| Findings synthesizer | Deduplicates review/shadow findings, retains or summarizes raw shadow artifacts when allowed, and records publication-safe findings through typed `csdlc-review`. | Does not auto-open one issue per finding and does not treat agreement as higher severity by itself. |

One named synthesizer records deduplicated findings. External shadows never
persist artifacts directly. The synthesizer or conductor owns raw Fable or
external-shadow artifact capture under uncommitted `.adl/local-artifacts/`;
only publication-safe normalized findings belong in SRP/SOR and issue-local
review artifacts.

## Global Controls

- Global writable-actor WIP cap: at most four writable issue/worktree actors
  may be active at once across implementation, card factory, acceptance,
  janitor fixes, and other mutation. Read-only shadows and watchers are
  excluded. A janitor that applies fixes consumes a slot.
- Integration cap: one integration/merge queue for the milestone. The conductor
  admits one merge candidate at a time when shared interfaces, selectors,
  runtime contracts, or deletion surfaces are touched.
- Serialized gates: actual review, publication, merge, post-merge validation,
  terminal closeout, and release-tail decisions are serialized even when code
  execution is parallel.
- Card-prep cap: prepare at most one dependency wave ahead. Do not mutate every
  future issue just because it exists.
- Interface freeze: after a producer issue publishes an interface consumed by a
  downstream wave, downstream work may proceed only against the exact reviewed
  revision or a conductor-approved replan.
- Findings default: route findings into the current issue by default. Split or
  follow up only when demonstrably out of scope and operator-approved.
- Deduplication key: surface + invariant + failure mode. Multiple reviewers
  agreeing increases confidence, not severity.
- Anti-sprawl: never open one issue per finding by default. Group by fix
  surface, dependency, and reviewability.
- External agents: read-only evidence producers only. They cannot be lifecycle
  actors, cannot create or close GitHub scope, and cannot substitute for the
  required internal review.

## Review Checkpoints

Normal checkpoints:

1. Design-ready checkpoint before implementation begins.
2. Final exact-revision pre-PR checkpoint after local validation and before
   publication.

Default shadow policy:

- Maximum two external shadows per checkpoint.
- Two-shadow work packages: WP-02, WP-07, WP-08, WP-11, WP-12, WP-13, WP-14A,
  and WP-19.
- WP-10 gets two shadows only if selector, default-generation, rollback, or
  operational cutover behavior changes.
- Other substantive implementation WPs get one shadow per checkpoint.
- Docs-only or card-prep work may use no external shadow when the internal
  review is enough and the issue records that choice.

## Wave 0: Readiness And Card Factory

Run in parallel, but only for the next dependency wave and with typed v2
authority:

1. Initialize missing issue projections with active templates.
2. Validate rendered SIP/STP/SPP/VPP/SRP/SOR structure.
3. Fill issue-specific SIP, STP, SPP, and VPP readiness fields.
4. Record dependencies, non-goals, proof lanes, and negative cases.
5. Stop before implementation.

Initial Wave 0 targets after WP-01 closes:

- WP-02 `#5336` stale-worktree recovery; do not regenerate its unpublished cards
- WP-03 `#5337`
- C-SDLC acceptance sidecar `#5358`
- Runtime v3 acceptance sidecar `#5361`

Next Wave 0 targets are admitted only after the conductor verifies predecessor
truth and WIP capacity.

After #5336 authority is integrated, prepare Runtime v3 Parity-A #5591. Only
after Parity-A's ingress contract is reviewed may #5592, #5589, and #5590 be
prepared or executed. Their capability lists do not by themselves prove
disjoint writes; explicit protected-path manifests are required.

## Wave 1: Early Acceptance Lanes

These lanes may start early after their cards are ready because their output
defines acceptance criteria rather than implementing ADL core internals:

- C-SDLC v2 acceptance lane: `#5358`
- Runtime v3 acceptance lane: `#5361`

They are parallel to early ADL core prep, but they cannot approve WP-14A or
v0.92 handoff alone. Their accepted revisions become WP-14A inputs.
C-SDLC acceptance retains `#5540` and `#5541` repair history; current tooling
defects `#5548` and `#5558` are owned by WP-20 `#5363` and do not block
`#5358` or WP-14A. Runtime v3 acceptance may start preflight early, but `#5361`
closure consumes the live distributed workcell proof from `#5501`.

## Wave 2: ADL Core Critical Path

Critical path:

1. WP-02 `#5336`: incumbent baseline and clean-room architecture.
2. WP-03 `#5337`: normalized characterization and determinism corpus.
3. WP-04 `#5339`: six-primitives language core.
4. WP-05 `#5338`: deterministic compiler.
5. WP-06 `#5340`: portable bounded execution engine.
6. WP-07 `#5342`: records, signing, and trust contracts.
7. WP-08 `#5341`: Runtime v3 adapter.
8. WP-09 `#5349`: provider and governed-tool adapters, with provider child
   `#5526`.
9. WP-10 `#5345`: thin ADL CLI and authoritative selector.

Parallelism rules:

- WP-04, WP-05, WP-06, and WP-07 do not execute as parallel implementation
  issues. Their interface-freeze and merge path is strict:
  `WP-04 -> WP-05 -> WP-06 -> WP-07`.
- Prep and review shadows may run one dependency wave ahead, but the actual
  implementation merge for each of WP-04 through WP-07 waits for the preceding
  reviewed interface.
- WP-08 prep may run while WP-07 is under review, but implementation waits for
  WP-06 and WP-07 reviewed interfaces plus the reviewed #5591 Runtime v3
  ingress contract. #5341 consumes that contract; it does not redefine it.
- WP-09 prep may run while WP-08 is under review, but implementation waits for
  WP-06 and WP-08 reviewed interfaces.
- WP-10 prep may run while WP-09 is under review, but implementation waits for
  WP-04 through WP-09 reviewed interfaces.

Before each critical-path gate, assign prep and review shadows to the next gate
so the executor receives ready cards, acceptance criteria, negative cases, and
review focus as soon as the predecessor closes.

## Wave 3: WP-10A Distributed Workcell

WP-10A starts after WP-09 reviewed readiness. After the WP-09 provider and
adapter contracts freeze, WP-10 `#5345` and WP-10A conductor `#5499` may run
concurrently on disjoint paths. Selector/default/rollback changes still enter
the single integration queue.

Order:

1. `#5499` conductor issue-graph-to-live-task plan.
2. `#5498` bounded Codex task and context-handoff adapter.
3. Parallel children after the task-adapter observation and output contracts are ready:
   - `#5500` read-only live workcell operator dashboard.
   - `#5502` output convergence and deterministic replanning.
4. `#5501` live distributed Codex workcell proof.
5. Umbrella `#5497` convergence/closeout.

WP-10 `#5345` may proceed in parallel with WP-10A after WP-09, but selector and
default behavior remain in the single integration queue.

## Wave 4: Parity, Cutover, Deletion

Parity, soak, cutover, acceptance, and deletion are constrained:

1. WP-11 `#5350`: exact-revision normalized shadow parity.
2. Runtime v3 acceptance `#5361`, consuming the four parity lanes and `#5501`.
3. WP-12 soak `#5344` after closed `#5361` acceptance.
4. WP-12 reversible default switch `#5343` after the soak proof.
5. C-SDLC v2 acceptance `#5358`, retaining `#5540` and `#5541` repair
   history, must close before deletion. WP-20 owns `#5548` and `#5558`; they
   do not block this acceptance or deletion gate.
6. WP-13 deletion may execute in parallel only on disjoint manifests:
   - `#5346` final replaced ADL language compiler engine and CLI.
   - `#5347` externally owned incumbent ADL bands.
7. WP-13 merges and post-merge validation remain serial.

Rationale: parity findings, rollback truth, selector state, acceptance proof,
and deletion eligibility are coupled. Parallel deletion is allowed only for
disjoint manifests after acceptance, and serialized merges/post-merge validation
keep the reviewed state observable.

## Wave 5: WP-14A Fan-Out And Convergence

WP-14A `#5384` is the integrated platform acceptance parent. It may fan out
read-only acceptance checks and child proof preparation after WP-13 and after
`#5358` and `#5361` are current.

Fan-out candidates:

- ADL v2 accepted/deployed exact revision.
- Runtime v3 accepted/deployed exact revision.
- C-SDLC v2 accepted/deployed exact revision.

Convergence remains serialized under the conductor: one acceptance packet, one
review synthesis, one integration queue, and one operator decision boundary.

WP-21 `#5362` independently owns the v0.92 handoff ledger, activation map,
launch-readiness, Memory Palace, identity/birthday, capability-envelope, and
Adaptive Learning inputs after WP-14A accepts the platform.

## Wave 6: Release Tail

WP-15 through WP-23 stay serial:

1. WP-15 `#5354`: integrated demos.
2. WP-16 `#5351`: integrated quality gate.
3. WP-17 `#5360`: docs and release truth alignment.
4. WP-18 `#5356`: internal milestone review.
5. WP-19 `#5357`: independent external review.
6. WP-20 `#5363`: remediation and release preflight.
7. WP-21 `#5362`: feature list and v0.92 planning truth.
8. WP-21A `#5355`: next-milestone closeout plan.
9. WP-22 `#5359`: next-milestone planning review.
10. WP-23 `#5348`: release ceremony and lifecycle closeout.

Release-tail work is deliberately serial because each step consumes reviewed
truth from the previous step.

## Non-Claims

- This plan does not configure GitHub milestones.
- This plan does not create issues.
- This plan does not execute v0.91.8 implementation.
- This plan does not approve ADL v2, Runtime v3, C-SDLC v2, WP-14A, v0.92, or
  release readiness.
