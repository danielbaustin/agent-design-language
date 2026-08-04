# Runtime v3 local TLS validation

Product correction: `d38b73bcf` (including the earlier local TLS repair commits).

## Focused Rust proof

- `adl-runtime/tests/local_tls.rs`: 13 passed.
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --target-dir /Volumes/FastWork/adl-wp-5713/target --bin adl-runtime-lifecycle-soak init_fixture_uses_stable_local_tls_bootstrap`: 1 passed.
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --target-dir /Volumes/FastWork/adl-wp-5713/target --all-targets -- -D warnings`: passed.
- `cargo fmt --manifest-path adl-runtime/Cargo.toml -- --check`: passed.
- `git diff --check`: passed.

All Cargo output used `/Volumes/FastWork/adl-wp-5713/target`.

Focused regressions prove:

- lifecycle soak prepares TLS through the shared stable local bootstrap path;
- certificate, public certificate, and private key are committed as one generation directory selected by one current-generation manifest;
- failed replacement preserves the last valid current-generation manifest;
- stale persistent lock files do not block reacquisition because the real writer exclusion is an OS advisory file lock;
- configured DNS/IP SAN drift fails closed on reuse;
- explicit replacement after SAN drift installs a matching reusable identity;
- the generated certificate is accepted by rustls for localhost and the private key remains restrictive.
- on native Windows, the protected current-user-only DACL is applied to the empty key file before private-key bytes are written.

## Native macOS proof

The repo-native `adl-runtime-local-tls-bootstrap` binary generated material under the issue worktree. A second invocation returned `local_certificate_reused` with the same SHA-256 identity. The public certificate and chain copies had identical SHA-256 digests; the private key mode was `0600`.

The platform-native command below completed successfully without changing the trust store:

```text
security verify-cert -N -L -p ssl -n localhost -c <public-certificate> -r <public-certificate> -t -v
```

The native inspection confirmed:

- ECDSA P-256 with SHA-256;
- server-auth extended key usage;
- DNS SAN `localhost`;
- IP SAN `127.0.0.1`;
- a 397-day browser-compatible validity window;
- successful SSL certificate verification when the generated self-signed certificate is supplied as the explicit trust root.

No certificate bytes, private-key bytes, or credential values are retained in this evidence.

## Remaining platform proof

Native Windows execution on `nessus.local` is not claimed in this local macOS artifact because the host did not resolve during the pre-PR run. The same Rust implementation and configuration schema include a native-Windows protected-DACL test. Native Windows execution remains an explicit pre-merge integration residual. Native Linux branch CI run `30683633058` is the authoritative integration lane for exact corrected head `8dd06e57e`; its result is not credited here while the run remains active.
