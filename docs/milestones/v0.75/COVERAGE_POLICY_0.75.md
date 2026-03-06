# v0.75 Coverage Policy

## Purpose

Define deterministic, CI-enforced coverage requirements for v0.75.

This policy applies to the Rust runtime workspace under `swarm/`.

## Coverage Gates

- Workspace line coverage must be at least **90%**.
- Per-file line coverage for `swarm/src/**/*.rs` must be at least **80%**.

Both gates are enforced in CI via:

- `.github/workflows/ci.yaml`
- `swarm/tools/enforce_coverage_gates.sh`

## Local Reproduction

From repo root:

```bash
cd ./swarm/
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
cargo llvm-cov report --json --summary-only --output-path coverage-summary.json
bash tools/enforce_coverage_gates.sh coverage-summary.json
```

## Exclusions (Per-File Floor Only)

The following files are excluded from the
per-file 80% gate (see `EXCLUDE_FROM_FILE_FLOOR_REGEX` in
`.github/workflows/ci.yaml`):

- legacy CLI compatibility shim bin
- legacy remote compatibility shim bin
- `./swarm/src/obsmem_contract.rs`

Rationale:

- These are legacy compatibility shim binaries retained for transition/backward compatibility behavior.
- They are covered by integration/CLI smoke tests and remain inside the stricter workspace 90% aggregate gate.
- Excluding them from the per-file floor avoids forcing low-value test inflation in wrapper-only entrypoints.
- `./swarm/src/obsmem_contract.rs` is a deterministic contract/normalization boundary with many defensive validation branches already exercised indirectly by adapter/integration tests; this exclusion keeps the WP-14 gate stable while preserving strict workspace coverage enforcement.

## Determinism / Security Notes

- Coverage checks run locally in CI without external network requirements.
- Coverage parsing uses normalized, repository-relative file paths.
- Gating output does not include secrets, prompts, tool arguments, or absolute host-path leakage.
