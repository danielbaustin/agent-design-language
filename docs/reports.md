# Reports Directory Guide

This document explains generated reports under `.adl/reports/`. The directory is created by tooling (it is not committed), so it may not exist until you run a report command.

## Key Subdirectories

- `.adl/reports/burst/`
- `.adl/reports/pr-cycle/`
- `.adl/reports/automation/`

## Usage Notes

- Treat `.adl/reports/` as operational output.
- Generate or refresh the index with `swarm/tools/update_reports_index.sh` after creating new reports.
- Link to report paths in issues/PRs rather than duplicating report text.
- Keep report format aligned with `swarm/tools/REPORT_SCHEMA.md`.
