# v0.75 Coverage Policy

## Purpose

Define deterministic, CI-enforced coverage requirements for v0.75.

This policy applies to the Rust runtime workspace under `./swarm/`.

## Coverage Gates

- Workspace line coverage must be at least **90%**.
- Per-file line coverage for canonical runtime source under `./swarm/src/**/*.rs`
  must be at least **80%**.
- Test code, fixtures, generated code, and other non-runtime paths are outside
  the per-file floor unless explicitly brought under policy later.

## Enforcement Surfaces

- `.github/workflows/ci.yaml`
- `.github/workflows/nightly-coverage-ratchet.yaml`
- `./swarm/tools/enforce_coverage_gates.sh`

## Local Reproduction

From repo root:

```bash
cd ./swarm/
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
cargo llvm-cov report --json --summary-only --output-path coverage-summary.json
bash tools/enforce_coverage_gates.sh coverage-summary.json
```

## Explicit Per-File Exclusions

The per-file floor excludes legacy compatibility shim binaries under
`./swarm/src/bin/` and also excludes:

- `./swarm/src/obsmem_contract.rs`

Rationale:

- Legacy compatibility shim bins are covered by integration/CLI smoke tests and
  remain inside the stricter workspace 90% aggregate gate.
- `obsmem_contract.rs` is a contract/validation boundary with many defensive
  branches exercised indirectly through adapter/integration tests.

## Determinism / Security Notes

- Coverage checks run locally in CI without external network requirements.
- Coverage parsing uses normalized, repository-relative file paths.
- Gating output must not include prompts, secrets, tool arguments, or absolute
  host paths.
