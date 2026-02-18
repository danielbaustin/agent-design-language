# ADL v0.5 Release Plan

## Metadata
- Milestone: `v0.5`
- Version: `0.5.0`
- Release date: `TBD`
- Release manager: `Daniel Austin`

---

## Purpose
Operational checklist for shipping ADL v0.5. This document tracks mechanical release steps only. Narrative belongs in release notes.

---

## 1) Release Readiness
- [ ] Milestone checklist complete (`docs/milestones/v0.5/MILESTONE_CHECKLIST_v0.5.md`)
- [ ] Release notes finalized (`docs/milestones/v0.5/RELEASE_NOTES_v0.5.md`)
- [ ] Go/no-go decision recorded (`docs/milestones/v0.5/DECISIONS_v0.5.md`)
- [ ] All v0.5 WBS work packages marked done (`docs/milestones/v0.5/WBS_v0.5.md`)
- [ ] Demo matrix fully runnable and validated

---

## 2) Branch And Tag Preparation
- [ ] Target branch confirmed (`main`)
- [ ] Working tree clean (`git status`)
- [ ] Version references validated in:
  - Root README
  - swarm/README.md
  - docs/milestones/v0.5/*
- [ ] Cargo metadata validated (if applicable)
- [ ] Tag created: `v0.5.0`
- [ ] Tag pushed and verified (`git push origin v0.5.0`)

Tag command:

```
git tag -a v0.5.0 -m "ADL v0.5.0"
git push origin v0.5.0
```

---

## 3) GitHub Release Steps
- [ ] GitHub Release draft created from `v0.5.0`
- [ ] Release body populated from `RELEASE_NOTES_v0.5.md`
- [ ] Links to key PRs and issues included
- [ ] Release visibility confirmed (final, not prerelease unless intended)
- [ ] Release published

CLI option (if GitHub API stable):

```
gh release create v0.5.0 \
  --title "ADL v0.5.0" \
  --notes-file docs/milestones/v0.5/RELEASE_NOTES_v0.5.md
```

---

## 4) Verification
- [ ] Post-release CI status checked
- [ ] Release links tested (README, docs, demo commands)
- [ ] All demo scripts verified against tagged release
- [ ] No critical regressions discovered
- [ ] Coverage signal acceptable (not red)

---

## 5) Communication
- [ ] Release announcement prepared (`docs/milestones/v0.5/RELEASE_NOTES_v0.5.md`)
- [ ] Public announcement published
- [ ] Roadmap/status updated
- [ ] Deferred items moved to next milestone

---

## Exit Criteria
- `v0.5.0` tag exists and is published.
- GitHub Release exists and is public.
- All checklist items satisfied or documented with owner + follow-up issue.
- Release artifacts and demos verified against tagged commit.
