# Public C-SDLC Prompt Packet: issue-3472

## Summary

This packet exports the public prompt-card record for `v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter` in `v0.91.5`.

## Source

- Source bundle: `.adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter`
- Output packet: `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter`
- Tracker URL: `https://github.com/danielbaustin/agent-design-language/issues/3472`

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
