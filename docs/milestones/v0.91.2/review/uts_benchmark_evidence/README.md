# UTS Benchmark Evidence Artifacts

This directory holds review evidence and run instructions for the `#3121` UTS benchmark harness.

## Current proof path

The supported execution path is documented in:

- `docs/milestones/v0.91.2/review/uts_benchmark_evidence/RUNBOOK.md`

The only supported Python benchmark runner is:

- `adl/tools/uts_benchmark_runner.py`

`adl/tools/run_uts_benchmark.sh` is a convenience wrapper around that canonical runner. Lane-specific Python runner scripts are retired and must not be used as PR proof.
`adl/tools/benchmark/portable_benchmark_common.py` is internal import-only
support and must not be treated as a second runner or a supported CLI surface.

Current benchmark-validity guards:

- requested model IDs must exist in the canonical model panel before a run starts
- governed fail-closed tasks only pass on explicit refusal; a forbidden proposal rejected downstream by ACC is still a benchmark failure
- the runner exits nonzero when any required lane does not reach `evaluated`, so provider/lane failures cannot be mistaken for proof
- hosted-provider setup is environment-driven; the canonical key-file template no longer records operator-local absolute key paths
- hosted governed-lane adapter traffic is token-guarded on localhost instead of exposing an unauthenticated loopback bridge
- current durable benchmark artifacts store redacted response markers and redacted provider-failure summaries rather than raw hosted model output

## Historical artifacts

Older frozen benchmark outputs have been moved to:

- `docs/milestones/v0.91.2/review/uts_benchmark_evidence/historical/`

Those artifacts are useful for review history only. They are not current publication proof because they predate the tightened prompt contract, 33-fixture self-check, one-runner harness, local isolation guard, and current canonical model list.

## Required validation

Before treating benchmark output as evidence, run:

```bash
python3 adl/tools/benchmark/deterministic_self_check.py
```

Expected output includes:

```text
"passed": true
"fixture_count": 33
"task_count": 11
```

Fresh benchmark rows should be produced under `artifacts/uts_runs/` and must include JSON, summary, details, provider status, and self-check artifacts as described in the runbook.

## Hosted credential setup

Run the credential doctor before hosted benchmarks:

```bash
python3 adl/tools/uts_benchmark_runner.py --doctor-hosted-auth
```

Use either direct provider environment variables or provider-specific file-path
environment variables. The canonical config template is:

- `adl/tools/benchmark/hosted_provider_key_files.json`

Supported file-path environment variables are:

- `ADL_OPENAI_API_KEY_FILE`
- `ADL_GEMINI_API_KEY_FILE`
- `ADL_ANTHROPIC_API_KEY_FILE`

Supported direct environment variables are:

- `OPENAI_API_KEY`
- `GEMINI_API_KEY`
- `ANTHROPIC_API_KEY`

Do not commit machine-local credential paths into the template or benchmark
evidence docs.
