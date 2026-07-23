# Structured Task Prompt

Template: 1.0.0

Issue: 5499

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare all issue contracts now; do not implement or claim product paths until #5340, #5341, #5342, and final WP-09 gate #5349 are merged and typed closed_out with retained ancestral receipts.

## Deliverables

- six issue-specific typed C-SDLC v2 cards
- reviewed conductor design and dependency diagram
- exact preparation-only protected-path claim
- executable dependency and preparation validation
- COTS, line/test/time budgets, PVF lanes, and negative-case contract
- bounded independent preparation review with findings fixed

## Acceptance

1. AC-1: Consume versioned typed C-SDLC v2 issue/claim/card state and ADL v2 plan records without creating a second lifecycle store
2. AC-2: Fail closed on missing cards, stale or absent claims, dependency cycles, unresolved dependencies, unknown validation lanes, WIP overflow, or ambiguous authority
3. AC-3: Detect exact and prefix-overlapping normalized protected/write paths and admit only disjoint writable shards
4. AC-4: Preserve serialized review, publication, merge, post-merge validation, and closeout gates
5. AC-5: Emit canonically ordered typed assignment or refusal records with content-derived correlation identifiers
6. AC-6: Remain a pure planning component with no task creation, network, GitHub, filesystem mutation, scheduler, or Runtime v2 dependency
7. AC-7: Stay within 3,000 implementation LoC, 3,000 test/fixture LoC, fewer than 120 focused tests, 180-second focused validation, and 600-second full validation

## Dependencies

- parent WP-10A umbrella #5497
- portable engine WP-06 #5340 live-merged and ancestral to execution base
- Runtime v3 adapter WP-08 #5341 live-merged and ancestral to execution base
- remaining substrate WP-07 #5342 live-merged and ancestral to execution base
- final provider and governed-tool interface freeze WP-09 #5349 live-merged and ancestral to execution base
- typed closeout and retained receipts are audit-only evidence and must not block readiness by themselves

## Inputs

- AGENTS.md
- docs/templates/prompts/current.json
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/SPRINT_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md
- issue #5499 and parent #5497

## Non Goals

- product implementation during preparation
- task creation, messaging, handoff, cancellation, or transcript retention
- autonomous issue creation, GitHub mutation, merge, publication, or closeout
- a second scheduler or lifecycle database
- Runtime v2 edits or imports
- AWS, provider, network, dashboard, convergence, or live-workcell execution
