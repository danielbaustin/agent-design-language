# ADL Planning Docs

This directory contains tracked, cross-milestone planning documents that should
survive individual milestone closeout.

Use this directory for living planning surfaces that are updated as each
milestone completes.

Active planning docs:

- `ADL_FEATURE_LIST.md` - full feature list, current status, implemented gaps,
  and target completion milestones through `v0.95`
- `ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md` - cross-milestone Runtime v2,
  v0.91 moral/emotional, and v0.92 birthday boundary roadmap

Planning provenance docs:

- `DEEP_AGENTS_COMPARATIVE_WAVE.md` - delivered/provenance planning note for
  the comparative wave that fed later bounded demos and remains useful only if
  a new public-positioning issue is opened
- `GEMINI_PROVIDER_HARMONY_AND_ECONOMICS_DEMO.md` -
  delivered/provenance planning note for provider-harmony demo work that fed
  later bounded Gemini demos and remains useful only if a new provider-routing
  issue is opened
- `NEXT_MILESTONE_DEMO_CANDIDATES.md` - closed candidate ledger; currently
  records delivered or deferred demo candidates rather than active next-sprint
  commitments
- `NEXT_MILESTONE_POSITIONING_CANDIDATES.md` - closed candidate ledger;
  currently records delivered or deferred positioning candidates rather than
  active next-sprint commitments
- `DOC_CLEANUP_RECONCILIATION_1762.md` - historical cleanup reconciliation for
  issue `1762`; keep as provenance, not as live cleanup backlog

## Directory Boundaries

This directory is for living cross-milestone planning and a very small amount
of tracked planning provenance. It should stay small and intentional.

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

When a planning doc stops being an active candidate, update this README in the
same PR that changes its status. Do not leave old "next milestone" language
behind after the candidate has landed, moved to backlog, or become provenance.
