# Release Plan — v0.86

## Metadata
- Milestone: `v0.86`
- Version: `0.86`
- Release date: `Target: end of 2-day execution window`
- Release manager: `Daniel Austin`

## Purpose
Define the exact release procedure for v0.86. This is not a template—this is the executable release contract for the milestone.

v0.86 must ship a **working bounded cognitive system with proof surfaces**, not just aligned documentation.

---

## 1) Release Readiness (GO / NO-GO Gate)

All of the following must be TRUE before proceeding:

- [ ] Milestone checklist complete (`docs/milestones/v0.86/MILESTONE_CHECKLIST_v0.86.md`)
- [ ] Demo program runs locally and proves the canonical bounded cognitive path
- [ ] Demo matrix matches actual runnable demos (`DEMO_MATRIX_v0.86.md`)
- [ ] DESIGN, WBS, and implementation are aligned (no conceptual drift)
- [ ] All v0.86 milestone-defining planning docs are implemented in at least one execution path and aligned with the tracked docs

### Explicit GO / NO-GO Questions

Answer ALL before release:

- [ ] Does the system execute a full bounded cognitive loop end-to-end (signals → arbitration → reasoning → execution → evaluation → reframing → memory → Freedom Gate)?
- [ ] Are arbitration decisions visible and correct (fast vs slow)?
- [ ] Do instinct and affect signals visibly influence arbitration and routing decisions?
- [ ] Is candidate selection (agency) observable and real?
- [ ] Does the Freedom Gate allow, defer, and refuse at least one case with inspectable artifacts?
- [ ] Can a reviewer run one command and find all proof artifacts?
- [ ] Does at least one run demonstrate bounded execution (iteration), evaluation affecting behavior, and reframing/adaptation?

If any answer is NO → DO NOT RELEASE.

Record decision:
- GO / NO-GO decision: `Pending milestone completion review`
- Decision record / notes: `To be recorded at release gate`

---

## 2) Branch And Tag Preparation

- [ ] Target branch confirmed: `main`
- [ ] Working tree clean (no uncommitted changes)
- [ ] All PRs for v0.86 merged via `pr.sh`
- [ ] Version references updated (if applicable)

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
- [ ] Release notes copied from `RELEASE_NOTES_v0.86.md`
- [ ] Links to key PRs/issues included (WP-01 through WP-23)
- [ ] Demo instructions included (one-command entry point)
- [ ] Release visibility set correctly (likely `final`, not prerelease)

Publish:

- [ ] Release published

---

## 4) Verification

Immediately after publishing:

- [ ] CI status checked on `main`
- [ ] Tag `v0.86` resolves correctly
- [ ] Demo commands run successfully from a clean checkout
- [ ] Demo artifacts are generated and inspectable
- [ ] Links in release notes work (docs, demos, PRs)

If any failure occurs:
- create issue
- triage immediately
- decide: patch vs defer

---

## 5) Communication

- [ ] Internal update posted (milestone completion)
- [ ] Roadmap/status docs updated

Optional (depending on timing/strategy):
- [ ] Public / LinkedIn announcement

---

## 6) Post-Release Closure

- [ ] All milestone issues closed with release reference
- [ ] Deferred items moved to next milestone
- [ ] Bugs or follow-ups captured as issues
- [ ] Next milestone planning (`#882`) is already in progress or complete

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
