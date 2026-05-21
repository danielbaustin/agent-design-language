# Review-To-Test Plan

## Required focused tests

1. Rust governed benchmark rejects wrong expected arguments.
   Evidence target: `adl/src/uts_acc_multi_model_benchmark.rs`.
   Assertion: a valid UTS proposal with the correct tool but wrong task argument is not `ValidUsable` and does not pass.

2. Profile/panel validation fails when benchmark profiles reference absent model IDs.
   Evidence target: `adl/tools/benchmark/*profile.json`, `adl/tools/uts_benchmark_runner.py`.
   Assertion: self-check or profile validation reports all missing model IDs before provider calls.

3. Publication evidence gate fails on `provider_failed`, `skipped`, or `not_run` lanes when run mode is publication/release evidence.
   Evidence target: `adl/tools/uts_benchmark_runner.py`.

4. Fail-closed scoring distinguishes explicit refusal from UTS-valid/ACC-invalid proposal.
   Evidence target: `adl/src/uts_acc_multi_model_benchmark.rs`.

5. Benchmark artifact writer redacts raw model excerpts, provider errors, and external absolute paths.
   Evidence target: Python runner and Rust benchmark artifact writers.

6. Hosted key-file config must not include operator-local paths in tracked defaults.
   Evidence target: `adl/tools/benchmark/hosted_provider_key_files.json` or replacement template.

7. CI workflow pinning test or lint catches floating `actions/checkout@v4`.
