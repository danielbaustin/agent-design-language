# Runtime v3 Horust Guardian

Horust `0.1.13` is the portable Unix external-guardian candidate for Runtime
v3. It is not linked into `adl-runtime-kernel`, and it is not approved for
production use while the qualification blocker below remains open.

The pinned source record is
`infra/horust/horust-0.1.13.provenance.json`. It records the crates.io source,
MIT license, Rust baseline, and SHA-256 of the published crate archive. Verify
the archive checksum before installing with Cargo:

```sh
infra/horust/verify-provenance.sh /path/to/horust-0.1.13.crate
```

Install the pinned guardian outside the repository build:

```sh
cargo install --locked horust --version 0.1.13
```

Set the native runtime binary, init file, continuity path, and public control identity,
then run the service definition:

```sh
export ADL_RUNTIME_BIN=/usr/local/bin/adl-runtime-kernel
export ADL_RUNTIME_INIT="$HOME/.adl/runtime-v3/runtime-init.toml"
export ADL_RUNTIME_CAPSULE="$HOME/.adl/runtime-v3/continuity.json"
export ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX=<ed25519-public-key-hex>
export ADL_RUNTIME_CONTROL_KEY_ID=operator
export ADL_RUNTIME_CONTROL_PRINCIPAL=operator
horust --services-path infra/horust/adl-runtime-kernel.toml
```

The service currently uses Horust 0.1.13's `on-failure` strategy with 100ms
incremental backoff. Native qualification found that this release does not
enforce `restart.attempts` for repeated post-start crashes. The blocker is
reported upstream as `FedericoPonzi/Horust#318`; this package must not claim a
bounded crash-loop budget until a corrected upstream release is pinned and
proved. Successful exits finish, configuration exit `78` is terminal, output
is forwarded directly, and shutdown uses `SIGTERM` with a ten-second grace
period. Platform-native managers remain optional adapters around the same child
contract.

Run Horust under a dedicated unprivileged host account. The service inherits
the launcher's OS identity and resource limits; the optional systemd adapter
adds Linux cgroup and service-account bounds. Use administrator-controlled,
absolute binary, init, and continuity paths without whitespace or quoting characters,
because Horust parses the interpolated command string. Production adoption
remains blocked by upstream issue 318. The qualification evidence and remaining
provenance gap are recorded in
`docs/architecture/runtime_v3_horust_qualification_evidence.v1.json`.

`adl-runtime-kernel-bakeoff.toml` is test-only. It injects one classified fatal
exit and proves that Horust restarts a fresh child which restores continuity.
