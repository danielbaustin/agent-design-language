# ADL Planning Docs

This directory contains tracked, cross-milestone planning documents that should
survive individual milestone closeout.

Use this directory for living planning surfaces that are updated as each
milestone completes.

Current planning docs:

- `ADL_FEATURE_LIST.md` - full feature list, current status, implemented gaps,
  and target completion milestones through `v0.95`
- `DEEP_AGENTS_COMPARATIVE_WAVE.md` - future comparative/public-positioning
  direction beyond the bounded `v0.88` proof row
- `NEXT_MILESTONE_POSITIONING_CANDIDATES.md` - curated positioning/demo
  candidates that are ready for milestone promotion discussion
- `GEMINI_PROVIDER_HARMONY_AND_ECONOMICS_DEMO.md` - future Gemini-centered
  provider-harmony demo direction beyond the bounded `v0.89` slice
- `NEXT_MILESTONE_DEMO_CANDIDATES.md` - curated future-demo candidates ready
  for milestone promotion discussion

## Directory Boundaries

This directory is for living cross-milestone planning. It should stay small and
intentional.

Use milestone directories for shipped or active milestone records, including
feature contracts, ideas/backgrounders, demo matrices, release notes, and WBS
files for a specific version.

Use `docs/records/` for historical task mirrors and closeout records that are
kept for auditability but are not active planning surfaces.

Use `docs/tooling/` for tool, editor, demo, and workflow-facing guides that
remain useful outside a single milestone.

Do not use this directory as a dumping ground for retired drafts, generated
review traces, temporary files, or local workspace artifacts. Those should
either be moved into a milestone archive, kept under local-only `.adl/`
workspace state, or removed as generated cruft.
