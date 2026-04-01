# Release Plan — v0.86

## Metadata
- Milestone: `v0.86`
- Version: `0.86`
- Release date: `Pending manual release ceremony`
- Release manager: `Daniel Austin`

## Purpose
Define the exact release procedure for v0.86. This is not a template—this is the executable release contract for the milestone.

v0.86 must ship a **working bounded cognitive system with proof surfaces**, not just aligned documentation.

---

## 1) Release Readiness (GO / NO-GO Gate)

All of the following must be TRUE before proceeding:

- [x] Milestone checklist prepared for manual ceremony (`docs/milestones/v0.86/MILESTONE_CHECKLIST_v0.86.md`)
- [x] Demo program runs locally and proves the canonical bounded cognitive path
- [x] Demo matrix matches actual runnable demos (`DEMO_MATRIX_v0.86.md`)
- [x] DESIGN, WBS, and implementation are aligned (no conceptual drift)
- [x] All promoted v0.86 feature-defining docs are implemented in at least one execution path and aligned with the tracked docs

### Explicit GO / NO-GO Questions

Answer ALL before release:

- [x] Does the system execute a full bounded cognitive loop end-to-end (signals → arbitration → reasoning → execution → evaluation → reframing → memory → Freedom Gate)?
- [x] Are arbitration decisions visible and correct (fast vs slow)?
- [x] Do instinct and affect signals visibly influence arbitration and routing decisions?
- [x] Is candidate selection (agency) observable and real?
- [x] Does the Freedom Gate allow, defer, and refuse at least one case with inspectable artifacts?
- [x] Can a reviewer run one command and find all proof artifacts?
- [x] Does at least one run demonstrate bounded execution (iteration), evaluation affecting behavior, and reframing/adaptation?

If any answer is NO → DO NOT RELEASE.

Record decision:
- GO / NO-GO decision: `Prepared for manual ceremony`
- Decision record / notes: `As of 2026-04-01, milestone prep is complete enough for manual tag/release execution: the quality gate passed, coverage is over the enforced limits (91.50% workspace line coverage against a 90% threshold, with the per-file >= 80% gate passing), D1-D5 are review-ready, internal and external review legs are complete, findings are resolved or explicitly deferred, and the remaining manual steps are tag creation, GitHub release drafting/publishing, and final release-link closeout.`

---

## 2) Branch And Tag Preparation

- [x] Target branch confirmed: `main`
- [ ] Working tree clean (no uncommitted changes)
- [x] All PRs required before ceremony are merged via `pr.sh`
- [x] Version references updated (`0.86.0`)

Create tag:

```bash
git tag v0.86
git push origin v0.86
```

- [ ] Tag created: `v0.86`
- [ ] Tag pushed and visible on GitHub

---

## 3) GitHub Release Steps

- [ ] GitHub Release draft created from `v0.86`
- [x] Release notes prepared in `RELEASE_NOTES_v0.86.md`
- [x] Links to key PRs/issues are prepared for inclusion in the GitHub release draft
- [x] Demo instructions included via the canonical demo-matrix reviewer entrypoint
- [ ] Release visibility set correctly (likely `final`, not prerelease)

Publish:

- [ ] Release published

---

## 4) Verification

Immediately after publishing:

- [x] CI status checked on `main`
- [ ] Tag `v0.86` resolves correctly
- [ ] Demo commands run successfully from a clean checkout
- [ ] Demo artifacts are generated and inspectable
- [x] Links and doc references in release notes were checked locally before ceremony

If any failure occurs:
- create issue
- triage immediately
- decide: patch vs defer

---

## 5) Communication

- [ ] Internal update posted (milestone completion)
- [x] Roadmap/status docs updated

Optional (depending on timing/strategy):
- [ ] Public / LinkedIn announcement

---

## 6) Post-Release Closure

- [ ] All milestone issues closed with release reference
- [x] Deferred items moved to next milestone
- [x] Bugs or follow-ups captured as issues
- [x] Next milestone planning (`#882`) is already in progress or complete

---

## Exit Criteria

The release is complete when:

- `v0.86` tag exists and is published
- GitHub Release is live and correct
- Demo program proves the bounded cognitive system
- Artifacts are inspectable, consistent, and span the full cognitive loop
- Docs match implementation (no drift)
- Repo is clean and ready for next milestone

---

## Notes

- This milestone is about **bounded cognition, not scale**.
- If the demos do not prove the full cognitive loop, the release is invalid.
- Do not ship partial cognition.
- The system must behave as one coherent cognitive architecture, not a collection of components.
- This document records a prepared pre-ceremony state until the manual tag and GitHub release steps are executed.
