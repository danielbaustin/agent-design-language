# WP-07 Runtime Hardening Repair (#5408)

## Authority Boundary

The `csm governed-stop` command now fails closed unless both conditions hold:

- `ADL_CSM_GOVERNED_STOP_AUTHORITY` is configured and exactly matches the
  supplied authorization value.
- The supplied operator identity is listed in the comma-separated
  `ADL_CSM_GOVERNED_STOP_OPERATORS` allowlist.
- The supplied operator identity matches the authenticated process OS identity
  from `USER` or `USERNAME`.

The raw authorization value is never retained. The stop artifact records only
its SHA-256 reference and records that authorization and operator verification
passed. Missing configuration, a forged authorization, or an unlisted caller
is rejected before checkpoint or stop artifacts are written.

## API Gateway Proof Boundary

The bridge proof enumerates the required runtime routes and probes every route
that is actually provisioned. `$default` is not accepted as a substitute for a
named route. Missing routes are retained explicitly, and missing-token plus
malformed-token denial are exercised. Upstream failure, degraded runtime, and
throttling require injected live fixtures and are retained as deferred failure
matrix cases until those fixtures exist.

Because the full failure matrix is not live-proven by this repair, the summary
status is `bounded_smoke`, not `passed`.

## Validation

- `cargo test --manifest-path adl/Cargo.toml api_gateway_bridge --lib`
- `python3 adl/tools/validate_runtime_hardening_5408.py`
- Focused governed-stop CLI coverage must provide the two authority environment
  variables and includes forged-authority and missing-operator negatives.

The authority and allowlist variables are supervisor-owned process configuration,
not caller-supplied request fields. The final CSM coherence gate remains owned by the broad milestone review lock;
this issue records the executable hardening/proof repair without claiming that
the locked release gate has been closed.
