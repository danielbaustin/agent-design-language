# v0.91.6 Public Prompt Records Reviewer Index

## Summary

This directory is the milestone-local reviewer-facing index for public prompt
records in `v0.91.6`.

It does not claim that `v0.91.6` generated a new packet root from local `.adl`
state during this docs-only wave. Instead, it provides one local reviewer
navigation surface over the currently accepted, carried-forward pilot packets
while the `#4002` through `#4006` proof set defines the export, redaction,
validation, indexing, and security-routing contract.

## Included validated packet set

| Issue | Packet | Surface | Selection reason | Status represented |
| --- | --- | --- | --- | --- |
| `#3472` | [issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter](../../../../../v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/) | Tooling/process | Canonical exporter implementation packet and primary happy-path export proof. | Closed issue / merged PR |
| `#3473` | [issue-3473-v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive](../../../../../v0.91.5/review/evidence/csdlc/issues/issue-3473-v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/) | Docs/local state | Proves public packets can coexist with explicit local `.adl` boundary truth. | Closed issue / merged PR |
| `#3562` | [issue-3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend](../../../../../v0.91.5/review/evidence/csdlc/issues/issue-3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/) | Review-provider lane | Keeps the bridge evidence from collapsing into exporter-only examples. | Closed issue / merged PR |

## Index rules

- Only accepted validated packet examples belong in this index.
- Refused or invalid packet attempts belong in proof notes or remediation
  records, not here.
- The reviewer index is navigation truth, not authoring truth.
- Local `.adl` task bundles remain the editable source of truth.
- This bridge index uses repo-relative links only.

## Bridge limitation

This index is a `v0.91.6` bridge closeout surface, not proof that the milestone
already has a newly exported packet root of its own. The current accepted
packet set is still the carried-forward validated pilot under `v0.91.5`, and
future automation may replace this bridge posture with milestone-local packet
generation.
