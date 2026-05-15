# Workspace Revision Mismatch And Authority Rules

## Revision Mismatch Rule

When the Workspace revision anchor no longer matches the reviewed promotion
packet input, the document must move to `blocked` or `diverged`. The bridge
must not silently refresh the target repo doc or silently overwrite the
promotion packet.

## Canonical Authority Rule

- Workspace is collaborative draft infrastructure.
- GitHub issues, PRs, and merged repository history remain canonical public
  truth.
- Post-start public planning-doc edits require an issue-backed PR.

## Live-Gated Capability Boundary

The first live adapter is expected to be `gws`, but only behind explicit
operator-visible capability boundaries:

- `gws.drive.folder_inventory`
- `gws.docs.read_snapshot`
- `gws.sheets.read_content_cards`
- `gws.sheets.write_content_cards`
- `gws.docs.prepare_promotion_packet`
- `gws.drive.record_revision_anchor`

Each capability remains bounded by:

- one explicit folder, doc, or sheet scope
- read/write posture
- sensitivity/redaction expectations
- dry-run or preview support when available
- skipped classification when live auth or tool availability is missing

## Non-Claims

- No automatic bidirectional sync is authorized.
- No Workspace action can override Git-backed milestone truth by default.
- No live secret-bearing Workspace flow is required for fixture validation.
