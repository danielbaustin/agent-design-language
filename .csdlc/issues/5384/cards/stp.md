# Structured Task Prompt

Template: 1.0.0

Issue: 5384

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare, validate, review, approve, bind, commit, and push issue-local lifecycle artifacts only; do not implement any WP-14A deliverable.

## Deliverables

- six current native typed C-SDLC cards
- issue-specific preparation design and dependency diagram
- declarative complete predecessor gate and deterministic receipt/ancestry checker
- COTS/reuse and bounded budget decisions
- PVF-classified validation plan
- reviewed preparation-only protected-path claim

## Acceptance

1. AC-1: The six typed cards, design, and diagram describe the same thin WP-14A acceptance boundary.
2. AC-2: Live GitHub truth confirms #5358, #5361, #5344, and #5343 are closed before execution.
3. AC-3: Preparation remains issue-local and cannot authorize platform implementation or publication.
4. AC-4: WP-13 #5346/#5347 is explicitly non-blocking for WP-14A and becomes mandatory immediately before #5356.
5. AC-5: Execution reuses retained proof, runs only focused fresh-consumer checks, and receives one bounded Gemini review before PR publication.

## Dependencies

- C-SDLC v2 acceptance #5358
- Runtime v3 acceptance #5361
- ADL v2 soak and rollback proof #5344
- ADL v2 reversible default switch #5343
- WP-13 deletion #5346/#5347 is deferred and non-blocking for WP-14A; it becomes a hard gate immediately before internal review #5356

## Inputs

- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md
- docs/templates/prompts/current.json
- .csdlc issues and shared-Git csdlc-v2 closeout receipts
- live GitHub issue and PR truth through an approved connector

## Non Goals

- WP-14A implementation, acceptance execution, deployment, or handoff execution
- product, Runtime, C-SDLC implementation, test, workflow, or shared milestone-document changes
- predecessor repair, waiver, closeout, merge, or claim takeover
- PR creation, typed publication, merge, or issue closeout
- AWS, Runtime v2, raw gh, provider execution, or credential access
- identity, consciousness, birthday, production-provider, or v0.92 readiness claims
