# C-SDLC Small Binaries Proof (`#3832`)

## Summary

`#3832` removes the structured-prompt validator path from recursive broad-binary
invocation and adds direct small-binary entrypoints for the most common
validator/editor surfaces:

- `adl-validate-structured-prompt`
- `adl-lint-prompt-spec`
- `adl-prompt-template`

`adl/tools/validate_structured_prompt.sh` and
`adl/tools/lint_prompt_spec.sh` now act as thin compatibility shims to those
direct binaries instead of routing through:

```text
adl -- tooling ...
```

## What Changed

### Direct validator binaries

- `adl-validate-structured-prompt` runs the structured-prompt validator
  directly.
- `adl-lint-prompt-spec` runs Prompt Spec lint directly.

Both binaries emit direct stderr observability lines for:

- command start
- command completed
- command failed

### Direct editor / renderer binary

- `adl-prompt-template` runs prompt-template render, edit, import, and schema
  validation operations directly.

It also emits direct stderr observability lines for:

- command start
- command completed
- command failed

### Compatibility shims

The following wrappers remain compatible but are no longer broad-binary
dispatchers:

- `adl/tools/validate_structured_prompt.sh`
- `adl/tools/lint_prompt_spec.sh`

Current shim behavior:

1. preserve legacy broad-delegate overrides from `ADL_TOOLING_RUST_BIN` /
   `ADL_PR_RUST_BIN` by invoking `tooling <subcommand>` on that binary
2. prefer an explicitly provided direct small-binary override for the specific
   validator/lint surface
3. prefer the built local debug binary under `adl/target/debug/`
4. otherwise run the direct binary with `cargo run --bin <small-binary>`

## Non-Claims

- This issue does **not** split the full PR lifecycle into many binaries.
- This issue does **not** remove `adl/tools/pr.sh`.
- This issue does **not** remove the compatibility `adl tooling` subcommands.
- This issue does **not** migrate every editor and transport surface yet.

## Migration Truth

### Direct owners now

- structured-prompt validation:
  - `adl-validate-structured-prompt`
- Prompt Spec lint:
  - `adl-lint-prompt-spec`
- prompt-template editing/rendering/import/validation:
  - `adl-prompt-template`

### Compatibility paths retained as shims

- `adl/tools/validate_structured_prompt.sh`
- `adl/tools/lint_prompt_spec.sh`
- `adl tooling validate-structured-prompt`
- `adl tooling lint-prompt-spec`
- `adl tooling prompt-template`

### Future splits still deferred

- remaining card editor binaries beyond prompt-template editor operations
- queue / PR lifecycle / GitHub transport decomposition
- any broader `pr.sh` lifecycle binary breakup

## Validation

Focused proofs that passed:

- `cargo test --manifest-path adl/Cargo.toml help_mentions_direct_ -- --nocapture`
- `bash adl/tools/test_structured_prompt_validation.sh`
- `bash adl/tools/test_prompt_template_workflow_integration.sh`
- `bash adl/tools/test_prompt_spec_lint.sh`
- `cargo build --manifest-path adl/Cargo.toml --bin adl-validate-structured-prompt --bin adl-lint-prompt-spec --bin adl-prompt-template`
- `git diff --check`

## Warm-Path Timing

Measured after building the direct binaries and then executing them directly on
one generated STP card:

- `adl-validate-structured-prompt`: `0.006748s`
- `adl-prompt-template validate-structure`: `0.007149s`

These measurements are direct built-binary runs, not `cargo run` startup times.
They establish the intended fast path for single-card docs-only validation.

## Compatibility-Shim Proof

The shell proof now covers both compatibility routes:

- direct small-binary override via `ADL_STRUCTURED_PROMPT_VALIDATOR_BIN`
- legacy broad-delegate override via `ADL_TOOLING_RUST_BIN`, proving the shim
  still dispatches `tooling validate-structured-prompt` /
  `tooling lint-prompt-spec` correctly when callers still provide the older
  delegate path
