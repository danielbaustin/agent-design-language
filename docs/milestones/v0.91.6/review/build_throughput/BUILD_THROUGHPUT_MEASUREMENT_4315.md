# Build Throughput Measurement Report for #4315

Status: `measured_local_baseline`
Issue: `#4315`
Sprint umbrella: `#4310`
Date: 2026-06-20

## Scope

This packet records one bounded local measurement pass for ADL build
throughput. It does not claim that `sccache`, linker changes, target-dir
relocation, cleanup policy, or remote validation have landed yet. Those remain
separate child issues in the `#4310` mini-sprint.

The goal here is narrower:

- establish a local cold/warm build baseline
- capture one adjacent-binary warm build observation
- capture focused proof-surface timing
- capture one owner-lane timing outcome
- identify obvious workspace hotspots without refactoring them

## Measurement Notes

- Measurements were run in the bound issue worktree for `#4315`.
- The local planning input named in the issue prompt,
  `.adl/docs/TBD/ADL_BUILD_IMPROVEMENTS.md`, was not present in this bound
  worktree. The measurement pass therefore relied on the tracked issue prompt,
  existing validation tooling, and tracked repository sources.
- `pr run` left unrelated tracked worktree residue in
  `docs/templates/prompts/current.json`. That file is not part of this issue's
  intended output and was not used as measurement evidence.

## Commands

```text
cargo clean --manifest-path adl/Cargo.toml
cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-doctor
cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-doctor
cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-ready

cargo test --quiet --manifest-path adl/Cargo.toml parse_doctor_args_accepts_modes_and_rejects_unknown_arg -- --exact
bash adl/tools/test_pr_run_ambiguity_policy.sh
bash adl/tools/test_control_plane_observability.sh
bash adl/tools/run_owner_validation_lane.sh csdlc

bash adl/tools/report_large_rust_modules.sh --format tsv
```

## Timing Observations

| Surface | Command shape | Result | Wall time | What it means |
|---|---|---:|---:|---|
| Cold single-binary build | `cargo build --bin adl-pr-doctor` after `cargo clean` | pass | `108.27s` | First local compile is still expensive even for one bounded owner binary. |
| Warm repeat build | same command, immediately repeated | pass | `0.77s` | Incremental repeat cost collapses once the binary is already built. |
| Warm adjacent binary build | `cargo build --bin adl-pr-ready` after `adl-pr-doctor` | pass | `20.33s` | Neighboring CLI binaries still pay noticeable compile cost even after a warm first build. |
| Focused shell proof | `bash adl/tools/test_pr_run_ambiguity_policy.sh` | pass | `0.20s` | Tight shell-level proof surfaces are effectively free relative to compile-heavy commands. |
| Focused observability proof | `bash adl/tools/test_control_plane_observability.sh` | fail | `0.06s` | The proof itself is cheap, but it currently fails before the owner lane can go green. |
| C-SDLC owner lane | `bash adl/tools/run_owner_validation_lane.sh csdlc` | fail | `4.15s` | Lane orchestration is not the main cost here; a fast failing proof blocks the lane. |
| Rust test filter attempt | `cargo test ... parse_doctor_args_accepts_modes_and_rejects_unknown_arg -- --exact` | weak / mis-targeted | `212.02s` | This selector compiled many harnesses, ran zero matching tests, and is not a trustworthy focused-test recipe yet. |

## Validation-Orchestration Finding

The C-SDLC owner lane is currently blocked by the observability contract proof.

Observed failing condition:

- `bash adl/tools/test_control_plane_observability.sh`
- failure reason: `ADL_OBSERVABILITY_STDERR=0` still wrote terminal output
- visible symptom: the test emitted an `adl_event` line to stderr during the
  stderr-suppressed path

This matters because the lane runtime itself is small. The bigger issue is that
validation trust for this lane is currently capped by a control-plane defect,
not by raw throughput.

## Focused-Test Finding

The first attempt to collect a focused Rust unit-test timing was not reliable.

Command:

```text
cargo test --quiet --manifest-path adl/Cargo.toml parse_doctor_args_accepts_modes_and_rejects_unknown_arg -- --exact
```

Observed result:

- it spent `212.02s` in aggregate compile/test-harness work
- it reported zero executed tests across many harnesses
- it is therefore evidence of poor selector discoverability, not evidence of a
  good focused-test lane

That is still useful to record: in the current workspace, an imprecise targeted
Rust test command can silently widen into a high-cost no-op.

## Workspace Hotspots

Top reported large Rust modules from
`bash adl/tools/report_large_rust_modules.sh --format tsv`:

| Path | LoC | Level |
|---|---:|---|
| `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs` | `5475` | `RATIONALE` |
| `adl/src/resilience.rs` | `5225` | `RATIONALE` |
| `adl/src/cli/pr_cmd/finish_support.rs` | `4017` | `RATIONALE` |
| `adl/src/csdlc_prompt_editor.rs` | `2927` | `RATIONALE` |
| `adl/src/cli/tests/pr_cmd_inline/basics.rs` | `2746` | `RATIONALE` |
| `adl/src/execute/tests.rs` | `1878` | `RATIONALE` |
| `adl/src/provider_adapter.rs` | `1832` | `RATIONALE` |
| `adl/src/long_lived_agent.rs` | `1703` | `RATIONALE` |
| `adl/src/cli/pr_cmd/github.rs` | `1630` | `RATIONALE` |
| `adl/src/provider_communication.rs` | `1595` | `RATIONALE` |

These do not prove direct rebuild causality by themselves, but they are strong
review candidates for rebuild amplification, test-harness size, and future
locality work.

## Conclusions

1. The cold-start cost is the dominant local pain point in this sample:
   `108.27s` for one clean owner-binary build.
2. Repeat local work can be fast once artifacts exist:
   immediate repeat was `0.77s`.
3. Warm neighboring binaries are still expensive enough to matter:
   `20.33s` for `adl-pr-ready` after `adl-pr-doctor`.
4. Focused shell proof surfaces are already cheap and should be preferred when
   they truly cover the changed contract.
5. Validation friction is not only about compile time:
   the current C-SDLC owner lane is blocked by an observability proof defect.
6. Focused Rust-test discoverability is currently weak enough that a plausible
   target command can spend minutes compiling and still execute zero tests.

## Recommendations For The Remaining Sprint

1. Prioritize `#4311`, `#4312`, and `#4313` as the best near-term levers for
   reducing the `108s` cold-start and `20s` warm-adjacent costs.
2. Treat the observability stderr-suppression failure as remediation input so
   owner-lane results are trustworthy again.
3. Improve or document canonical focused Rust-test selectors before using
   ad hoc `cargo test <name>` recipes as measurement or validation proof.
4. Use the hotspot list to route future locality/refactor candidates only after
   the lower-risk cache/linker/target-dir wins are measured.

## Non-Claims

- This report does not claim `sccache` is enabled.
- This report does not claim any linker acceleration is active.
- This report does not claim `CARGO_TARGET_DIR` relocation has been adopted.
- This report does not claim CI throughput changed.
- This report does not claim the owner lane is green.
