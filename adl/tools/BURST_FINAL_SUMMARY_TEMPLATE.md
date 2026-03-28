# Burst Final Summary Template

Use this structure for `.adl/reports/burst/<timestamp_utc_z>/final_summary.md`.

## Run Metadata

- Parent issue:
- Burst timestamp (UTC ISO 8601 / RFC3339 with trailing `Z`):
- Operator:
- Mode: sequential

## Created Issues

- `#<issue>` `<title>`

## Execution Order And Results

1. `#<issue>` - `SUCCESS|FAILED|SKIPPED` - PR: `<url|n/a>` - notes

## Halt Reason

- `none` (all complete) or explicit first-failure/policy/human-decision reason.

## Suggested Tags

- `release-note:*`
- `area:*`
- `risk:*`

## Next Action

- Exactly one command.
