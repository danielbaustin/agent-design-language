# Runtime v3 Horust Guardian

Horust `0.1.13` is the selected portable Unix external guardian for
Runtime v3. It is not linked into `adl-runtime-kernel`.

Install the pinned guardian outside the repository build:

```sh
cargo install --locked horust --version 0.1.13
```

Set the native runtime binary, continuity path, and public control identity,
then run the service definition:

```sh
export ADL_RUNTIME_BIN=/usr/local/bin/adl-runtime-kernel
export ADL_RUNTIME_CAPSULE="$HOME/.adl/runtime-v3/continuity.json"
export ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX=<ed25519-public-key-hex>
export ADL_RUNTIME_CONTROL_KEY_ID=operator
export ADL_RUNTIME_CONTROL_PRINCIPAL=operator
horust --services-path infra/horust/adl-runtime-kernel.toml
```

The service uses `on-failure` restart, 100ms incremental backoff, three startup
attempts, direct stdout/stderr forwarding, and `SIGTERM` with a ten-second
grace period. Configuration exit `78` is terminal and is not restarted.
Platform-native managers remain optional adapters around the same child contract.

Run Horust under a dedicated unprivileged host account. The service inherits
the launcher's OS identity and resource limits; the optional systemd adapter
adds Linux cgroup and service-account bounds. Use administrator-controlled,
absolute binary and continuity paths without whitespace or quoting characters,
because Horust parses the interpolated command string. Production packaging and
cross-host qualification continue in `#5211`.

`adl-runtime-kernel-bakeoff.toml` is test-only. It injects one classified fatal
exit and proves that Horust restarts a fresh child which restores continuity.
