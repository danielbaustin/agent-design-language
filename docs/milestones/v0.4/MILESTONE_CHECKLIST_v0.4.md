# ADL v0.4 Milestone Checklist

## Metadata
- Milestone: `v0.4`
- Version: `0.4.0`
- Target release date: `2026-02-18`
- Owner: Daniel Austin

## Planning
- [x] Milestone goal defined (`docs/milestones/v0.4/DESIGN_v0.4.md`)
- [x] Scope + non-goals documented (`docs/milestones/v0.4/DESIGN_v0.4.md`)
- [x] WBS created and mapped to issues (`docs/milestones/v0.4/WBS_v0.4.md`)
- [x] Decision log finalized (`docs/milestones/v0.4/DECISIONS_v0.4.md`)
- [x] Sprint summary finalized (`docs/milestones/v0.4/SPRINT_v0.4.md`)

## Execution Discipline
- [x] Issue work tracked with input/output cards
- [x] Burst artifacts captured where required
- [x] Draft PR flow used before merge
- [x] Transient failures documented and self-healed
- [x] Green-only merge policy followed

## Quality Gates
- [x] `cargo fmt` passes
- [x] `cargo clippy --all-targets -- -D warnings` passes
- [x] `cargo test` passes
- [x] CI is green on merge target
- [x] Coverage signal not red
- [x] No unresolved high-priority runtime blockers for v0.4 scope

## Release Packaging
- [x] Release notes finalized (`docs/milestones/v0.4/RELEASE_NOTES_v0.4.md`)
- [x] Tag verified: `v0.4.0`
- [x] GitHub Release drafted/published (`https://github.com/danielbaustin/agent-design-language/releases/tag/v0.4.0`)
- [x] Milestone links validated in docs

## Post-Release
- [x] Demo pass merged and issue closed (#306 / #307)
- [x] Remaining open milestone umbrella issues closed with release links (`#290`, `#291`)
- [ ] Deferred items moved to next milestone backlog (v0.5)
- [ ] Retrospective summary recorded

## Exit Criteria
Milestone implementation and release publication are complete; remaining work is post-release process follow-through.
