# Redaction Playbook

Use this playbook when interpreting the deterministic helper output or
performing a manual audit.

## Blockers

- Secret-like values: API keys, private keys, OAuth tokens, GitHub tokens, Slack
  tokens, cloud access keys, or environment assignments containing credentials.
- Private host paths in a customer-facing or public candidate report.
- Raw prompt or tool argument dumps that expose hidden operating context.
- Source excerpts that are too long for the review purpose.
- Manifest says `publication_allowed: false` and the target is a
  customer-facing or public candidate artifact.

## Warnings

- Internal URLs, localhost URLs, or private IP addresses.
- Provider/model names or infrastructure details not needed by the audience.
- Evidence references that are ambiguous or not repo-relative.
- Missing manifest or missing audience declaration.

## Safe Defaults

- Mask secret samples.
- Preserve evidence path shape without absolute host roots.
- Downgrade publication readiness when uncertain.
- Send remediation back to the owner of the source artifact instead of editing
  it in place.

## Handoff Targets

- Packet shape problems: `repo-packet-builder`.
- Review over-disclosure: the specialist review skill that generated the
  artifact.
- Report template drift: synthesis or product report writer.
- Security review gaps: security reviewer or threat-model skill.

