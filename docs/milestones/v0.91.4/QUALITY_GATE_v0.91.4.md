# v0.91.4 Quality Gate

## Status

Planned quality gate.

## Required Validation

The milestone must run:

- lifecycle validator fixture tests
- doctor/conductor routing tests
- editor-skill repair tests
- sprint-conductor closeout tests
- evidence bundle and review synthesis tests
- ObsMem handoff validation
- repeated five-minute-sprint metrics capture
- combined C-SDLC lane validation
- active-issue migration policy review with sampled issue routing
- process-drift regression fixture results for legacy SRP, stale SOR, skipped
  closeout, and unsafe state advancement
- docs/adoption review for operator, skill, and onboarding surfaces
- release evidence packet covering feature proof, tail-work proof, residual
  risks, and follow-on routing

## Blockers

The milestone is blocked if:

- new issue cards can still bootstrap with legacy SRP semantics
- conductor routing skips required editor/lifecycle stages
- sprint state can advance past unclosed child truth
- SOR closeout can overclaim merge/main truth
- memory handoff can consume stale SRP or SOR records
- five-minute-sprint evidence weakens governance or review
- active issues can remain in an ambiguous lifecycle state without an explicit
  migrate/defer/no-op decision
- regression fixtures do not cover the process-drift failures that motivated
  v0.91.3 and v0.91.4
- docs and release evidence can claim milestone completion without `WP-11`,
  `WP-12`, `WP-15`, and `WP-16` proof
