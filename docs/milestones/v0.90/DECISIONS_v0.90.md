# Decisions - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: pre-third-party-review readiness

## Decision Log

| ID | Decision | Status | Rationale | Impact |
| --- | --- | --- | --- | --- |
| D-01 | Treat v0.90 as the long-lived-agent runtime milestone | accepted | This is the strongest cohesive package in the promoted planning lane and directly extends v0.89.1 proof discipline | Centers the milestone around supervisor, cycle, continuity, safety, and demo work |
| D-02 | Keep full v0.92 identity out of v0.90 | accepted | Long-lived agents need continuity handles now, but full identity/capability substrate remains later work | Prevents overclaiming and keeps migration boundary explicit |
| D-03 | Use ideas/ for conceptual or later-band docs | accepted | The v0.89 ideas lane pattern should continue so rationale is visible without becoming feature scope | Keeps temporal/society docs accounted for without promoting them as implementation commitments |
| D-04 | Use `#1940` / `v0.89.1` WP-19 as the tracked promotion gate | accepted | Early planning should speed up the milestone, not bypass release discipline | The v0.90 package is promoted as tracked planning material before v0.89.1 release ceremony |
| D-05 | Add a bounded demo extension lane | accepted | We expect to add or extend demos, but the stock-league demo must remain the primary long-lived proof | Adds WP-09 with proof-claim and validation requirements |
| D-06 | Ratchet coverage to 93 percent in this revision | accepted | A 93 percent tranche is a safer near-term quality step than jumping directly to 95 percent | Adds WP-10 and preserves 95 percent as a later target |
| D-07 | Pilot milestone compression carefully | accepted | Compression matters, but write automation should not outrun state-model truth | Adds WP-11 for canonical milestone state and drift checks |
| D-08 | Add repo visibility as a bounded prototype | accepted | Repo-aware cognition is useful now, but the first slice should be a manifest and linkage proof | Adds WP-12 without claiming full repo indexing |
| D-09 | Add explicit Rust refactoring near the release tail | accepted | We are doing refactoring anyway, so it should be visible, bounded, and validated | Adds WP-14 for maintainability/testability/review-driven refactors |
| D-10 | Run WP-18 before third-party review | accepted | The reviewer should receive current quality, tracker, docs, compression, visibility, and closeout truth rather than stale release-tail placeholders | Reorders the review tail to WP-15, WP-18, WP-16, WP-17, WP-19, WP-20 |

## Dispositioned Questions

- Signed trace and TQL remain later-band or narrow inspection context unless a
  later issue promotes them explicitly.
- The hypothesis engine reasoning graph remains later reasoning-surface context,
  not a v0.90 runtime commitment.
- Urgency/prioritization remains governance and later scheduler-policy context.
- The stock league demo uses deterministic fixture replay as the canonical
  reviewer path.
- WP-09 selected the stock-league reviewer evidence index extension.
- WP-14 refactoring targets are driven by the maintainability hotspot plan and
  child issues, not broad aesthetic cleanup.
- WP-12 used the long-lived runtime and stock-league proof path for the first
  repo visibility linkage proof.
- WP-10 accepted the rounded `93%` tranche based on current tracker evidence:
  `92.40%` workspace line coverage, with workspace and per-file gates passing
  and no active file-floor exclusion.
- WP-14 accepted the current Rust tracker state as managed follow-up rather than
  a release blocker: one `RATIONALE`, nineteen `REVIEW`, and fourteen `WATCH`
  items after the child split wave.

## Exit Criteria

- v0.90 WP-01 turned proposed decisions into accepted/deferred tracked milestone decisions.
- Every open question has a work-package or defer home in the opened issue wave
  or a recorded later-band planning surface.
