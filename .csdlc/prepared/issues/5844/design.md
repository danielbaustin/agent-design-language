# Issue 5844 Design: Ten-Article Launch Series

## Decision

WP-24 authors all ten operator-preferred Medium articles as complete,
review-ready source packets. Each article has a bounded source packet, explicit
audience and claim boundary, full prose, links/citations status, and editorial
review record. Drafting may begin after #5819; final release claims remain
gated by #5843 and operator publication authority.

## Source Baseline

- `docs/milestones/v0.91.2/review/publication_program/ARXIV_AND_MEDIUM_PUBLICATION_BACKLOG_v0.91.2.md`
- `docs/milestones/v0.91.2/review/publication_program/PUBLICATION_REVIEW_GATES_v0.91.2.md`
- `adl/tools/skills/medium-article-writer/`
- `docs/milestones/v0.92/external_launch/`
- `docs/milestones/v0.92/SPRINT_v0.92.md`

## Owned Paths

- `docs/milestones/v0.92/publication/articles/01-what-is-adl/` through `10-whats-next-for-adl/`
- `docs/milestones/v0.92/publication/articles/SERIES_ARC_AND_CLAIM_MATRIX.md`
- `docs/milestones/v0.92/publication/articles/PUBLICATION_DISPOSITION.md`
- `.csdlc/evidence/5844/`

Each article directory contains `source-packet.md`, `article.md`, and
`editorial-review.md`. The implementation session must claim individual
article directories or the exact common root after confirming no concurrent
author owns it.

## Canonical Series

1. What is ADL?
2. The ADL Runtime and the Cognitive Spacetime Model
3. Godel Agents and the Godel-Hadamard-Bayes Algorithm
4. The Freedom Gate
5. UTS and ACC: Making Agents With Tools Safe
6. CodeFriend and the Cognitive SDLC
7. Continuous Adversarial Verification For Continuous Security
8. Agent Economics
9. ADL and Social Intelligence
10. What's Next for ADL?

## Execution Plan

1. Verify #5819 before drafting and resolve repository naming/link changes.
2. Build ten bounded source packets from current canonical and exact evidence.
3. Author complete articles using the medium-article-writer contract with stop-before-publish true.
4. Run per-article claim, citation/link, privacy, and historical-versus-current scans.
5. Review the ten-article arc for duplication, sequencing, terminology, and audience fit.
6. After #5843, update only release-dependent claims and record a publication disposition.

## Production Wave Budget

The ten articles are ten bounded production waves, not one two-hour writing
task. Each article wave budgets 4 hours and 74,000 model tokens: 60 minutes and
20,000 tokens for source-packet research, 90 minutes and 30,000 tokens for the
complete draft, 45 minutes and 12,000 tokens for editorial/claim review, 30
minutes and 8,000 tokens for revisions, and 15 minutes and 4,000 tokens for
validation and packaging. The aggregate effort budget is 40 agent-hours and
740,000 tokens. With five non-overlapping article owners, allow 8-12 hours of
parallel drafting plus 4-6 hours for cross-series review, revisions, final
validation, and #5843 reconciliation. A wave that exhausts its budget stops for
re-estimation rather than producing an outline or half-reviewed article.

## Negative Cases

- Outline, topic card, generated summary, or partial draft is not complete.
- Missing or fabricated citation blocks review readiness.
- Historical evidence cannot be written as current delivery truth.
- Birthday, governance, provider, security, or release claims must stay within accepted evidence.
- No upload, scheduling, or publication occurs without operator authorization.

## Non-Goals

- Direct Medium publication, channel scheduling, or autonomous submission.
- Paper/manuscript work, podcast production, or milestone release authority.
- Invented citations, metrics, customer claims, or future-feature completion.

## Exit Evidence

All ten complete article packets pass source, link/citation, claim-boundary,
series-arc, and editorial review gates. The disposition remains review-ready or
operator-approved for a later publication decision, never silently published.
