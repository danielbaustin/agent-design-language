# CodeFriend GWS Git Workspace Boundary Runbook

## Rule Zero

Google Workspace supports draft workflow and bounded content-card operations.
GitHub issue/PR flow remains canonical for tracked repository truth.

## Use Workspace For

- draft planning and review doc inventory
- bounded doc snapshot reads
- bounded content-card sheet reads
- bounded preview/apply contract flows
- promotion-packet preparation context

## Use GitHub For

- milestone-doc truth
- tracked repo file changes
- public planning-document updates
- issue-backed promotion decisions
- merge approval and final publication

## Boundary Tests

If an operator asks "can Workspace do this directly?", answer with these tests:

1. Is the target a tracked repository artifact?
   - If yes, use GitHub issue/PR flow.
2. Is the target only draft planning/content-card state inside the bounded
   Workspace scope?
   - If yes, Workspace may be appropriate.
3. Does the action widen authority or affect canonical truth?
   - If yes, stop and route through issue/PR review.

## Common Safe Pattern

- Workspace prepares or updates bounded draft/context state.
- ADL records the evidence packet.
- GitHub issue/PR controls canonical tracked changes.

## Common Unsafe Pattern

- Workspace content changes are treated as the new canonical project truth.
- The repo is silently edited from Workspace state.
- Execute mode is used without explicit operator awareness.

## Operator Reminder

The bridge is valuable precisely because it is bounded. If a project starts
wanting broad, ambient, or hidden Workspace authority, that is a sign to stop
and open a new issue rather than stretching this package beyond its contract.
