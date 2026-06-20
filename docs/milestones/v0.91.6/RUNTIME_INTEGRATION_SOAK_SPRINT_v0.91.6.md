# Runtime Integration Soak Sprint

Issue: `#4185`
Milestone seed: `v0.91.6`
Execution window: Soak #1 in `v0.91.6`; Soak #2, and Soak #3 if needed, in
`v0.91.7` before `v0.92` runtime-coherence or activation claims.

## Purpose

The runtime should not be considered coherent merely because the individual
pieces exist. Tokio integration gives ADL an async substrate, but the runtime
still needs one explicit integration sprint that boots the pieces together and
keeps them alive long enough to expose coordination, cancellation, heartbeat,
logging, and recovery failures.

This document deliberately separates the first walking-skeleton runtime proof
from the later full feature-list integration proof. Soak #1 stays in this
milestone. Soak #2 is the v0.91.7 target where every required feature should be
working together. Soak #3 is a contingency only if Soak #2 exposes blockers
that cannot be honestly closed in one pass.

## Placement

- Soak #1 route: execute in late `v0.91.6` after Tokio substrate readiness as a
  walking-skeleton integration proof.
- Soak #2 route: execute in `v0.91.7` as the full feature-list integration
  proof where every required runtime feature works together.
- Soak #3 route: execute in `v0.91.7` only if Soak #2 finds real blockers that
  need a second integration pass.
- `v0.92` remains blocked from claiming runtime coherence until Soak #2 is
  complete, or Soak #3 is complete when needed, or unresolved surfaces are
  explicitly blocked/deferred with operator approval.

## Dependency Gate

The soak sprint may start only after the Tokio integration can:

- boot the runtime under Tokio without a one-off demo harness;
- own task spawning, cancellation, timeout, and shutdown boundaries;
- expose enough runtime status for heartbeat/watchdog observation;
- run the focused Tokio substrate validation required by its own issue.

## Soak Sequence

| Soak | Milestone | Purpose | Acceptance posture |
| --- | --- | --- | --- |
| Soak #1 | `v0.91.6` | Prove the runtime can boot under Tokio and run a small integrated walking skeleton. | Runtime, ACIP, one temporary agent, AEE path, scheduler/resilience/logging basics, Observatory visibility, and one memory handoff are present together. |
| Soak #2 | `v0.91.7` | Prove the feature-list runtime surface works together before `v0.92`. | Every required feature-list surface has a working integrated proof, negative case, or explicit blocker. This is the default full-readiness gate. |
| Soak #3 | `v0.91.7` | Confirm fixes after Soak #2 if the full-readiness gate exposes blockers. | Only needed if Soak #2 cannot honestly close. Must burn down named blockers rather than widen scope. |

## Soak #1 Minimum Integrated Surface

Soak #1 must run these surfaces together:

- Tokio runtime boot and shutdown.
- ACIP message path sufficient for governed send/receive between runtime
  participants.
- Temporary agent execution through the AEE, even if agents are intentionally
  limited.
- Scheduler choice points for at least one local, remote, delayed, and
  governor-like routing case.
- Resilience layer behavior for retry, timeout, cancellation, partial failure,
  circuit/backoff, and restart or resume handoff.
- Provider/model action logging and runtime action logging on the active
  stdout/stderr contract.
- Heartbeat, watchdog, progress, and timeout diagnostics.
- Observatory/Unity consumption of live runtime state rather than only canned
  demo data.
- Memory/ObsMem checkpoint or handoff record for long-running context.

Soak #1 does not need every feature-list surface to be feature-complete. It
must prove the pieces can run as one system and produce evidence for what still
needs Soak #2.

## Soak #2 Feature-List Integration Matrix

Soak #2 must account for every required runtime feature before `v0.92`:

| Feature-list surface | Soak #2 requirement | Proof mode |
| --- | --- | --- |
| Tokio runtime substrate | integrated boot, spawn, cancellation, timeout, and shutdown | long-running run log and shutdown trace |
| Agent lifecycle | temporary agents move through startup, active, paused/degraded where applicable, and final states | lifecycle status snapshots and negative restart/copy cases |
| AEE | agents execute through the governed AEE path, not direct test helpers | AEE execution trace and control-path record |
| ACIP/A2A | governed send/receive plus malformed, denied, and failed delivery cases | ACIP trace, loopback/session proof, denied-access fixture |
| Provider/model substrate | local/remote/provider routes classify success and failure | provider action log, model identity, failure classification |
| Scheduler | local, cheap-remote, premium, delayed, and governor-like choices are explainable | scheduler decision packet |
| Resilience | retry, timeout, cancellation, degraded mode, restart/resume, and partial failure are exercised | failure-injection register and resilience events |
| Logging/observability | stdout/stderr contract, action logs, correlation, heartbeat, progress, timeout, and redaction hold under load | log-channel proof and redaction scan |
| Observatory/Unity | live runtime state is consumed, not just canned demo data | live Observatory/Unity consumption record |
| ObsMem and memory handoff | checkpoint/handoff survives long-running context boundary without raw private leakage | memory handoff packet and redaction check |
| Identity and continuity | startup, wake, snapshot, copied state, and true continuity are distinguishable | continuity fixtures and negative cases |
| Capability envelope | provider/model/tool/skill/authority limits are visible for temporary agents and birthday prep | capability envelope fixture |
| Security/CAV boundary | unauthorized access, malformed output, and provider/message trust failures fail closed | security review packet and malformed-output fixtures |
| Curiosity/Constructability, if landed before Soak #2 | at least one governed discovery or admissibility cycle is integrated | constructability/discovery proof packet |

