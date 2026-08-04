# Structured Task Prompt

Template: 1.0.0

Issue: 5501

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare all six cards, reviewed design and diagram, exact dependency and proof gates, COTS, budgets, PVF, and review evidence; do not execute tasks or claim product paths.

## Deliverables

- all six issue-specific current-registry typed cards
- live distributed workcell proof design and Mermaid diagram
- exact preparation-only protected paths and fail-closed live merge plus ancestry dependency gate
- executable preparation and future live-proof validation contracts
- zero-new-dependency COTS posture, LoC/shard/time budgets, and PVF classification

## Acceptance

1. AC-1: The admitted plan creates at least two real parallel writable Codex shards with distinct issues, typed claims, branches, worktrees, and disjoint protected/write paths; fixture or prose substitutes fail
2. AC-2: The conductor emits the admitted plan and at least one real fail-closed negative case, while bounded context transfer binds source revision, task, issue, claim, paths, artifacts, and digests without credentials or unrelated transcript content
3. AC-3: #5500 dashboard observations are derived from typed lifecycle and live task/dependency state without manual green assertions, and every output is reviewed then classified by #5502 as integrate, replan, or blocked
4. AC-4: Publication, PR/checks, review, merge, post-merge validation, and closeout remain serialized and truthful, with exact-revision continuity and no autonomous lifecycle authority
5. AC-5: The retained packet records timing, coordination overhead, failures, retries, and a fair bounded single-agent baseline over equivalent declared work
6. AC-6: The proof adds no product implementation by default, keeps issue-local harness and structured fixtures within 2,500 lines, uses two to four writable shards, fewer than 100 focused assertions, 1,800-second live and baseline limits, a 3,600-second complete limit, and zero new dependencies

## Dependencies

- parent WP-10A umbrella #5497 remains open until this proof and child convergence complete
- WP-09 provider and governed-tool adapter #5349 live merged head must be ancestral to the execution revision
- conductor #5499 live merged head must be ancestral to the execution revision
- Codex task and bounded context adapter #5498 live merged head must be ancestral to the execution revision
- live dashboard #5500 live merged head must be ancestral to the execution revision
- output convergence and deterministic replanning #5502 live merged head must be ancestral to the execution revision
- typed closeout state and retained receipts for #5349, #5499, #5498, #5500, and #5502 are audit-only and must not block execution readiness when live merge plus ancestry truth is satisfied
- Runtime v3 acceptance #5361 and shadow parity #5350 consume the completed live proof

## Inputs

- AGENTS.md
- GitHub issue #5501 live body
- docs/templates/prompts/current.json
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md
- docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
- future terminal dependency receipts and exact reviewed public contracts

## Non Goals

- live task execution or product implementation during preparation
- fixture-only, mock-only, prose-only, screenshot-only, or library-only substitution for real Codex workcell proof
- reimplementing conductor, task transport, dashboard, convergence, Runtime, scheduler, or lifecycle storage
- automatic issue creation, review approval, publication, GitHub mutation, merge, or closeout
- Runtime v2 edits, AWS, provider credentials, paid services, unbounded sessions, hidden network, or private transcript retention
