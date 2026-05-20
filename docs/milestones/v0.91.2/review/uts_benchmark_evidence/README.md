# UTS Benchmark Evidence Artifacts

These artifacts preserve benchmark outputs gathered during the `#3121` UTS
benchmark-toolkit work.

## Review Status

The artifacts in this directory are historical evidence, not final publication
proof.

After these rows were gathered, review found that earlier scored prompts were
too assisted:

- earlier `UTS-only` prompts included task-specific answer examples
- earlier governed `UTS+ACC` prompts embedded proposal templates generated from
  the expected tool

Those prompt surfaces have since been tightened. Therefore, any row produced
before the tightened prompt contract should be treated as provisional or stale
until rerun with the current harness.

## Current Use

Use this directory to inspect what was run, how failures were classified, and
which rows motivated further work. Do not use these artifacts alone as external
benchmark claims.

Publication-grade evidence must come from a fresh run using:

- `adl/tools/run_uts_pack.sh`
- `adl/tools/benchmark/hosted_core_models.txt`
- `adl/tools/benchmark/remote_open_core_models.txt`
- `adl/tools/benchmark/uts_33_task_panel.json`
