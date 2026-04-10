# Closed v0.87.1 Issue Record Hygiene Pass (#1546)

## Summary

This pass scanned closed `version:v0.87.1` GitHub issues and the tracked `docs/records/v0.87.1/tasks/` SOR surface for stale post-merge lifecycle claims.

## Scope

- Closed `version:v0.87.1` GitHub issues scanned: 53
- Existing tracked v0.87.1 SORs inspected: 27
- Additional closed v0.87.1 SOR without the version label inspected: `issue-1525`
- Tracked SORs corrected in this pass: 27

## Corrections Applied

The following tracked SORs had stale `worktree_only` or `pr_open` integration claims normalized to `merged` because the associated issue branch has merged to main:

- `issue-1436` via PR `#1456`
- `issue-1437` via PR `#1466`
- `issue-1438` via PR `#1471`
- `issue-1439` via PR `#1475`
- `issue-1440` via PR `#1484`
- `issue-1441` via PR `#1492`
- `issue-1442` via PR `#1506`
- `issue-1449` via PR `#1452`
- `issue-1455` via PR `#1465`
- `issue-1462` via PR `#1517`
- `issue-1468` via PR `#1489`
- `issue-1469` via PR `#1472`
- `issue-1473` via PR `#1539`
- `issue-1474` via PR `#1476`
- `issue-1477` via PR `#1481`
- `issue-1480` via PR `#1482`
- `issue-1500` via PR `#1522`
- `issue-1501` via PR `#1523`
- `issue-1502` via PR `#1526`
- `issue-1518` via PR `#1524`
- `issue-1519` via PR `#1536`
- `issue-1525` via PR `#1530`
- `issue-1527` via PR `#1535`
- `issue-1528` via PR `#1531`
- `issue-1529` via PR `#1532`
- `issue-1533` via PR `#1534`
- `issue-1541` via PR `#1543`

For each corrected record, this pass preserved the original validation commands and execution evidence, while updating only the final integration lifecycle truth.

## Missing Or Deferred Records

The following closed `version:v0.87.1` issues do not currently have tracked SORs under `docs/records/v0.87.1/tasks/`:

- `issue-1354`
- `issue-1419`
- `issue-1430`
- `issue-1431`
- `issue-1432`
- `issue-1435`
- `issue-1443`
- `issue-1444`
- `issue-1445`
- `issue-1450`
- `issue-1467`
- `issue-1478`
- `issue-1479`
- `issue-1485`
- `issue-1486`
- `issue-1487`
- `issue-1488`
- `issue-1490`
- `issue-1491`
- `issue-1493`
- `issue-1504`
- `issue-1507`
- `issue-1508`
- `issue-1509`
- `issue-1520`
- `issue-1521`
- `issue-1544`

This pass did not invent SORs for those issues. They should be backfilled only if a later pass can recover sufficient source evidence from the merged PR, local bundle, or issue discussion.

## Notes

- Local ignored `.adl` task/card surfaces are not treated as required tracked artifacts.
- Live-provider and runtime archive outputs remain operator-local unless a specific issue explicitly promotes them.
- This report is a hygiene index, not a replacement for the individual SORs.
