# Structured Task Prompt

Template: 1.0.0

Issue: 5502

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare all six cards, design, diagram, exact gates, COTS, budgets, PVF, and review evidence; do not claim or edit product paths.

## Deliverables

- all six issue-specific current-registry typed cards
- convergence/replanning design and Mermaid diagram
- exact preparation-only protected paths
- executable live merge/ancestry dependency, preparation, and future validation contracts
- COTS, LoC/module/test/time budgets and PVF classification
- preparation evidence without adding a second review before the required pre-PR review gate

## Acceptance

1. AC-1: Outputs bind exactly to task, issue, claim, branch, worktree, source revision, declared paths, artifacts, validation, and review evidence; missing, stale, forged, overlapping, or out-of-scope values fail closed
2. AC-2: Integration order follows the admitted #5499 dependencies and interface freezes; changed assumptions emit a typed deterministic replan or blocked record and never silently expand scope
3. AC-3: Review, publication, merge, post-merge validation, and closeout remain serialized and independently authorized; #5502 has no task, GitHub, filesystem, network, or lifecycle mutation capability
4. AC-4: Partial success and residual blockers remain present in a canonical read-only projection suitable for #5500 and #5501, with stable ordering and content-derived decision identity
5. AC-5: Implementation stays within 2,500 product LoC, 2,500 test/fixture LoC, fewer than 100 focused tests, modules below 500 LoC, 120-second focused and 600-second complete validation, and only reviewed COTS

## Dependencies

- parent WP-10A umbrella #5497
- issue-graph-to-live-task conductor #5499 live GitHub merged and merge revision ancestral to #5502 execution base
- bounded Codex task/context adapter #5498 live GitHub merged and merge revision ancestral to #5502 execution base
- typed closeout receipts and claim release for #5499 and #5498 remain audit-only signals and MUST NOT block execution readiness
- live proof #5501 consumes the completed #5502 decision contract

## Inputs

- AGENTS.md
- GitHub issue #5502 and its dependency comment
- docs/templates/prompts/current.json
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md
- future #5499 and #5498 public contracts plus audit-only closeout receipts when present

## Non Goals

- product implementation during preparation
- task creation/transport, conductor planning, dashboard rendering, or live workcell proof
- automatic review approval, publication, GitHub mutation, merge, or closeout
- a second lifecycle database, scheduler, workflow engine, or orchestration framework
- Runtime v2 edits, AWS, provider calls, credentials, hidden network, or private transcript retention
