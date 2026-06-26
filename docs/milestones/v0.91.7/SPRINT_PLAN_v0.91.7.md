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
| 2 | v0.91.6 C-SDLC integration control-plane truth gate | WP-02, WP-03, WP-04 | Consume v0.91.6 `#4388`-`#4398` plus late `#4405`, `#4412`-`#4413`, `#4417`-`#4421` plus `#4425`, `#4431`, `#4441`, `#4433`-`#4438`, and `#4442`-`#4443`: VPP defaults, externalized PVF lanes, SEP automation, session ledger, forward metric capture, bounded v0.91.6 metric backfill, GitHub/octocrab convergence, prompt-card/template edge repair, runtime dependency routing, logging/reliability rough edges, watcher/lifecycle automation, operational adoption, lifecycle shepherding, and FastContext evaluation. Only create v0.91.7 follow-ons for incomplete or explicitly blocked surfaces. | planned |
| 3 | Scheduler/provider/local-agent sprint | WP-05 | Can run alongside build-throughput work after WP-03 boundaries are stable. | planned |
| 4 | Build throughput and validation-cost sprint | WP-06 | Can run in parallel with scheduler/provider work; isolate CI/workflow changes carefully. | planned |
| 5 | Runtime fire-up / Soak #2 sprint | WP-07, WP-08 | Starts after enough scheduler/build/runtime substrate is ready; AWS/SSM/SNS work can parallelize with local soak proof. Execution packet: `RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md`. | planned |
| 6 | Observatory and birthday-visible demo sprint | WP-09 | Can overlap late runtime work if data contracts are stable. | planned |
| 7 | Conceptual bridge sprint | WP-10, WP-11, WP-13 | Curiosity, Constructability, reasoning graph, affect/Godel/economics/guilds can be split across agents with shared non-claim review. | planned |
| 8 | Security and protocol residual sprint | WP-12 | Can overlap conceptual bridge but must feed final handoff. | planned |
| 9 | Launch and v0.92 handoff sprint | WP-14 | Depends on all prior dispositions; planning/public-facing language must be reviewed carefully. | planned |
| 10 | Canonical closeout-tail sprint | WP-15 through WP-23 | Demo convergence, quality gate, docs alignment, internal review, external review, remediation/preflight, next milestone planning/review, and release ceremony should stay as separate issues; review and remediation lanes can parallelize after each gate is opened. | planned |

## Execution Policy

- Each tracked issue follows `SIP -> STP -> SPP -> SRP -> SOR`.
- Each sprint should start with a sprint-level `/goal` prompt and each child issue should keep its issue-level goal.
- Each issue should declare an expected PVF/VPP validation lane during planning; the v0.91.6 control-plane completion stream and `#4425` are the gates for making this generated/default behavior rather than chat-memory policy.
- Each issue should record estimated and actual time/token/resource cost in its SOR when template/tool support exists; `#4431` is the forward-capture gate and `#4441` is v0.91.6-only backfill.
- Sprint watchers should track issue/PR/check status so completed issues close promptly and failed lanes are routed quickly.
- Planning docs alone never prove runtime readiness.
- Each surface exits as complete, blocked, deferred, or routed.
- `v0.92` remains blocked until bridge truth is reviewed.

## Risks / Dependencies

- Dependency: v0.91.6 release-tail and open closeout issues must not remain ambiguous.
- Dependency: template/version, session-ledger, validation-manager, VPP generation, and goal/time-token changes must either land or leave explicit blockers/follow-ons before v0.92 execution depends on them.
- Risk: the milestone becomes too broad.
  - Mitigation: every source item is either implemented, explicitly routed, blocked, or deferred; no narrative-only expansion.
- Risk: runtime proof arrives too late.
  - Mitigation: start runtime Soak #2 immediately after process/build/scheduler prerequisites are stable and use `RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md` as the pre-run gate packet instead of rediscovering scope from chat.
- Risk: launch planning expands v0.92.
  - Mitigation: launch/birthday handoff states consumption limits and non-claims.

## Demo / Review Plan

Required review should inspect:

- source-capture completeness;
- v0.91.6 closeout truth;
- whether every open issue/carryover has a disposition;
- whether v0.91.6 `#4388`-`#4398`, `#4405`, `#4412`-`#4413`, `#4417`-`#4421` plus `#4425`, `#4431`, `#4441`, `#4433`-`#4438`, and `#4442`-`#4443` work is complete, blocked, deferred, or routed before sprint execution depends on it;
- whether runtime Soak #2 and Observatory proof are concrete enough for v0.92;
- whether security/protocol residuals remain activation-path work;
- whether launch/birthday docs avoid unsupported product, affect, wellbeing, or runtime claims.
- whether the closeout tail follows the canonical pattern: demo convergence, quality gate, docs alignment, internal review, external review, remediation/preflight, next milestone planning, next milestone review, and release ceremony.

## Closeout Bar

- All planned docs exist and are source-backed.
- Every source-capture row is complete, blocked, deferred, or routed.
- Review findings are fixed or routed.
- `#3780` handoff truth is explicit in `V092_HANDOFF_v0.91.7.md`.
- Closeout-tail WPs are complete, blocked, deferred, or routed in canonical order before release ceremony.
