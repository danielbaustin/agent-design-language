# Public C-SDLC Prompt Packet: issue-3562

## Summary

This packet exports the public prompt-card record for `v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend` in `v0.91.5`.

## Source

- Source bundle: `.adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend`
- Output packet: `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend`
- Tracker URL: `https://github.com/danielbaustin/agent-design-language/issues/3562`

## Contents

- `cards/sip.md`
- `cards/stp.md`
- `cards/spp.md`
- `cards/srp.md`
- `cards/sor.md`
- `manifest.json`

## Safety Boundary

The exporter refuses obvious host-local paths, secret-like tokens, private key markers, local scratch paths, and unresolved template markers. It does not rewrite card content during export.

## Non-Claims

- This packet does not make local `.adl` state canonical public truth.
- This packet does not claim runtime validation was executed.
- This packet is a reviewable prompt-record surface only.
