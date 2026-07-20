# WP-07 Runtime Hardening Repair (#5408)

## Authority Boundary

The `csm governed-stop` command now fails closed unless all conditions hold:

- The agent has a pre-established locked spec.
- `safety.governed_stop_authority.public_key_b64` in that locked spec is a
  valid Ed25519 public key.
- The authorization is an Ed25519 signature over the agent/request tuple,
  including operator, intent, timestamp, and reason.
- The operator identity is listed in the locked spec policy and matches the
  authenticated Unix account resolved from the process effective UID via
  `geteuid()` and `getpwuid()` (with the platform-specific account resolver on
  non-Unix hosts).
- The signed request is fresh and its authorization reference is consumed once
  in the state-root ledger, preventing replay.
- The supplied operator identity matches that authenticated account identity.

The raw authorization value is never retained. The stop artifact records only
its SHA-256 reference and records that signature, authority, operator, and OS
identity verification passed. Missing policy, a forged signature, or an
unlisted caller is rejected before checkpoint or stop artifacts are written.

## API Gateway Proof Boundary

The bridge proof enumerates the required runtime routes and probes every route
that is actually provisioned. `$default` is not accepted as a substitute for a
named route. Missing routes are retained explicitly, and missing-token plus
malformed-token denial are exercised. Upstream failure, degraded runtime, and
throttling require injected live fixtures and are retained as deferred failure
matrix cases until those fixtures exist.

Because the full failure matrix is not live-proven by this repair, the summary
status is `bounded_smoke`; the retained proof validators use that same truthful
status rather than claiming a full live pass.

Service-manager shutdown uses the internal lifecycle stop record; it does not
forge an operator emergency-stop authorization. Operator emergency stop uses
the signed policy path above.

## Validation

- `cargo test --manifest-path adl/Cargo.toml api_gateway_bridge --lib`
- `python3 adl/tools/validate_runtime_hardening_5408.py`
- Focused governed-stop coverage includes a valid spec-bound signature and a
  forged-signature negative.

The authority and allowlist variables are supervisor-owned process configuration,
not caller-supplied request fields. The final CSM coherence gate remains owned by the broad milestone review lock;
this issue records the executable hardening/proof repair without claiming that
the locked release gate has been closed.
