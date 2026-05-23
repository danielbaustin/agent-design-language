# v0.91.4 Release Notes

## Metadata

- Product: `Agent Design Language`
- Version: `v0.91.4`
- Release date: pending release ceremony
- Tag: `v0.91.4`

## How To Use

These are draft release notes for the planned milestone package. Final release
notes must be refreshed during the release ceremony and must describe only
shipped behavior.

# `Agent Design Language` `v0.91.4` Release Notes

## Summary

`v0.91.4` completes the C-SDLC rollout by hardening the first slice into a
repeatable default development lane for future ADL software work.

## Highlights

- Makes C-SDLC the default path for future ADL software-development issues.
- Hardens validators, doctor, conductor, and editor routing around lifecycle
  truth.
- Adds Software Development Polis actor-standing and shard-ownership support
  needed for repeated execution.
- Tracks durable workflow records in Git.
- Adds signed trace proof and verification evidence.
- Integrates SRP/SOR outcome truth with ObsMem handoff.
- Measures repeatable five-minute-sprint behavior, including validation-tail
  and proof-latency behavior.
- Plans a Parallel Validation Fabric so proof can become shardable without
  hiding pending or failed validation.
- Carries a bounded CodeFriend pre-alpha repo/S3 welcome-page setup sidecar,
  if completed or truthfully blocked by release time.

## What's New In Detail

### Lifecycle And Routing

- Validators reject or route stale lifecycle states.
- Workflow conductor, sprint conductor, doctor, and editor skills align on
  C-SDLC stages.
- Process-drift regression fixtures catch known card and closeout failures.

### Transition Operation

- Actor roles, standing, shard ownership, and interface-freeze rules become
  explicit enough for repeated C-SDLC execution.
- Evidence convergence and review synthesis are repeatable.
- Merge-readiness gates preserve issue, PR, branch, CI, review, evidence, trace,
  and closeout truth.

### Durable State, Trace, And Memory

- Durable C-SDLC records move into tracked Git workflow namespace.
- Signed trace bundles provide verification evidence.
- ObsMem ingestion consumes tracked evidence rather than local-only lore.

## Upgrade Notes

- Future ADL software-development issues should use the C-SDLC default lane.
- Durable workflow records must be tracked; local `.adl` state is not enough.
- Active issues should follow the migration policy rather than ad hoc rewrites.

## Known Limitations

- Broader CodeFriend alpha product work remains separate from this C-SDLC
  completion milestone; only the bounded repo/S3 welcome-page sidecar is
  scheduled here.
- Wider Software Development Polis feature expansion beyond execution standing
  may be scheduled later.
- External product packaging remains outside the release unless explicitly
  scheduled.

## Validation Notes

- Final validation must come from lifecycle/tooling tests, signed trace
  verification, demo matrix, quality gate, review packets, and release ceremony.
- This draft must be reconciled against completed issues before release.

## What's Next

- Later milestones can build faster productized review, CodeFriend alpha work,
  broader social-cognition features, and post-`v0.95` repo strategy on a stable
  C-SDLC foundation.

## Exit Criteria

- Release notes reflect only shipped behavior after final refresh.
- Known limitations and future work remain explicit.
- Final text is ready for the GitHub Release body.
