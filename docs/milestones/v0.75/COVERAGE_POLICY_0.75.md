# v0.75 Coverage Policy

## Purpose

Define deterministic, CI-enforced coverage requirements for v0.75.

This policy applies to the Rust runtime workspace under `swarm/`.

## Coverage Gates

- Workspace line coverage must be at least **90%**.
- Per-file line coverage for canonical runtime source under `swarm/src/**/*.rs`
  must be at least **80%**.
- Test code, fixtures, generated code, and other non-runtime paths are outside
  the per-file floor unless explicitly brought under policy later.

Both gates are enforced in CI via:

- `.github/workflows/ci.yaml`
- `swarm/tools/enforce_coverage_gates.sh`

## Local Reproduction

From repo root:

```bash
cd swarm/
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
cargo llvm-cov report --json --summary-only --output-path coverage-summary.json
bash tools/enforce_coverage_gates.sh coverage-summary.json
```

## Exclusions (Per-File Floor Only)

The per-file 80% gate excludes legacy compatibility shim binaries under
`swarm/src/bin/` and also excludes:

- `swarm/src/obsmem_contract.rs`

Rationale:

- Legacy compatibility shim bins are covered by integration/CLI smoke tests and
  remain inside the stricter workspace 90% aggregate gate.
- Excluding shim wrappers from per-file thresholds avoids low-value test
  inflation in transition entrypoints.
- `swarm/src/obsmem_contract.rs` is a deterministic contract/normalization
  boundary with many defensive validation branches exercised indirectly through
  adapter/integration tests.

## Determinism / Security Notes

- Coverage checks run locally in CI without external network requirements.
- Coverage parsing uses normalized, repository-relative file paths.
- Gating output does not include secrets, prompts, tool arguments, or absolute
  host-path leakage.
