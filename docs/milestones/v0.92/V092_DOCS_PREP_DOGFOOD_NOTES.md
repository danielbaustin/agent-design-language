# v0.92 Docs Prep Dogfood Notes

## Status

Issue `#3434` dogfood notes for the first clean v0.92 planning-doc preparation
pass using the versioned planning-template process.

These notes are not a review packet and do not approve the v0.92 milestone.
They record process observations that should inform future planning-template
and editor work.

## What Worked

- `pr create` rejected an incomplete authored issue body until the required
  `Notes` section was present.
- The issue was created with all five C-SDLC cards upfront.
- `pr doctor` correctly blocked execution until the `SPP` moved from `draft`
  to a reviewed execution-ready state.
- The existing v0.92 docs package already had most of the substantive content
  needed for the first-birthday milestone.
- The planning-template shape made missing metadata, source-map, and issue-wave
  readiness surfaces easy to see.

## Problems Or Friction Observed

- The generated issue body scaffold still depends on section-name validation;
  a missing `Notes` section blocked creation even though the intent was clear.
- The created `SPP` was specific enough to be useful, but still required a
  manual status promotion to become execution-ready.
- The current v0.92 docs predated the versioned planning templates, so several
  canonical docs lacked `Metadata` sections even though their content was good.
- The existing v0.92 docs said there was no issue wave, but WP-01 needed a
  concrete candidate issue-wave seed to move quickly later.
- The ACIP feature doc still referenced local-only `.adl/docs/TBD/` source
  notes as if they were canonical public inputs.
- Adding template metadata is not enough. The planning-template validator
  requires exact required section headings, so legacy-but-good docs still fail
  until they are structurally normalized.
- The planning-template validator validates one document and one template key
  at a time. A package-level validation helper would make milestone review
  faster and less error-prone.
- In zsh, using `path` as a loop variable can shadow the shell command search
  path. One validation loop failed before rerunning with a safer variable name.

## Actions Taken In This Pass

- Added planning-template metadata to the v0.92 canonical docs touched by this
  issue.
- Added `#3377` as a required first-birthday readiness source across the
  birthday package.
- Added `WP_ISSUE_WAVE_v0.92.yaml` as a draft pre-open issue-wave seed for
  v0.92 WP-01.
- Clarified that the candidate issue wave is not an opened GitHub issue wave.
- Reframed local-only ACIP source notes as provenance inputs that WP-01 must
  promote or route as gaps.
- Normalized the ten canonical v0.92 planning docs against
  `docs/templates/planning/1.0.0`.
- Normalized the v0.92 feature-doc package against the active `feature_doc`
  template.
- Added missing feature contracts for cross-polis continuity/migration planning
  and first-birthday demo/governance handoff coverage.
- Added `ADR_PLAN_v0.92.md` so WP-01 and the review tail can track candidate
  architecture decisions explicitly.
- Ran focused structural validation for every canonical planning doc and every
  v0.92 feature doc.

## Follow-Up Candidates

- Make planning-template metadata insertion more automated for existing
  milestone docs.
- Add a planning-doc readiness checker that detects missing metadata, missing
  source maps, local-only source references, and absent issue-wave preflight.
- Add a package-level planning-template validator that maps canonical milestone
  filenames to template keys and validates feature-doc directories in one
  command.
- Consider a stricter generator mode that can create a candidate
  `WP_ISSUE_WAVE_VERSION.yaml` from a WBS without opening GitHub issues.
