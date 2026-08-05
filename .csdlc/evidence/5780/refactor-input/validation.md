# Validation Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_finish`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_cleanup`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate10a`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`
- `bash adl/tools/install_owner_binaries.sh --check`
- `rg` negative guards for the deleted binary, skill, mutation requests, receipt writers, reconciliation commands, and prune coupling
