# adl-records

`adl-records` defines portable, bounded ADL v2 record contracts and a
deterministic Ed25519 envelope. Verification is fail-closed and requires both
an external trust policy and an external replay guard.

The crate performs no filesystem, network, clock, environment, key-store, or
runtime operations. Callers supply keys, logical time, trust, and replay state.

ADL v2 crates use the focused standalone CI lane for tests, formatting, and Clippy.
