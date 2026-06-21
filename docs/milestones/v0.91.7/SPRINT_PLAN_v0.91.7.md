# v0.91.7 Sprint Plan

## Metadata

- Sprint family: `v0.91.7-final-pre-v0.92-readiness`
- Milestone: `v0.91.7`
- Start date: not scheduled
- End date: not scheduled
- Owner: ADL maintainers
- Status: planned
- Source capture: `PLANNING_SOURCE_CAPTURE_v0.91.7.md`

## Status

Planned. This document defines execution order; it does not start runtime work.

## Sprint Overview

Complete the final bridge/readiness tranche before `v0.92` activation refresh. The milestone should move from source capture to operational readiness, then to conceptual/security/protocol bridge closure, then to launch/birthday handoff.

## Sprint Goals

- Promote a complete v0.91.7 planning package from the refreshed source capture.
- Finish or route v0.91.6 closeout truth and ADR release-tail decisions before v0.92 depends on them.
- Make sprint execution predictable through SEP, VPP, PVF, templates, goals, watchers, and closeout discipline.
- Reduce validation/build/cognitive-resource bottlenecks enough for v0.92 to run quickly.
- Fire up and soak the runtime integration path before birthday activation.
- Keep demo, Observatory, security, protocol, and launch surfaces visible.
- Preserve strong non-claim boundaries for affect, Godel mechanics, economics, guilds, and public birthday evidence.

## Recommended Sprint Order

| Order | Sprint / workstream | Primary WPs | Parallelism notes | Status |
| --- | --- | --- | --- | --- |
| 1 | Planning promotion, closeout-truth, and ADR release-tail gate | WP-01, WP-02 | Must start first; can run issue-list/source-capture checks and ADR route checks in parallel. | planned |
| 2 | SEP/VPP/PVF/template process sprint | WP-03, WP-04 | Can run template/schema work, sprint skills, closeout skills, and goal/metrics docs in parallel if branch boundaries are clear. | planned |
| 3 | Scheduler/provider/local-agent sprint | WP-05 | Can run alongside build-throughput work after WP-03 boundaries are stable. | planned |
| 4 | Build throughput and validation-cost sprint | WP-06 | Can run in parallel with scheduler/provider work; isolate CI/workflow changes carefully. | planned |
| 5 | Runtime fire-up / Soak #2 sprint | WP-07, WP-08 | Starts after enough scheduler/build/runtime substrate is ready; AWS/SSM/SNS work can parallelize with local soak proof. | planned |
| 6 | Observatory and birthday-visible demo sprint | WP-09 | Can overlap late runtime work if data contracts are stable. | planned |
| 7 | Conceptual bridge sprint | WP-10, WP-11, WP-13 | Curiosity, Constructability, reasoning graph, affect/Godel/economics/guilds can be split across agents with shared non-claim review. | planned |
| 8 | Security and protocol residual sprint | WP-12 | Can overlap conceptual bridge but must feed final handoff. | planned |
| 9 | Launch and v0.92 handoff sprint | WP-14 | Depends on all prior dispositions; planning/public-facing language must be reviewed carefully. | planned |
| 10 | Internal review and remediation sprint | WP-15, WP-16 | Review lanes should run in parallel; remediation issues should be grouped by owner/surface. | planned |

## Execution Policy

- Each tracked issue follows `SIP -> STP -> SPP -> SRP -> SOR`.
- Each sprint should start with a sprint-level `/goal` prompt and each child issue should keep its issue-level goal.
- Each issue should declare an expected PVF/VPP validation lane during planning.
- Each issue should record estimated and actual time/token/resource cost in its SOR when the template support exists.
- Sprint watchers should track issue/PR/check status so completed issues close promptly and failed lanes are routed quickly.
- Planning docs alone never prove runtime readiness.
- Each surface exits as complete, blocked, deferred, or routed.
- `v0.92` remains blocked until bridge truth is reviewed.

## Risks / Dependencies

- Dependency: v0.91.6 release-tail and open closeout issues must not remain ambiguous.
- Dependency: template/version changes may affect VPP/goal/time-token fields.
- Risk: the milestone becomes too broad.
  - Mitigation: every source item is either implemented, explicitly routed, blocked, or deferred; no narrative-only expansion.
- Risk: runtime proof arrives too late.
  - Mitigation: start runtime Soak #2 immediately after process/build/scheduler prerequisites are stable.
- Risk: launch planning expands v0.92.
  - Mitigation: launch/birthday handoff states consumption limits and non-claims.

## Demo / Review Plan

Required review should inspect:

- source-capture completeness;
- v0.91.6 closeout truth;
- whether every open issue/carryover has a disposition;
- whether VPP/PVF/SEP work is scheduled before sprint execution depends on it;
- whether runtime Soak #2 and Observatory proof are concrete enough for v0.92;
- whether security/protocol residuals remain activation-path work;
- whether launch/birthday docs avoid unsupported product, affect, wellbeing, or runtime claims.

## Closeout Bar

- All planned docs exist and are source-backed.
- Every source-capture row is complete, blocked, deferred, or routed.
- Review findings are fixed or routed.
- `#3780` handoff truth is explicit in `V092_HANDOFF_v0.91.7.md`.
