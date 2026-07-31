# Structured Task Prompt

Template: 1.0.0

Issue: 5498

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare all issue contracts now; do not implement or claim product paths until #5499 and #5349 are merged and typed closed_out with retained ancestral receipts.

## Deliverables

- six issue-specific typed C-SDLC v2 cards
- reviewed task/context adapter design and dependency diagram
- preparation-only protected-path claim disjoint from #5499, #5500, and #5502
- executable dependency and preparation validation
- COTS, line/test/time budgets, PVF lanes, privacy, idempotency, and negative-case contracts
- bounded independent preparation review with every actionable finding fixed

## Acceptance

1. AC-1: Define explicit typed create, attach, message, handoff, inspect, cancel, and escalate operations with versioned request and result envelopes
2. AC-2: Bind every writable task operation to one issue, live claim, branch, worktree, normalized protected-path set, dependency snapshot, expected output, validation contract, and freshness token
3. AC-3: Carry bounded context packets with provenance, scope, dependencies, expected output, validation, freshness, and content digests while excluding credentials and private transcript bodies from retained evidence
4. AC-4: Make duplicate requests idempotent and fail closed on stale owner, task identity, revision, claim, dependency, path, or operation-sequence collisions
5. AC-5: Emit sanitized canonically ordered observation records for #5500 and typed output/handoff references for #5502 without copying private task transcripts
6. AC-6: Preserve operator control: cancellation and escalation are explicit, time bounded, observable, and cannot grant merge, publication, closeout, issue creation, or scope-widening authority
7. AC-7: Remain independent of Runtime v2 and stay within 2,500 implementation LoC, 2,500 test/fixture LoC, fewer than 100 focused tests, 180-second focused validation, 600-second aggregate local validation, and 3,600-second complete validation including hosted CI

## Dependencies

- parent WP-10A umbrella #5497
- conductor #5499 merged and typed closed_out with retained ancestral receipt
- final provider and governed-tool interface gate WP-09 #5349 merged and typed closed_out with retained ancestral receipt
- Memory Palace #4760 remains an integration dependency only at WP-14 and is not absorbed into this transport adapter

## Inputs

- AGENTS.md
- docs/templates/prompts/current.json
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/SPRINT_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md
- issues #5498, #5499, #5500, #5502, #5349, and #4760

## Non Goals

- product implementation during preparation
- conductor planning, dashboard rendering, output convergence, or live-workcell proof
- private transcript persistence or treating chat as canonical project truth
- autonomous issue creation, scope expansion, review, publication, merge, or closeout
- provider-independent federation claims or a second lifecycle database
- Runtime v2 edits or imports, AWS, provider calls, or live task execution
