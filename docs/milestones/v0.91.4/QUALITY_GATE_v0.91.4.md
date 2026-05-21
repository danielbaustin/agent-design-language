# v0.91.4 Quality Gate

## Status

Planned quality gate.

## Required Validation

The milestone must run:

- lifecycle validator fixture tests
- doctor/conductor routing tests
- editor-skill repair tests
- actor-standing and authority-boundary fixture tests
- sprint-conductor closeout tests
- evidence bundle and review synthesis tests
- ObsMem handoff validation
- repeated five-minute-sprint metrics capture
- combined C-SDLC lane validation
- active-issue migration policy review with sampled issue routing
- process-drift regression fixture results for legacy SRP, stale SOR, skipped
  closeout, and unsafe state advancement
- docs/adoption review for operator, skill, and onboarding surfaces
- internal review and external / 3rd-party review over the corrected package
- review-finding remediation with explicit finding dispositions
- next-milestone planning and final next-milestone review pass before ceremony
- release evidence packet covering feature proof, tail-work proof, residual
  risks, and follow-on routing
- tracked workflow-record path checks for durable C-SDLC cards, sprint state,
  closeout, reviews, proof packets, traces, and release evidence under
  `workflow/c-sdlc/v0.91.4/`
- signed trace bundle verification for durable C-SDLC proof
- ObsMem ingestion check over tracked evidence

## Blockers

The milestone is blocked if:

- new issue cards can still bootstrap with legacy SRP semantics
- conductor routing skips required editor/lifecycle stages
- sprint state can advance past unclosed child truth
- actor standing, role authority, or shard ownership can be claimed from
  chat-only or local-only evidence
- SOR closeout can overclaim merge/main truth
- memory handoff can consume stale SRP or SOR records
- five-minute-sprint evidence weakens governance or review
- active issues can remain in an ambiguous lifecycle state without an explicit
  migrate/defer/no-op decision
- regression fixtures do not cover the process-drift failures that motivated
  v0.91.3 and v0.91.4
- docs and release evidence can claim milestone completion without migration,
  regression, proof coverage, quality gate, docs/adoption review, internal
  review, external review, remediation, next-milestone planning,
  next-milestone review, and release proof
- durable C-SDLC records remain only in ignored `.adl` or `artifacts/` paths
  instead of the documented `workflow/c-sdlc/v0.91.4/` namespace
- signed trace bundles are missing or unverifiable for durable C-SDLC proof
- ObsMem ingestion depends on untracked local evidence
