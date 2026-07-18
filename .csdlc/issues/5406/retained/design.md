# Issue 5406 Design

## Goal

Make typed C-SDLC v2 lifecycle records sufficient for clean-checkout audit and
truthful post-bind corrections without restoring sunset v1 command surfaces.

## Contract Changes

- Add collision-checked active-claim scope amendment after conflicting claims
  are terminally released.
- Add typed SPP step-status mutation with lifecycle and transition guards.
- Add typed VPP lane/proof-role replacement with validation and audit truth.
- Retain historical issue, PR, revision, review, validation, and terminal
  references in portable tracked evidence.

## Boundaries

The implementation is confined to independent `csdlc-v2`, its tests, and a
new retained #5406 evidence packet. It does not restore v1 wrappers, mutate
Runtime code, or rewrite historical proof.
