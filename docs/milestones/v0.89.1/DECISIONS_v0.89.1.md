# Decisions - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Date: `2026-04-14`
- Owner: `Daniel Austin`

## Purpose

Capture milestone-critical scope and packaging decisions for `v0.89.1`.

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | Treat `v0.89.1` as the explicit adversarial/runtime follow-on milestone rather than a vague carry-forward placeholder. | accepted | `v0.89` already made the carry-forward explicit; leaving it as a floating note would recreate drift. | Keep only local planning docs and no tracked milestone package. | Makes the follow-on band reviewable and seedable. | `#1860` |
| D-02 | Promote the strongest non-empty adversarial/runtime docs into tracked `v0.89.1` feature docs. | accepted | The source corpus is already substantial enough to justify a tracked feature package. | Leave all inputs local-only. | Gives the milestone a real canonical feature set. | `FEATURE_DOCS_v0.89.1.md` |
| D-03 | Keep delegation/refusal, negotiation, proposed skills, and the empty provider/security demo notes as supporting inputs rather than promoted tracked feature commitments. | accepted | They matter, but not all of them are mature enough to overstate as first-line tracked contracts. | Promote every local `v0.89.1` planning input into tracked features. | Keeps the package bounded and honest. | `FEATURE_DOCS_v0.89.1.md` |
| D-04 | Preserve the standard ADL release-tail pattern (`WP-11` - `WP-20`) instead of inventing a special short tail for this milestone. | accepted | The existing milestone cadence has worked and keeps review/release discipline legible. | Compress the tail into a smaller custom pattern. | Makes `v0.89.1` consistent with recent milestones. | `WBS_v0.89.1.md` |
| D-05 | Make the package review-ready and mechanically issueizable before the wave opens. | accepted | The next milestone should open quickly from settled docs, not by rediscovering scope during issue creation. | Leave the package merely “good enough” and rely on later ad hoc clarifications. | Improves fast-start discipline and reduces kickoff drift. | `README.md`, `SPRINT_v0.89.1.md`, `WP_ISSUE_WAVE_v0.89.1.yaml` |
| D-06 | Keep the bounded `arxiv-paper-writer` skill as an explicit `v0.89.1` backlog/follow-on candidate rather than promoting it into the tracked feature set before the runtime band opens. | accepted | The need is real, but the repo does not yet have a dedicated arXiv-writing skill contract and the milestone should not over-promise extra writing/publication scope as if it were already a first-line commitment. | Promote it immediately as a tracked feature, or leave it only in chat/local memory. | Preserves the next-milestone opportunity without widening the milestone canon dishonestly. | `FEATURE_DOCS_v0.89.1.md`, `.adl/docs/TBD/LOCAL_BACKLOG.md` |

## Open Questions

- How much of the operational-skill substrate should land as code in `v0.89.1` versus remain design-contract work? (Owner: Daniel Austin)
- Should provider security capabilities extension become part of this milestone proper, or remain a later security-extension slice? (Owner: Daniel Austin)
- Which proof surfaces are sufficient for `v0.89.1` itself versus intentionally deferred to later security/governance bands? (Owner: Daniel Austin)
- Should the bounded `arxiv-paper-writer` skill be pulled into the official `v0.89.1` issue wave under the operational-skills/proof band, or remain a local backlog candidate until the core adversarial/runtime work settles? (Owner: Daniel Austin)

## Exit Criteria

- all milestone-critical scope and packaging decisions are logged with rationale
- promotion and non-promotion decisions are explicit rather than implicit
- open questions have a clear home in the future issue wave
