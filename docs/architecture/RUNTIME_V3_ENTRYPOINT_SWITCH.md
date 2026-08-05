# Runtime v3 Entrypoint Switch

## Status

As of v0.91.7, Runtime v3 is selectable through an explicit CLI compatibility
boundary. Runtime v2 remains the default runtime until the cutover proof gate
authorizes a default switch.

## Selection Report Surface

- `adl runtime-v3 select [--runtime v2|v3] [--json]`
- `adl-runtime runtime-v3 select [--runtime v2|v3] [--json]`

These compatibility entrypoints report the requested runtime and the Runtime v3
control API policy. They are not runtime launchers: they invoke neither Runtime
v2 nor Runtime v3 and do not change global defaults. Runtime v3 execution uses
the independent `adl-runtime-kernel serve` command below.

The selector reports `DEFAULT_CHANGED=false` for both Runtime v2 and Runtime v3
selection. `SELECTION_DIFFERS_FROM_DEFAULT=true` is the reversible selection
signal for explicit Runtime v3 use while Runtime v2 remains the default.

## Selection Rules

- No selector: Runtime v2 remains selected.
- `--runtime v3`: the report records an explicit Runtime v3 request.
- `--runtime v2`: the report records an explicit Runtime v2 request.
- `ADL_RUNTIME_SELECTION=v3`: Runtime v3 is selected when `--runtime` is
  omitted.
- Unknown values fail closed.

`--runtime` takes precedence over `ADL_RUNTIME_SELECTION`.

## Runtime v3 Control Policy

Runtime v3 uses the local HTTPS control API endpoint:

```text
https://localhost:20997
```

The independent Runtime v3 kernel launch command reported by this surface is:

```text
adl-runtime-kernel serve --init infra/runtime-v3/runtime-init.toml
```

The kernel terminates TLS through the maintained `axum-server` Rustls adapter.
The init file supplies the certificate-chain and private-key PEM paths; no
private key is checked into the repository. The ready event and Observatory
feed report the port actually bound by the listener rather than assuming
`20997`.

## Local Development TLS Bootstrap

Runtime v3 local development may use the repo-native Rust bootstrap command to
create one durable self-signed localhost server certificate under an explicit
absolute state root:

```text
adl-runtime-local-tls-bootstrap --config runtime-local-tls.toml
```

The bootstrap config must explicitly set `mode = "local_self_signed"` or
`mode = "managed_external"`. Missing mode fails closed. Production and
enterprise deployments should use `managed_external` and continue to provide
operator-managed PEM certificate and private-key paths; the bootstrap validates
those paths through rustls and does not mutate them.

Local self-signed mode requires an absolute `state_root`, a relative `tls_dir`,
relative certificate/key paths beneath that TLS directory, configured DNS/IP
SANs, and a separate public certificate path. The command creates the private
key with restrictive permissions on Unix, writes the public certificate as a
separate PEM file for trust import, validates the pair with rustls, and reuses
the same certificate on ordinary restarts. Replacement requires
`replace = true`; after replacement the operator must trust the new public
certificate again. Local certificates use a browser-compatible 397-day
validity period. Before expiry, the operator explicitly replaces the local
certificate and trusts the new public copy; ordinary restarts never rotate it.

The runtime does not edit the macOS, Windows, or Linux trust store. Operators
trust the generated public certificate once using the platform-native trust UI
or command approved for their machine. The private key remains under the
configured state root and must not be copied into evidence, committed, or
printed.

## Non-Covered Surfaces

This issue does not:

- make Runtime v3 the default;
- delete or decommission Runtime v2;
- migrate every Runtime v2 demo command;
- introduce a custom supervisor;
- start the Runtime v3 daemon implicitly.

Runtime v2 decommission remains gated by the aggregate Runtime v3 cutover proof
and an explicit default-switch decision.

## v0.91.7 Decision

#5254 records the final v0.91.7 default-switch decision: Runtime v2 remains the
default runtime, Runtime v3 remains explicit opt-in only, and Runtime v2
decommission is not authorized. See
`docs/architecture/RUNTIME_V3_CUTOVER_DECISION_5254.md`.
