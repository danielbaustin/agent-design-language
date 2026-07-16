# WP-07A Runtime Rearchitecture Repair (#5409)

## Boundary

This repair completes the runtime-owned assembly boundary identified by #5121.
The production topology is now a typed assembly contract rather than only a
JSON projection. Assembly validates that every declared runtime component has
one supervision policy and that the complete typed channel matrix is present
before the static assembly capability projection can report `ready`.

The `resident_agents` component is included in the supervised component set so
provider-backed resident agents cannot exist as an unsupervised topology entry.
Runtime API credentials renew inside the overlap window before expiry and retain
the existing fail-closed revoked/expired behavior.

## Implemented Proof Surface

- `adl-runtime/src/topology.rs`
  - `CsmRuntimeAssembly::production()` validates component/policy parity and
    typed channel coverage.
- `RuntimeReadiness` reports all supervised components and channels in the
  static assembly capability projection; live component health remains owned
  by the supervision outcomes and runtime API health routes.
  - bounded 100-cycle assembled readiness soak verifies deterministic stability.
- `adl-runtime/src/supervision.rs`
  - adds `resident_agents` to `ComponentId::ALL` and gives it an explicit
    provider-admission supervision policy.
- `adl-runtime/src/runtime_api_auth.rs`
  - renews credentials inside the 15-minute overlap window and tests rotation.

## Validation

```text
cargo test --manifest-path adl-runtime/Cargo.toml
CARGO_TARGET_DIR=/Volumes/FastWork/adl-wp-5409-target cargo check --manifest-path adl/Cargo.toml
git diff --check
```

Observed locally:

- `adl-runtime`: 119 unit tests and 1 independence test passed.
- `adl` compile check passed.
- assembled readiness soak: 100 deterministic production-assembly cycles.

This is a local assembled-runtime contract/soak proof. It does not claim a
live external provider, cloud, or API Gateway environment, and it does not
override the separately governed #4906 coherence gate.
