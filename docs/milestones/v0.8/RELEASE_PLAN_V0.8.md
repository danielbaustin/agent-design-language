
# Release Plan — v0.8

## Metadata
- Milestone: `v0.8`
- Version: `0.8`
- Release date: `TBD`
- Release manager: `Daniel Austin / Agent Logic`

## How To Use
- Execute sections in order and capture links/artifacts for each completed step.
- Keep this doc focused on shipping mechanics; use release notes for narrative and positioning.
- Mark blockers immediately; do not publish until all required gates pass or explicit deferrals are documented.
- The release tail must occur in the sequence: **documentation freeze → 3rd party review → review fixes/deferrals → release ceremony**.

## Release Strategy
v0.8 is intended to ship as a focused milestone for **controlled experimentation and authoring**. The release should present ADL as:

- a deterministic workflow/runtime substrate extended with experiment artifacts
- a framework for structured card/prompt driven execution
- a provider-agnostic, auditable system for bounded improvement workflows
- a platform capable of demonstrating a serious engineering workflow via the Rust transpiler / migration demo

The release should avoid over-claiming. If any major v0.8 feature is deferred, that deferral must be explicit in the release notes.

## 1) Release Readiness
- [ ] Milestone checklist complete (`docs/milestones/v0.8/MILESTONE_CHECKLIST_V0.8.md`)
- [ ] Design / WBS / sprint / decisions docs are aligned and current
- [ ] Release notes approved (`docs/milestones/v0.8/RELEASE_NOTES_V0.8.md`)
- [ ] Go/no-go decision recorded in milestone artifacts
- [ ] No unknown critical blockers remain

## 2) Branch And Tag Preparation
- [ ] Target branch confirmed (`main`, unless explicitly overridden)
- [ ] Working tree clean
- [ ] Version string(s) validated in docs/code where applicable
- [ ] Release tag name confirmed (`v0.8.0` unless superseded)
- [ ] Tag created locally
- [ ] Tag pushed and verified remotely

## 3) Required Pre-Release Sequence
### 3.1 Documentation Freeze
- [ ] v0.8 milestone docs aligned with implemented repo state
- [ ] Demo commands and examples validated
- [ ] No unresolved placeholders remain (`{{...}}`, TODO, FIXME)
- [ ] Docs explicitly marked frozen for review

### 3.2 3rd Party Review
- [ ] Independent / 3rd party review executed
- [ ] Findings captured in an output card or equivalent review artifact
- [ ] Blocking findings fixed or explicitly deferred with rationale

### 3.3 Review Convergence
- [ ] Review-fix PRs merged
- [ ] Any deferred findings listed explicitly in release notes or follow-up issues
- [ ] Final go/no-go check performed after review convergence

## 4) GitHub Release Steps
- [ ] GitHub Release draft created from the approved tag
- [ ] Release body populated from approved release notes
- [ ] Links to key PRs/issues/docs included
- [ ] Release visibility confirmed (draft / prerelease / final)
- [ ] Release published

## 5) Verification
- [ ] Post-release CI status checked
- [ ] Declared deterministic demos verified or links to their final validated artifacts included
- [ ] Release links tested (docs, notes, key demo references)
- [ ] Immediate regressions triaged and tracked explicitly

## 6) Communication
- [ ] Community/public announcement prepared or explicitly skipped
- [ ] Internal update posted or recorded
- [ ] Roadmap / status docs updated
- [ ] Key positioning points aligned with release notes

## Release Artifacts Checklist
- [ ] Release notes
- [ ] Tag / GitHub Release
- [ ] Demo matrix references
- [ ] Review artifact(s)
- [ ] Decision log / checklist alignment
- [ ] Any explicit deferral list

## Blocking Conditions
Do **not** publish the release if any of the following are true:

- milestone docs are not frozen before review
- 3rd party review has not occurred
- blocking review findings remain unresolved and undeferred
- CI is red on the merge target
- release notes materially misdescribe shipped functionality

## Exit Criteria
- Tag and GitHub Release are published and accessible.
- Documentation freeze, 3rd party review, review convergence, and release ceremony occurred in order.
- Verification completed with no unknown critical failures.
- Communication artifacts or explicit skip decisions are captured.
- The release can be audited end-to-end via docs, cards, PRs, and release artifacts.