If a row is not working by Soak #2, the closeout must name the blocker and
either schedule Soak #3 in `v0.91.7` or record an explicit operator-approved
defer/block before `v0.92`.

## Soak #1 Target

The Soak #1 execution sprint should define three levels:

- Smoke: 10-15 minutes, proves boot, message flow, one temporary agent, and
  clean shutdown.
- Standard: at least 4 hours, proves long-running heartbeat/progress evidence,
  provider action logging, bounded failure injection, and checkpoint handoff.
- Extended: overnight or operator-approved longer run, optional unless the
  standard soak exposes time-dependent failures that need confirmation.

The standard run is the default Soak #1 acceptance target. If it cannot complete, the
closeout must classify whether the blocker is Tokio substrate, ACIP, AEE,
provider/model, resilience, Observatory, memory handoff, or tooling evidence.

## Soak #1 Failure Injection Matrix

At minimum, Soak #1 should inject and observe:

| Failure | Expected runtime behavior | Required evidence |
| --- | --- | --- |
| transient provider failure | retry/backoff or routed degradation | provider log, scheduler decision, resilience event |
| agent timeout | cancellation, timeout diagnostic, no orphaned task claim | heartbeat/progress log and final status |
| ACIP delivery failure | governed failure result, retry/defer/blocked classification | ACIP trace and runtime status snapshot |
| partial memory handoff failure | bounded error and no false continuity claim | ObsMem or memory handoff record |
| Observatory consumer disconnect | runtime continues and reports degraded observer | runtime log and Observatory reconnect/degrade state |
| shutdown during active work | orderly cancellation and durable final status | shutdown trace and closeout status |

## Soak #1 Proof Packet

Soak #1 must produce a reviewable packet containing:

- run configuration and version;
- start/end timestamps and duration;
- runtime status snapshots;
- ACIP message trace sample;
- AEE temporary-agent execution trace;
- scheduler decisions used during the run;
- resilience/failure-injection register;
- logging channel proof for machine stdout and human observability stderr;
- Observatory/Unity consumption record;
- Memory/ObsMem handoff or checkpoint record;
- closeout classification of completed, blocked, deferred, and residual
  runtime surfaces.

## Soak #1 Candidate Issue Wave

The Soak #1 execution sprint should split into small issues only if the Tokio
substrate is ready:

1. Soak umbrella and execution packet.
2. Runtime boot/shutdown harness under Tokio.
3. ACIP temporary-agent message path.
4. AEE temporary-agent execution lane.
5. Scheduler/resilience/failure-injection lane.
6. Logging/watchdog/heartbeat/progress proof.
7. Observatory/Unity live consumption proof.
8. Memory/ObsMem checkpoint or handoff proof.
9. Standard soak run and closeout review.

If the substrate is not ready, open only the umbrella/setup issue and record the
Tokio blocker. Do not create child issues that pretend runnable integration
exists.

## Soak #2 / #3 v0.91.7 Issue Wave

The `v0.91.7` issue wave should be created after Soak #1 closeout and should
start from the feature-list matrix above:

1. Soak #2 umbrella and execution packet.
2. Full feature-list integration matrix implementation and fixture setup.
3. Long-running standard Soak #2 run.
4. Soak #2 review and blocker register.
5. Soak #3 remediation run, only if Soak #2 cannot honestly close.
6. Final pre-`v0.92` runtime-coherence disposition.

## Non-Goals

- Do not build full-featured production agents.
- Do not claim Unity Observatory inhabitant readiness from canned data.
- Do not treat Soak #1 as full feature-list readiness.
- Do not treat a short smoke run as the standard soak.
- Do not move unresolved Tokio substrate work into this sprint.
- Do not claim v0.92 runtime coherence until Soak #2, or Soak #3 if needed,
  closes every required feature-list row as working, blocked, deferred, or
  operator-approved out of scope.

## Review Questions

- Did the runtime run as one system rather than as isolated component proofs?
- Did the run produce evidence that can be inspected after the fact?
- Did failures route through resilience and logging surfaces instead of
  disappearing?
- Did Observatory/Unity consume live state?
- Did Memory/ObsMem record enough continuity evidence for long-running context?
- Are blocked surfaces named with owning follow-up issues?
