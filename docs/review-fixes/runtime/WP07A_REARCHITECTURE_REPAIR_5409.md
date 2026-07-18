# WP-07A Runtime Rearchitecture Repair (#5409, #5494)

## Boundary

PR #5420 established useful component, supervision-policy, channel, and
credential-renewal contracts, but its retained proof overstated completion.
The production daemon supervises one real `long_lived_agent_tick` cycle; it
does not spawn sixteen independent component tasks. This repair makes that
execution model explicit and removes the static all-ready projection.

The fifteen-component CSM catalog remains the policy and observation contract.
The live runtime API normalizes observed component health and fails readiness
closed when a required component or policy-required typed channel is missing
or unhealthy. Cloud bridge and observability components remain explicitly
degradable under their existing supervision policies, while their Audit and
Evidence channels remain required by channel policy.

The Runtime v3 host-weather service remains separately implemented in
`adl-runtime/src/weather.rs`; this repair does not duplicate it or add another
weather service.

## Implemented Proof Surface

- `adl-runtime/src/topology.rs`
  - reports the real daemon-supervised-cycle execution model;
  - no longer claims static component readiness or independent component tasks;
  - excludes Runtime v3-owned weather from the CSM component assembly;
  - runs 100 supervised task cycles through the real Runtime v3 typed-channel
    fabric, injects one task failure, and verifies restart plus retained
    lifecycle-journal sequence and readiness replay.
- `adl/src/long_lived_agent/tests.rs`
  - invokes the unmodified production daemon entrypoint for three real bounded
    ticks;
  - injects one workflow failure between ticks and proves a clean recovery.
- `adl/src/csm_runtime_api.rs`
  - projects observed health for all fifteen CSM catalog components;
  - derives required-component readiness from supervision policy;
  - checks each required typed-channel observation rather than trusting only a
    top-level status string;
  - binds the API listener before persisting credential readiness, so the
    readiness artifact cannot advertise a port the runtime does not yet own.
- `adl-runtime/src/runtime_api_auth.rs`
  - retains one previous bearer generation for a bounded five-minute overlap;
  - applies that overlap to gateway signatures from the same authenticated
    credential generation without accepting mixed-generation headers;
  - automatically recovers an expired non-revoked generation without overlap;
  - serializes creation, rotation, renewal, and revocation with the existing
    `fs2` lock so terminal revocation cannot be overwritten concurrently;
  - holds final authorization decisions under a shared credential lock so
    terminal revocation cannot commit between the revocation check and an
    authenticated result;
  - rejects the previous generation after overlap expiry;
  - clears both generations on terminal revocation;
  - retains redacted rotation and revocation events without credential material.

## Validation

```text
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/wp-5494-runtime \
  cargo test --manifest-path adl-runtime/Cargo.toml
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/wp-5494-adl \
  cargo test --manifest-path adl/Cargo.toml csm_runtime_api
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/wp-5494-adl \
  cargo test --manifest-path adl/Cargo.toml \
  --lib production_daemon_executes_real_ticks_and_recovers_after_child_failure
git diff --check
```

Observed locally:

- `adl-runtime`: 126 unit tests and 1 independence test passed, including 12
  focused credential-lifecycle tests.
- integrated CSM runtime API: 44 focused tests passed.
- Runtime v3 behavioral soak: 100 completed supervised task cycles and 101
  attempts through all seven real typed channels, including one injected
  failure, restart, recovery, and valid retained lifecycle-journal sequence and
  readiness replay; focused test execution completed in 1.20 seconds.
- production daemon integration: three completed real ticks with one injected
  workflow failure and clean recovery; focused test execution completed in
  16.77 seconds.
- GitHub run `29647927552`: required Rust, formatting/lint, demo, tooling,
  path-policy, hosted coverage, and aggregate gates passed; hosted coverage
  completed in 13m18s and the full Rust lane completed in 15m55s.

This local proof does not claim a live external provider, cloud, API Gateway,
GPU, or Runtime v3 integration run. It does not override the separately
governed #4906 coherence gate.

All twelve exact-revision review findings have implementation and validation
evidence. The final exact-head review was clean. PR #5504 merged as
`51e2a5494270ad640d074eba06ba96a3e719527c`, corrective issue #5494 reached
typed `closed_out` state, and #5409 closed after that terminal evidence was
retained.
