# v0.91.4 Active Issue Migration Audit

## Status

Tracked WP-11 sampled migration audit.

## Scope

This packet demonstrates how the active-issue migration policy classifies a
small representative set of open issues without silently rewriting all active
work.

It does not claim:

- that every currently open issue has already been migrated
- that fold/no-op close decisions may happen without explicit evidence
- that in-flight PRs should be rewritten mid-stream just for policy purity

## Method

Each sampled issue was classified from public issue truth and current workflow
shape:

- milestone/version labels
- issue type and area labels
- whether the issue is already in an active sprint lane
- whether the issue already has an open PR
- whether migration would preserve or rewrite current truth

## Sampled Classifications

| Issue | Current shape | Classification | Evidence | Migration / routing note |
| --- | --- | --- | --- | --- |
| `#3426` `[v0.91.4][docs] Reconcile WP-08 handoff lifecycle truth` | Open `v0.91.4` docs task with no open PR surfaced by the current repository PR search | `migrate_now` | Issue is open, current-milestone, docs-bounded, and not already presenting visible in-flight PR truth in the current repository | Safe to execute directly on the current C-SDLC path at next handling |
| `#3359` `[v0.91.4][WP-10][demo-tools] Five-minute-sprint repeatability and Parallel Validation Fabric` | Open Sprint 3 child with draft PR `#3441` and bound worktree execution already recorded | `leave_unchanged` | The issue is already on the current C-SDLC lane and forced migration would only rewrite in-flight truth | Continue normal review/merge/closeout instead of remigrating |
| `#3361` `[v0.91.4][WP-12][tests-tools] Regression fixtures for process drift` | Open Sprint 3 child with five-card bundle and planned execution after WP-11 | `leave_unchanged` | The issue is already seeded on the target contract and only needs normal execution | No migration action beyond the standard lifecycle |
| `#3434` `[v0.91.4][docs] Prepare v0.92 milestone docs package` | Open planning/docs task aimed at the next milestone package | `defer` | The issue points at `v0.92` packaging work rather than the active Sprint 3 cutover lane | Re-evaluate on the next-milestone planning boundary |
| `#3419` `[v0.91.4][multi-agent][demo] Run bounded multi-agent C-SDLC proof sprint` | Open high-complexity demo issue with open prerequisite sibling work (`#3416`-`#3418`) | `block` | Migration/execution should not proceed automatically while prerequisite work and execution readiness remain unresolved | Keep blocked until explicit operator judgment and prerequisite truth say the lane is safe |

## Unexercised Categories In This Sample

- `fold_or_noop_close`
  - No sampled open issue truthfully qualified for fold/no-op close during this
    bounded audit. When that category does apply, route it through
    `issue-folding` with explicit evidence instead of treating it as migration.

## Policy Conclusions

1. Not every open issue should be migrated immediately.
   - Current, low-risk, not-yet-in-flight issues can migrate now.
   - Future-milestone planning work should usually defer.

2. In-flight truth wins over cosmetic purity.
   - An issue already running on the intended lane should be left alone.
   - An issue with an active PR should not be rewritten just to make the cards
     look newer.

3. Future defaults matter more than mass backfill.
   - The safest steady state is new ADL software-development issues starting on
     the five-card C-SDLC contract from day one.

4. Block and fold/no-op remain explicit governance decisions.
   - They should never happen as silent cleanup.
