# CLI Review Compatibility Boundary For #3599

Issue `#3599` introduces `adl-review` as the review-tooling compatibility
binary for the v0.91.5 CLI decomposition mini-sprint.

## Ownership

`adl-review` owns review-facing tooling command entrypoints:

- `adl-review code-review`
- `adl-review card-surface`
- `adl-review runtime-surface`
- `adl-review verify-output-provenance`
- `adl-review verify-repo-contract`

The implementation intentionally delegates to the existing review tooling
modules. This issue proves command ownership and compatibility routing; it does
not move review internals into a separate crate.

## Compatibility

Existing commands remain available during migration:

- `adl tooling code-review`
- `adl tooling review-card-surface`
- `adl tooling review-runtime-surface`
- `adl tooling verify-review-output-provenance`
- `adl tooling verify-repo-review-contract`

`adl-review` rejects non-review command families:

- C-SDLC issue work remains under `adl/tools/pr.sh` and `adl-csdlc`.
- Runtime workflow execution remains under `adl-runtime`.

## Proof Surface

Focused proof for this boundary is:

- `bash adl/tools/test_adl_review_compatibility.sh`
- `cargo test --manifest-path adl/Cargo.toml review_dispatch -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml --test cli_smoke adl_review -- --nocapture`
- `cargo check --manifest-path adl/Cargo.toml --bin adl --bin adl-csdlc --bin adl-runtime --bin adl-review`

The expected behavior is compatibility, not feature expansion.
