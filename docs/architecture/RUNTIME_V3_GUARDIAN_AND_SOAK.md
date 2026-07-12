# Runtime v3 Guardian And Soak

Issue `#5175` closes the Runtime v3 mini-sprint with an external-guardian
bakeoff, native packaging, adversarial process proof, bounded integrated
soak, and an explicit disposition. Runtime v2 remains the default and no
artifact in this packet authorizes cutover.

![Runtime v3 guardian and soak](runtime_v3_guardian_soak.svg)

## Guardian-Neutral Child Contract

Every guardian starts the same `adl-runtime-kernel serve <continuity-path>`
process. The child owns component supervision, readiness, typed bounded
channels, continuity, and graceful shutdown. The external guardian owns
environment injection, stdout/stderr capture, signal delivery, child reaping,
restart delay, and process restart. The child accepts `SIGINT` and `SIGTERM`,
classifies fatal exits, and never embeds guardian-specific configuration.

## Bakeoff

| Guardian | Liveness/restart | Signals/reaping/output | Portability and maintenance | Security posture | Disposition |
|---|---|---|---|---|---|
| systemd | Native service and restart/backoff; cgroup ownership | `SIGTERM`, `KillMode=control-group`, journal capture | Linux only | Dynamic user, state directory, resource and capability bounds | Optional Linux adapter; Linux-host execution pending |
| rustysd | Common unit subset supports `Restart=always`; fixed pre-start delay | Process supervision and output capture | Linux proof candidate; upstream describes itself as work in progress | Smaller surface, insufficient production hardening evidence | Retained proof candidate only |
| Horust 0.1.13 | Executed `on-failure` restart with 100ms incremental backoff; configured attempts do not bound repeated post-start crashes | `SIGTERM`, output forwarding, child supervision | Built on macOS and targets Unix hosts; upstream documents macOS orphan-reaping limitations | Inherits launcher identity and limits; dedicated unprivileged account required | Selected portable Unix candidate; bounded-restart qualification blocked by upstream #318 |
| launchd | KeepAlive and native process ownership | Native signal/log integration | Supported macOS platform service | Native service isolation controls | Compatible future adapter; not packaged here |

`initd` remains out of scope because it is a PID 1 bootstrap layer rather than
the child service manager this contract requires.

The retained native Horust proof executes both sides of the guardian contract:
one injected fatal child exit is restarted and restores continuity at generation
2; terminating Horust forwards `SIGTERM`, lets the kernel checkpoint generation
1, and leaves control port `20997` closed.

Horust 0.1.13's `OnFailure` path does not apply the configured attempt limit,
and its `Never` startup-failure counter resets when a child reaches `Started`.
A native always-failing child therefore remains in a crash loop instead of
exhausting the configured budget. This is retained as a cutover blocker and
reported upstream at `https://github.com/FedericoPonzi/Horust/issues/318`.

## Packaging

- `infra/horust/adl-runtime-kernel.toml` is the selected portable Unix
  guardian package.
- `infra/systemd/adl-runtime-kernel.service` is an optional Linux adapter.
- `infra/rustysd/adl-runtime-kernel.service` remains a compatibility fixture,
  not a production recommendation.

## Soak Gate

The bounded soak repeats integrated kernel execution and continuity recovery,
then injects component restart, child crash, queue saturation, corrupt
continuity, degraded clock startup, and shutdown-deadline failure. Separate
adversarial parity tests cover process-tree leaks, timeouts, oversized output,
invalid fixture identifiers, and deliberately divergent runtime output.

The retained report records exact iteration count and outcomes. It is a bounded
engineering soak, not a claim of multi-day production endurance. A later
deployment qualification must add host-specific duration, telemetry delivery,
and operational SLO evidence.

## Current Disposition

The pre-soak Fable 5 architecture review recommended **continue incubation**
and identified five P1 findings in the parity evidence boundary. Those findings
were remediated before the bounded soak ran. Final Fable 5 delta verification
and the independent review stack found no remaining actionable findings and
marked `#5175` ready for closeout under continued incubation. Automatic cutover
is never an available outcome.

Process groups bound ordinary descendants during the parity soak, but they are
not an OS security boundary against a trusted fixture that deliberately creates
a new session. Killing a numeric process group after reaping its leader also has
a theoretical identifier-reuse race when no descendants remain. Linux cgroup
containment and production host resource limits are deployment qualification
work tracked by `#5211`.
