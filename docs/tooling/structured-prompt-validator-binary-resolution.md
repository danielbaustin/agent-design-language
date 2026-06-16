# Structured Prompt Validator Binary Resolution

`adl/tools/validate_structured_prompt.sh` is a compatibility wrapper around the
dedicated `adl-validate-structured-prompt` binary. The wrapper is intentionally
boring: it should find an already-built validator in known repo-owned layouts,
or fail with an actionable diagnostic unless explicit cargo fallback is enabled.

## Supported lookup order

The wrapper resolves the ADL manifest root first, then tries validator binaries
in this order:

1. `ADL_STRUCTURED_PROMPT_VALIDATOR_BIN`
2. `CARGO_TARGET_DIR/debug/adl-validate-structured-prompt`
3. `CARGO_LLVM_COV_TARGET_DIR/debug/adl-validate-structured-prompt`
4. `<resolved-root>/adl/target/debug/adl-validate-structured-prompt`
5. `<primary-root>/adl/target/debug/adl-validate-structured-prompt`
6. `<resolved-root>/adl/target/llvm-cov-target/debug/adl-validate-structured-prompt`
7. `<primary-root>/adl/target/llvm-cov-target/debug/adl-validate-structured-prompt`
8. `adl-validate-structured-prompt` on `PATH`, unless disabled
9. legacy broad tooling binary fallback, only when explicit cargo fallback is
   enabled
10. `cargo run --bin adl-validate-structured-prompt`, only when explicit cargo
    fallback is enabled

For relative `CARGO_TARGET_DIR` or `CARGO_LLVM_COV_TARGET_DIR` values, the
wrapper checks the current directory, resolved repo root, and primary checkout
root forms. For absolute values, it checks the absolute target directory.

## Coverage-target contract

Coverage and fast-validation lanes may set `CARGO_TARGET_DIR` or
`CARGO_LLVM_COV_TARGET_DIR`. If the dedicated validator binary is present under
that target directory's `debug/` subdirectory, the wrapper must use it without
falling back to `PATH` or `cargo run`.

This is the supported non-default layout for coverage-style validation. The
wrapper does not claim arbitrary Cargo target layouts beyond the documented
target-dir forms and the fixed `target/llvm-cov-target` compatibility path.

## Failure behavior

When no dedicated validator binary is found and cargo fallback is not enabled,
the wrapper exits with status `75` and lists the supported lookup surfaces.

Enable cargo fallback only for explicit bootstrap or debugging:

```sh
ADL_STRUCTURED_PROMPT_VALIDATOR_ALLOW_CARGO_FALLBACK=1 \
  bash adl/tools/validate_structured_prompt.sh --type stp --input path/to/stp.md
```

Normal workflow lanes should build or expose the dedicated binary instead.

## Focused proof

The focused proof lives in:

```sh
bash adl/tools/test_validate_structured_prompt_parallel.sh
```

That proof covers:

- ordinary debug target lookup
- `CARGO_TARGET_DIR` lookup
- `CARGO_LLVM_COV_TARGET_DIR` lookup
- missing-binary diagnostics
- explicit cargo-fallback lock cleanup
