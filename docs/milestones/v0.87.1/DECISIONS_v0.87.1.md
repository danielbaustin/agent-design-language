# Decisions - v0.87.1

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Date: `2026-04-06`
- Owner: `Daniel Austin`

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
| D-02 | Promote the `v0.87.1` runtime feature-doc set into the tracked milestone surface so planning, demos, and review can anchor to concrete runtime architecture. | superseded | The milestone is now a real runtime-completion effort, so the promoted feature docs must be part of the canonical tracked review surface rather than deferred. | Keep the feature-doc set out of scope and rely on milestone docs alone. | Makes milestone planning auditable against the actual runtime architecture and removes earlier ambiguity. | #1415 |
| D-03 | Treat `v0.87.1` as a large runtime-completion milestone rather than another docs-only alignment pass. | accepted | The project needs one milestone that actually completes the runtime substrate with implementation, demos, review surfaces, and release mechanics before moving into later cognitive work. | Keep `v0.87.1` limited to milestone-doc alignment and defer runtime completion again. | Makes the milestone implementation-heavy and forces the docs, review surfaces, and proof plan to match that reality. | #1415 |
| D-04 | Preserve explicit internal review, external review preparation, review-remediation, next-milestone planning, and release ceremony as separate closeout steps. | accepted | Large substrate milestones need an auditable review tail rather than collapsing closeout into one vague final issue. | Collapse review and release work into fewer tail issues. | Keeps closeout traceable, reviewable, and easier to execute deterministically. | #1415 |

## Open Questions
- Which bounded runtime demos should be treated as the primary reviewer entry surfaces for the milestone? (Owner: `Daniel Austin`) (Planned tracking surface: `WP-13`)

## Exit Criteria
- All milestone-critical decisions are logged with a rationale.
- Deferred/rejected/superseded options are explicitly recorded.
- Open questions have owners and tracking links.
