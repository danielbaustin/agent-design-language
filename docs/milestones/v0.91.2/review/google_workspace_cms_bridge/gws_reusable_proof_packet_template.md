# GWS Reusable Proof Packet Template

## Purpose

Provide the minimum evidence bundle a future CodeFriend/ADL project should
capture when using the GWS CMS bridge.

## Required Artifacts

- bounded scope declaration
- auth/safety result
- live capability result
- content-card roundtrip result
- GitHub issue/PR route for any canonical tracked change

The content-card roundtrip artifact may be:

- a dry-run proving packet
- an execute-mode proving packet
- a truthful skipped packet with an explicit reason

It is required as a surface, not as a guarantee of live mutation success.

## Recommended Artifact Set

- `gws_live_safety_package_report.json`
- `gws_live_safety_runbook.md`
- `gws_live_capability_execution_report.json`
- `gws_live_capability_execution_snapshot.json`
- `gws_live_content_card_roundtrip_report.json`
- one short project-local summary explaining:
  - dry-run vs execute posture
  - current bounded scope
  - skipped reasons if any
  - GitHub issue/PR route for canonical promotion

## Minimum Questions The Packet Must Answer

- What exact folder/doc/sheet scope was used?
- Was the run dry-run or execute?
- If skipped, why?
- If execute was attempted, what prevented or allowed mutation?
- What still required GitHub issue/PR control?

## Non-Claim Rule

The packet must not imply:

- canonical repo truth moved to Workspace
- broad Workspace authority exists
- live writes are always available
- GitHub review can be bypassed
