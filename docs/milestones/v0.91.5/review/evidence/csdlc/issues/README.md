# v0.91.5 Public C-SDLC Prompt Packet Pilot

## Summary

This directory is the reviewer-facing pilot index for public C-SDLC prompt
packets in `v0.91.5`. It proves that selected issue prompt records can be
exported from local `.adl` execution state into tracked, reviewable repository
evidence without making raw local `.adl` state canonical public truth.

## Pilot Selection

| Issue | Packet | Surface | Selection reason | Status represented |
| --- | --- | --- | --- | --- |
| `#3472` | [issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter](issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/) | Tooling/process | Exporter implementation issue; proves the tool can publish its own prompt record. | Closed issue / merged PR |
| `#3473` | [issue-3473-v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive](issue-3473-v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/) | Docs/local state | Local `.adl` disposition issue; proves public packets can coexist with explicit local-cache boundaries. | Closed issue / merged PR |
| `#3562` | [issue-3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend](issue-3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/) | Review-provider lane | Review/provider contract issue; gives the pilot a review-lane packet rather than only docs/tooling records. | Closed issue / merged PR |

## Packet Contents

Each pilot packet includes:

- `manifest.json`
- `README.md`
- `cards/sip.md`
- `cards/stp.md`
- `cards/spp.md`
- `cards/srp.md`
- `cards/sor.md`

The manifest records:

- issue number and tracker URL
- exported card paths
- source card bundle path relative to the repository
- prompt-template registry
- redaction mode and checks
- non-claims for local `.adl` state and runtime validation

## Provenance

The source prompt-card bundles were local `.adl/v0.91.5/tasks/` execution
records for the selected issues. The export process copies card text after
validation and public-output hygiene checks; it does not rewrite card content
to make the history look cleaner.

The tracked packet is the review surface. The source `.adl` task bundle remains
local execution cache and is not tracked by this issue.

## Validation And Redaction Posture

The exporter refuses prompt cards with:

- host-local absolute paths
- secret-like token markers
- private-key markers
- local scratch path markers
- unresolved template placeholders
- invalid structured prompt shape for the card kind during export-time source
  hygiene checks

This pilot also runs focused validation over the tracked packet set:

- manifest JSON parse checks
- exported card presence checks
- diagnostic current-template structure checks
- Markdown link checks
- public-output redaction scan
- `git diff --check`

The current-template structure checks are diagnostic for this pilot, not a
claim that every historical completed card already conforms to the latest
bootstrap-oriented locked-line schema. The pilot found that completed `SPP`,
`SRP`, and `SOR` records can fail the current structure validator because they
truthfully contain execution or review-result prose where the bootstrap schema
expects pre-run template text. See
[`PUBLIC_PROMPT_PACKET_PILOT_VALIDATION_3474.md`](PUBLIC_PROMPT_PACKET_PILOT_VALIDATION_3474.md).

## Known Limitations

- This is a pilot, not proof that every historical `.adl` card is ready for
  publication.
- The exporter is a refuse-not-rewrite path; historical cards that fail
  hygiene must be repaired or deliberately excluded before export.
- Runtime behavior is not revalidated by these packets.
- The packet status reflects issue/PR truth at export time; closeout drift in
  older local cards remains possible and should be caught by later validation
  gates.
- Current prompt-template structure validation is not yet sufficient as a
  universal public-packet gate for completed cards; `#3475` should distinguish
  bootstrap-template preservation from completed-record truth validation.
- Demo and multi-agent packets remain future candidates; this first pilot uses
  a review-provider issue for the third representative lane because the demo
  and multi-agent v0.91.5 work packages are not closed yet.

## Follow-On Work

- `#3475` should add reusable validation and redaction gates for public prompt
  packets, including completed-card-aware validation semantics.
- Future packets should be generated during issue finish/closeout rather than
  recovered manually from local `.adl` task bundles.
- Future reviewer indexes should include machine-readable summary output once
  the validation gate format stabilizes.
