# UTS Benchmark Evidence Artifacts

This directory holds review evidence and run instructions for the `#3121` UTS benchmark harness.

## Current proof path

The supported execution path is documented in:

- `docs/milestones/v0.91.2/review/uts_benchmark_evidence/RUNBOOK.md`

The only supported Python benchmark runner is:

- `adl/tools/uts_benchmark_runner.py`

`adl/tools/run_uts_pack.sh` is a convenience wrapper around that canonical runner. Lane-specific Python runner scripts are retired and must not be used as PR proof.

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
