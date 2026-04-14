# Release Notes - v0.88

## Status

Final ceremony draft.

`v0.88` has a completed implementation wave through `WP-13` plus a completed review and planning tail through `WP-19`.
These notes are now aligned to shipped milestone truth and are waiting only on the final release-ceremony publish steps on `main`.
The release claim is based on delivered code, tests, demos, artifacts, and completed review-tail evidence rather than on planning alignment alone.

Planning/package issues already represented:
- `#1527`
- `#1579`
- `#1497`
- `#1643`

Completed implementation issues already represented:
- `#1644`
- `#1646`
- `#1648`
- `#1650`
- `#1651`
- `#1653`
- `#1655`
- `#1645`
- `#1649`
- `#1654`
- `#1656`
- `#1657`

Final ceremony issue:
- `#1663`

## Current Public Package
- chronosense / substance-of-time foundation
- temporal schema and anchors
- continuity and identity semantics
- temporal query and retrieval
- commitments and deadlines
- temporal causality and explanation
- execution policy and cost model
- PHI-style integration metrics
- instinct model
- instinct runtime surface
- Paper Sonata flagship demo
- deep-agents comparative proof surface

## Scope Notes
- Scope is closed for `v0.88`.
- The only accepted supporting backlog pull-ins are `#1614` and `#1618`.
- protected local follow-on planning remains for deepening `Paper Sonata`
  beyond the bounded `v0.88` slice.
- The implementation and closeout issue wave exists; the remaining work is reviewer-truth convergence, review execution, remediation, and ceremony.
- The closeout tail ran in the standard bounded order:
  - `WP-14` quality gate
  - `WP-15` docs + review pass
  - `WP-16` internal review
  - `WP-17` 3rd-party review
  - `WP-18` review findings remediation
  - `WP-19` next milestone planning
  - `WP-20` release ceremony

## Quality Gate

The canonical Sprint 3 quality posture is:

- `docs/milestones/v0.88/QUALITY_GATE_v0.88.md`
- `bash adl/tools/demo_v088_quality_gate.sh`
- primary artifact: `artifacts/v088/quality_gate/quality_gate_record.json`

The final ceremony gate is backed by:

- `bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.88`
- `bash adl/tools/check_release_notes_commands.sh`
- latest green `main` CI run on commit `6bf4ef71c93a95d19819cbe53811253bbd724381`
