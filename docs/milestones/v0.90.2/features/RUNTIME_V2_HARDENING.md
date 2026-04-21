# Runtime v2 Hardening

## Purpose

Define the end-to-end first-run and hardening target for v0.90.2.

## Required Proof

The milestone should prove that Runtime v2 can:

- inherit the v0.90.1 prototype truthfully
- boot a bounded CSM manifold
- admit identity-bearing citizens
- run one governed episode
- schedule under resource pressure
- route action through the Freedom Gate
- snapshot, rehydrate, and wake locally
- emit Observatory-visible packet/report evidence
- detect invariant gaps
- reject illegal state transitions
- emit stable violation artifacts
- distinguish safe recovery from quarantine
- preserve evidence when execution must stop
- contain one governed adversarial pressure path under explicit rules of
  engagement
- record duplicate activation, snapshot integrity, and trace/replay gap
  negative probes
- produce one integrated first-run proof packet that ties WP-05 through WP-13
  evidence together
- expose reviewer/operator inspection surfaces
- prove one bounded security boundary through normal kernel/policy flow

## WP-14 D10 Integrated Demo

WP-14 adds the code-backed D10 integrated proof packet and runnable demo command:

```sh
adl runtime-v2 integrated-csm-run-demo --out artifacts/v0902/demo-d10-integrated-csm-run
```

Primary proof artifact:

- `runtime_v2/csm_run/integrated_first_run_transcript.jsonl`
- `runtime_v2/csm_run/integrated_first_run_proof_packet.json`

The command writes the bounded first-run evidence bundle, including the run
packet, trace, Observatory packet/report, recovery decisions, quarantine
evidence, governed hardening artifacts, and a ten-stage execution transcript.
It also prints the stage summary and Observatory report to stdout. The proof remains bounded: it
connects evidence already produced by v0.90.2 WPs and does not claim unbounded
live execution or first true Gödel-agent birth.

## Non-Goals

- first true Gödel-agent birthday
- full moral/emotional civilization
- cross-polis migration or v0.92 identity rebinding
- complete red/blue/purple security ecology
