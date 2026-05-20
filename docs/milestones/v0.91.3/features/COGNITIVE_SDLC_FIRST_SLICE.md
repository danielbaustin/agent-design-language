# Cognitive SDLC First Slice

## Metadata

- Feature Name: Cognitive SDLC First Slice
- Milestone Target: `v0.91.3`
- Status: planned
- Planned WP Home: WP-01 through WP-13

## Purpose

Prove one governance-complete Cognitive State Transition inside ADL's existing
GitHub issue/PR workflow.

## Core Contract

The slice must preserve the corrected card lifecycle:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

The transition must show:

- issue intent
- task transformation
- issue-local operative execution plan
- review prompt and review results
- outcome truth
- evidence bundle
- merge-readiness gate
- memory handoff boundary
- tracked source package and trace/proof references

## Acceptance Criteria

- The transition is reviewable end to end.
- Serial work, parallel shards, and synchronization barriers are explicit.
- Review results and residual risks are recorded before PR publication.
- SOR outcome truth is normalized after merge or intentional closure.
- GitHub issue/PR/CI/human review remains authoritative for repository change.
- C-SDLC planning evidence is tracked in the milestone package, not only in
  local `.adl/docs/TBD` notes.
- The operative `SPP` contract is public/tracked and issue-local; sprint
  orchestration remains outside the `SPP` role.
- Trace/proof references are repo-relative and ready for v0.91.4 signed trace
  bundles.

## Non-Claims

This feature does not claim all future ADL development uses C-SDLC by default.
That belongs to v0.91.4.
