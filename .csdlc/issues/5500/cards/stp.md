# Structured Task Prompt

Template: 1.0.0

Issue: 5500

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare all dashboard contracts now; do not claim or edit product paths until #5498 and final WP-09 gate #5349 satisfy the exact terminal dependency predicate.

## Deliverables

- six issue-specific typed C-SDLC v2 cards
- reviewed dashboard design and architecture diagram
- preparation-only protected-path claim disjoint from #5502 and other workcell children
- executable dependency, preparation, and future dashboard validation contracts
- COTS, LoC/test/time budgets, PVF lanes, security and non-authority boundaries
- bounded independent preparation review with findings fixed

## Acceptance

1. AC-1: Extend docs/tooling/milestone-dashboard and compose with the existing Runtime v3 Observatory rather than creating another dashboard framework or backend
2. AC-2: Render issue/task ownership, claim heartbeat, branch/worktree, lifecycle phase, dependencies, protected paths, PR/check/review state, outputs, blockers, and agent topology from typed bounded inputs
3. AC-3: Mark every value live, retained, stale, unknown, blocked, or non-authoritative with source, revision or timestamp, and freshness; missing evidence never becomes green
4. AC-4: Remain strictly read-only with zero mutation requests, no hidden authority, no private transcript or secret retention, and authenticated HTTPS-only live Runtime access
5. AC-5: Treat all input as untrusted and prove schema/version checks, text-safe rendering, URL/origin restrictions, payload/count limits, timeouts/backoff, and stale downgrade
6. AC-6: Work at mobile-sized layouts and pass deterministic retained/live/partial/stale/security fixtures without network access
7. AC-7: Stay within 2,000 implementation LoC, 2,000 test/fixture LoC, fewer than 100 focused cases, a 120-second focused dashboard validation cap, a 3,600-second complete typed orchestration envelope, and zero new direct dependencies by default

## Dependencies

- parent WP-10A umbrella #5497
- bounded Codex task and context-handoff adapter #5498 live-merged on origin/main and ancestral to the #5500 execution base
- final provider and governed-tool interface freeze WP-09 #5349 live-merged on origin/main and ancestral to the #5500 execution base
- typed closeout receipts and retained lifecycle records for dependencies are audit evidence only, not readiness blockers
- existing Runtime v3 Observatory feed as a read-only observation input

## Inputs

- AGENTS.md
- docs/templates/prompts/current.json
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md
- docs/tooling/milestone-dashboard/README.md
- demos/v0.91.7/html-observatory/README.md
- issue #5500 and parent #5497

## Non Goals

- dashboard implementation during preparation
- task control, output convergence, replanning, issue or GitHub mutation, merge, publication, or closeout
- a new dashboard framework, backend service, sidecar, or source of truth
- Runtime v2 edits, AWS use, provider calls, unauthenticated HTTP, hard-coded IPs, or credential retention
- owning #5502 or another workcell child's product paths
