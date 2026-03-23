# Release Plan — v0.85

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Release date: `TBD`
- Release manager: `Daniel Austin / Agent Logic`

## How To Use
- Execute sections in order and capture links for each completed step.
- Keep this document focused on **release mechanics**, not milestone narrative.
- Use `RELEASE_NOTES_v0.85.md` for descriptive content.
- If a blocker appears, stop the process and record it explicitly.

---

# 1) Release Readiness

Before beginning the release process the following must be true:

- [ ] Milestone checklist complete (`MILESTONE_CHECKLIST_v0.85.md`)
- [ ] Internal review completed
- [ ] External review completed
- [ ] Review findings resolved or explicitly deferred
- [ ] Release notes approved (`RELEASE_NOTES_v0.85.md`)
- [ ] Go / No‑Go decision recorded in decision log (`DECISIONS_v0.85.md`)

These steps correspond to the **Release Tail** defined in the milestone sprint plan.

---

# 2) Branch and Tag Preparation

Prepare the repository state for the release.

- [ ] Target branch confirmed (`main`, unless otherwise specified)
- [ ] Working tree clean
- [ ] All intended PRs merged
- [ ] CI passing
- [ ] Version strings validated (Cargo manifests / docs if applicable)

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

# 4) Post‑Release Verification

Confirm that the release is valid and visible.

- [ ] Post‑release CI run verified
- [ ] Documentation links tested
- [ ] Release notes formatting verified
- [ ] Repository state confirmed stable

If any immediate regression is detected:

- [ ] Regression issue opened
- [ ] Hotfix decision recorded if needed

---

# 5) Communication

Publish the release externally and internally.

- [ ] GitHub release visible
- [ ] Roadmap / milestone status updated
- [ ] Internal project update posted

Optional (depending on project stage):

- [ ] Community announcement
- [ ] Documentation site update

---

# Exit Criteria

The release process is complete when:

- The release tag exists and is publicly accessible.
- The GitHub Release is published.
- CI is green after the release.
- Release notes and links are verified.
- No unknown critical regressions remain.

At that point the milestone can be considered **successfully shipped**.
