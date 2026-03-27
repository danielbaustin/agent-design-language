# Release Plan — v0.85

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Release date: `TBD`
- Release manager: `Daniel Austin / Agent Logic`

## How To Use
- Execute sections in order and capture links for each completed step.
- Keep this document focused on release mechanics, not milestone narrative.
- Use `RELEASE_NOTES_v0.85.md` for descriptive content.
- Use `QUALITY_GATE_v0.85.md` for the canonical quality-gate definition.
- If a blocker appears, stop the process and record it explicitly.

---

# 1) Release Readiness

Before beginning the release process the following must be true:

- [x] Milestone checklist complete (`MILESTONE_CHECKLIST_v0.85.md`)
- [x] Documentation consistency pass completed
- [x] Internal review completed
- [x] External review completed
- [x] Review findings remediation completed or explicit deferrals recorded
- [x] Release notes approved (`RELEASE_NOTES_v0.85.md`)
- [x] Go / No-Go decision recorded in decision log (`DECISIONS_v0.85.md`)
- [x] Required demo proof surfaces reviewed:
  - steering/queueing/checkpoint
  - HITL/editor/review flow
  - five-command editing lifecycle
  - Gödel runtime behavior
  - affect-plus-Gödel/reasoning behavior

These steps correspond to the Sprint 4 closeout path in the milestone plan.

The canonical quality-gate truth surface for these checks is
`QUALITY_GATE_v0.85.md`.

The final active-surface `swarm` -> `adl` cutover is part of that closeout path. It should be executed only after the other v0.85 code changes, review findings, and milestone docs have otherwise stabilized.

---

# 2) Branch and Tag Preparation

Prepare the repository state for the release.

- [x] Target branch confirmed (`main`, unless otherwise specified)
- [x] Working tree clean
- [x] All intended PRs merged
- [x] CI passing
- [x] Version strings validated (Cargo manifests / docs if applicable)
- [x] Final cutover preconditions confirmed (`SWARM_REMOVAL_PLANNING.md`)
  - other milestone code changes merged or effectively frozen
  - review findings resolved or explicitly deferred
  - cutover branch can proceed without competing path churn

Tag creation:

- [x] Tag created: `v0.85.0`
- [x] Tag pushed to GitHub
- [x] Tag presence verified

---

# 3) GitHub Release Creation

Create the GitHub release artifact.

- [x] GitHub Release draft created from tag `v0.85.0`
- [x] Release body populated from `RELEASE_NOTES_v0.85.md`
- [x] Links to key PRs and issues included
- [x] Release visibility confirmed (draft / prerelease / final)
- [x] Release published

---

# 4) Post-Release Verification

Confirm that the release is valid and visible.

- [x] Post-release CI run verified
- [x] Documentation links tested
- [x] Release notes formatting verified
- [x] Repository state confirmed stable
- [x] Final `swarm` -> `adl` active-surface cutover completed or explicitly deferred with rationale

If any immediate regression is detected:

- [x] Regression issue opened
- [x] Hotfix decision recorded if needed

---

# 5) Communication

Publish the release externally and internally.

- [x] GitHub release visible
- [x] Roadmap / milestone status updated
- [x] Internal project update posted
- [x] Cutover/migration note published if the final `swarm` -> `adl` rename lands in this release

Optional (depending on project stage):

- [x] Community announcement
- [x] Documentation site update

---

# 6) Closeout and Next Milestone Planning

- [x] Milestone / epic issues closed with release links
- [x] Deferred items moved to the next milestone backlog
- [x] Next milestone planning docs/templates prepared before milestone closure
- [x] Retrospective summary recorded

---

# Exit Criteria

The release process is complete when:

- The release tag exists and is publicly accessible.
- The GitHub Release is published.
- CI is green after the release.
- Release notes and links are verified.
- Required demo proof surfaces have been reviewed as milestone evidence.
- No unknown critical regressions remain.
- Next milestone planning is ready before v0.85 is considered fully closed.

At that point the milestone can be considered successfully shipped.
