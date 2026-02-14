# ADL Report Schema v1

This schema applies to report files under:

- `.adl/reports/automation/*/*/*.md`
- `.adl/reports/pr-cycle/*/*/report.md`

## Required Header

Each report starts with:

1. `# <Report Title>`
2. `Schema: adl-report/v1`

## Required Sections

- `## Inputs`
- `## Actions`
- `## Results`
- `## Next Action`

## Determinism Rules

- Include exactly one `## Next Action` command.
- Keep timestamps in sortable formats when possible.
- Do not include non-deterministic placeholder text.
