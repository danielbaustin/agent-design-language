# Decisions - v0.87.1

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Date: `2026-04-06`
- Owner: `TBD`

## Purpose
Capture significant decisions (architecture, scope, process) at the time they are made.

## How To Use
- Add one row per decision.
- Prefer links to issues/PRs over long prose.
- Keep status current: `accepted`, `rejected`, `deferred`, `superseded`.

## Decision Log
| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | Create the tracked `docs/milestones/v0.87.1/` shell before promoting `v0.87.1` feature docs. | accepted | The roadmap now treats runtime completion as its own sub-milestone, so it needs a public tracked surface first. | Delay milestone-shell creation until feature promotion time. | Enables consistent tracked docs and later feature promotion. | #1354 |
| D-02 | Keep `v0.87.1` feature-doc promotion out of scope for the milestone-shell seed pass. | accepted | This issue is about structure and naming, not public promotion of internal planning docs. | Promote the runtime docs immediately. | Keeps the issue bounded and reduces accidental public overcommitment. | #1354 |

## Open Questions
- {{open_question_1}} (Owner: {{owner_oq1}}) (Issue: {{issue_oq1}})
- {{open_question_2}} (Owner: {{owner_oq2}}) (Issue: {{issue_oq2}})

## Exit Criteria
- All milestone-critical decisions are logged with a rationale.
- Deferred/rejected/superseded options are explicitly recorded.
- Open questions have owners and tracking links.
