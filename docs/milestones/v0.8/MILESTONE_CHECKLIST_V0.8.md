# Milestone Checklist — v0.8

## Metadata
- Milestone: `v0.8`
- Version: `0.8`
- Target release date: `TBD`
- Owner: `Daniel Austin / Agent Logic`
- Last validated: `2026-03-14`

## Purpose
Ship/no-ship gate for v0.8. Check items only when evidence exists.

Status note:
- v0.8 is in active development and not yet released.
- This checklist tracks release readiness and must reflect repository truth.

Deferral convention:
- Unchecked items in this document are explicitly deferred to release-candidate/final ceremony unless marked `IN_PROGRESS`.
- Deferred owner: v0.8 release-tail finalization.

## Planning
- [x] Milestone goal defined (`docs/milestones/v0.8/DESIGN_V0.8.md`)
- [x] Scope + non-goals documented (`docs/milestones/v0.8/VISION_0.80.md`)
- [x] WBS created and mapped to issues (`docs/milestones/v0.8/WBS_V0.8.md`)
- [x] Decision log initialized (`docs/milestones/v0.8/DECISIONS_V0.8.md`)
- [x] Sprint plan created (`docs/milestones/v0.8/SPRINT_V0.8.md`)

## Execution Discipline
- [ ] Each in-scope issue has complete input/output card traceability (deferred to final release-tail audit)
- [ ] Draft PR opened for each issue before merge (deferred to final release-tail audit)
- [ ] Deterministic evidence retained for demo and review surfaces (deferred to final release-tail audit)
- [ ] Transient failures retried and documented (deferred to final release-tail audit)
- [ ] Green-only merge policy consistently enforced (deferred to final release-tail audit)

## Quality Gates
- [ ] `cargo fmt` passes on release candidate branch (deferred to release-candidate cut)
- [ ] `cargo clippy --all-targets -- -D warnings` passes on release candidate branch (deferred to release-candidate cut)
- [ ] `cargo test` passes on release candidate branch (deferred to release-candidate cut)
- [ ] `bash swarm/tools/check_no_new_legacy_swarm_refs.sh` passes on release candidate branch (deferred to release-candidate cut)
- [ ] `bash tools/check_release_notes_commands.sh` passes on release candidate branch (deferred to release-candidate cut)
- [ ] `bash swarm/tools/demo_smoke_v07_story.sh` passes on release candidate branch (deferred to release-candidate cut)
- [ ] Coverage gates pass (`cargo llvm-cov` + `tools/enforce_coverage_gates.sh`) (deferred to release-candidate cut)
- [ ] CI is green on merge target (deferred to release-candidate cut)
- [ ] v0.8 quality gate status is green (`docs/milestones/v0.8/QUALITY_GATE_V0.8.md`) (deferred to release-candidate cut)
- [ ] No unresolved blocker-grade findings (`docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md` + review artifacts) (deferred to final review closeout)

## Release Packaging
- [ ] Release notes finalized (`docs/milestones/v0.8/RELEASE_NOTES_V0.8.md`) (`IN_PROGRESS`: issue `#811`)
- [ ] Tag verified (`v0.8.0` or superseding tag decision) (deferred to release ceremony)
- [ ] GitHub Release drafted (`IN_PROGRESS`: issue `#813`)
- [ ] Links validated in release body (deferred to release ceremony)
- [ ] Release published (deferred to release ceremony)

## Post-Release
- [ ] Milestone/epic issues closed with release links (deferred to post-release)
- [ ] Deferred items moved to next milestone backlog (deferred to post-release)
- [ ] Follow-up bugs/tech debt captured as issues (deferred to post-release)
- [ ] Roadmap/status docs updated (deferred to post-release)
- [ ] Retrospective summary recorded (deferred to post-release)

## Exit Criteria
- All required gates are checked, or each exception has an owner + due date.
- Milestone can be audited end-to-end via linked evidence.
