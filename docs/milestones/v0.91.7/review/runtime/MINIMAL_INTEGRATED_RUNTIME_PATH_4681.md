# Minimal Integrated Runtime Path Proof for #4681

Status: `implemented_with_retained_runtime_v2_evidence`

Issue: `#4681`

## Scope

This packet records the v0.91.7 WP-07 minimal integrated runtime path.

The issue adds an in-product `runtime-v2` entrypoint that assembles the
existing D10 integrated CSM run substrate into one reproducible runtime path.
It emits retained local evidence under a stable issue-scoped artifact root and
records the negative cases that keep the proof from becoming an overclaim.

This work does not consume pending `#4842` runtime-v2 substrate changes.

## Implemented Surfaces

- `adl/src/runtime_v2/minimal_integrated_runtime_path.rs`
  - Adds the issue-bound summary contract
    `runtime_v2.minimal_integrated_runtime_path_summary.v1`.
  - Validates issue/milestone binding, repository-relative evidence refs,
    retained evidence inventory, negative-case inventory, runnable validation
    commands, and non-claims for `#4682`, `#4718`, `#4842`, and v0.92
    activation readiness.
  - Reuses `runtime_v2_csm_integrated_run_contract()` instead of forking the
    D10 substrate.

- `adl/src/cli/runtime_v2_cmd/commands.rs`
  - Adds `adl runtime-v2 minimal-integrated-runtime-path --out <dir>`.
  - Writes the integrated CSM bundle plus
    `issue_4681/minimal_integrated_runtime_path_summary.json`.
  - Emits the existing governed trace artifacts used by the D10 proof path.

- `adl/src/cli/runtime_v2_cmd/helpers.rs` and `adl/src/cli/usage.rs`
  - Adds command dispatch, help text, and path-hygiene behavior consistent with
    the existing runtime-v2 commands.

## Retained Evidence

The proof command writes this ignored local artifact root:

```text
artifacts/v0917/issue-4681-minimal-integrated-runtime-path
```

Primary retained refs inside that root:

- `issue_4681/minimal_integrated_runtime_path_summary.json`
- `runtime_v2/csm_run/integrated_first_run_proof_packet.json`
- `runtime_v2/csm_run/integrated_first_run_transcript.jsonl`
- `runtime_v2/observatory/visibility_packet.json`
- `runtime_v2/observatory/operator_report.md`
- `artifacts/runtime-v2-governed-demo-run/logs/activation_log.json`
- `artifacts/runtime-v2-governed-demo-run/governed/result.redacted.json`

The generated bundle contained 38 files during local proof capture.

The command output reported all ten integrated CSM stages as `PASS`:

```text
run_packet_loaded
boot_admission_validated
governed_episode_projected
freedom_gate_mediated
invalid_action_refused
wake_continuity_proved
observatory_rendered
recovery_quarantine_checked
hardening_probes_passed
integrated_proof_emitted
```

## Validation

Local validation run from the #4681 worktree:

```text
ADL_RUST_WARM_CACHE_SOURCE_TARGET=<primary-checkout>/adl/target ADL_RUST_WARM_CACHE_DEST_TARGET=<issue-worktree>/adl/target ADL_RUST_WARM_CACHE_MANIFEST_PATH=<issue-worktree>/adl/Cargo.toml bash adl/tools/rust_validation_warm_cache.sh
cargo fmt --manifest-path adl/Cargo.toml
cargo test --manifest-path adl/Cargo.toml trace_runtime_v2_minimal_integrated_runtime_path -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_minimal_integrated_runtime_path -- --nocapture
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 minimal-integrated-runtime-path --out artifacts/v0917/issue-4681-minimal-integrated-runtime-path
rg '<absolute-host-path-patterns>' artifacts/v0917/issue-4681-minimal-integrated-runtime-path
```

Result:

- cache warmup: `PASS` (`5863` files linked, `0` errors; acceleration only)
- formatting: `PASS`
- focused CLI tests: `PASS` (`2 passed`)
- focused runtime contract tests: `PASS` (`2 passed`)
- runnable runtime-v2 command: `PASS`
- absolute host path scan: `PASS` (`rg` returned no matches)

An earlier concurrent run of the two cargo filters produced one stale
unresolved-import compile failure while the sibling cargo process still held
build locks. The same filter passed when rerun by itself against the completed
target and is not treated as final proof.

## Negative Cases

Unit and CLI tests prove:

- absolute `--out` paths are rejected before evidence is written;
- parent traversal `--out` paths are rejected before evidence is written;
- summary validation rejects missing retained transcript evidence;
- summary validation rejects issue-binding drift away from `#4681`;
- summary validation rejects missing output-path negative-case truth;
- the underlying integrated CSM proof still rejects absolute artifact refs,
  missing hardening evidence, non-proving classifications, and
  birthday-readiness overclaims.

## Non-Claims

- This does not close Runtime Soak #2; `#4682` owns the broader soak run.
- This does not claim integrated logging or OTel readiness; `#4718` owns that
  proof.
- This does not depend on pending `#4842` runtime-v2 substrate changes.
- This does not claim v0.92 birthday activation readiness.
