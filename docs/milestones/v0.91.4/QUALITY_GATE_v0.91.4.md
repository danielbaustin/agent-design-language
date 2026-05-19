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

## Blockers

The milestone is blocked if:

- new issue cards can still bootstrap with legacy SRP semantics
- conductor routing skips required editor/lifecycle stages
- sprint state can advance past unclosed child truth
- SOR closeout can overclaim merge/main truth
- memory handoff can consume stale SRP or SOR records
- five-minute-sprint evidence weakens governance or review

