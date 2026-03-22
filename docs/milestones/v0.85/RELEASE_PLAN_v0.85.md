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

- [ ] Milestone checklist complete (`MILESTONE_CHECKLIST_v0.85.md`)
- [ ] Documentation consistency pass completed
- [ ] Internal review completed
- [ ] External review completed
- [ ] Review findings remediation completed or explicit deferrals recorded
- [ ] Release notes approved (`RELEASE_NOTES_v0.85.md`)
- [ ] Go / No-Go decision recorded in decision log (`DECISIONS_v0.85.md`)
- [ ] Required demo proof surfaces reviewed:
  - steering/queueing/checkpoint
  - HITL/editor/review flow
  - Gödel runtime behavior
  - affect-plus-Gödel/reasoning behavior

These steps correspond to the Sprint 4 closeout path in the milestone plan.

The canonical quality-gate truth surface for these checks is
`QUALITY_GATE_v0.85.md`.

The final active-surface `swarm` -> `adl` cutover is part of that closeout path. It should be executed only after the other v0.85 code changes, review findings, and milestone docs have otherwise stabilized.

---

# 2) Branch and Tag Preparation

Prepare the repository state for the release.

- [ ] Target branch confirmed (`main`, unless otherwise specified)
- [ ] Working tree clean
- [ ] All intended PRs merged
- [ ] CI passing
- [ ] Version strings validated (Cargo manifests / docs if applicable)
- [ ] Final cutover preconditions confirmed (`SWARM_REMOVAL_PLANNING.md`)
  - other milestone code changes merged or effectively frozen
  - review findings resolved or explicitly deferred
  - cutover branch can proceed without competing path churn

Tag creation:

- [ ] Tag created: `v0.85.0`
- [ ] Tag pushed to GitHub
- [ ] Tag presence verified

---

# 3) GitHub Release Creation

Create the GitHub release artifact.

- [ ] GitHub Release draft created from tag `v0.85.0`
- [ ] Release body populated from `RELEASE_NOTES_v0.85.md`
- [ ] Links to key PRs and issues included
- [ ] Release visibility confirmed (draft / prerelease / final)
- [ ] Release published

---

# 4) Post-Release Verification

Confirm that the release is valid and visible.

- [ ] Post-release CI run verified
- [ ] Documentation links tested
- [ ] Release notes formatting verified
- [ ] Repository state confirmed stable
- [ ] Final `swarm` -> `adl` active-surface cutover completed or explicitly deferred with rationale

If any immediate regression is detected:

- [ ] Regression issue opened
- [ ] Hotfix decision recorded if needed

---

# 5) Communication

Publish the release externally and internally.

- [ ] GitHub release visible
- [ ] Roadmap / milestone status updated
- [ ] Internal project update posted
- [ ] Cutover/migration note published if the final `swarm` -> `adl` rename lands in this release

Optional (depending on project stage):

- [ ] Community announcement
- [ ] Documentation site update

---

# 6) Closeout and Next Milestone Planning

- [ ] Milestone / epic issues closed with release links
- [ ] Deferred items moved to the next milestone backlog
- [ ] Next milestone planning docs/templates prepared before milestone closure
- [ ] Retrospective summary recorded

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
