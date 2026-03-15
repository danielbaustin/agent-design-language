# Milestone Checklist — v0.8

## Metadata
- Milestone: `v0.8`
- Version: `0.8`
- Target release date: `TBD`
- Owner: `Daniel Austin / Agent Logic`

## Purpose
Ship/no-ship gate for v0.8. Check items only when evidence exists.

Status note:
- v0.8 is in active development.
- This checklist tracks milestone completion and should not be interpreted as release-ready.

## Planning
- [x] Milestone goal defined (`docs/milestones/v0.8/DESIGN_V0.8.md`)
- [x] Scope + non-goals documented (`docs/milestones/v0.8/VISION_0.80.md`)
- [x] WBS created and mapped to issues (`docs/milestones/v0.8/WBS_V0.8.md`)
- [x] Decision log initialized (`docs/milestones/v0.8/DECISIONS_V0.8.md`)
- [x] Sprint plan created (`docs/milestones/v0.8/SPRINT_V0.8.md`)

## Execution Discipline
- [ ] Each in-scope issue has complete input/output card traceability
- [ ] Draft PR opened for each issue before merge
- [ ] Deterministic evidence retained for demo and review surfaces
- [ ] Transient failures retried and documented
- [ ] Green-only merge policy consistently enforced

## Quality Gates
- [ ] `cargo fmt` passes on release candidate branch
- [ ] `cargo clippy --all-targets -- -D warnings` passes on release candidate branch
- [ ] `cargo test` passes on release candidate branch
- [ ] `bash swarm/tools/check_no_new_legacy_swarm_refs.sh` passes on release candidate branch
- [ ] `bash tools/check_release_notes_commands.sh` passes on release candidate branch
- [ ] `bash swarm/tools/demo_smoke_v07_story.sh` passes on release candidate branch
- [ ] Coverage gates pass (`cargo llvm-cov` + `tools/enforce_coverage_gates.sh`)
- [ ] CI is green on merge target
- [ ] v0.8 quality gate status is green (`docs/milestones/v0.8/QUALITY_GATE_V0.8.md`)
- [ ] No unresolved blocker-grade findings (`docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md` + review artifacts)

## Release Packaging
- [ ] Release notes finalized (`docs/milestones/v0.8/RELEASE_NOTES_V0.8.md`)
- [ ] Tag verified (`v0.8.0` or superseding tag decision)
- [ ] GitHub Release drafted
- [ ] Links validated in release body
- [ ] Release published

## Post-Release
- [ ] Milestone/epic issues closed with release links
- [ ] Deferred items moved to next milestone backlog
- [ ] Follow-up bugs/tech debt captured as issues
- [ ] Roadmap/status docs updated
- [ ] Retrospective summary recorded

## Exit Criteria
- All required gates are checked, or each exception has an owner + due date.
- Milestone can be audited end-to-end via linked evidence.
